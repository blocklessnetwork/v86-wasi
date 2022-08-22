use std::{rc::Weak};

use wasmtime::Store;

use crate::{rtc::RTC, Emulator, EmulatorTrait, CPU};

pub enum Dev {
    Empty,
    CPU(Weak<Store<Emulator>>),
    RTC(Weak<Store<Emulator>>),
}

impl Dev {
    pub(crate) fn rtc_mut(self: &Dev) -> Option<&mut RTC> {
        match *self {
            Dev::RTC(ref e) => {
                e.rtc_mut()
            },
            _ => None,
        }
    }

    pub(crate) fn cpu_mut(self: &Dev) -> Option<&mut CPU> {
        match *self {
            Dev::CPU(ref e) => {
                e.cpu_mut()
            },
            _ => None,
        }
    }
}
