use crate::Token;


#[repr(transparent)]
pub struct Event(libc::epoll_event);

pub type Events = Vec<Event>;

impl Event {
    #[inline]
    pub fn token(&self) -> Token {
        Token(self.0.u64 as _)
    }

    pub fn is_readable(&self) -> bool {
        (self.0.events as libc::c_int & libc::EPOLLIN) != 0
            || (self.0.events as libc::c_int & libc::EPOLLPRI) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.0.events as libc::c_int & libc::EPOLLOUT) != 0
    }
}
