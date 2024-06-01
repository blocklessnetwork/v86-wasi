use std::time::Duration;

use crate::{Device, Interest, Token};
use crate::{Result, Error};
use super::event::Events;

pub struct FakePoller {
    
}

impl FakePoller {
    pub fn new() -> Self {
        Self{

        }
    }
}

impl FakePoller {
    pub fn register(
        &mut self, 
        tap: &impl Device, 
        token: Token, 
        interest: Interest
    ) -> Result<()> {
        Ok(())
    }

    pub fn reregister(&mut self, tap: &impl Device, token: Token, interest: Interest) -> Result<()> {
        Ok(())
    }

    pub fn unregister(&mut self, tap: &impl Device) -> Result<()> {
        Ok(())
    }

    pub fn poll(&mut self, events: &mut Events, t: Option<Duration>) -> Result<()> {
        Ok(())
    }
}