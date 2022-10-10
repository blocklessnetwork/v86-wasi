#![allow(non_snake_case)]
use crate::{log::LOG, ContextTrait, Dev, StoreT};

const PIC_LOG_VERBOSE: bool = true;

lazy_static::lazy_static! {
    static ref INT_LOG2_TABLE: [i32; 256] = {
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
fn int_log2_byte(i: u8) -> i32 {
    INT_LOG2_TABLE[i as usize]
}

struct InnerPIC {
    store: StoreT,
    irq_mask: i32,
    is_master: bool,
    name: &'static str,
    isr: i32,
    irr: i32,
    irq_map: u8,
    irq_value: i32,
    requested_irq: i32,
    expect_icw4: bool,
    state: u8,
    read_isr: bool,
    auto_eoi: u8,
    special_mask_mode: bool,
    elcr: i32,
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

    #[inline]
    pub fn get_isr(&self) -> u8 {
        self.isr as u8
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
        dbg_log!(LOG::PIC, "20 write: {:#X}", data_byte);
        if data_byte & 0x10 > 0 {
            // icw1
            dbg_log!(LOG::PIC, "icw1 = {:#X}", data_byte);
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
            dbg_log!(LOG::PIC, "ocw3: {:#X}", data_byte);
            if data_byte & 2 > 0 {
                self.read_isr = data_byte & 1 > 0;
            }
            if data_byte & 4 > 0 {
                assert!(false, "unimplemented: polling");
            }
            if data_byte & 0x40 > 0 {
                self.special_mask_mode = (data_byte & 0x20) == 0x20;
                dbg_log!(LOG::PIC, "special mask mode: {}", self.special_mask_mode);
            }
        } else {
            // ocw2
            // end of interrupt
            dbg_log!(LOG::PIC, "eoi: {:#X} ({})", data_byte, self.name);

            let eoi_type = data_byte >> 5;
            if eoi_type == 1 {
                // non-specific eoi
                self.isr &= self.isr - 1;
                dbg_log!(LOG::PIC, "new isr: {:#X}", self.isr);
            } else if eoi_type == 3 {
                // specific eoi
                self.isr &= !(1 << (data_byte & 7));
            } else if (data_byte & 0xC8) == 0xC0 {
                // os2 v4
                let priority = data_byte & 7;
                dbg_log!(LOG::PIC, "lowest priority: {:#X}", priority);
            } else {
                dbg_log!(LOG::PIC, "Unknown eoi: {:#X}", data_byte);
                assert!(false);
                self.isr &= self.isr - 1;
            }
            self.check_irqs();
        }
    }

    fn port20_read(&self) -> u8 {
        if self.read_isr {
            dbg_log!(LOG::PIC, "read port 20h (isr): {:#X}", self.isr);
            return self.isr as u8;
        } else {
            dbg_log!(LOG::PIC, "read port 20h (irr): {:#X}", self.irr);
            return self.irr as u8;
        }
    }

    fn port21_write(&mut self, data_byte: u8) {
        // dbg_log!(LOG::PIC, "21 write: {:#X}", data_byte);
        if self.state == 0 {
            if self.expect_icw4 {
                // icw4
                self.expect_icw4 = false;
                self.auto_eoi = data_byte & 2;
                dbg_log!(
                    LOG::PIC,
                    "icw4: {:#X} autoeoi={}",
                    data_byte,
                    self.auto_eoi
                );

                if (data_byte & 1) == 0 {
                    assert!(false, "unimplemented: not 8086 mode");
                }
            } else {
                // ocw1
                self.irq_mask = !(data_byte as i32);
                if PIC_LOG_VERBOSE {
                    dbg_log!(
                        LOG::PIC,
                        "interrupt mask: {:#X} ({})",
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
                LOG::PIC,
                "interrupts are mapped to {:#X} ({})",
                self.irq_map,
                self.name
            );
            self.state += 1;
        } else if self.state == 2 {
            // icw3
            self.state = 0;
            dbg_log!(LOG::PIC, "icw3: {:#X}", data_byte);
        }
    }

    fn port21_read(&self) -> u8 {
        dbg_log!(LOG::PIC, "21h read {:#X}", !self.irq_mask & 0xff);
        (!self.irq_mask & 0xFF) as u8
    }

    fn port4D0_read(&self) -> u8 {
        dbg_log!(LOG::PIC, "elcr read: {:#X}", self.elcr);
        self.elcr as u8
    }

    fn port4D0_write(&mut self, value: u8) {
        dbg_log!(LOG::PIC, "elcr write: {:#X}", value);
        // set by seabios to 00 0C (only set for pci interrupts)
        self.elcr = value as i32;
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
                dbg_log!(
                    LOG::PIC,
                    "master> Already requested irq: {}",
                    self.requested_irq
                );
            }
            self.store.cpu_mut().map(|cpu| cpu.handle_irqs());
            return;
        }
        let enabled_irr: i32 = self.irr as i32  & self.irq_mask;
        if enabled_irr == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(
                    LOG::PIC,
                    "master> no unmasked irrs. irr={:#X} mask={:#X} isr={:#X}",
                    self.irr,
                    self.irq_mask & 0xff,
                    self.isr,
                );
            }
            return;
        }
        let irq_mask = enabled_irr & -enabled_irr;
        let special_mask = if self.special_mask_mode {
            self.irq_mask
        } else {
            -1
        };
        let isr: i32 = self.isr as i32;
        if self.isr != 0 && (isr & -isr & special_mask) <= irq_mask {
            dbg_log!(
                LOG::PIC,
                "master> higher prio: isr={:#X} mask={:#X} irq={:#X}",
                self.isr,
                self.irq_mask & 0xff,
                self.irq_mask
            );
            return;
        }
        assert!(irq_mask != 0);
        let irq_number = int_log2_byte(irq_mask as u8);
        assert!(irq_mask == (1 << irq_number));
        if PIC_LOG_VERBOSE {
            dbg_log!(LOG::PIC, "master> request irq {}", irq_number);
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
                dbg_log!(
                    LOG::PIC,
                    "master> spurious requested={}",
                    self.requested_irq
                );
            }
            self.requested_irq = -1;
            return;
        }
        assert!(self.irr > 0); // spurious
        assert!(self.requested_irq >= 0);
        let irq_mask: i32 = 1 << self.requested_irq;

        // not in level mode
        if (self.elcr & irq_mask) == 0 {
            self.irr &= !irq_mask;
        }

        if self.auto_eoi == 0 {
            self.isr |= irq_mask;
        }

        if PIC_LOG_VERBOSE {
            dbg_log!(LOG::PIC, "master> acknowledge {}", self.requested_irq);
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
                dbg_log!(
                    LOG::PIC,
                    "slave > spurious requested={}",
                    self.requested_irq
                );
            }
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
            dbg_log!(LOG::PIC, "slave > acknowledge {}", self.requested_irq);
        }
        self.store.cpu_mut().map(|cpu| {
            cpu.pic_call_irq(self.irq_map as i32 | self.requested_irq as i32);
        });
        self.requested_irq = -1;
        self.check_irqs();
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
        assert!(irq_number < 8);
        let irq_mask = 1 << irq_number;
        if (self.irq_value & irq_mask) == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(LOG::PIC, "slave > set irq {}", irq_number);
            }
            self.irr |= irq_mask;
            self.irq_value |= irq_mask;
            self.check_irqs();
        } else {
            if PIC_LOG_VERBOSE {
                dbg_log!(LOG::PIC, "slave > set irq {}: already set!", irq_number);
            }
        }
    }

    fn set_master_irq(&mut self, irq_number: u8) {
        assert!(irq_number < 16);
        if irq_number >= 8 {
            self.store.pic_mut().map(|pic| {
                pic.slave.set_irq(irq_number - 8);
            });
            return;
        }

        let irq_mask = 1 << irq_number;
        if (self.irq_value & irq_mask) == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(LOG::PIC, "master> set irq {}", irq_number);
            }
            self.irr |= irq_mask;
            self.irq_value |= irq_mask;
            self.check_irqs();
        } else {
            if PIC_LOG_VERBOSE {
                dbg_log!(LOG::PIC, "master> set irq {}: already set!", irq_number);
            }
        }
    }

    fn clear_master_irq(&mut self, irq_number: u8) {
        assert!(irq_number < 16);
        if PIC_LOG_VERBOSE {
            dbg_log!(LOG::PIC, "master> clear irq {}", irq_number);
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
        assert!(irq_number < 8);
        if PIC_LOG_VERBOSE {
            dbg_log!(LOG::PIC, "slave > clear irq {}", irq_number);
        }

        let irq_mask: i32 = 1 << irq_number;
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
                dbg_log!(
                    LOG::PIC,
                    "slave > Already requested irq:  {}",
                    self.requested_irq
                );
            }
            self.store.cpu_mut().map(|cpu| {
                cpu.handle_irqs();
            });
            return;
        }

        let enabled_irr = self.irr as i32 & self.irq_mask;

        if enabled_irr == 0 {
            if PIC_LOG_VERBOSE {
                dbg_log!(
                    LOG::PIC,
                    "slave > no unmasked irrs. irr={:#X} mask={:#X} isr={:#X}",
                    self.irr,
                    self.irq_mask & 0xff,
                    self.isr
                );
            }
            return;
        }

        let irq_mask = enabled_irr & -enabled_irr;
        let special_mask = if self.special_mask_mode {
            self.irq_mask
        } else {
            -1
        };
        let isr = self.isr as i32;
        if self.isr > 0 && (isr & -isr & special_mask) <= irq_mask {
            // wait for eoi of higher or same priority interrupt
            dbg_log!(
                LOG::PIC,
                "slave > higher prio: isr={:#X} irq={:#X}",
                self.isr,
                self.irq_mask
            );
            return;
        }

        assert!(irq_mask != 0);
        let irq_number = int_log2_byte(irq_mask as u8);
        assert!(irq_mask == (1 << irq_number));
        if PIC_LOG_VERBOSE {
            dbg_log!(LOG::PIC, "slave> request irq {}", irq_number);
        }
        self.requested_irq = irq_number;
        self.store.pic_mut().map(|pic| pic.master.set_irq(2));
    }
}

pub(crate) struct PIC {
    master: InnerPIC,
    slave: InnerPIC,
}

impl PIC {
    pub(crate) fn new(store: StoreT) -> Self {
        let master = InnerPIC::new(store.clone(), true);
        let slave = InnerPIC::new(store.clone(), false);
        Self { master, slave }
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
