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
        addr_in.sin_family = libc::AF_INET as _;
        addr_in.sin_port = 0;
        addr
    }
}

trait Sockaddr2Ipv4 {
    fn to_ipv4(&self) -> Ipv4Addr;
}

impl Sockaddr2Ipv4 for libc::sockaddr {
    fn to_ipv4(&self) -> Ipv4Addr {
        let sockaddr_in: libc::sockaddr_in = unsafe { std::mem::transmute(*self) };
        sockaddr_in.sin_addr.s_addr.to_le_bytes().into()
    }
}