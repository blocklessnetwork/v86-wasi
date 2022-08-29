use std::rc::Weak;
use wasmtime::Store;
use crate::{Emulator, EmulatorTrait, Dev};

const PIC_LOG_VERBOSE: bool = false;


struct InnerPIC {
    is_master: bool,
    irq_mask: u8,
    name: &'static str,
    irq_map: u8,
    isr: u8,
    irr: u8,
    irq_value: u8,
    requested_irq: i32,
    expect_icw4: bool,
    store: Weak<Store<Emulator>>,
    state: u8,
    read_isr: bool,
    auto_eoi: u8,
    special_mask_mode: bool,
    elcr: u8,
}

impl InnerPIC {
    fn new(store: Weak<Store<Emulator>>, is_master: bool) -> Self {
        let name = if is_master {
            "master"
        } else {
            "slave"
        };
        Self {
            is_master,
            irq_mask: 0,
            name,
            store,
            state: 0,
            irq_map: 0,
            isr: 0,
            irr: 0,
            irq_value: 0,
            requested_irq: -1,
            expect_icw4: false,
            read_isr: false,
            auto_eoi: 1,
            special_mask_mode: false,
            elcr: 0,
        }
    }

    fn register_io(&mut self) {
        const MASTER_IO_BASE: u32 = 0x20;
        const SLAVE_IO_BASE: u32 = 0xA0;
        const MASTER_IOBASE_HIGH: u32 = 0x4D0;
        const SLAVE_IOBASE_HIGH: u32 = 0x4D1;
        let (io_base, iobase_high)  = if self.is_master {
            (MASTER_IO_BASE, MASTER_IOBASE_HIGH)
        } else {
            (SLAVE_IO_BASE, SLAVE_IOBASE_HIGH)
        };
        self.store.io_mut().map(|io| {
            io.register_write8(io_base, crate::Dev::Emulator(self.store.clone()), |dev: &Dev, port: u32, w8: u8| {
                dev.pic_mut().map(|pic| {
                    let inner = if port == MASTER_IO_BASE {
                        pic.master.as_mut()    
                    } else {
                        pic.slave.as_mut()
                    };
                    inner.map(|inner| inner.port20_write(w8));
                });
            });

            io.register_read8(io_base, crate::Dev::Emulator(self.store.clone()), |dev: &Dev, port: u32| {
                dev.pic_mut().map_or(0, |pic| {
                    let inner = if port == MASTER_IO_BASE {
                        pic.master.as_mut()    
                    } else {
                        pic.slave.as_mut()
                    };
                    inner.map_or(0, |inner| inner.port20_read())
                })
            });

            io.register_write8(io_base|1, crate::Dev::Emulator(self.store.clone()), |dev: &Dev, port: u32, w8: u8| {
                dev.pic_mut().map(|pic| {
                    let inner = if port == MASTER_IO_BASE {
                        pic.master.as_mut()
                    } else {
                        pic.slave.as_mut()
                    };
                    inner.map(|inner| inner.port21_write(w8));
                });
            });

            io.register_read8(io_base|1, crate::Dev::Emulator(self.store.clone()), |dev: &Dev, port: u32| {
                dev.pic_mut().map_or(0, |pic| {
                    let inner = if port == MASTER_IO_BASE {
                        pic.master.as_mut()
                    } else {
                        pic.slave.as_mut()
                    };
                    inner.map_or(0, |inner| inner.port21_read())
                })
            });

            io.register_write8(iobase_high, crate::Dev::Emulator(self.store.clone()), |dev: &Dev, port: u32, w8: u8| {
                dev.pic_mut().map(|pic| {
                    let inner = if port == MASTER_IO_BASE {
                        pic.master.as_mut()
                    } else {
                        pic.slave.as_mut()
                    };
                    inner.map(|inner| inner.port4D0_write(w8));
                });
            });

            io.register_read8(iobase_high, crate::Dev::Emulator(self.store.clone()), |dev: &Dev, port: u32| {
                dev.pic_mut().map_or(0, |pic| {
                    let inner = if port == MASTER_IO_BASE {
                        pic.master.as_mut()
                    } else {
                        pic.slave.as_mut()
                    };
                    inner.map_or(0, |inner| inner.port4D0_read())
                })
            });
        });
    }

