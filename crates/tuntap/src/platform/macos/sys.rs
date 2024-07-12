#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::ops::{Deref, DerefMut};

use ioctl::ioctl;
use libc::{c_char, sockaddr};

pub const IFNAMSIZ: usize = 16;

pub const ETHER_ADDR_LEN: usize = 6;

#[repr(transparent)]
pub struct ifreq(libc::ifreq);

impl ifreq {
    pub fn new(name: &str) -> Self {
        unsafe {
            let mut req: libc::ifreq = std::mem::zeroed();
            set_ifname!(req.ifr_name, name);
            Self(req)
        }
    }

    pub fn name(&self) -> String {
        let mut name = String::new();
        get_ifname!(self.0.ifr_name, name);
        name
    }
}

impl Deref for ifreq {
    type Target = libc::ifreq;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ifreq {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct ifmtu {
    pub ifran: [c_char; IFNAMSIZ],
    pub mtu: [u8;4],
    pub padding: [u8;12],
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct ifaliasreq {
    pub ifran: [c_char; IFNAMSIZ],
    pub addr: sockaddr,
    pub broadaddr: sockaddr,
    pub mask: sockaddr,
}

impl ifaliasreq {
    pub fn new(name: &str) -> Self {
        let mut req: ifaliasreq = unsafe { std::mem::zeroed() };
        set_ifname!(req.ifran, name);
        req
    }


    pub fn name(&self) -> String {
        let mut name = String::new();
        get_ifname!(self.ifran, name);
        name
    }
}

pub const IFF_UP: i32 = libc::IFF_UP;
pub const IFF_RUNNING: i32 = libc::IFF_RUNNING;

ioctl!(readwrite siocgifaddr with 'i', 33; ifreq);
ioctl!(write siocsifaddr with 'i', 12; ifreq);
ioctl!(write siocsifalias with 'i', 26; ifaliasreq);
ioctl!(write siocsifaddr_eth with 'i', 60; ifreq);
ioctl!(readwrite siocgifmut with 'i', 51; ifmtu);
ioctl!(write siocsifmut with 'i', 52; ifmtu);
ioctl!(readwrite siocgifdestaddr with 'i', 34; ifreq);
ioctl!(write siocsifdestaddr with 'i', 14; ifreq);

ioctl!(readwrite siocgifbrdaddr with 'i', 35; ifreq);
ioctl!(write siocsifbrdaddr with 'i', 19; ifreq);

ioctl!(readwrite siocgifnetmask with 'i', 37; ifreq);
ioctl!(write siocsifnetmask with 'i', 22; ifreq);


ioctl!(readwrite siocgifflags with 'i', 17; ifreq);
ioctl!(write siocsifflags with 'i', 16; ifreq);
