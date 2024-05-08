mod dev;
mod poller;
mod token;
mod event;
mod error;
mod r#async;
mod address;
mod interest;
mod platform;
mod configuration;

pub use error::{Error, Result};

pub use platform::Fd;

pub use address::{EtherAddr, IntoAddress};

pub use configuration::Configuration;

pub use interest::Interest;

pub use event::{Event, Events};

pub use token::Token;

pub use poller::Poller;

pub use dev::{Tap, Device};

#[derive(Clone, Debug, Copy)]
pub enum Model {
    Tap,
    Tun
}

impl Default for Model {
    fn default() -> Self {
        Self::Tap
    }
}
