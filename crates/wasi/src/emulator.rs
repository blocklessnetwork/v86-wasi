use std::{cell::Cell, rc::{Rc, Weak}, time};

use wasmtime::{Instance, Store};

use crate::{CPU, Setting};

struct InnerEmulator {
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
        self.cpu.as_mut().map(|c| c.init(&self.setting));
    }
}

#[derive(Clone)]
pub struct Emulator {
    inner: Rc<Cell<InnerEmulator>>,
}

impl Emulator {
    pub fn new(setting: Setting) -> Self {
        let inner = Rc::new(Cell::new(InnerEmulator::new(setting)));
        Emulator {
            inner: inner,
        }
    }

    pub fn time_elapsed(&self) -> f64 {
        self.inner().start_time.elapsed().as_millis() as f64
    }

    pub fn start(&mut self, inst: Instance, store: Weak<Store<Emulator>>) {
        self.inner_mut().init(inst, store);
    }

    pub fn inner_strong_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }

    pub fn cpu(&self) -> Option<&CPU> {
        self.inner().cpu.as_ref()
    }

    fn inner(&self) -> &InnerEmulator {
        unsafe {
            let rc = &(*self.inner);
            &mut (*rc.as_ptr())
        }
    }

    fn inner_mut(&self) -> &mut InnerEmulator {
        unsafe {
            let rc = &(*self.inner);
            &mut (*rc.as_ptr())
        }
    }
}
