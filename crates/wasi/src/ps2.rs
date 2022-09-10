use std::rc::Weak;

use wasmtime::Store;

use crate::{Emulator, EmulatorTrait, StoreT, bus::BusData, log::Module};


pub(crate) struct PS2 {
    store: StoreT,
    kbd_buffer: Vec<u8>,
    mouse_buffer: Vec<u8>,
    command_register: u8,
    next_byte_is_aux: bool,
    next_byte_is_ready: bool,
    enable_keyboard_stream: bool,
}

impl PS2 {
    pub fn new(store: StoreT) -> Self {
        let kbd_buffer = Vec::with_capacity(1024);
        let mouse_buffer = Vec::with_capacity(1024);

        Self {
            store,
            kbd_buffer,
            mouse_buffer,
            command_register: 0,
            next_byte_is_aux: false,
            next_byte_is_ready: false,
            enable_keyboard_stream: false,
        }
    }

    pub fn init(&mut self) {
        self.store.bus_mut().map(|bus| {
            bus.register("keyboard-code", |store: &StoreT, data: &BusData| {
                match data {
                    &BusData::U8(data) => {
                        store.ps2().map(|ps2| {
                            ps2.kbd_send_code(data)
                        }); 
                    }
                    _ => {}
                }
            });
        });
    }

    fn kbd_send_code(&mut self, code: u8) {
        if self.enable_keyboard_stream {
            dbg_log!(Module::PS2, "adding kbd code: {:#X}", code);
            self.kbd_buffer.push(code);
            self.raise_irq();
        }
    }

    fn raise_irq(&mut self) {
        if self.next_byte_is_ready {
            // Wait until previous byte is read
            // http://halicery.com/Hardware/8042/8042_1503033_TXT.htm
            return;
        }

        // Kbd has priority over aux
        if self.kbd_buffer.len() > 0 {
            self.kbd_irq();
        } else if self.mouse_buffer.len() > 0 {
            self.mouse_irq();
        }
    }

    fn mouse_irq(&mut self) {
        self.next_byte_is_ready = true;
        self.next_byte_is_aux = true;

        if self.command_register & 2 > 0 {
            dbg_log!(Module::PS2, "Mouse irq");

            // Pulse the irq line
            // Note: can't lower immediately after rising, so lower before rising
            // http://www.os2museum.com/wp/ibm-ps2-model-50-keyboard-controller/
            self.store.cpu_mut().map(|cpu| {
                cpu.device_lower_irq(12);
                cpu.device_raise_irq(12);
            });
        }
    }

    fn kbd_irq(&mut self) {
        self.next_byte_is_ready = true;
        self.next_byte_is_aux = false;

        if self.command_register & 1 > 0 {
            dbg_log!(Module::PS2, "Keyboard irq");

            // Pulse the irq line
            // Note: can't lower immediately after rising, so lower before rising
            // http://www.os2museum.com/wp/ibm-ps2-model-50-keyboard-controller/
            self.store.cpu_mut().map(|cpu| {
                cpu.device_lower_irq(1);
                cpu.device_raise_irq(1);
            });
        }
    }
}