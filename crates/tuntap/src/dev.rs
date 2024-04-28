use std::io::{Write, Read};
use std::net::Ipv4Addr;

use crate::address::EtherAddr;
use crate::configuration::Configuration;
use crate::{platform, Fd, Result};


pub trait Device: Read + Write {
    /// Reconfigure the device.
    fn configure(&mut self, config: &Configuration) -> Result<()> {
        if let Some(ip) = config.address {
            self.set_address(ip)?;
        }

        if let Some(ip) = config.destination {
            self.set_destination(ip)?;
        }

        if let Some(ip) = config.broadcast {
            self.set_broadcast(ip)?;
        }

        if let Some(ip) = config.netmask {
            self.set_netmask(ip)?;
        }

        if let Some(mtu) = config.mtu {
            self.set_mtu(mtu)?;
        }

        if let Some(enabled) = config.enabled {
            self.enabled(enabled)?;
        }

        Ok(())
    }

    /// Get the device name.
    fn name(&self) -> &str;

    /// Set the device name.
    fn set_name(&mut self, name: &str) -> Result<()>;

    /// Turn on or off the interface.
    fn enabled(&mut self, value: bool) -> Result<()>;

    /// Get the address.
    fn address(&self) -> Result<Ipv4Addr>;

    /// Set the address.
    fn set_address(&mut self, value: Ipv4Addr) -> Result<()>;

    /// Get the destination address.
    fn destination(&self) -> Result<Ipv4Addr>;

    /// Set the destination address.
    fn set_destination(&mut self, value: Ipv4Addr) -> Result<()>;

    /// Get the broadcast address.
    fn broadcast(&self) -> Result<Ipv4Addr>;

    /// Set the broadcast address.
    fn set_broadcast(&mut self, value: Ipv4Addr) -> Result<()>;

    /// Get the netmask.
    fn netmask(&self) -> Result<Ipv4Addr>;

    /// Set the netmask.
    fn set_netmask(&mut self, value: Ipv4Addr) -> Result<()>;

    /// Get the MTU.
    fn mtu(&self) -> Result<i32>;

    /// Set the MTU.
    fn set_mtu(&mut self, value: i32) -> Result<()>;

    /// Set the ether address.
    fn set_ether_address(&mut self, eth: EtherAddr) -> Result<()>;

    fn fd(&self) -> &Fd; 
}

#[repr(transparent)]
pub struct Tap {
    inner: platform::Tap
}

impl Device for Tap {

    #[inline(always)]
    fn name(&self) -> &str {
        self.inner.name()
    }

    #[inline(always)]
    fn set_name(&mut self, name: &str) -> Result<()> {
        self.inner.set_name(name)
    }

    #[inline(always)]
    fn enabled(&mut self, value: bool) -> Result<()> {
        self.inner.enabled(value)
    }

    #[inline(always)]
    fn address(&self) -> Result<Ipv4Addr> {
        self.inner.address()
    }

    #[inline(always)]
    fn set_address(&mut self, value: Ipv4Addr) -> Result<()> {
        self.inner.set_address(value)
    }

    #[inline(always)]
    fn destination(&self) -> Result<Ipv4Addr> {
        self.inner.destination()
    }

    #[inline(always)]
    fn set_destination(&mut self, value: Ipv4Addr) -> Result<()> {
        self.inner.set_destination(value)
    }

    #[inline(always)]
    fn broadcast(&self) -> Result<Ipv4Addr> {
        self.inner.broadcast()
    }

    #[inline(always)]
    fn set_broadcast(&mut self, value: Ipv4Addr) -> Result<()> {
        self.inner.set_broadcast(value)
    }

    #[inline(always)]
    fn netmask(&self) -> Result<Ipv4Addr> {
        self.inner.netmask()
    }

    #[inline(always)]
    fn set_netmask(&mut self, value: Ipv4Addr) -> Result<()> {
        self.inner.set_netmask(value)
    }

    #[inline(always)]
    fn mtu(&self) -> Result<i32> {
        self.inner.mtu()
    }

    #[inline(always)]
    fn set_mtu(&mut self, value: i32) -> Result<()> {
        self.inner.set_mtu(value)
    }

    #[inline(always)]
    fn set_ether_address(&mut self, eth: EtherAddr) -> Result<()> {
        self.inner.set_ether_address(eth)
    }

    #[inline(always)]
    fn fd(&self) -> &Fd {
        self.inner.fd()
    }
}

impl Read for Tap {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for Tap {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