    fn port20_write(&mut self, data_byte: u8) {
            //dbg_log("20 write: " + h(data_byte), LOG_PIC);
            if data_byte & 0x10 > 0 {
            // icw1
            dbg_log!("icw1 = {:x}", data_byte);
            self.isr = 0;
            self.irr = 0;
            self.irq_mask = 0;
            self.irq_value = 0;
            self.auto_eoi = 1;
            self.requested_irq = -1;

            self.expect_icw4 = data_byte & 1 > 0;
            self.state = 1;
        } else if data_byte & 8 > 0 {
            // ocw3
            dbg_log!("ocw3: 0x{:x}", data_byte);
            if data_byte & 2 > 0 {
                self.read_isr = data_byte & 1 > 0;
            } if data_byte & 4 >0 {
                assert!(false, "unimplemented: polling");
            } if data_byte & 0x40 > 0 {
                self.special_mask_mode = (data_byte & 0x20) == 0x20;
                dbg_log!("special mask mode: {}" , self.special_mask_mode);
            }
        } else {
            // ocw2
            // end of interrupt
            dbg_log!("eoi: 0x{:x} ({})", data_byte, self.name);

            let eoi_type = data_byte >> 5;
            if eoi_type == 1 {
                // non-specific eoi
                self.isr &= self.isr - 1;
                dbg_log!("new isr: 0x{:02x}", self.isr);
            } else if eoi_type == 3 {
                // specific eoi
                self.isr &= !(1 << (data_byte & 7));
            } else if (data_byte & 0xC8) == 0xC0 {
                // os2 v4
                let priority = data_byte & 7;
                dbg_log!("lowest priority: 0x{:02x}", priority);
            } else {
                dbg_log!("Unknown eoi: 0x{:02x}", data_byte);
                assert!(false);
                self.isr &= self.isr - 1;
            }
            self.check_irqs();
        }
    }

    fn port20_read(&self) -> u8 {
        if self.read_isr {
            dbg_log!("read port 20h (isr): 0x{:02x}", self.isr);
            return self.isr;
        } else {
            dbg_log!("read port 20h (irr): 0x{:02x}", self.irr);
            return self.irr;
        }
    }

    fn port21_write(&mut self, data_byte: u8) {
        //dbg_log("21 write: " + h(data_byte), LOG_PIC);
        if self.state == 0 {
            if self.expect_icw4 {
                // icw4
                self.expect_icw4 = false;
                self.auto_eoi = data_byte & 2;
                dbg_log!("icw4: 0x{:0x} autoeoi={}", data_byte, self.auto_eoi);
    
                if (data_byte & 1) == 0 {
                    assert!(false, "unimplemented: not 8086 mode");
                }
            } else {
                // ocw1
                self.irq_mask = !data_byte;
    
                if PIC_LOG_VERBOSE {
                    dbg_log!("interrupt mask: 0x{:x} ({})", self.irq_mask & 0xFF, self.name);
                }
                self.check_irqs();
            }
        } else if(self.state == 1) {
            // icw2
            self.irq_map = data_byte;
            dbg_log!("interrupts are mapped to 0x{:x} ({})", self.irq_map, self.name);
            self.state += 1;
        }
        else if self.state == 2 {
            // icw3
            self.state = 0;
            dbg_log!("icw3: {}", data_byte);
        }
    }

    fn port21_read(&self) -> u8{
        dbg_log!("21h read 0x{:x}", !self.irq_mask & 0xff);
        !self.irq_mask & 0xFF
    }

    fn port4D0_read(&self) -> u8 {
        dbg_log!("elcr read: 0x{:02x}", self.elcr );
        self.elcr
    }

    fn port4D0_write(&mut self, value: u8) {
        dbg_log!("elcr write: 0x{:02}", value);
        // set by seabios to 00 0C (only set for pci interrupts)
        self.elcr = value;
    }


    fn check_irqs(&mut self) {
        
    }
    fn check_master_irqs(&mut self) {
        if self.requested_irq >= 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!("master> Already requested irq: {}", self.requested_irq)
            }
            self.store.cpu_mut().map(|cpu| {
                cpu.handle_irqs()
            });
            return
        }
        let enabled_irr: i8 = (self.irr & self.irq_mask) as i8;
        if enabled_irr == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(
                    "master> no unmasked irrs. irr=0x{:x} mask=0x{:x} isr=0x{:x}", 
                    self.irr,
                    self.irq_mask & 0xff,
                    self.isr,
                );
            }
            return
        }
        let irq_mask = enabled_irr & -enabled_irr;
        let special_mask = if self.special_mask_mode {
            self.irq_mask
        } else {
            0xFF
        };
    }

}

pub(crate) struct PIC {
    store: Weak<Store<Emulator>>,
    master: Option<InnerPIC>,
    slave: Option<InnerPIC>,
}

impl PIC {
    pub(crate) fn new(store: Weak<Store<Emulator>>) -> Self {
        Self {
            store,
            master: None,
            slave: None,
        }
    }
}
