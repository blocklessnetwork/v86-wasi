use std::{io, os::fd::RawFd, ptr};

use crate::{dev::Device, Interest, Token};


pub struct Epoll {
    fd: RawFd
}

impl Epoll {

    pub fn new() -> Self {
        let fd = unsafe {libc::epoll_create1(libc::EPOLL_CLOEXEC)};
        Self {
            fd
        }
    }
    
    pub fn register(&mut self, tap: &impl Device, token: Token, interest: Interest) -> io::Result<()> {
        let mut event = libc::epoll_event {
            events: interests_to_epoll(interest),
            u64: token.0 as u64,
        };
        unsafe {
            if libc::epoll_ctl(
                self.fd, 
                libc::EPOLL_CTL_ADD, 
                tap.fd().0, 
                &mut event
            ) < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    pub fn unregister(&mut self, tap: &impl Device, token: Token, interest: Interest) -> io::Result<()> {
        let mut event = libc::epoll_event {
            events: interests_to_epoll(interest),
            u64: token.0 as u64,
        };
        unsafe {
            if libc::epoll_ctl(
                self.fd, 
                libc::EPOLL_CTL_DEL, 
                tap.fd().0, 
                ptr::null_mut(),
            ) < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }
}

fn interests_to_epoll(interest: Interest) -> u32 {
    let mut kind = libc::EPOLLET;

    if interest.is_readable() {
        kind = kind | libc::EPOLLIN | libc::EPOLLRDHUP;
    }

    if interest.is_writable() {
        kind |= libc::EPOLLOUT;
    }

    kind as u32
}