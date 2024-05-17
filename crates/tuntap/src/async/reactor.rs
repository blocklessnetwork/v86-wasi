use std::{
    io, 
    time::Duration,
    task::{Poll, Waker}, 
    collections::HashMap, 
    sync::{Arc, Mutex, OnceLock}, 
};

use crate::{
    Events, Interest, Poller, Token
};

pub fn global_reactor() -> Arc<Mutex<Reactor>> {
    static GLOBAL_REACTOR: OnceLock<Arc<Mutex<Reactor>>> = OnceLock::new();
    GLOBAL_REACTOR.get_or_init(|| {
        Arc::new(Mutex::new(Reactor::new(Duration::from_millis(2))))
    }).clone()
}

pub struct Reactor {
    poller: Poller,
    time_interval: Duration,
    reader_wakers: HashMap<Token, Waker>,
    writer_wakers: HashMap<Token, Waker>,
}

impl Reactor {
    pub fn new(time_interval: Duration) -> Self {
        let poller = Poller::new();
        let reader_wakers = Default::default();
        let writer_wakers = Default::default();
        Self {
            poller,
            reader_wakers,
            writer_wakers,
            time_interval,
        }
    }

    /// run once events.
    pub fn run_once(&mut self) {
        let mut events = Events::with_capacity(1024);
        self.poller.poll(&mut events, Some(self.time_interval)).unwrap();
        for event in &events {
            if event.is_readable() {
                self.wakeup(event.token(), Interest::READABLE);
            }
            if event.is_writable() {
                self.wakeup(event.token(), Interest::WRITABLE);
            }
        }
    }

    /// main loop for reactor
    /// this loop is for walking up waker.
    pub fn run_loop(&mut self) {
        loop {
            self.run_once();
        }
    }

    /// walk up the waker by interest.
    /// when the waker is waking up, should remove from handles.
    fn wakeup(&mut self, token: Token, interest: Interest) {
        if interest.is_readable() {
            self.reader_wakers.remove(&token)
                .map(|waker| waker.wake());
        }

        if interest.is_writable() {
            self.writer_wakers.remove(&token)
                .map(|waker| waker.wake());
        }
    }

    #[inline]
    fn add_waker(token: Token, waker: Waker, interest: Interest) {
        if interest.is_readable() {
            global_reactor().lock().unwrap().reader_wakers.insert(token, waker);
        } else if interest.is_writable() {
            global_reactor().lock().unwrap().writer_wakers.insert(token, waker);
        }
    }

    #[inline]
    fn remove_waker(token: Token) {
        global_reactor().lock().unwrap().reader_wakers.remove(&token);
        global_reactor().lock().unwrap().writer_wakers.remove(&token);
    }
}

#[inline]
pub fn remove_token(token:  Token) {
    Reactor::remove_waker(token);
}

/// try to invoke io function, 
/// if call fail add it into task list waiting for wake up.
#[inline]
pub fn try_invoke<F, T>(
    token: Token, 
    waker: Waker,
    interest: Interest,
    mut f: F
) -> Poll<io::Result<T>>
where 
    F: FnMut() -> io::Result<T>
{
    loop {
        match f() {
            Ok(r) => {
                break Poll::Ready(Ok(r));
            },
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                Reactor::add_waker(token, waker, interest);
                break Poll::Pending;
            },
            Err(err) if err.kind() == io::ErrorKind::Interrupted => {
                continue;
            }
            Err(err) => {
                break Poll::Ready(Err(err));
            }
        }
    }
}