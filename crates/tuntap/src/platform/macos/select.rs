use std::{
    collections::HashMap, io, mem::MaybeUninit, ptr, time::Duration 
};

use crate::{
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
    pub fn register(&mut self, tap: &impl Device, token: Token, interest: Interest) -> io::Result<()> {
        let fd = tap.fd().0;
        unsafe {
            if self.nfds <= fd + 1 {
                self.nfds = fd + 1
            }
            if interest.is_readable() {
                libc::FD_SET(fd, self.r_sets.as_mut_ptr());
            }
            if interest.is_writeable() {
                libc::FD_SET(fd, self.w_sets.as_mut_ptr());
            }
        }
        self.tokens.insert(fd, token);
        Ok(())
    }

    /// unregistor the tap fd.
    pub fn unregister(&mut self, tap: &impl Device) -> io::Result<()> {
        let fd = tap.fd().0;
        self.tokens.remove(&fd);
        unsafe {
            libc::FD_CLR(fd, self.r_sets.as_mut_ptr());
            libc::FD_CLR(fd, self.w_sets.as_mut_ptr());
        }
        Ok(())
    }

    pub fn poll(&mut self, events: &mut Events, t: Option<Duration>) -> io::Result<()> {
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
        unsafe {
            let max_fd = libc::select(
                self.nfds,
                r_sets.as_mut_ptr(),
                w_sets.as_mut_ptr(),
                std::ptr::null_mut(), 
                timeout,
            );
            let r_ptr = r_sets.as_ptr();
            let w_ptr = w_sets.as_ptr();
            if max_fd < 0 {
                return Err(io::Error::last_os_error());
            }
            for n in 0..self.nfds {
                let r = libc::FD_ISSET(n, r_ptr);
                let w = libc::FD_ISSET(n, w_ptr);
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
        }
        Ok(())
    }
}