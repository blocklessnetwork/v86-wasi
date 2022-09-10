#![allow(non_snake_case)]
use crate::{Dev, Emulator, EmulatorTrait, log::Module, StoreT};
use std::rc::Weak;
use wasmtime::Store;

const PIC_LOG_VERBOSE: bool = false;

lazy_static::lazy_static! {
    static ref INT_LOG2_TABLE: [i8; 256] = {
        let mut table = [0; 256];
        let mut b = -2;
        for i in 0..256 {
            if (i & i - 1) == 0 {
                b += 1;
            }
            table[i] = b;
        }
        table
    };
}

#[inline]
fn int_log2_byte(i: u8) -> i8 {
    INT_LOG2_TABLE[i as usize]
}

struct InnerPIC {
    store: StoreT,
    is_master: bool,
    irq_mask: u8,
    name: &'static str,
    irq_map: u8,
    isr: u8,
    irr: u8,
    irq_value: u8,
    requested_irq: i8,
    expect_icw4: bool,
    state: u8,
    read_isr: bool,
    auto_eoi: u8,
    special_mask_mode: bool,
    elcr: u8,
}

impl InnerPIC {
    fn new(store: StoreT, is_master: bool) -> Self {
        let name = if is_master { "master" } else { "slave" };
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

    pub fn get_isr(&self) -> u8 {
        self.isr
    }

    fn init(&mut self) {
        const MASTER_IO_BASE: u32 = 0x20;
        const SLAVE_IO_BASE: u32 = 0xA0;
        const MASTER_IOBASE_HIGH: u32 = 0x4D0;
        const SLAVE_IOBASE_HIGH: u32 = 0x4D1;
        let (io_base, iobase_high) = if self.is_master {
            (MASTER_IO_BASE, MASTER_IOBASE_HIGH)
        } else {
            (SLAVE_IO_BASE, SLAVE_IOBASE_HIGH)
        };
        self.store.io_mut().map(|io| {
            io.register_write8(
                io_base,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, port: u32, w8: u8| {
                    dev.pic_mut().map(|pic| {
                        let inner = if port == MASTER_IO_BASE {
                            &mut pic.master
                        } else {
                            &mut pic.slave
                        };
                        inner.port20_write(w8);
                    });
                },
            );

            io.register_read8(
                io_base,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, port: u32| {
                    dev.pic_mut().map_or(0, |pic| {
                        let inner = if port == MASTER_IO_BASE {
                            &mut pic.master
                        } else {
                            &mut pic.slave
                        };
                        inner.port20_read()
                    })
                },
            );

            io.register_write8(
                io_base | 1,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, port: u32, w8: u8| {
                    dev.pic_mut().map(|pic| {
                        let inner = if port == MASTER_IO_BASE | 1 {
                            &mut pic.master
                        } else {
                            &mut pic.slave
                        };
                        inner.port21_write(w8);
                    });
                },
            );

            io.register_read8(
                io_base | 1,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, port: u32| {
                    dev.pic_mut().map_or(0, |pic| {
                        let inner = if port == MASTER_IO_BASE | 1 {
                            &mut pic.master
                        } else {
                            &mut pic.slave
                        };
                        inner.port21_read()
                    })
                },
            );

            io.register_write8(
                iobase_high,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, port: u32, w8: u8| {
                    dev.pic_mut().map(|pic| {
                        let inner = if port == MASTER_IOBASE_HIGH {
                            &mut pic.master
                        } else {
                            &mut pic.slave
                        };
                        inner.port4D0_write(w8);
                    });
                },
            );

            io.register_read8(
                iobase_high,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, port: u32| {
                    dev.pic_mut().map_or(0, |pic| {
                        let inner = if port == MASTER_IOBASE_HIGH {
                            &mut pic.master
                        } else {
                            &mut pic.slave
                        };
                        inner.port4D0_read()
                    })
                },
            );
        });
    }

    fn port20_write(&mut self, data_byte: u8) {
        //dbg_log("20 write: " + h(data_byte), LOG_PIC);
        if data_byte & 0x10 > 0 {
            // icw1
            dbg_log!(Module::PIC, "icw1 = {:x}", data_byte);
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
            dbg_log!(Module::PIC, "ocw3: 0x{:x}", data_byte);
            if data_byte & 2 > 0 {
                self.read_isr = data_byte & 1 > 0;
            }
            if data_byte & 4 > 0 {
                assert!(false, "unimplemented: polling");
            }
            if data_byte & 0x40 > 0 {
                self.special_mask_mode = (data_byte & 0x20) == 0x20;
                dbg_log!(Module::PIC, "special mask mode: {}", self.special_mask_mode);
            }
        } else {
            // ocw2
            // end of interrupt
            dbg_log!(Module::PIC, "eoi: 0x{:x} ({})", data_byte, self.name);

            let eoi_type = data_byte >> 5;
            if eoi_type == 1 {
                // non-specific eoi
                self.isr &= self.isr - 1;
                dbg_log!(Module::PIC, "new isr: 0x{:02x}", self.isr);
            } else if eoi_type == 3 {
                // specific eoi
                self.isr &= !(1 << (data_byte & 7));
            } else if (data_byte & 0xC8) == 0xC0 {
                // os2 v4
                let priority = data_byte & 7;
                dbg_log!(Module::PIC, "lowest priority: 0x{:02x}", priority);
            } else {
                dbg_log!(Module::PIC, "Unknown eoi: 0x{:02x}", data_byte);
                assert!(false);
                self.isr &= self.isr - 1;
            }
            self.check_irqs();
        }
    }

    fn port20_read(&self) -> u8 {
        if self.read_isr {
            dbg_log!(Module::PIC, "read port 20h (isr): 0x{:02x}", self.isr);
            return self.isr;
        } else {
            dbg_log!(Module::PIC, "read port 20h (irr): 0x{:02x}", self.irr);
            return self.irr;
        }
    }

    fn port21_write(&mut self, data_byte: u8) {
        // dbg_log!("21 write: 0x{:02x}", data_byte);
        if self.state == 0 {
            if self.expect_icw4 {
                // icw4
                self.expect_icw4 = false;
                self.auto_eoi = data_byte & 2;
                dbg_log!(Module::PIC, "icw4: 0x{:0x} autoeoi={}", data_byte, self.auto_eoi);

                if (data_byte & 1) == 0 {
                    assert!(false, "unimplemented: not 8086 mode");
                }
            } else {
                // ocw1
                self.irq_mask = !data_byte;

                if PIC_LOG_VERBOSE {
                    dbg_log!(
                        Module::PIC, 
                        "interrupt mask: 0x{:x} ({})",
                        self.irq_mask & 0xFF,
                        self.name
                    );
                }
                self.check_irqs();
            }
        } else if self.state == 1 {
            // icw2
            self.irq_map = data_byte;
            dbg_log!(
                Module::PIC, 
                "interrupts are mapped to 0x{:x} ({})",
                self.irq_map,
                self.name
            );
            self.state += 1;
        } else if self.state == 2 {
            // icw3
            self.state = 0;
            dbg_log!(Module::PIC, "icw3: {}", data_byte);
        }
    }

    fn port21_read(&self) -> u8 {
        dbg_log!(Module::PIC, "21h read 0x{:x}", !self.irq_mask & 0xff);
        !self.irq_mask & 0xFF
    }

    fn port4D0_read(&self) -> u8 {
        dbg_log!(Module::PIC, "elcr read: 0x{:02x}", self.elcr);
        self.elcr
    }

    fn port4D0_write(&mut self, value: u8) {
        dbg_log!(Module::PIC, "elcr write: 0x{:02}", value);
        // set by seabios to 00 0C (only set for pci interrupts)
        self.elcr = value;
    }

    fn check_irqs(&mut self) {
        if self.is_master {
            self.check_master_irqs();
        } else {
            self.check_slave_irqs();
        }
    }

    fn check_master_irqs(&mut self) {
        if self.requested_irq >= 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(Module::PIC, "master> Already requested irq: {}", self.requested_irq);
            }
            self.store.cpu_mut().map(|cpu| cpu.handle_irqs());
            return;
        }
        let enabled_irr: i8 = (self.irr & self.irq_mask) as i8;
        if enabled_irr == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(
                    Module::PIC, 
                    "master> no unmasked irrs. irr=0x{:x} mask=0x{:x} isr=0x{:x}",
                    self.irr,
                    self.irq_mask & 0xff,
                    self.isr,
                );
            }
            return;
        }
        let irq_mask: u8 = (enabled_irr & -enabled_irr) as u8;
        let special_mask = if self.special_mask_mode {
            self.irq_mask
        } else {
            0xFF
        };
        let isr: i8 = self.isr as i8;
        if self.isr > 0 && ((isr & -isr) as u8 & special_mask) <= irq_mask {
            dbg_log!(
                Module::PIC, 
                "master> higher prio: isr=0x{:02x} mask=0x{:02x} irq=0x{:02x}",
                self.isr,
                self.irq_mask & 0xff,
                self.irq_mask
            );
            return;
        }
        assert!(irq_mask != 0);
        let irq_number = int_log2_byte(irq_mask);
        assert!(irq_mask == (1 << irq_number));
        if PIC_LOG_VERBOSE {
            dbg_log!(Module::PIC, "master> request irq {}", irq_number);
        }
        self.requested_irq = irq_number;
        self.store.cpu_mut().map(|cpu| {
            cpu.handle_irqs();
        });
    }

    fn acknowledge_irq(&mut self) {
        if self.is_master {
            self.acknowledge_master_irq();
        } else {
            self.acknowledge_slave_irq();
        }
    }

    fn acknowledge_master_irq(&mut self) {
        if self.requested_irq == -1 {
            return;
        }

        if self.irr == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(Module::PIC, "master> spurious requested={}", self.requested_irq);
            }
            self.requested_irq = -1;
            return;
        }
        assert!(self.irr > 0); // spurious
        assert!(self.requested_irq >= 0);
        let irq_mask = 1 << self.requested_irq;

        // not in level mode
        if (self.elcr & irq_mask) == 0 {
            self.irr &= !irq_mask;
        }

        if self.auto_eoi == 0 {
            self.isr |= irq_mask;
        }

        if PIC_LOG_VERBOSE {
            dbg_log!(Module::PIC, "master> acknowledge {}", self.requested_irq);
        }
        if self.requested_irq == 2 {
            self.store.pic_mut().map(|pic| {
                pic.slave.acknowledge_irq();
            });
        } else {
            self.store.cpu_mut().map(|cpu| {
                cpu.pic_call_irq(self.irq_map as i32 | self.requested_irq as i32);
            });
        }
        self.requested_irq = -1;
        self.check_irqs();
    }

    fn acknowledge_slave_irq(&mut self) {
        if self.requested_irq == -1 {
            return;
        }

        if self.irr == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(Module::PIC, "slave > spurious requested={}", self.requested_irq);
                self.requested_irq = -1;
                self.store.pic_mut().map(|pic| {
                    pic.master.irq_value &= !(1 << 2);
                });
                self.store.cpu_mut().map(|cpu| {
                    cpu.pic_call_irq((self.irq_map | 7) as i32);
                });
                return;
            }

            assert!(self.irr > 0); // spurious
            assert!(self.requested_irq >= 0);

            let irq_mask = 1 << self.requested_irq;
            // not in level mode
            if (self.elcr & irq_mask) == 0 {
                self.irr &= !irq_mask;
            }

            if self.auto_eoi == 0 {
                self.isr |= irq_mask;
            }

            self.store.pic_mut().map(|pic| {
                pic.master.irq_value &= !(1 << 2);
            });
            if PIC_LOG_VERBOSE {
                dbg_log!(Module::PIC, "slave > acknowledge {}", self.requested_irq);
            }
            self.store.cpu_mut().map(|cpu| {
                cpu.pic_call_irq(self.irq_map as i32 | self.requested_irq as i32);
            });
            self.requested_irq = -1;
            self.check_irqs();
        }
    }

    #[inline]
    fn set_irq(&mut self, irq_number: u8) {
        if self.is_master {
            self.set_master_irq(irq_number);
        } else {
            self.set_slave_irq(irq_number);
        }
    }

    

    fn set_slave_irq(&mut self, irq_number: u8) {
        assert!(irq_number >= 0 && irq_number < 8);
        let irq_mask = 1 << irq_number;
        if (self.irq_value & irq_mask) == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(Module::PIC, "slave > set irq {}", irq_number);
            }
            self.irr |= irq_mask;
            self.irq_value |= irq_mask;
            self.check_irqs();
        } else {
            if PIC_LOG_VERBOSE {
                dbg_log!(Module::PIC, "slave > set irq {}: already set!", irq_number);
            }
        }
    }

    fn set_master_irq(&mut self, irq_number: u8) {
        assert!(irq_number >= 0 && irq_number < 16);
        if irq_number >= 8 {
            self.store.pic_mut().map(|pic| {
                pic.slave.set_irq(irq_number - 8);
            });
            return;
        }

        let irq_mask = 1 << irq_number;
        if (self.irq_value & irq_mask) == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(Module::PIC, "master> set irq {}", irq_number);
            }
            self.irr |= irq_mask;
            self.irq_value |= irq_mask;
            self.check_irqs();
        } else {
            if PIC_LOG_VERBOSE {
                dbg_log!(Module::PIC, "master> set irq {}: already set!", irq_number);
            }
        }
    }

    fn clear_master_irq(&mut self, irq_number: u8) {
        assert!(irq_number >= 0 && irq_number < 16);
        if PIC_LOG_VERBOSE {
            dbg_log!(Module::PIC, "master> clear irq {}", irq_number);
        }

        if irq_number >= 8 {
            self.store.pic_mut().map(|pic| {
                pic.slave.clear_irq(irq_number - 8);
            });
            return;
        }

        let irq_mask = 1 << irq_number;
        if self.irq_value & irq_mask > 0 {
            self.irq_value &= !irq_mask;
            self.irr &= !irq_mask;
            self.check_irqs();
        }
    }

    fn clear_slave_irq(&mut self, irq_number: u8) {
        assert!(irq_number >= 0 && irq_number < 8);
        if PIC_LOG_VERBOSE {
            dbg_log!(Module::PIC, "slave > clear irq {}", irq_number);
        }

        let irq_mask = 1 << irq_number;
        if self.irq_value & irq_mask > 0 {
            self.irq_value &= !irq_mask;
            self.irr &= !irq_mask;
            self.check_irqs();
        }
    }

    #[inline]
    fn clear_irq(&mut self, irq_number: u8) {
        if self.is_master {
            self.clear_master_irq(irq_number);
        } else {
            self.clear_slave_irq(irq_number);
        }
    }

    fn check_slave_irqs(&mut self) {
        if self.requested_irq >= 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(Module::PIC, "slave > Already requested irq:  {}", self.requested_irq);
            }
            self.store.cpu_mut().map(|cpu| {
                cpu.handle_irqs();
            });
            return;
        }

        let enabled_irr: i8 = (self.irr & self.irq_mask) as i8;

        if enabled_irr == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(
                    Module::PIC, 
                    "slave > no unmasked irrs. irr=0x{:02x} mask=0x{:02x} isr=0x{:02x}",
                    self.irr,
                    self.irq_mask & 0xff,
                    self.isr
                );
            }
            return;
        }

        let irq_mask = (enabled_irr & -enabled_irr) as u8;
        let special_mask = if self.special_mask_mode {
            self.irq_mask
        } else {
            0xFF
        };
        let isr: i8 = self.isr as i8;
        if self.isr > 0 && ((isr & -isr) as u8 & special_mask) <= irq_mask {
            // wait for eoi of higher or same priority interrupt
            dbg_log!(
                Module::PIC, 
                "slave > higher prio: isr=0x{:02x} irq=0x{:02x}",
                self.isr,
                self.irq_mask
            );
            return;
        }

        assert!(irq_mask != 0);
        let irq_number = int_log2_byte(irq_mask);
        assert!(irq_mask == (1 << irq_number));
        if PIC_LOG_VERBOSE {
            dbg_log!(Module::PIC, "slave> request irq {}", irq_number);
        }
        self.requested_irq = irq_number;
        self.store.pic_mut().map(|pic| pic.master.set_irq(2));
    }
}

pub(crate) struct PIC {
    store: StoreT,
    master: InnerPIC,
    slave: InnerPIC,
}

impl PIC {
    pub(crate) fn new(store: StoreT) -> Self {
        let master = InnerPIC::new(store.clone(), true);
        let slave = InnerPIC::new(store.clone(), false);
        Self {
            store,
            master,
            slave,
        }
    }

    #[inline]
    pub fn acknowledge_irq(&mut self) {
        self.master.acknowledge_irq();
    }

    #[inline]
    pub fn set_irq(&mut self, irq_number: u8) {
        self.master.set_irq(irq_number);
    }

    #[inline]
    pub fn clear_irq(&mut self, irq_number: u8) {
        self.master.clear_irq(irq_number);
    }

    #[inline]
    pub(crate) fn init(&mut self) {
        self.master.init();
        self.slave.init();
    }
}
