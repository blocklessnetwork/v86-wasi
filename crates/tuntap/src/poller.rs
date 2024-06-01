use std::time::Duration;

use crate::{
    dev::Device, interest::Interest, platform, token::Token, Events, Result 
};


pub struct Poller {
    #[cfg(macos)]
    inner: platform::Selector,
    #[cfg(linux)]
    inner: platform::Epoll,
    #[cfg(windows)]
    inner: platform::FakePoller,
}

impl Poller {
    pub fn new() -> Self {
        #[cfg(target_os="macos")]
        let inner = platform::Selector::new();
        #[cfg(target_os="linux")]
        let inner =  platform::Epoll::new();
        #[cfg(windows)]
        let inner =  platform::FakePoller::new();
        Self {
            inner
        }
    }

    pub fn register(&mut self, tap: &impl Device, token: Token, interest: Interest) -> Result<()> {
        self.inner.register(tap, token, interest)
    }

    pub fn reregister(&mut self, tap: &impl Device, token: Token, interest: Interest) -> Result<()> {
        self.inner.reregister(tap, token, interest)
    }

    pub fn unregister(&mut self, tap: &impl Device) -> Result<()> {
        self.inner.unregister(tap)
    }

    pub fn poll(&mut self, events: &mut Events, t: Option<Duration>) -> Result<()> {
        self.inner.poll(&mut events.inner, t)
    }

}

