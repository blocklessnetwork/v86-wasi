use std::{io, os::fd::RawFd, ptr};

use crate::{dev::Device, Interest, Token};

use std::time::Duration;

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
                **tap.fd(), 
                &mut event
            ) < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    pub fn unregister(&mut self, tap: &impl Device) -> io::Result<()> {
        unsafe {
            if libc::epoll_ctl(
                self.fd, 
                libc::EPOLL_CTL_DEL, 
                **tap.fd(), 
                ptr::null_mut(),
            ) < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    pub fn poll(&mut self, events: &mut super::event::Events, t: Option<Duration>) -> io::Result<()> {
        let timeout = t
            .map(|to| {
                // `Duration::as_millis` truncates, so round up. This avoids
                // turning sub-millisecond timeouts into a zero timeout, unless
                // the caller explicitly requests that by specifying a zero
                // timeout.
                to.checked_add(Duration::from_nanos(999_999))
                    .unwrap_or(to)
                    .as_millis() as libc::c_int
            })
            .unwrap_or(-1);
        events.clear();
        unsafe {
            let n = libc::epoll_wait(
                self.fd,
                events.as_mut_ptr() as _,
                events.capacity() as i32,
                timeout,
            );
            if n < 0 {
                return Err(io::Error::last_os_error());
            }
            events.set_len(n as _);
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