use std::{cell::Cell, rc::Weak};

use crate::rtc::RTC;

pub enum Dev {
    Empty,
    Cpu(),
    RTC(Weak<Cell<RTC>>),
}

impl Dev {
    pub fn rtc_mut(self: &Dev) -> Option<&mut RTC> {
        match *self {
            Dev::RTC(ref rtc) => {
                if rtc.weak_count() == 0 {
                    None
                } else {
                    let cell = rtc.as_ptr();
                    unsafe {Some(&mut (*(*cell).as_ptr()))}
                }
            },
            _ => None,
        }
    }
}
