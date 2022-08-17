use std::{cell::Cell, rc::{Rc, Weak}, time};

use wasmtime::{Instance, Store};

use crate::{CPU, Setting};

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

    fn cpu(&mut self) -> Option<&mut CPU> {
        self.cpu.as_mut()
    }

    fn init(&mut self, inst: Instance, store: Weak<Store<Emulator>>) {
        self.cpu = Some(CPU::new(inst, store));
    }

    pub(crate) fn microtick(&self) -> f64 {
        self.start_time.elapsed().as_millis() as f64
    }

    fn start(&mut self) {
        self.cpu.as_mut().map(|c| {
            c.init(&self.setting);
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
        Emulator {
            inner: inner,
        }
    }

    pub fn microtick(&self) -> f64 {
        self.inner().microtick()
    }

    pub fn start(&mut self, inst: Instance, store: Weak<Store<Emulator>>) {
        self.inner_mut().init(inst, store);
        self.cpu_mut().map(|c| {
            c.set_emulator(Some(Rc::downgrade(&self.inner)));
        });
        self.inner_mut().start();
    }

    pub fn inner_strong_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }

    pub fn cpu_mut(&self) -> Option<&mut CPU> {
        self.inner_mut().cpu.as_mut()
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
