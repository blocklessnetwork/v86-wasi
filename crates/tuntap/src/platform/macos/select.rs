use std::{
    collections::HashMap, io, mem::MaybeUninit, os::fd::RawFd, ptr, time::Duration 
};

use crate::{
    Error,
    Result,
    dev::Device, 
    token::Token,
    interest::Interest, 
};

use super::event::{
    Event, Events
};

pub struct Selector {
    nfds: i32,
    r_sets: MaybeUninit<libc::fd_set>,
    w_sets: MaybeUninit<libc::fd_set>,
    tokens: HashMap<i32, Token>,
}

impl Selector {
    pub fn new() -> Self {
        let r_sets = MaybeUninit::zeroed();
        let w_sets = MaybeUninit::zeroed();
        Self {
            nfds: 0,
            r_sets,
            w_sets,
            tokens: HashMap::new(),
        }
    }

    /// registor the tap fd in to select fd bit set.
    pub fn register(&mut self, tap: &impl Device, token: Token, interest: Interest) -> Result<()> {
        let fd: RawFd = **tap.fd();
        unsafe {
            if self.nfds <= fd + 1 {
                self.nfds = fd + 1
            }
            libc::FD_CLR(fd, self.r_sets.as_mut_ptr());
            libc::FD_CLR(fd, self.w_sets.as_mut_ptr());
            if interest.is_readable() {
                libc::FD_SET(fd, self.r_sets.as_mut_ptr());
            } 
            if interest.is_writable() {
                libc::FD_SET(fd, self.w_sets.as_mut_ptr());
            }
        }
        self.tokens.insert(fd, token);
        Ok(())
    }

    /// reregistor the tap fd in to select fd bit set.
    pub fn reregister(&mut self, tap: &impl Device, token: Token, interest: Interest) -> Result<()> {
        let fd: RawFd = **tap.fd();
        self.tokens.remove(&fd);
        unsafe {
            if self.nfds <= fd + 1 {
                self.nfds = fd + 1
            }
            libc::FD_CLR(fd, self.r_sets.as_mut_ptr());
            libc::FD_CLR(fd, self.w_sets.as_mut_ptr());
            if interest.is_readable() {
                libc::FD_SET(fd, self.r_sets.as_mut_ptr());
            } 
            if interest.is_writable() {
                libc::FD_SET(fd, self.w_sets.as_mut_ptr());
            }
        }
        self.tokens.insert(fd, token);
        Ok(())
    }

    /// unregistor the tap fd.
    pub fn unregister(&mut self, tap: &impl Device) -> Result<()> {
        let fd = **tap.fd();
        self.tokens.remove(&fd);
        unsafe {
            libc::FD_CLR(fd, self.r_sets.as_mut_ptr());
            libc::FD_CLR(fd, self.w_sets.as_mut_ptr());
        }
        Ok(())
    }

    pub fn poll(&mut self, events: &mut Events, t: Option<Duration>) -> Result<()> {
        let mut timeval: MaybeUninit<libc::timeval> = MaybeUninit::zeroed();
        let timeout = t.map_or(ptr::null_mut(), |t| {
            let mircos = t.as_micros();
            unsafe {
                (&mut *timeval.as_mut_ptr()).tv_usec = (mircos%1000_000) as _;
                (&mut *timeval.as_mut_ptr()).tv_sec = (mircos/1000_000) as _;
            }    
            timeval.as_mut_ptr()
        });
        events.clear();
        // inital the from r_sets, the select system call will change the context in r_sets.
        let mut r_sets = self.r_sets.clone();
        let mut w_sets = self.w_sets.clone();
        let _max_fd = syscall!(libc::select(
            self.nfds,
            r_sets.as_mut_ptr(),
            w_sets.as_mut_ptr(),
            std::ptr::null_mut(), 
            timeout,
        ));
        let r_ptr = r_sets.as_ptr();
        let w_ptr = w_sets.as_ptr();
        for n in 0..self.nfds {
            let (r, w) = unsafe {
                (libc::FD_ISSET(n, r_ptr), libc::FD_ISSET(n, w_ptr))
            };
            if r || w {
                let mut interest = if r {
                    Interest::READABLE
                } else {
                    Interest::WRITEABLE
                };
                if r {
                    interest = interest.add(Interest::READABLE);
                }
                if w {
                    interest = interest.add(Interest::WRITEABLE);
                }
                self.tokens.get(&n).map(|t| {
                    events.push(Event {
                        token: *t,
                        interest,
                    });
                });
            }
        }
        Ok(())
    }
}