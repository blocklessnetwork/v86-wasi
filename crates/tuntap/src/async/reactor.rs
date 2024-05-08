use std::{
    collections::HashMap, 
    io, 
    sync::Once, 
    task::{Poll, Waker}, 
    time::Duration
};

use crate::{
    Events, Poller, Token
};

pub fn global_reactor() -> &'static mut Reactor {
    static ONCE: Once = Once::new();
    static mut GLOBAL_REACTOR: Option<Reactor> = None;
    unsafe {
        ONCE.call_once(|| {
            GLOBAL_REACTOR = Some(Reactor::new(Duration::from_millis(2)));
        });
        GLOBAL_REACTOR.as_mut().unwrap()
    }
}

pub struct Reactor {
    poller: Poller,
    time_interval: Duration,
    wakers: HashMap<Token, Waker>,
}

impl Reactor {
    fn new(time_interval: Duration) -> Self {
        let poller = Poller::new();
        let wakers = Default::default();
        Self {
            poller,
            wakers,
            time_interval,
        }
    }

    /// main loop for reactor
    /// this loop is for walking up waker.
    fn run_once(&mut self) {
        let mut events = Events::with_capacity(1024);
        self.poller.poll(&mut events, Some(self.time_interval)).unwrap();
        for event in &events {
            self.wakeup(event.token());
        }
    }

    /// walk up the waker by interest.
    /// when the waker is waking up, should remove from handles.
    fn wakeup(&mut self, token: Token) {
        self.wakers.remove(&token)
            .map(|waker| waker.wake());
    }

    fn add_waker(&mut self, token: Token, waker: Waker) {
        self.wakers
            .insert(token, waker);

    }

    fn remove_waker(&mut self, token: Token) {
        self.wakers
            .remove(&token);

    }
}

/// try to invoke io function, if call fail add it into task list waiting for wake up.
pub fn try_invoke<F, T>(
    token: Token, 
    waker: Waker, 
    mut f: F
) -> Poll<io::Result<T>>
where 
    F: FnMut() -> io::Result<T>
{
    global_reactor().add_waker(token, waker);
    loop {
        match f() {
            Ok(r) => {
                global_reactor().remove_waker(token);
                return Poll::Ready(Ok(r));
            },
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                return Poll::Pending;
            },
            Err(err) if err.kind() == io::ErrorKind::Interrupted => {
                continue;
            }
            Err(err) => {
                global_reactor().remove_waker(token);
                return Poll::Ready(Err(err));
            }
        }
    }
}