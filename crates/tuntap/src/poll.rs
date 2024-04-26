use std::{io, time::Duration};

use crate::{
    Events,
    platform, 
    dev::Device, 
    token::Token, 
    interest::Interest, 
};


pub struct Poll {
    inner: platform::Selector
}

impl Poll {
    pub fn new() -> Self {
        let inner = platform::Selector::new();
        Self {
            inner
        }
    }

    pub fn register(&mut self, tap: &impl Device, token: Token, interest: Interest) -> io::Result<()> {
        self.inner.register(tap, token, interest)
    }

    pub fn unregister(&mut self, tap: &impl Device) -> io::Result<()> {
        self.inner.unregister(tap)
    }

    pub fn poll(&mut self, events: &mut Events, t: Option<Duration>) -> io::Result<()> {
        self.inner.poll(&mut events.inner, t)
    }

}

