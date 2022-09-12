#![allow(unused)]
use crate::{
    bus::BUS, debug::Debug, dma::DMA, io::IO, pci::PCI, pic::PIC, rtc::RTC, Emulator,
    ContextTrait, CPU, vga::VGAScreen, uart::UART, StoreT, ps2::PS2, floppy::FloppyController, pit::PIT,
};

#[derive(Clone)]
pub enum Dev {
    Empty,
    Emulator(StoreT),
}

impl Dev {
    #[inline]
    pub(crate) fn rtc_mut(&self) -> Option<&mut RTC> {
        match *self {
            Dev::Emulator(ref e) => e.rtc_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn debug_mut(&self) -> Option<&mut Debug> {
        self.cpu_mut().map(|cpu| &mut cpu.debug)
    }

    #[inline]
    pub(crate) fn bus_mut(&self) -> Option<&mut BUS> {
        match *self {
            Dev::Emulator(ref e) => e.bus_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn bus(&self) -> Option<&BUS> {
        match *self {
            Dev::Emulator(ref e) => e.bus(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn cpu_mut(&self) -> Option<&mut CPU> {
        match *self {
            Dev::Emulator(ref e) => e.cpu_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn microtick(&self) -> f64 {
        match *self {
            Dev::Emulator(ref e) => e.microtick(),
            _ => 0.,
        }
    }

    #[inline]
    pub(crate) fn io_mut(&self) -> Option<&mut IO> {
        match *self {
            Dev::Emulator(ref e) => e.io_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn io(&self) -> Option<&IO> {
        match *self {
            Dev::Emulator(ref e) => e.io(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn dma_mut(&self) -> Option<&mut DMA> {
        match *self {
            Dev::Emulator(ref e) => e.dma_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pic_mut(&self) -> Option<&mut PIC> {
        match *self {
            Dev::Emulator(ref e) => e.pic_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pic(&self) -> Option<&PIC> {
        match *self {
            Dev::Emulator(ref e) => e.pic(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pci_mut(&self) -> Option<&mut PCI> {
        match *self {
            Dev::Emulator(ref e) => e.pci_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn uart0(&self) -> Option<&UART> {
        match *self {
            Dev::Emulator(ref e) => e.uart0(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn uart0_mut(&self) -> Option<&mut UART> {
        match *self {
            Dev::Emulator(ref e) => e.uart0_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pci(&self) -> Option<&PCI> {
        match *self {
            Dev::Emulator(ref e) => e.pci(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn bios(&self) -> Option<&[u8]> {
        match *self {
            Dev::Emulator(ref e) => e.emulator().bios().map(|b| &b[..]),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn vga_bios(&self) -> Option<&[u8]> {
        match *self {
            Dev::Emulator(ref e) => e.emulator().vga_bios().map(|b| &b[..]),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn vga_mut(&self) -> Option<&mut VGAScreen> {
        match *self {
            Dev::Emulator(ref e) => e.vga_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn vga(&self) -> Option<&VGAScreen> {
        match *self {
            Dev::Emulator(ref e) => e.vga(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn ps2_mut(&self) -> Option<&mut PS2> {
        match *self {
            Dev::Emulator(ref e) => e.ps2_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn ps2(&self) -> Option<&PS2> {
        match *self {
            Dev::Emulator(ref e) => e.ps2(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn fdc_mut(&self) -> Option<&mut FloppyController> {
        match *self {
            Dev::Emulator(ref e) => e.fdc_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn fdc(&self) -> Option<&FloppyController> {
        match *self {
            Dev::Emulator(ref e) => e.fdc(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pit_mut(&self) -> Option<&mut PIT> {
        match *self {
            Dev::Emulator(ref e) => e.pit_mut(),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn pit(&self) -> Option<&PIT> {
        match *self {
            Dev::Emulator(ref e) => e.pit(),
            _ => None,
        }
    }
}
