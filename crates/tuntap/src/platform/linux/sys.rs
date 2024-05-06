#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::mem;
use ioctl::ioctl;


const IFNAMSIZ: usize = libc::IFNAMSIZ;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
    pub ifr_ifrn: ifreq_ifrn,
    pub ifr_ifru: ifreq_ifru,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifreq_ifrn {
    pub ifrn_name: [::std::os::raw::c_char; 16usize],
    align: [u8; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifreq_ifru {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: ::std::os::raw::c_short,
    pub ifru_ivalue: ::std::os::raw::c_int,
    pub ifru_mtu: ::std::os::raw::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [::std::os::raw::c_char; 16usize],
    pub ifru_newname: [::std::os::raw::c_char; 16usize],
    pub ifru_data: *mut ::std::os::raw::c_char,
    align: [u64; 3usize],
}

type sockaddr = libc::sockaddr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ifmap {
    pub mem_start: ::std::os::raw::c_ulong,
    pub mem_end: ::std::os::raw::c_ulong,
    pub base_addr: ::std::os::raw::c_ushort,
    pub irq: ::std::os::raw::c_uchar,
    pub dma: ::std::os::raw::c_uchar,
    pub port: ::std::os::raw::c_uchar,
}

impl ifreq {
    pub fn new(name: &str) -> Self {
        let mut req: ifreq = unsafe { mem::zeroed() };
        set_ifname!(req.ifr_ifrn.ifrn_name, name);
        req
    }

    pub fn name(&self) -> String {
        let mut name = String::new();
        unsafe {
            get_ifname!(self.ifr_ifrn.ifrn_name, name);
        }
        name
    }
}


pub const IFF_UP: u16         = libc::IFF_UP as _;
pub const IFF_RUNNING: u16    = libc::IFF_RUNNING as _;
pub const IFF_TUN: u16            = libc::IFF_TUN as _;
pub const IFF_TAP: u16        = libc::IFF_TAP as _;
const IFF_NO_PI: u16          = 0x0100;
const IFF_ONE_QUEUE: u16      = 0x0200;
const IFF_VNET_HDR: u16       = 0x0400;
const IFF_TUN_EXCL: u16       = 0x0800;



const TUNSETIFF: u64      = 0x400454ca;
const TUNSETOWNER: u64    = 0x400454cc;
const TUNSETGROUP: u64    = 0x400454ce;

const SIOCGIFFLAGS: u64   = libc::SIOCGIFFLAGS;
const SIOCSIFFLAGS: u64   = libc::SIOCSIFFLAGS;
const SIOCSIFADDR: u64    = libc::SIOCSIFADDR;
const SIOCGIFADDR: u64    = libc::SIOCGIFADDR;
const SIOCGIFMTU: u64     = libc::SIOCGIFMTU;
const SIOCSIFMTU: u64     = libc::SIOCSIFMTU;
const SIOCSIFNAME: u64    = libc::SIOCSIFNAME;
const SIOCSIFHWADDR: u64  = libc::SIOCSIFHWADDR;
const SIOCGIFHWADDR: u64  = libc::SIOCGIFHWADDR;
const SIOCGIFINDEX: u64   = libc::SIOCGIFINDEX;
const SIOGIFINDEX: u64    = libc::SIOGIFINDEX; // same as SIOCGIFINDEX
const SIOCSIFNETMASK: u64 = libc::SIOCSIFNETMASK;
const SIOCGIFNETMASK: u64 = libc::SIOCGIFNETMASK;
const SIOCGIFBRDADDR: u64 = libc::SIOCGIFBRDADDR;
const SIOCSIFBRDADDR: u64 = libc::SIOCSIFBRDADDR;
const SIOCGIFDSTADDR: u64 = libc::SIOCGIFDSTADDR;
const SIOCSIFDSTADDR: u64 = libc::SIOCSIFDSTADDR;



ioctl!(bad write siocsifname with TUNSETIFF; ifreq);
ioctl!(bad read siocgifmtu with SIOCGIFMTU; ifreq);
ioctl!(bad write siocsifmtu with SIOCSIFMTU; ifreq);
ioctl!(bad write siocsifhwaddr with SIOCSIFHWADDR; ifreq);
ioctl!(bad read siocgifhwaddr with SIOCGIFHWADDR; ifreq);
ioctl!(bad write siocsifaddr with SIOCSIFADDR; ifreq);
ioctl!(bad read siocgifaddr with SIOCGIFADDR; ifreq);
ioctl!(bad read siocgifnetmask with SIOCGIFNETMASK; ifreq);
ioctl!(bad write siocsifnetmask with SIOCSIFNETMASK; ifreq);

ioctl!(bad read siocgifflags with SIOCGIFFLAGS; ifreq);
ioctl!(bad write siocsifflags with SIOCSIFFLAGS; ifreq);

ioctl!(bad read siocgifbroadcast with SIOCGIFBRDADDR; ifreq);
ioctl!(bad write siocsifbroadcast with SIOCSIFBRDADDR; ifreq);

ioctl!(bad read siocgifdestaddr with SIOCGIFDSTADDR; ifreq);
ioctl!(bad write siocsifdestaddr with SIOCSIFDSTADDR; ifreq);
