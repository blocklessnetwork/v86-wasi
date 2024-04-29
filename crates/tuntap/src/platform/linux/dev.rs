use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::Ipv4Addr;
use std::os::fd::AsRawFd;
use crate::{Configuration, Fd};
use crate::{Error, Result};
use crate::dev::Device;

use super::sys::{ifreq, siocgifaddr, siocgifmtu, siocsifaddr, siocsifmtu, siocsifname};


pub struct Tap {
    fd: Fd,
    sock4: i32,
    file: File,
    name: String,
    config: Configuration,
}

impl Tap {


    pub fn new(config: Configuration) -> Result<Self> {
        let file = Self::try_open()?;
        let fd = Fd::new(file.as_raw_fd())
            .map_err(|_| Error::Io(io::Error::last_os_error()))?;
        unsafe {
            let sock4 = unsafe { 
                libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP) 
            };
            if sock4 < 0 {
                return Err(Error::Io(io::Error::last_os_error()))
            }
            let name = config.name.map_or(String::new(), |n| n.to_string());
            let cfg = config.clone();
            let mut tap = Self {
                fd,
                file,
                name,
                sock4,
                config,
            };
            tap.configure(&cfg)?;
            Ok(tap)
        }
    }

    fn try_open() -> Result<File> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun");
        file.map_err(|e: io::Error| Error::Io(e))
    }

}

impl Device for Tap {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) -> Result<()> {
        let req = ifreq::new(name);
        unsafe {
            siocsifname(self.fd.0, &req);
        }
        self.name = req.name();
        Ok(())
    }

    fn enabled(&mut self, value: bool) -> Result<()> {
        todo!()
    }

    fn address(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            if siocgifaddr(self.fd.0, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        let sa = req.ifr_ifru.ifru_addr.sa_data;
        let ipv4 = Ipv4Addr::new(sa[0] as _, sa[1] as _, sa[2] as _, sa[3] as _);
        Ok(ipv4)
    }

    fn set_address(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        let sa = req.ifr_ifru.ifru_addr.sa_data;
        req.ifr_ifru.ifru_addr.sa_family = libc::AF_INET as _;
        unsafe {
            if siocsifaddr(self.fd.0, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn destination(&self) -> Result<std::net::Ipv4Addr> {
        todo!()
    }

    fn set_destination(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        todo!()
    }

    fn broadcast(&self) -> Result<std::net::Ipv4Addr> {
        todo!()
    }

    fn set_broadcast(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        todo!()
    }

    fn netmask(&self) -> Result<std::net::Ipv4Addr> {
        todo!()
    }

    fn set_netmask(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        todo!()
    }

    fn mtu(&self) -> Result<i32> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            if siocgifmtu(self.fd.0, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(req.ifr_ifru.ifru_mtu)
    }

    fn set_mtu(&mut self, value: i32) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_mtu = value;
        unsafe {
            if siocsifmtu(self.fd.0, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn set_ether_address(&mut self, eth: crate::EtherAddr) -> Result<()> {
        todo!()
    }

    fn fd(&self) -> &Fd {
        &self.fd
    }
}

impl Read for Tap {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }
}

impl Write for Tap {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.flush()
    }
}

