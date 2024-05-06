#![allow(dead_code)]
use std::{
    mem, io::{Read, Write}, time::Duration
};
use crossbeam_channel::{
    Receiver, Sender, TryRecvError, TrySendError
};

use tuntap::{
    Tap,
    Device, 
    Token,
    Events,
    Interest,
    EtherAddr,
    Configuration, 
};

#[allow(unused_imports)]
use crate::ContextTrait;

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
    ether_addr: EtherAddr,
    vm_eth0_mac: Option<[u8; 6]>,
    vm_channel_rx: Receiver<Vec<u8>>,
    vm_channel_tx: Sender<Vec<u8>>,
}
#[allow(dead_code)]
const BORDCAST: [u8; 6] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
//This value is htons 0x806
#[allow(dead_code)]
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
        ether_addr: Option<String>,
        vm_channel_tx: Sender<Vec<u8>>,
        vm_channel_rx: Receiver<Vec<u8>>,
    ) -> Self {
        let ether_addr: EtherAddr = ether_addr
            .as_ref()
            .map(|addr| addr.as_str().into())
            .unwrap_or(
                [0x00, 0x22, 0x15, rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()].into()
            );
        Self {
            vm_eth0_mac: None,
            address,
            netmask,
            ether_addr,
            vm_channel_tx,
            vm_channel_rx,
        }
    }

    pub fn start(self) {
        let mut config = Configuration::default();
        config.address(&self.address)
            .netmask(&self.netmask)
            .ether_address(self.ether_addr)
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
        let mut tap_poll = tuntap::Poll::new();
        let mut events = Events::with_capacity(10);
        let tap_token = Token(0);
        
        // the ethernet frame size is 1514. so change to 2048 is sufficient.
        let mut buf = [0; 2048];
        let mut try_sent_buf: Option<Vec<u8>> = None;

        macro_rules! try_channel_send {
            ($sent: ident) => {
                try_sent_buf = match self.vm_channel_tx.try_send($sent) {
                    Err(TrySendError::Full(b)) => {
                        Some(b)
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        break;
                    },
                    _ => None
                }
            }
        }
        macro_rules! recv_from_vm_channel {
            () => {
                match self.vm_channel_rx.try_recv() {
                    Ok(buf) => Some(buf),
                    Err(TryRecvError::Empty) => None,
                    Err(e) => {
                        eprintln!("recv from vm error:{}", e);
                        break;
                    }
                }
            }
        }
        let mut tap_sent_handle = None;
        let mut interests = Interest::READABLE;
        tap_poll.register(&tap, tap_token, interests).unwrap();
        loop {
            tap_poll.reregister(&tap, tap_token, interests).unwrap();
            let rs = tap_poll.poll(&mut events, Some(Duration::from_millis(1)));
            let mut event = None;
            if let Ok(_) = rs {
                for e in &events {
                    if e.token() == tap_token {
                        event = Some(e);
                        break;
                    }
                };
            }
            
            let writeable = if event.is_some() {
                let (readable, writeable) = 
                    event.map_or((false, false), 
                        |e| (e.is_readable(), e.is_writable())
                    );
                if try_sent_buf.is_some() {
                    let sent = try_sent_buf.take().unwrap();
                    try_channel_send!(sent);
                } else {
                    if readable {
                        let l = tap.read(&mut buf);
                        if let Ok(l) = l {
                            let sent = buf[0..l].to_vec();
                            try_channel_send!(sent);
                        } else {
                            break;
                        }
                    }
                }
                writeable
            } else {
                false
            };

            // no data sent to tap, recv it from vm
            if tap_sent_handle.is_none() {
                tap_sent_handle = recv_from_vm_channel!();
            }

            // if the tap is writeable, write the data tap which from vm
            // write success reset the handle.
            if writeable && tap_sent_handle.is_some() {
                let mut max_send_limited: u8 = 16;
                while max_send_limited > 0 {
                    match tap_sent_handle {
                        Some(ref b) => tap.write(b).unwrap(),
                        None => break,
                    };
                    tap_sent_handle = recv_from_vm_channel!();
                    max_send_limited -= 1;
                } 
                tap_sent_handle = None;
            }

            if tap_sent_handle.is_some() {
                interests = Interest::RDWR;
            } else {
                interests = Interest::READABLE;
            }

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
            d_arp_h.arp_sha = self.ether_addr.into();

            d_eth_h.ether_dhost = s_eth_h.ether_shost;
            d_eth_h.ether_shost = s_eth_h.ether_dhost;
            self.vm_eth0_mac = Some(s_eth_h.ether_shost);
            let end = std::mem::size_of::<EtherHdr>() + std::mem::size_of::<ArpHdr>();
            tap.write(&send_data[0..end]).unwrap();
            let _ = self.vm_channel_tx.send(send_data);
        }
    }
}