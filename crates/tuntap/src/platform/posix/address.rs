use std::{net::Ipv4Addr, mem};

pub trait IntoSockAddr {
    fn to_sockaddr(&self) -> libc::sockaddr;
}

impl IntoSockAddr for Ipv4Addr {
    fn to_sockaddr(&self) -> libc::sockaddr {
        let mut addr  = unsafe {
            mem::zeroed::<libc::sockaddr>()
        };
        let addr_in = unsafe {
            &mut *(&mut addr as *mut libc::sockaddr as * mut libc::sockaddr_in)
        };
        addr_in.sin_addr.s_addr = u32::from_ne_bytes(self.octets());
        addr
    }
}