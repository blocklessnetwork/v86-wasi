use std::net::Ipv4Addr;

use crate::address::EtherAddr;

use super::address::IntoAddress;


/// Configuration builder for a TUN interface.
#[derive(Clone, Default, Debug)]
pub struct Configuration {
    pub(crate) name: Option<String>,
    pub(crate) address: Option<Ipv4Addr>,
    pub(crate) ether_address: Option<EtherAddr>,
    pub(crate) broadcast: Option<Ipv4Addr>,
    pub(crate) netmask: Option<Ipv4Addr>,
    pub(crate) mtu: Option<i32>,
    pub(crate) enabled: bool,
}

impl Configuration {
    
    pub fn new() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }

    }

    /// Set the name.
    pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.as_ref().into());
        self
    }

    /// Set the address.
    pub fn address<A: IntoAddress>(&mut self, value: A) -> &mut Self {
        self.address = Some(value.into_address().unwrap());
        self
    }

    /// Set the broadcast address.
    pub fn broadcast<A: IntoAddress>(&mut self, value: A) -> &mut Self {
        self.broadcast = Some(value.into_address().unwrap());
        self
    }

    /// Set the netmask.
    pub fn netmask<A: IntoAddress>(&mut self, value: A) -> &mut Self {
        self.netmask = Some(value.into_address().unwrap());
        self
    }

    /// Set the MTU.
    pub fn mtu(&mut self, value: i32) -> &mut Self {
        self.mtu = Some(value);
        self
    }

    /// Set the interface to be enabled once created.
    pub fn ether_address(&mut self, addr: EtherAddr) -> &mut Self {
        self.ether_address = Some(addr);
        self
    }

    /// Set the interface to be enabled once created.
    pub fn up(&mut self) -> &mut Self {
        self.enabled = true;
        self
    }

    /// Set the interface to be disabled once created.
    pub fn down(&mut self) -> &mut Self {
        self.enabled = false;
        self
    }

    

}
