use std::{
    cell::Cell,
    rc::{Rc, Weak},
    time,
};

use wasmtime::{Instance, Store};

use crate::{rtc::RTC, Setting, CPU, io::IO};

pub(crate) struct InnerEmulator {
    start_time: time::Instant,
    setting: Setting,
    cpu: Option<CPU>,
}

impl InnerEmulator {
    fn new(setting: Setting) -> Self {
        Self {
            start_time: time::Instant::now(),
            setting,
            cpu: None,
        }
    }

    fn init(&mut self, inst: Instance, store: Weak<Store<Emulator>>) {
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
    #[inline(always)]
    pub fn microtick(&self) -> f64 {
        self.inner().microtick()
    }

    pub fn start(&mut self, inst: Instance, store: Weak<Store<Emulator>>) {
        self.inner_mut().init(inst, store);
        self.inner_mut().start();
    }

    #[inline(always)]
    pub fn inner_strong_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }

    #[inline(always)]
    pub fn cpu_mut(&self) -> Option<&mut CPU> {
        self.inner_mut().cpu.as_mut()
    }

    #[inline(always)]
    pub fn io_mut(&self) -> Option<&mut IO> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.io)
    }

    #[inline(always)]
    pub fn cpu(&self) -> Option<&CPU> {
        self.inner_mut().cpu.as_ref()
    }

    #[inline(always)]
    pub(crate) fn rtc_mut(&self) -> Option<&mut RTC> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.rtc)
    }

    #[inline(always)]
    fn inner(&self) -> &InnerEmulator {
        unsafe {
            let rc = &(*self.inner);
            &mut (*rc.as_ptr())
        }
    }

    #[inline(always)]
    fn inner_mut(&self) -> &mut InnerEmulator {
        unsafe {
            let rc = &(*self.inner);
            &mut (*rc.as_ptr())
        }
    }

    #[inline(always)]
    pub fn setting(&self) -> &Setting {
        &self.inner().setting
    }
}
