use std::collections::HashMap;
use std::time::Duration;
use std::{io, ptr};

use winapi::shared::winerror::WAIT_TIMEOUT;
use winapi::um::winbase::{WAIT_FAILED, WAIT_OBJECT_0};
use winapi::um::winnt::HANDLE;

use crate::{Device, Interest, Token};
use crate::{Result, Error};
use super::event::Events;
use super::{ffi, Event, Fd};

#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate)struct InnerHandle(HANDLE);


unsafe impl Sync for InnerHandle {}
unsafe impl Send for InnerHandle {}

impl From<HANDLE> for InnerHandle {
    fn from(value: HANDLE) -> Self {
        Self(value)
    }
}

/// the InnerHandle is the index, the event handle is drop by 
pub struct EventPoller {
    events: HashMap<Token, Event>,
    handle_token: HashMap<InnerHandle, Token>,
}

impl EventPoller {
    pub fn new() -> Self {
        Self{
            handle_token: HashMap::new(),
            events: HashMap::new()
        }
    }
}

impl EventPoller {
    pub fn register(
        &mut self, 
        tap: &impl Device, 
        token: Token, 
        interest: Interest
    ) -> Result<()> {
        let fd: &Fd = tap.fd();
        let read_event = if interest.is_readable() {
            fd.read_event()
        } else {
            ptr::null_mut()
        };
        let write_event = if interest.is_writable() {
            fd.write_event()
        } else {
            ptr::null_mut()
        };
        if interest.is_readable() {
            self.handle_token.insert(InnerHandle(read_event), token);
        }
        if interest.is_writable() {
            self.handle_token.insert(InnerHandle(write_event), token);
        }
        let event = Event {
            token,
            read_event: InnerHandle(read_event),
            write_event: InnerHandle(write_event),
            interest
        };
        self.events.insert(token, event);
        Ok(())
    }

    pub fn reregister(&mut self, tap: &impl Device, token: Token, interest: Interest) -> Result<()> {
        self.register(tap, token, interest)
    }

    pub fn unregister(&mut self, tap: &impl Device) -> Result<()> {
        let token = tap.token();
        if let Some(ev) = self.events.get(&token) {
            if ev.is_readable() {
                self.handle_token.remove(&ev.read_event);
            }
            if ev.is_writable() {
                self.handle_token.remove(&ev.write_event);
            }
        }
        self.events.remove(&token);
        Ok(())
    }

    pub fn poll(&mut self, events: &mut Events, t: Option<Duration>) -> Result<()> {
        let mut raw_events = Vec::new();
        for evt in events.iter() {
            if evt.is_readable() {
                raw_events.push(evt.read_event.0);
            }
            if evt.is_writable() {
                raw_events.push(evt.write_event.0);
            }
        }
        events.clear();
        let t = t.map(|t| t.as_millis()).unwrap_or(0);
        let ret = ffi::wait_for_multi_objects(&raw_events, t as _)?;
        let idx: usize = match ret {
            val if val - WAIT_OBJECT_0 > 0  && val - WAIT_OBJECT_0 < events.len() as _  => { (val - WAIT_OBJECT_0) as usize },
            WAIT_TIMEOUT => return Ok(()),
            WAIT_FAILED => return Err(Error::Io(io::Error::last_os_error())),
            _ => unimplemented!("should not be reach here."),
        };
        raw_events.get(idx)
            .and_then(|handle| self.handle_token.get(&InnerHandle(*handle)))
            .and_then(|t| self.events.get(t))
            .map(|e| events.push(e.clone()));
        Ok(())
    }
}