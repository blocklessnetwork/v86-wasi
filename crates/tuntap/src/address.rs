use std::net::{IpAddr, Ipv4Addr};
use std::net::{SocketAddr, SocketAddrV4};
use crate::{Result, Error};

/// Helper trait to convert things into IPv4 addresses.
#[allow(clippy::wrong_self_convention)]
pub trait IntoAddress {
    /// Convert the type to an `Ipv4Addr`.
    fn into_address(&self) -> Result<Ipv4Addr>;
}

impl IntoAddress for u32 {
    fn into_address(&self) -> Result<Ipv4Addr> {
        Ok(Ipv4Addr::new(
            ((*self) & 0xff) as u8,
            ((*self >> 8) & 0xff) as u8,
            ((*self >> 16) & 0xff) as u8,
            ((*self >> 24) & 0xff) as u8,
        ))
    }
}

impl IntoAddress for i32 {
    fn into_address(&self) -> Result<Ipv4Addr> {
        (*self as u32).into_address()
    }
}

impl IntoAddress for (u8, u8, u8, u8) {
    fn into_address(&self) -> Result<Ipv4Addr> {
        Ok(Ipv4Addr::new(self.0, self.1, self.2, self.3))
    }
}

impl IntoAddress for str {
    fn into_address(&self) -> Result<Ipv4Addr> {
        self.parse().map_err(|_| Error::InvalidAddress)
    }
}

impl<'a> IntoAddress for &'a str {
    fn into_address(&self) -> Result<Ipv4Addr> {
        (*self).into_address()
    }
}

impl IntoAddress for String {
    fn into_address(&self) -> Result<Ipv4Addr> {
        (&**self).into_address()
    }
}

impl<'a> IntoAddress for &'a String {
    fn into_address(&self) -> Result<Ipv4Addr> {
        (&**self).into_address()
    }
}

impl IntoAddress for Ipv4Addr {
    fn into_address(&self) -> Result<Ipv4Addr> {
        Ok(*self)
    }
}

impl<'a> IntoAddress for &'a Ipv4Addr {
    fn into_address(&self) -> Result<Ipv4Addr> {
        (&**self).into_address()
    }
}

impl IntoAddress for IpAddr {
    fn into_address(&self) -> Result<Ipv4Addr> {
        match *self {
            IpAddr::V4(ref value) => Ok(*value),

            IpAddr::V6(_) => Err(Error::InvalidAddress),
        }
    }
}

impl<'a> IntoAddress for &'a IpAddr {
    fn into_address(&self) -> Result<Ipv4Addr> {
        (&**self).into_address()
    }
}

impl IntoAddress for SocketAddrV4 {
    fn into_address(&self) -> Result<Ipv4Addr> {
        Ok(*self.ip())
    }
}

impl<'a> IntoAddress for &'a SocketAddrV4 {
    fn into_address(&self) -> Result<Ipv4Addr> {
        (&**self).into_address()
    }
}

impl IntoAddress for SocketAddr {
    fn into_address(&self) -> Result<Ipv4Addr> {
        match *self {
            SocketAddr::V4(ref value) => Ok(*value.ip()),

            SocketAddr::V6(_) => Err(Error::InvalidAddress),
        }
    }
}

impl<'a> IntoAddress for &'a SocketAddr {
    fn into_address(&self) -> Result<Ipv4Addr> {
        (&**self).into_address()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EthAddr([u8; 6]);

impl EthAddr {

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

impl AsRef<[u8]> for EthAddr {

    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }

}

impl From<[u8; 6]> for EthAddr {

    fn from(v: [u8; 6]) -> Self {
        EthAddr(v)
    }

}

impl From<&str> for EthAddr {

    fn from(v: &str) -> Self {
        "192".parse::<Ip4Addr>;
        
    }
    
}