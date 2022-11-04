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
        let val = Parse::new(v).read_eth_addr().unwrap();
        EthAddr(val)
    }
    
}

struct Parse<'a>(&'a [u8]);

impl<'a> Parse<'a> {

    fn new(s: &'a str) -> Self {
        Self(s.as_bytes())
    }

    fn read_char(&mut self) -> Option<char> {
        self.0.split_first().map(|(f, tail)| {
            self.0 = tail;
            char::from(*f)
        })
    }

    fn read_atomically<T, F>(&mut self, inner: F) -> Option<T>
    where
        F: FnOnce(&mut Parse<'_>) -> Option<T>,
    {
        let state = self.0;
        let result = inner(self);
        if result.is_none() {
            self.0 = state;
        }
        result
    }

    fn read_num(&mut self) -> Option<u32> {
        const MAX_NUM: usize = 2;
        self.read_atomically(|p| {
            let mut maddr = 0u32;
            let mut num = 0;
            while let Some(a) = p.read_atomically(|p| p.read_char()?.to_digit(16)) {
                maddr = maddr.checked_mul(16)? + a;
                num += 1;
                if num > MAX_NUM {
                    return None;
                }
            }
            if num == 0 {
                return None;
            } else {
                return Some(maddr);
            }
        })
    }

    fn read_given_char(&mut self, target: char) -> Option<()> {
        self.read_atomically(|p| {
            p.read_char().and_then(|c| if c == target { Some(()) } else { None })
        })
    }

    fn read_separator<T, F>(&mut self, sep: char, index: usize, inner: F) -> Option<T>
    where
        F: FnOnce(&mut Parse<'_>) -> Option<T>,
    {
        self.read_atomically(move |p| {
            if index > 0 {
                p.read_given_char(sep)?;
            }
            inner(p)
        })
    }

    fn read_eth_addr(&mut self) -> Option<[u8; 6]> {
        self.read_atomically(|p| {
            let mut groups = [0; 6];

            for (i, slot) in groups.iter_mut().enumerate() {
                *slot = p.read_separator(':', i, |p| {
                    p.read_num().map(|i| i as u8)
                })?;
            }
            Some(groups)
        })
    }
}

mod test {
    use super::Parse;

    #[test]
    fn test() {
        let val  = Parse::new("01:02:03:04:5:ff").read_eth_addr();
        assert_eq!(val, Some([1,2,3,4,5,0xff]));

        let val  = Parse::new("f:02:03:04:5:fe").read_eth_addr();
        assert_eq!(val, Some([0xf,2,3,4,5,0xfe]));
    }
}