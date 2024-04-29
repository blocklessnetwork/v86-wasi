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
        if !name.is_empty() {
            let mut ifname: [i8; IFNAMSIZ as _] = [0; IFNAMSIZ as _];
            for (i, c) in name.as_bytes().iter().enumerate() {
                if i > ifname.len() - 1 {
                    break;
                }
                ifname[i] = *c as _;
            }
            req.ifr_ifrn.ifrn_name = ifname;
        }
        req
    }

    pub fn name(&self) -> String {
        let mut name = String::new();
        for i in 0..IFNAMSIZ as _ {
            let c = unsafe { self.ifr_ifrn.ifrn_name }[i] as u8 as char;
            if c != '\0' {
                name.push(c)
            }
        }
        name
    }
}


const IFF_UP: u16         = 1<<0;
const IFF_RUNNING: u16    = 1<<6;
const IFF_TUN: u16        = 0x0001;
const IFF_TAP: u16        = 0x0002;
const IFF_NO_PI: u16      = 0x0100;
const IFF_ONE_QUEUE: u16  = 0x0200;
const IFF_VNET_HDR: u16   = 0x0400;
const IFF_TUN_EXCL: u16   = 0x0800;



const TUNSETIFF: u64      = 0x400454ca;
const TUNSETOWNER: u64    = 0x400454cc;
const TUNSETGROUP: u64    = 0x400454ce;

const SIOCGIFFLAGS: u64   = 0x8913;
const SIOCSIFFLAGS: u64   = 0x8914;
const SIOCSIFADDR: u64    = 0x8916;
const SIOCGIFADDR: u64    = 0x8915;
const SIOCGIFMTU: u64     = 0x8921;
const SIOCSIFMTU: u64     = 0x8922;
const SIOCSIFNAME: u64    = 0x8923;
const SIOCSIFHWADDR: u64  = 0x8924;
const SIOCGIFHWADDR: u64  = 0x8927;
const SIOCGIFINDEX: u64   = 0x8933;
const SIOGIFINDEX: u64    = 0x8933; // same as SIOCGIFINDEX




ioctl!(bad write siocsifname with TUNSETIFF; ifreq);
ioctl!(bad write siocgifmtu with SIOCGIFMTU; ifreq);
ioctl!(bad read siocsifmtu with SIOCSIFMTU; ifreq);
ioctl!(bad write siocsifhwaddr with SIOCSIFHWADDR; ifreq);
ioctl!(bad read siocgifhwaddr with SIOCGIFHWADDR; ifreq);
ioctl!(bad write siocsifaddr with SIOCSIFADDR; ifreq);
ioctl!(bad read siocgifaddr with SIOCGIFADDR; ifreq);
