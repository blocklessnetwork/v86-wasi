use std::rc::Weak;

use wasmtime::Store;

use crate::{rtc::RTC, Emulator, EmulatorTrait, CPU};

pub enum Dev {
    Empty,
    Emulator(Weak<Store<Emulator>>)
}

impl Dev {
    pub(crate) fn rtc_mut(self: &Dev) -> Option<&mut RTC> {
        match *self {
            Dev::Emulator(ref e) => e.rtc_mut(),
            _ => None,
        }
    }

    pub(crate) fn cpu_mut(self: &Dev) -> Option<&mut CPU> {
        match *self {
            Dev::Emulator(ref e) => e.cpu_mut(),
            _ => None,
        }
    }
}
