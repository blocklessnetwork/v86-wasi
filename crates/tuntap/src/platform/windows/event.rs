
use std::os::windows::raw::HANDLE;

use crate::{interest::Interest, token::Token};

pub struct Event {
    pub(crate) token: Token,
    pub(crate) interest: Interest,
    pub(crate) raw_event: HANDLE,
}

impl Event {
    #[inline]
    pub fn token(&self) -> Token {
        self.token
    }

    #[inline]
    pub fn is_readable(&self) -> bool {
        self.interest.is_readable()
    }

    #[inline]
    pub fn is_writable(&self) -> bool {
        self.interest.is_writable()
    }
}

pub type Events = Vec<Event>;
