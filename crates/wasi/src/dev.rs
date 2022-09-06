use std::rc::Weak;

use wasmtime::Store;

use crate::{
    bus::BUS, debug::Debug, dma::DMA, io::IO, pci::PCI, pic::PIC, rtc::RTC, Emulator,
    EmulatorTrait, CPU, vga::VGAScreen,
};

#[derive(Clone)]
pub enum Dev {
    Empty,
    Emulator(Weak<Store<Emulator>>),
}

impl Dev {
    #[inline]
    pub(crate) fn rtc_mut(self: &Dev) -> Option<&mut RTC> {
        match *self {
            Dev::Emulator(ref e) => e.rtc_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn debug_mut(self: &Dev) -> Option<&mut Debug> {
        self.cpu_mut().map(|cpu| &mut cpu.debug)
    }

    #[inline]
    pub(crate) fn bus_mut(self: &Dev) -> Option<&mut BUS> {
        match *self {
            Dev::Emulator(ref e) => e.bus_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn bus(self: &Dev) -> Option<&BUS> {
        match *self {
            Dev::Emulator(ref e) => e.bus(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn cpu_mut(self: &Dev) -> Option<&mut CPU> {
        match *self {
            Dev::Emulator(ref e) => e.cpu_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn io_mut(self: &Dev) -> Option<&mut IO> {
        match *self {
            Dev::Emulator(ref e) => e.io_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn io(self: &Dev) -> Option<&IO> {
        match *self {
            Dev::Emulator(ref e) => e.io(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn dma_mut(self: &Dev) -> Option<&mut DMA> {
        match *self {
            Dev::Emulator(ref e) => e.dma_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pic_mut(self: &Dev) -> Option<&mut PIC> {
        match *self {
            Dev::Emulator(ref e) => e.pic_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pic(self: &Dev) -> Option<&PIC> {
        match *self {
            Dev::Emulator(ref e) => e.pic(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pci_mut(self: &Dev) -> Option<&mut PCI> {
        match *self {
            Dev::Emulator(ref e) => e.pci_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pci(self: &Dev) -> Option<&PCI> {
        match *self {
            Dev::Emulator(ref e) => e.pci(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn vga_mut(self: &Dev) -> Option<&mut VGAScreen> {
        match *self {
            Dev::Emulator(ref e) => e.vga_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn vga(self: &Dev) -> Option<&VGAScreen> {
        match *self {
            Dev::Emulator(ref e) => e.vga(),
            _ => None,
        }
    }
}
