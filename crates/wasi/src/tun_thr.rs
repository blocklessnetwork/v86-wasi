use std::mem;

use futures_util::{SinkExt, StreamExt};
use tokio::{runtime::Builder, sync::{mpsc::{Receiver, Sender}}};


#[allow(unused_imports)]
use crate::ContextTrait;
use crate::LOG;
use tun::{self, TunPacket};

#[repr(C)]
#[derive(Debug)]
struct EtherHdr {
    ether_dhost: [u8;6],
    ether_shost: [u8;6],
    ether_type: u16,
}

#[repr(C)]
#[derive(Debug)]
struct ArpHdr {
    arp_hdr: u16,
    arp_pro: u16,
    arp_hln: u8,
    arp_pln: u8,
    arp_opt: u16,
    arp_sha: [u8;6],
    arp_spa: [u8;4],
    arp_tha: [u8;6],
    arp_tpa: [u8;4],
}

pub struct TunThread {
    address: String,
    netmask: String,
    vm_mac: Option<[u8; 6]>,
    mac: [u8; 6],
    rx: Receiver<Vec<u8>>,
    tx: Sender<Vec<u8>>,
}

const BORDCAST: [u8; 6] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
//This value is htons 0x806
const ARP_PROTO: u16 = 0x608;
const ARP_HDR: u16 = 256;
const ARP_OPT: u16 = 256;

#[inline]
fn htons(s: u16) -> u16 {
    s.to_be()
}

impl TunThread {
    pub fn new(
        address: String, 
        netmask: String,
        tx: Sender<Vec<u8>>,
        rx: Receiver<Vec<u8>>,
    ) -> Self {
        let mac: [u8; 6] = [0x00, 0x22, 0x15, rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()];
        Self {
            vm_mac: None,
            address,
            netmask,
            mac,
            tx,
            rx,
        }
    }

    pub fn start(mut self) {
        let runtime = Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();
        let mut config = tun::Configuration::default();
        
        config
            .address("192.168.0.1")
            // .netmask(&self.netmask)
            .layer(tun::Layer::L2)
            .up();

        #[cfg(target_os = "linux")]
        config.platform(|config| {
            config.packet_information(true);
        });
        
        runtime.block_on(async move {
            let dev = tun::create_as_async(&config).unwrap();
            let (mut sink, mut stream) = dev.into_framed().split();
            let tx = self.tx.clone();
            tokio::spawn(async move {
                while let Some(packet) = stream.next().await {
                    if let Ok(data) = packet {
                        if self.vm_mac.is_none() {
                            continue;
                        }
                        let d = data.get_bytes();
                        println!("recv:{}", d.len());
                        println!("recv:{:}", self.mac[0]);
                        
                        println!("{:X}:{:X}", d[0], d[1]);
                        if let Err(_) = tx.send(data.into_bytes().to_vec()).await {
                            return;
                        }
                    } else {
                        return;
                    }
                }
            });
            while let Some(data) = self.rx.recv().await {
                if data.len() < mem::size_of::<EtherHdr>() {
                    continue;
                }
                let eth_h = unsafe {
                    &*(data.as_ptr() as *const EtherHdr)
                };
                if eth_h.ether_dhost == BORDCAST {
                    match eth_h.ether_type {
                        //should be htons for eth_h.ether_type 0x806 
                        ARP_PROTO => {
                            self.process_arp(data).await;
                            continue;
                        }
                        _ => {}
                    };
                }
                
                if let Err(e) = sink.send(TunPacket::new(data)).await {
                    dbg_log!(LOG::NET, "send to tunnel error, {}", e);
                }
            }
        });
    }

    //arp reply
    async fn process_arp(&mut self, data: Vec<u8>) {
        let eth_h_len = mem::size_of::<EtherHdr>();
        let s_eth_h = unsafe {
            &*(data.as_ptr() as *const EtherHdr)
        };
        let s_arp_h = unsafe {
            &*(data.as_ptr().offset(eth_h_len as isize) as *const ArpHdr)
        };
        if s_arp_h.arp_hdr == ARP_HDR && s_arp_h.arp_opt == ARP_OPT {
            let mut send_data = data.clone();
            let d_arp_h = unsafe {
                &mut *(send_data.as_mut_ptr().offset(eth_h_len as isize) as *mut ArpHdr)
            };
            let d_eth_h = unsafe {
                &mut *(send_data.as_mut_ptr() as *mut EtherHdr)
            };
            d_arp_h.arp_opt = htons(0x2);
            d_arp_h.arp_tha = s_arp_h.arp_sha;
            d_arp_h.arp_tpa = s_arp_h.arp_spa;
            d_arp_h.arp_spa = s_arp_h.arp_tpa;
            d_arp_h.arp_sha = self.mac;

            d_eth_h.ether_dhost = s_eth_h.ether_shost;
            d_eth_h.ether_shost = s_eth_h.ether_dhost;
            self.vm_mac = Some(s_eth_h.ether_shost);
            let _ = self.tx.send(send_data).await;
        }
    }
}