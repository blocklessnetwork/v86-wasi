use std::io::{Read, Write};

use libc::{AF_INET, SOCK_DGRAM, sockaddr};
use std::io;

use crate::address::EtherAddr;
use crate::{platform::posix::Fd, dev::Device, configuration::Configuration};
use crate::{Result, Error};
use std::ptr;
use crate::platform::posix::IntoSockAddr;

use super::sys::{
    ifreq, 
    siocsifaddr_eth, 
    ETHER_ADDR_LEN, 
    ifaliasreq, 
    ifmtu, 
    siocifmut
};

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

    fn ifaliasreq(&self) -> ifaliasreq {
        unsafe {
            let mut req: ifaliasreq = std::mem::zeroed();
            ptr::copy_nonoverlapping(
                self.name.as_ptr(), 
                req.ifran.as_mut_ptr() as _, 
                self.name.len()
            );
            req
        }
    }

    fn ifreq(&self) -> ifreq {
        unsafe {
            let mut req: ifreq = std::mem::zeroed();
            ptr::copy_nonoverlapping(
                self.name.as_ptr(), 
                req.ifrn.name.as_mut_ptr() as _, 
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
            let mut rs = 0;
            let mut idx = 0;
            for n in 0 ..= 15 {
                let path = format!("/dev/tap{}\0", n);
                rs = libc::open(path.as_ptr() as _, libc::O_RDWR|libc::O_NONBLOCK);
                idx = n;
                if rs > 0 {
                    break;
                }
            }
            Ok((rs, idx))
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
        Err(Error::NotImplemented)
    }

    fn set_address(&mut self, _value: std::net::Ipv4Addr) -> Result<()> {
        Ok(())
    }

    fn destination(&self) -> Result<std::net::Ipv4Addr> {
        Err(Error::NotImplemented)
    }

    fn set_destination(&mut self, _value: std::net::Ipv4Addr) -> Result<()> {
        Err(Error::NotImplemented)
    }

    fn broadcast(&self) -> Result<std::net::Ipv4Addr> {
        Err(Error::NotImplemented)
    }

    fn set_broadcast(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = self.ifaliasreq();
        req.broadaddr = value.to_addr();
        req.broadaddr.sa_family = libc::AF_INET as _;
        req.broadaddr.sa_len = std::mem::size_of::<sockaddr>() as _;
        let rs = unsafe {
            super::sys::siocsifaddr(self.ctl.0, &req)
        };
        if rs < 0 {
            Err(Error::Io(io::Error::last_os_error()))
        }  else {
            Ok(())
        }
    }

    fn netmask(&self) -> Result<std::net::Ipv4Addr> {
        Err(Error::NotImplemented)
    }

    fn set_netmask(&mut self, _value: std::net::Ipv4Addr) -> Result<()> {
        Ok(())
    }

    fn mtu(&self) -> Result<i32> {
        let mut imtu = self.ifmtu();
        unsafe {
            if siocifmut(self.ctl.0, &mut imtu as *mut _) < 0 {
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
        req.ifru.addr.sa_len = ETHER_ADDR_LEN as _;
        req.ifru.addr.sa_family = libc::AF_LINK as _;
        unsafe {
            ptr::copy_nonoverlapping(
                eth.as_ptr(), 
                req.ifru.addr.sa_data.as_mut_ptr() as _,
                ETHER_ADDR_LEN,
            );
            if siocsifaddr_eth(self.ctl.0, &req) < 0 {
                Err(Error::Io(io::Error::last_os_error()))
            } else {
                Ok(())
            }
        }
    }

    fn configure(&mut self, config: &Configuration) -> Result<()> {
        
        let mut req = self.ifaliasreq();
        if let Some(ip) = config.address {
            req.addr = ip.to_addr();
            req.addr.sa_family = libc::AF_INET as _;
            req.addr.sa_len = std::mem::size_of::<sockaddr>() as _;

            config.netmask.map(|ip| {
                req.mask = ip.to_addr();
                req.mask.sa_family = libc::AF_INET as _;
                req.mask.sa_len = std::mem::size_of::<sockaddr>() as _;
            });

            config.broadcast.map(|ip| {
                req.mask = ip.to_addr();
                req.mask.sa_family = libc::AF_INET as _;
                req.mask.sa_len = std::mem::size_of::<sockaddr>() as _;
            });
            let rs = unsafe {
                super::sys::siocsifaddr(self.ctl.0, &req)
            };
            if rs < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }

        if let Some(mtu) = config.mtu {
            self.set_mtu(mtu)?;
        }

        if let Some(eth) = config.ether_address {
            self.set_ether_address(eth)?;
        }

        if let Some(enabled) = config.enabled {
            self.enabled(enabled)?;
        }

        Ok(())
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




