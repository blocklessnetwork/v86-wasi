use ioctl::ioctl;
use libc::{c_char, sockaddr};

pub const IFNAMSIZ: usize = 16;

pub const ETH_ADDR_LEN: usize = 6;

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
pub struct ifaliasreq {
    pub ifran: [c_char; IFNAMSIZ],
    pub addr: sockaddr,
    pub broadaddr: sockaddr,
    pub mask: sockaddr,
}

ioctl!(write siocaifaddr with 'i', 0x1a; ifaliasreq);

ioctl!(write siocaifaddr_eth with 'i', 0x3c; ifreq);
