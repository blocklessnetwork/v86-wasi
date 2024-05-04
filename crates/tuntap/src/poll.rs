use std::{io, time::Duration};

use crate::{
    Events,
    platform, 
    dev::Device, 
    token::Token, 
    interest::Interest, 
};


pub struct Poll {
    #[cfg(target_os="macos")]
    inner: platform::Selector,
    #[cfg(target_os="linux")]
    inner: platform::Epoll,
}

impl Poll {
    pub fn new() -> Self {
        #[cfg(target_os="macos")]
        let inner = platform::Selector::new();
        #[cfg(target_os="linux")]
        let inner =  platform::Epoll::new();
        Self {
            inner
        }
    }

    pub fn register(&mut self, tap: &impl Device, token: Token, interest: Interest) -> io::Result<()> {
        self.inner.register(tap, token, interest)
    }

    pub fn reregister(&mut self, tap: &impl Device, token: Token, interest: Interest) -> io::Result<()> {
        self.inner.reregister(tap, token, interest)
    }

    pub fn unregister(&mut self, tap: &impl Device) -> io::Result<()> {
        self.inner.unregister(tap)
    }

    pub fn poll(&mut self, events: &mut Events, t: Option<Duration>) -> io::Result<()> {
        self.inner.poll(&mut events.inner, t)
    }

}

