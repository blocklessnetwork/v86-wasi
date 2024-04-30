use crate::{interest::Interest, token::Token};

pub struct Event {
    pub(crate) token: Token,
    pub(crate) interest: Interest,
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


