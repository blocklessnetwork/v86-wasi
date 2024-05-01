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
pub struct ifmtu {
    pub ifran: [c_char; IFNAMSIZ],
    pub mtu: [u8;4],
    pub padding: [u8;12],
}

#[repr(C)]
#[derive(Copy, Clone)]
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

ioctl!(readwrite siocgifaddr with 'i', 33; ifreq);
ioctl!(write siocsifaddr with 'i', 12; ifreq);
ioctl!(write siocsifalias with 'i', 26; ifaliasreq);
ioctl!(write siocsifaddr_eth with 'i', 60; ifreq);
ioctl!(readwrite siocifmut with 'i', 51; ifmtu);
ioctl!(readwrite siocgifdestaddr with 'i', 34; ifreq);
ioctl!(write siocsifdestaddr with 'i', 14; ifreq);

ioctl!(readwrite siocgifbrdaddr with 'i', 35; ifreq);
ioctl!(write siocsifbrdaddr with 'i', 19; ifreq);

ioctl!(readwrite siocgifnetmask with 'i', 37; ifreq);
ioctl!(write siocsifnetmask with 'i', 22; ifreq);
