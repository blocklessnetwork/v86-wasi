#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::{ffi::c_char, mem::MaybeUninit};
use std::{
    fs, io::Write, process::{Command, Stdio}
};
use std::mem;

use super::NatError;


const ETHTOOL_FWVERS_LEN: usize = 32; 
const ETHTOOL_BUSINFO_LEN: usize = 32; 
const ETHTOOL_EROMVERS_LEN: usize = 32; 
const SIOCETHTOOL: usize = 0x8946;
const ETHTOOL_GDRVINFO: usize = 0x00000003;
const ETH_GSTRING_LEN: usize = 32;
const ETHTOOL_GSTRINGS: usize = 32;
const ETH_SS_STATS: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
struct ethtool_gstrings {
    cmd: u32,
    string_set: u32,
    len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ethtool_drvinfo {
    cmd: u32,
    driver: [c_char; 32],
    version: [c_char; 32],
    fw_version: [c_char; ETHTOOL_FWVERS_LEN],
    bus_info: [c_char; ETHTOOL_BUSINFO_LEN],
    erom_version: [c_char; ETHTOOL_EROMVERS_LEN],
    reserved2: [c_char; 12],
    n_priv_flags: u32,
    n_stats: u32,
    testinfo_len: u32,
    eedump_len: u32,
    regdump_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
    pub ifr_ifrn: ifreq_ifrn,
    pub ifr_ifru: ifreq_ifru,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifreq_ifrn {
    pub ifrn_name: [i8; 16usize],
    align: [u8; 16usize],
}

type sockaddr = libc::sockaddr;

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifreq_ifru {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: ::std::os::raw::c_short,
    pub ifru_uflags: ::std::os::raw::c_ushort,
    pub ifru_ivalue: ::std::os::raw::c_int,
    pub ifru_mtu: ::std::os::raw::c_int,
    pub ifru_bool: ::std::os::raw::c_uchar,
    pub ifru_map: ifmap,
    pub ifru_slave: [::std::os::raw::c_char; 16usize],
    pub ifru_newname: [::std::os::raw::c_char; 16usize],
    pub ifru_data: *mut ::std::os::raw::c_char,
    align: [u64; 3usize],
}

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

pub(crate)fn forward(enable: bool) -> Result<(), NatError> {
    let mut ip_fwf = io_wrap!(fs::OpenOptions::new()
        .write(true)
        .open("/proc/sys/net/ipv4/ip_forward"));
    let enable = if enable { b"1" } else { b"0" };
    io_wrap!(ip_fwf.write(enable));
    Ok(())
}

fn eth_info(devn: &str) -> Result<(), NatError> {
    let sock: i32 = syscall!(libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0)) as _;
    let mut ifreq = MaybeUninit::<ifreq>::zeroed();
    let mut drv_info = MaybeUninit::<ethtool_drvinfo>::zeroed();
    unsafe {
        drv_info.assume_init_mut().cmd = ETHTOOL_GDRVINFO as _;
        let req = ifreq.assume_init_mut();
        req.ifr_ifru.ifru_data = drv_info.as_mut_ptr() as _;
        req.ifr_ifrn.ifrn_name.as_mut_ptr().copy_from(devn.as_ptr() as _, devn.len());
    }
    syscall!(libc::ioctl(sock, SIOCETHTOOL as _, ifreq.as_mut_ptr()));
    let n_stats = unsafe {
         drv_info.assume_init_ref().n_stats
    };
    if n_stats <= 0 {
        return Err(NatError::NoStatError)
    }
    let size: usize = mem::size_of::<ethtool_gstrings>() + n_stats as usize* ETH_GSTRING_LEN;
    let mut gstrings_data = vec![0u8; size] ;

    unsafe {
        let gstr = &mut *(gstrings_data.as_mut_ptr() as *mut ethtool_gstrings);
        gstr.cmd = ETHTOOL_GSTRINGS as _;
        gstr.string_set = ETH_SS_STATS as _;
        gstr.len = n_stats as _;
        let ifreq = ifreq.assume_init_mut();
        ifreq.ifr_ifru.ifru_data = gstrings_data.as_mut_ptr() as _;
    }
    syscall!(libc::ioctl(sock, SIOCETHTOOL as _, ifreq.as_mut_ptr()));
    let data = gstrings_data.as_ptr();
    Ok(())
}

pub(crate)fn iptable() -> Result<(), NatError> {
    let active_if = find_active_eth()?;
    let mut command = Command::new("iptables");
    command.args(&["-t", "nat", "-A", "POSTROUTING", "-j", "MASQUERADE", "-o", &active_if]);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    let child = io_wrap!(command.spawn());
    let output = io_wrap!(child.wait_with_output());
    if output.status.success() {
        Ok(())
    } else {
        Err(NatError::CommandError)
    }
}

fn find_active_eth() -> Result<String, NatError> {
    let net_path = std::path::Path::new("/sys/class/net");
    let read_dir = io_wrap!(fs::read_dir(&net_path));
    for entry in read_dir {
        let entry = io_wrap!(entry);
        let path = entry.path();
        if path.is_symlink() {
            let path = io_wrap!(fs::read_link(&path));
            let path_str = path.to_str();
            if let Some(s) = path_str {
                if s.contains("virtual") {
                    continue;
                }
                let split = s.split("/");
                if let Some(s) = split.last() {
                    return Ok(s.into());
                }
            }
        }
    }
    Err(NatError::NoInterfaceFound)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eth_info() {
        if let Ok(s) = find_active_eth() {
            let rs = eth_info(&s);
            assert!(rs.is_ok());
        }
    }
}