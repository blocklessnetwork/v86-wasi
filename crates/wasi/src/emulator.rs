use std::{
    cell::Cell,
    rc::{Rc, Weak},
    time,
};

use wasmtime::{Instance, Store};

use crate::{bus::BUS, dma::DMA, io::IO, pci::PCI, pic::PIC, rtc::RTC, Setting, CPU, vga::VGAScreen};

pub(crate) struct InnerEmulator {
    start_time: time::Instant,
    setting: Setting,
    cpu: Option<CPU>,
    bus: Option<BUS>,
}

impl InnerEmulator {
    fn new(setting: Setting) -> Self {
        Self {
            start_time: time::Instant::now(),
            setting,
            cpu: None,
            bus: None,
        }
    }

    fn init(&mut self, inst: Instance, store: Weak<Store<Emulator>>) {
        self.bus = Some(BUS::new(store.clone()));
        self.cpu = Some(CPU::new(inst, store));
    }

    pub(crate) fn microtick(&self) -> f64 {
        self.start_time.elapsed().as_millis() as f64
    }

    fn start(&mut self) {
        self.cpu.as_mut().map(|c| {
            c.init();
            c.main_run();
        });
    }
}

#[derive(Clone)]
pub struct Emulator {
    inner: Rc<Cell<InnerEmulator>>,
}

impl Emulator {
    pub fn new(setting: Setting) -> Self {
        let inner = Rc::new(Cell::new(InnerEmulator::new(setting)));
        Emulator { inner: inner }
    }
    #[inline]
    pub fn microtick(&self) -> f64 {
        self.inner().microtick()
    }

    pub fn start(&mut self, inst: Instance, store: Weak<Store<Emulator>>) {
        self.inner_mut().init(inst, store);
        self.inner_mut().start();
    }

    #[inline]
    pub(crate) fn vga_mut(&self) -> Option<&mut VGAScreen> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.vga)
    }

    #[inline]
    pub fn inner_strong_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }

    #[inline]
    pub(crate) fn cpu_mut(&self) -> Option<&mut CPU> {
        self.inner_mut().cpu.as_mut()
    }

    #[inline]
    pub(crate) fn bus_mut(&self) -> Option<&mut BUS> {
        self.inner_mut().bus.as_mut()
    }

    #[inline]
    pub(crate) fn bus(&self) -> Option<&BUS> {
        self.inner_mut().bus.as_ref()
    }

    #[inline]
    pub(crate) fn pic_mut(&self) -> Option<&mut PIC> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.pic)
    }

    #[inline]
    pub(crate) fn pic(&self) -> Option<&PIC> {
        self.inner_mut().cpu.as_ref().map(|cpu| &cpu.pic)
    }

    #[inline]
    pub(crate) fn io_mut(&self) -> Option<&mut IO> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.io)
    }

    #[inline]
    pub(crate) fn io(&self) -> Option<&IO> {
        self.inner_mut().cpu.as_ref().map(|cpu| &cpu.io)
    }

    #[inline]
    pub(crate) fn dma_mut(&self) -> Option<&mut DMA> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.dma)
    }

    #[inline]
    pub(crate) fn cpu(&self) -> Option<&CPU> {
        self.inner_mut().cpu.as_ref()
    }

    #[inline]
    pub(crate) fn rtc_mut(&self) -> Option<&mut RTC> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.rtc)
    }

    #[inline]
    pub(crate) fn pci_mut(&self) -> Option<&mut PCI> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.pci)
    }

    #[inline]
    pub(crate) fn pci(&self) -> Option<&PCI> {
        self.inner_mut().cpu.as_mut().map(|cpu| &cpu.pci)
    }

    #[inline]
    fn inner(&self) -> &InnerEmulator {
        unsafe {
            let rc = &(*self.inner);
            &mut (*rc.as_ptr())
        }
    }

    #[inline]
    fn inner_mut(&self) -> &mut InnerEmulator {
        unsafe {
            let rc = &(*self.inner);
            &mut (*rc.as_ptr())
        }
    }

    #[inline]
    pub fn setting(&self) -> &Setting {
        &self.inner().setting
    }
}
