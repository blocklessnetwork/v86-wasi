use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::Ipv4Addr;
use std::os::fd::AsRawFd;

use crate::{Configuration, Fd};
use crate::{Error, Result};
use crate::dev::Device;


use super::sys::*;


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
        let sock4 = unsafe { 
            libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP) 
        };
        if sock4 < 0 {
            return Err(Error::Io(io::Error::last_os_error()))
        }
        let name = config.name.as_ref().map_or(String::new(), |n| n.to_string());
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

    fn try_open() -> Result<File> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun");
        file.map_err(|e: io::Error| Error::Io(e))
    }

}

trait Sockaddr2Ipv4 {
    fn to_ipv4(&self) -> Ipv4Addr;
}

trait Ipv42Sockaddr {
    fn to_sockaddr(&self) -> libc::sockaddr;
} 

impl Sockaddr2Ipv4 for libc::sockaddr {
    fn to_ipv4(&self) -> Ipv4Addr {
        let sockaddr_in: libc::sockaddr_in = unsafe { std::mem::transmute(*self) };
        sockaddr_in.sin_addr.s_addr.to_le_bytes().into()
    }
}

impl Ipv42Sockaddr for Ipv4Addr {
    fn to_sockaddr(&self) -> libc::sockaddr {
        let mut sockaddr_in: libc::sockaddr_in = unsafe { std::mem::zeroed() };

        sockaddr_in.sin_family = libc::AF_INET as u16;
        sockaddr_in.sin_addr = libc::in_addr {
            s_addr: u32::from_le_bytes(self.octets()),
        };
        sockaddr_in.sin_port = 0;

        unsafe { std::mem::transmute(sockaddr_in) }
    }
}

impl Device for Tap {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) -> Result<()> {
        let mut req = ifreq::new(name);
        req.ifr_ifru.ifru_flags = super::sys::IFF_TAP as _;
        unsafe {
            if siocsifname(self.fd.0, &req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        self.name = req.name();
        Ok(())
    }

    fn enabled(&mut self, value: bool) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            if siocgifflags(self.fd.0, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
            if value {
                req.ifr_ifru.ifru_flags |= (IFF_UP|IFF_RUNNING) as i16;
            } else {
                req.ifr_ifru.ifru_flags &= !(IFF_UP|IFF_RUNNING) as i16;
            }
            if siocsifflags(self.sock4, &req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn address(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            if siocgifaddr(self.sock4, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
            Ok(req.ifr_ifru.ifru_addr.to_ipv4())
        }
    }

    fn set_address(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_addr = value.to_sockaddr();
        
        unsafe {
            if siocsifaddr(self.sock4, &req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn destination(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            if siocgifdestaddr(self.sock4, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
            Ok(req.ifr_ifru.ifru_dstaddr.to_ipv4())
        }
    }

    fn set_destination(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_dstaddr = value.to_sockaddr();
        unsafe {
            if siocsifdestaddr(self.sock4, &req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn broadcast(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            if siocgifbroadcast(self.sock4, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
            Ok(req.ifr_ifru.ifru_broadaddr.to_ipv4())
        }
    }

    fn set_broadcast(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_broadaddr = value.to_sockaddr();
        unsafe {
            if siocsifbroadcast(self.sock4, &req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn netmask(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            if siocgifnetmask(self.sock4, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
            Ok(req.ifr_ifru.ifru_netmask.to_ipv4())
        }
    }

    fn set_netmask(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_netmask = value.to_sockaddr();
        unsafe {
            if siocsifnetmask(self.sock4, &req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn mtu(&self) -> Result<i32> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            if siocgifmtu(self.sock4, &mut req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
            Ok(req.ifr_ifru.ifru_mtu)
        }
    }

    fn set_mtu(&mut self, value: i32) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_mtu = value;
        unsafe {
            if siocsifmtu(self.sock4, &req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(())
    }

    fn set_ether_address(&mut self, eth: crate::EtherAddr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            for (i, v) in eth.as_ref().iter().enumerate() {
                req.ifr_ifru.ifru_hwaddr.sa_data[i] = *v as _;
            }
            if  siocsifhwaddr(self.fd.0, &req) < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
        }
        Ok(())
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
        self.file.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
