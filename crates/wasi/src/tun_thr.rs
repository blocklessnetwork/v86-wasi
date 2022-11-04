use std::{mem, io::{Read, Write}, time::{Duration, Instant}};
use crossbeam_channel::{Receiver, Sender, RecvTimeoutError};

use tuntap::{Tap, Configuration};
use wasmtime::Store;

#[allow(unused_imports)]
use crate::ContextTrait;
use crate::StoreT;

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
    vm_eth0_mac: Option<[u8; 6]>,
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
            vm_eth0_mac: None,
            address,
            netmask,
            mac,
            tx,
            rx,
        }
    }

    pub fn start(self) {
        let mut config = Configuration::default();
        config.address(&self.address)
            .netmask(&self.netmask)
            .eth_address(self.mac.into())
            .up();
        let tap = Tap::new(config);
        let mut tap = match tap {
            Ok(tap) => tap,
            Err(_) => {
                eprintln!("tap open with configure fail. please check config, \
                tab mod must be grant the root privileges, if you use macos \
                please install tuntap kext.");
                return;
            },
        };
        tap.set_nonblock().unwrap();
        loop {
            let mut buf = vec![0; 1024];
            let l = tap.read(&mut buf);
            if let Ok(l) = l {
                self.tx.send(buf[0..l].to_vec()).unwrap();
            }
            let now = Instant::now();
            match self.rx.recv_deadline(now + Duration::from_millis(10)) {
                Ok(buf) => tap.write(&buf).unwrap(),
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => break,
            };
        }
    }

    //arp reply
    #[allow(unused)]
    fn process_arp(&mut self, data: &Vec<u8>, tap: &mut Tap) {
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
            self.vm_eth0_mac = Some(s_eth_h.ether_shost);
            let end = std::mem::size_of::<EtherHdr>() + std::mem::size_of::<ArpHdr>();
            tap.write(&send_data[0..end]).unwrap();
            let _ = self.tx.send(send_data);
        }
    }
}