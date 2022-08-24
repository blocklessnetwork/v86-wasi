use std::rc::Weak;

use wasmtime::Store;

use crate::{debug::Debug, rtc::RTC, Emulator, EmulatorTrait, CPU, io::IO, dma::DMA};

pub enum Dev {
    Empty,
    Emulator(Weak<Store<Emulator>>),
}

impl Dev {
    pub(crate) fn rtc_mut(self: &Dev) -> Option<&mut RTC> {
        match *self {
            Dev::Emulator(ref e) => e.rtc_mut(),
            _ => None,
        }
    }

    pub(crate) fn debug_mut(self: &Dev) -> Option<&mut Debug> {
        self.cpu_mut().map(|cpu| &mut cpu.debug)
    }

    pub(crate) fn cpu_mut(self: &Dev) -> Option<&mut CPU> {
        match *self {
            Dev::Emulator(ref e) => e.cpu_mut(),
            _ => None,
        }
    }

    pub(crate) fn io_mut(self: &Dev) -> Option<&mut IO> {
        match *self {
            Dev::Emulator(ref e) => e.io_mut(),
            _ => None,
        }
    }

    pub(crate) fn dma_mut(self: &Dev) -> Option<&mut DMA> {
        match *self {
            Dev::Emulator(ref e) => e.dma_mut(),
            _ => None,
        }
    }
}
