use std::io::{Read, Write};

use libc::{
    AF_INET, SOCK_DGRAM
};
use std::io;

use crate::address::EtherAddr;
use crate::platform::macos::sys::*;
use crate::platform::Sockaddr2Ipv4;
use crate::{
    dev::Device, 
    platform::posix::Fd, 
    configuration::Configuration
};
use crate::{Result, Error};
use crate::platform::posix::IntoSockAddr;
use std::ptr;

use super::sys::*;

pub struct Tap {
    fd: Fd,
    ctl: Fd,
    name: String,
    _config: Configuration,
}

impl Tap {
    pub fn set_nonblock(&mut self) -> io::Result<()> {
        self.fd.set_nonblock()
    }

    fn ifreq(&self) -> ifreq {
        unsafe {
            let mut req: ifreq = std::mem::zeroed();
            ptr::copy_nonoverlapping(
                self.name.as_ptr(), 
                req.ifr_name.as_mut_ptr() as _, 
                self.name.len()
            );
            req
        }
    }

    fn ifmtu(&self) -> ifmtu {
        unsafe {
            let mut req: ifmtu = std::mem::zeroed();
            ptr::copy_nonoverlapping(
                self.name.as_ptr(), 
                req.ifran.as_mut_ptr() as _, 
                self.name.len()
            );
            req
        }
    }
    
    pub fn new(_config: Configuration) -> Result<Self> {
        let (fd, idx) = Self::try_open()?;
        let fd = Fd::new(fd)
            .map_err(|_| Error::Io(io::Error::last_os_error()))?;
        unsafe {
            let ctl = Fd::new(libc::socket(AF_INET, SOCK_DGRAM, 0))
                    .map_err(|_| io::Error::last_os_error())?;
            let name = format!("tap{}", idx);
            let cfg = _config.clone();
            let mut tap = Self {
                fd,
                ctl,
                name,
                _config,
            };
            tap.configure(&cfg)?;
            Ok(tap)
        }
    }

    fn try_open() -> Result<(libc::c_int, i8)> {
        unsafe {
            let mut ret = 0;
            let mut idx = 0;
            for n in 0 ..= 15 {
                let path = format!("/dev/tap{}\0", n);
                ret = libc::open(path.as_ptr() as _, libc::O_RDWR|libc::O_NONBLOCK);
                idx = n;
                if ret > 0 {
                    break;
                }
            }
            Ok((ret, idx))
        }
    }
}

impl Device for Tap {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, _name: &str) -> Result<()> {
        Ok(())
    }

    fn enabled(&mut self, _value: bool) -> Result<()> {
        Ok(())
    }

    fn address(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        syscall!(siocgifaddr(*self.ctl, &mut req));
        unsafe {
            Ok((*req).ifr_ifru.ifru_addr.to_ipv4())
        }
    }

    fn set_address(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_addr = value.to_sockaddr();
        syscall!(siocsifaddr(*self.ctl, &req));
        Ok(())
    }

    fn destination(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        syscall!(siocgifdestaddr(*self.ctl, &mut req));
        unsafe {
            Ok((*req).ifr_ifru.ifru_dstaddr.to_ipv4())
        }
    }

    fn set_destination(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_dstaddr = value.to_sockaddr();
        syscall!(siocsifdestaddr(*self.ctl, &req));
        Ok(())
    }

    fn broadcast(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        syscall!(siocgifbrdaddr(*self.ctl, &mut req));
        unsafe {
            Ok((*req).ifr_ifru.ifru_broadaddr.to_ipv4())
        }
    }

    fn set_broadcast(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_broadaddr = value.to_sockaddr();
        syscall!(siocsifbrdaddr(*self.ctl, &req));
        Ok(())
    }

    fn netmask(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        syscall!(siocgifnetmask(*self.ctl, &mut req));
        unsafe {
            Ok((*req).ifr_ifru.ifru_addr.to_ipv4())
        }
    }

    fn set_netmask(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_addr = value.to_sockaddr();
        syscall!(siocsifnetmask(*self.ctl, &req));
        Ok(())
    }

    fn mtu(&self) -> Result<i32> {
        let mut imtu = self.ifmtu();
        unsafe {
            if siocifmut(*self.ctl, &mut imtu as *mut _) < 0 {
                return Err(Error::Io(std::io::Error::last_os_error()));
            }
        }
        let i = i32::from_ne_bytes(imtu.mtu);
        Ok(i)
    }

    fn set_mtu(&mut self, _value: i32) -> Result<()> {
        Ok(())
    }

    fn set_ether_address(&mut self, eth: EtherAddr) -> Result<()> {
        let mut req = self.ifreq();
        (*req).ifr_ifru.ifru_addr.sa_len = ETHER_ADDR_LEN as _;
        (*req).ifr_ifru.ifru_addr.sa_family = libc::AF_LINK as _;
        unsafe {
            ptr::copy_nonoverlapping(
                eth.as_ptr(), 
                (*req).ifr_ifru.ifru_addr.sa_data.as_mut_ptr() as _,
                ETHER_ADDR_LEN,
            );
            if siocsifaddr_eth(*self.ctl, &req) < 0 {
                Err(Error::Io(io::Error::last_os_error()))
            } else {
                Ok(())
            }
        }
    }

    fn fd(&self) -> &Fd {
        &self.fd
    }
}

impl Read for Tap {

    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.fd.read(buf)
    }
}

impl Write for Tap {

    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.fd.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.fd.flush()
    }
}




