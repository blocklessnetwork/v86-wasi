use std::rc::Weak;

use wasmtime::Store;

use crate::{Dev, Emulator, EmulatorTrait, ALL_DEBUG, log::Module};

pub(crate) struct Debug {
    bios_dbg: Vec<u8>,
    store: Weak<Store<Emulator>>,
}

impl Debug {
    pub fn new(store: Weak<Store<Emulator>>) -> Self {
        Self {
            store,
            bios_dbg: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        if !ALL_DEBUG {
            return;
        }

        self.store.io_mut().map(|io| {
            let dev = Dev::Emulator(self.store.clone());
            let dev1 = Dev::Emulator(self.store.clone());
            let handle = |dev: &Dev, _port: u32, v: u8| {
                dev.debug_mut().map(|debug| {
                    if v == b'\n' {
                        dbg_log!(
                            Module::BIOS, "{}", 
                            unsafe {std::str::from_utf8_unchecked(&debug.bios_dbg)}
                        );
                        debug.bios_dbg.clear();
                    } else {
                        debug.bios_dbg.push(v);
                    }
                });
            };
            io.register_write8(0x402, dev, handle);
            io.register_write8(0x500, dev1, handle);
        });
    }
}
