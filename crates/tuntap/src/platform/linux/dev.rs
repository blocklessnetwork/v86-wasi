use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::fd::AsRawFd;

use crate::{Configuration, Fd};
use crate::{Error, Result};
use crate::dev::Device;
use crate::platform::posix::{IntoSockAddr, Sockaddr2Ipv4};

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
        let sock4 = syscall! (libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP));
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

    fn model(&self) -> Model {
        self._config.model
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
    fn set_nonblock(&mut self) -> Result<()> {
        self.fd.set_nonblock().map_err(|e| Error::Io(e))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) -> Result<()> {
        let mut req = ifreq::new(name);
        req.ifr_ifru.ifru_flags = match self.model() {
            Model::Tap => super::sys::IFF_TAP,
            Model::Tun => super::sys::IFF_TUN,
        } as _;
        syscall!(siocsifname(*self.fd, &req));
        self.name = req.name();
        Ok(())
    }

    fn enabled(&mut self, value: bool) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        syscall!(siocgifflags(self.sock4, &mut req));
        unsafe {
            if value {
                req.ifr_ifru.ifru_flags |= (IFF_UP|IFF_RUNNING) as i16;
            } else {
                req.ifr_ifru.ifru_flags &= !(IFF_UP|IFF_RUNNING) as i16;
            }
        }
        syscall!(siocsifflags(self.sock4, &req));
        Ok(())
    }

    fn address(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        syscall!(siocgifaddr(self.sock4, &mut req));
        unsafe {
            Ok(req.ifr_ifru.ifru_addr.to_ipv4())
        }
    }

    fn set_address(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_addr = value.to_sockaddr();
        syscall!(siocsifaddr(self.sock4, &req));
        Ok(())
    }

    fn broadcast(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        syscall!(siocgifbroadcast(self.sock4, &mut req));
        unsafe {
            Ok(req.ifr_ifru.ifru_broadaddr.to_ipv4())
        }
    }

    fn set_broadcast(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_broadaddr = value.to_sockaddr();
        syscall!(siocsifbroadcast(self.sock4, &req));
        Ok(())
    }

    fn netmask(&self) -> Result<std::net::Ipv4Addr> {
        let mut req = ifreq::new(&self.name);
        syscall!(siocgifnetmask(self.sock4, &mut req));
        unsafe {
            Ok(req.ifr_ifru.ifru_netmask.to_ipv4())
        }
    }

    fn set_netmask(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_netmask = value.to_sockaddr();
        syscall!(siocsifnetmask(self.sock4, &req));
        Ok(())
    }

    fn mtu(&self) -> Result<i32> {
        let mut req = ifreq::new(&self.name);
        syscall!(siocgifmtu(self.sock4, &mut req));
        unsafe {
            Ok(req.ifr_ifru.ifru_mtu)
        }
    }

    fn set_mtu(&mut self, value: i32) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        req.ifr_ifru.ifru_mtu = value;
        syscall!(siocsifmtu(self.sock4, &req));
        Ok(())
    }

    fn set_ether_address(&mut self, eth: crate::EtherAddr) -> Result<()> {
        let mut req = ifreq::new(&self.name);
        unsafe {
            for (i, v) in eth.as_ref().iter().enumerate() {
                req.ifr_ifru.ifru_hwaddr.sa_data[i] = *v as _;
            }
        }
        syscall!(siocsifhwaddr(*self.fd, &req));
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
