use std::{mem::MaybeUninit, time::Duration};

use crate::dev::Device;

pub struct Select {
    nfds: i32,
    r_sets: MaybeUninit<libc::fd_set>,
}

impl Select {
    pub fn new() -> Select {
        let mut r_sets = MaybeUninit::uninit();
        unsafe {
            libc::FD_ZERO(r_sets.as_mut_ptr());
        }
        Select {
            nfds: 0,
            r_sets,
        }
    }

    pub fn register(&mut self, tap: &impl Device) {
        unsafe {
            if self.nfds <= tap.fd().0 + 1 {
                self.nfds = tap.fd().0 + 1
            }
            libc::FD_SET(tap.fd().0, self.r_sets.as_mut_ptr());
        }
    }

    pub fn unregister(&mut self, tap: &impl Device) {
        unsafe {
            libc::FD_CLR(tap.fd().0, self.r_sets.as_mut_ptr());
        }
    }

    pub fn poll(&mut self, t: Duration) -> i32 {
        let mut timeout: MaybeUninit<libc::timeval> = MaybeUninit::zeroed();
        unsafe {
            (&mut *timeout.as_mut_ptr()).tv_usec = (t.as_micros()%1000_000) as _;
            (&mut *timeout.as_mut_ptr()).tv_sec = (t.as_millis()/1000) as _;
        }
        unsafe {
            libc::select(
                self.nfds, 
                self.r_sets.as_mut_ptr(), 
                std::ptr::null_mut(), 
                std::ptr::null_mut(), 
                timeout.as_mut_ptr()
            )
        }
    }
}