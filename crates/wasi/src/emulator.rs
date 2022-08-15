use std::{time, cell::Cell, rc::Rc};

use wasmtime::{Instance, Store};

use crate::CPU;

struct InnerEmulator {
    start_time: time::Instant,
    cpu: Option<CPU>,
}

impl InnerEmulator {
    fn new() -> Self {
        Self { 
            start_time: time::Instant::now(),
            cpu: None,
        }
    }

    fn init(&mut self, inst: Instance, store: &mut Store<Emulator>) {
        self.cpu = Some(CPU::new(inst, store));
        self.cpu.as_mut().map(|c| c.init(store));
    }
}

impl Drop for InnerEmulator {
    fn drop(&mut self) {
        println!("drop InnerEmulator")
    }
}


pub struct Emulator {
    inner: *mut Rc<Cell<InnerEmulator>>,
}

impl Clone for Emulator {
    fn clone(&self) -> Self {
        let inner = unsafe {Box::from_raw(self.inner)};
        let d: Box<Rc<_>> = inner.clone();
        let _ = Box::into_raw(inner);
        Self { inner:  Box::into_raw(d)}
    }
}

impl Drop for Emulator {
    fn drop(&mut self) {
        unsafe {Box::from_raw(self.inner)};
    }
}

impl Emulator {
    pub fn new() -> Self {
        let inner = Box::new(Rc::new(Cell::new(InnerEmulator::new())));
        Emulator {
            inner: Box::into_raw(inner),
        }
    }

    pub fn time_elapsed(&self) -> f64 {
        self.inner().start_time.elapsed().as_millis() as f64
    }

    pub fn start(&mut self, inst: Instance, store: &mut Store<Emulator>) {
        self.inner().init(inst, store);
    }

    pub fn inner_strong_count(&self) -> usize {
        let rc: &Rc<_> = unsafe {&(*self.inner)};
        Rc::strong_count(rc)
    }

    fn inner(&self) -> &mut InnerEmulator {
        unsafe{
            let rc = &(*self.inner);
            &mut (*rc.as_ptr())
        }
    }
    
}