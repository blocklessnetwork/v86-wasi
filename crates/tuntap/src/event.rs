use crate::{platform, token::Token};

/// The event api, inner is platfor event
/// the macos taptun only support select, the event is for select.
/// the linux taptun support epoll.
#[repr(transparent)]
pub struct Event {
    inner: platform::Event,
}

impl Event {
    #[inline]
    pub fn token(&self) -> Token {
        self.inner.token()
    }

    #[inline]
    pub fn is_readable(&self) -> bool {
        self.inner.is_readable()
    }

    #[inline]
    pub fn is_writable(&self) -> bool {
        self.inner.is_writable()
    }

    /// The event repr is the transparent that mean the the memory align is same as inner
    #[inline]
    fn from_sys(e: &platform::Event) -> &Self {
        unsafe {&*(e as *const _ as *const Self)}
    }
}

pub struct Events {
    pub(crate) inner: platform::Events
}

impl Events {
    
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let inner = platform::Events::with_capacity(capacity);
        Self {
            inner
        }
    }

    #[inline]
    pub fn new() -> Self {
        Self {
            inner: platform::Events::new()
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            evts:self,
            pos: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

}

pub struct Iter<'a> {
    evts: &'a Events,
    pos: usize,
}


/// the events iterator.
impl<'a> Iterator for Iter<'a> {
    type Item = &'a Event;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self
            .evts
            .inner.get(self.pos)
            .map(Event::from_sys);
        self.pos += 1;
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.evts.inner.len();
        (l, Some(l))
    }

    fn count(self) -> usize {
        self.evts.inner.len()
    }

}

/// for rust loop syntax sugar.
impl<'a> IntoIterator for &'a Events {
    type Item = &'a Event;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
