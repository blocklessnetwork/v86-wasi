use ioctl::ioctl;
use libc::{c_char, sockaddr};

pub const IFNAMSIZ: usize = 16;

pub const ETHER_ADDR_LEN: usize = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifrn {
    pub name: [c_char; IFNAMSIZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifru {
    pub addr: sockaddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
    pub ifrn: ifrn,
    pub ifru: ifru, 
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

ioctl!(write siocsifaddr with 'i', 26; ifaliasreq);
ioctl!(write siocsifaddr_eth with 'i', 60; ifreq);
ioctl!(readwrite siocifmut with 'i', 51; ifmtu);
