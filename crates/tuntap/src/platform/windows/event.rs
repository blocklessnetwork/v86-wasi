use crate::{interest::Interest, token::Token};

use super::event_poll::InnerHandle;

#[derive(Clone)]
pub struct Event {
    pub(crate) token: Token,
    pub(crate) interest: Interest,
    pub(crate) read_event: InnerHandle,
    pub(crate) write_event: InnerHandle,
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
