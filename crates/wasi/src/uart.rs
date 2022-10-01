use crate::{bus::BusData, log::LOG, ContextTrait, Dev, StoreT, IO};

const UART_IER_MSI: u8 = 0x08; /* Modem Status Changed int. */
const UART_IIR_THRI: u8 = 0x02; /* Transmitter holding register empty */
const UART_IER_THRI: u8 = 0x02; /* Enable Transmitter holding register int. */
const DLAB: u8 = 0x80;
const UART_IIR_CTI: u8 = 0x0c; /* Character timeout */
const UART_IER_RDI: u8 = 0x01; /* Enable receiver data interrupt */
const UART_IIR_MSI: u8 = 0x00; /* Modem status interrupt (Low priority) */
const UART_IIR_NO_INT: u8 = 0x01;
const UART_LSR_DATA_READY: u8 = 0x1; // data available
const UART_LSR_TX_EMPTY: u8 = 0x20; // TX (THR) buffer is empty
const UART_LSR_TRANSMITTER_EMPTY: u8 = 0x40; // TX empty and line is idle

pub(crate) struct UART {
    store: StoreT,
    irq: u8,
    lsr: u8,
    ier: u8,
    iir: u8,
    com: u8,
    ints: u8,
    port: u32,
    current_line: Vec<u8>,
    input: Vec<u8>,
    baud_rate: u16,
    line_control: u8,
    fifo_control: u8,
    modem_control: u8,
    modem_status: u8,
    scratch_register: u8,
}

impl UART {
    pub fn new(store: StoreT, port: u32) -> Self {
        let ints = 1 << UART_IIR_THRI;
        let (com, irq) = match port {
            0x3F8 => (0, 4),
            0x2F8 => (1, 3),
            0x3E8 => (2, 4),
            0x2E8 => (3, 3),
            _ => (0, 4),
        };
        let lsr = UART_LSR_TRANSMITTER_EMPTY | UART_LSR_TX_EMPTY;
        Self {
            ints,
            store,
            com,
            irq,
            lsr,
            port,
            ier: 0,
            iir: 0,
            baud_rate: 0,
            line_control: 0,
            fifo_control: 0,
            modem_status: 0,
            modem_control: 0,
            scratch_register: 0,
            input: Vec::with_capacity(4086),
            current_line: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        self.store.bus_mut().map(|bus| {
            bus.register(
                &format!("serial{}-input", self.com),
                |s: &StoreT, data: &BusData| {
                    match data {
                        BusData::U8(data) => {
                            s.uart0_mut().map(|uart| {
                                uart.data_received(*data);
                            });
                        }
                        _ => {}
                    };
                },
            );
        });
        self.store.io_mut().map(|io| {
            io.register_write(
                self.port,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.uart0_mut().map(|uart| {
                        uart.write_data(v);
                    });
                },
                |dev: &Dev, _addr: u32, v: u16| {
                    dev.uart0_mut().map(|uart| {
                        uart.write_data((v & 0xFF) as u8);
                        uart.write_data((v >> 8) as u8);
                    });
                },
                IO::empty_write32,
            );

            io.register_write8(
                self.port | 1,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, out_byte: u8| {
                    dev.uart0_mut().map(|uart| {
                        if uart.line_control & DLAB > 0 {
                            uart.baud_rate = uart.baud_rate & 0xFF | (out_byte as u16) << 8;
                            dbg_log!(LOG::SERIAL, "baud rate: {:#X}", uart.baud_rate);
                        } else {
                            uart.ier = out_byte & 0xF;
                            dbg_log!(LOG::SERIAL, "interrupt enable: {:#X}", out_byte);
                            uart.check_interrupt();
                        }
                    });
                },
            );
            //register port read8
            io.register_read8(
                self.port,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.uart0_mut().map_or(0, |uart| {
                        if uart.line_control & DLAB > 0 {
                            (uart.baud_rate & 0xFF) as u8
                        } else {
                            let data = match uart.input.pop() {
                                None => {
                                    dbg_log!(LOG::SERIAL, "Read input empty");
                                    0xFF
                                }
                                Some(d) => {
                                    if d == 0xFF {
                                        dbg_log!(LOG::SERIAL, "Read input empty");
                                    } else {
                                        dbg_log!(LOG::SERIAL, "Read input: {:#X}", d);
                                    }
                                    d
                                }
                            };
                            if uart.input.len() == 0 {
                                uart.lsr &= !UART_LSR_DATA_READY;
                                uart.clear_interrupt(UART_IIR_CTI);
                            }
                            return data;
                        }
                    })
                },
            );

            //register port|1 read8
            io.register_read8(
                self.port | 1,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.uart0_mut().map_or(0, |uart| {
                        if uart.line_control & DLAB > 0 {
                            return (uart.baud_rate >> 8) as u8;
                        } else {
                            return (uart.ier & 0xF) as u8;
                        }
                    })
                },
            );

            //register port|2 read8
            io.register_read8(
                self.port | 2,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.uart0_mut().map_or(0, |uart| {
                        let ret = uart.iir & 0xF | 0xC0;
                        dbg_log!(
                            LOG::SERIAL,
                            "read interrupt identification: {:#X}",
                            uart.iir
                        );

                        if uart.iir == UART_IIR_THRI {
                            uart.clear_interrupt(UART_IIR_THRI);
                        }
                        return ret;
                    })
                },
            );

            //register port|2 write8
            io.register_write8(
                self.port | 2,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, out_byte: u8| {
                    dev.uart0_mut().map(|uart| {
                        dbg_log!(LOG::SERIAL, "fifo control: {:#X}", out_byte);
                        uart.fifo_control = out_byte;
                    });
                },
            );

            //register port|3 read8
            io.register_read8(
                self.port | 3,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.uart0_mut().map_or(0, |uart| {
                        dbg_log!(
                            LOG::SERIAL,
                            "read line control: {:#X}",
                            uart.line_control
                        );
                        return uart.line_control;
                    })
                },
            );

            //register port|3 write8
            io.register_write8(
                self.port | 3,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, out_byte: u8| {
                    dev.uart0_mut().map(|uart| {
                        dbg_log!(LOG::SERIAL, "line control: {:#X}", out_byte);
                        uart.line_control = out_byte;
                    });
                },
            );

            //register port|4 read8
            io.register_read8(
                self.port | 4,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.uart0_mut().map_or(0, |uart| {
                        return uart.modem_control;
                    })
                },
            );

            //register port|4 write8
            io.register_write8(
                self.port | 4,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, out_byte: u8| {
                    dev.uart0_mut().map(|uart| {
                        dbg_log!(LOG::SERIAL, "modem control: {:#X}", out_byte);
                        uart.modem_control = out_byte;
                    });
                },
            );

            //register port|5 read8
            io.register_read8(
                self.port | 5,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.uart0_mut().map_or(0, |uart| {
                        dbg_log!(LOG::SERIAL, "read line status: {:#X}", uart.lsr);
                        return uart.lsr;
                    })
                },
            );

            //register port|5 write8
            io.register_write8(
                self.port | 5,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, _out_byte: u8| {
                    dev.uart0_mut().map(|_uart| {
                        dbg_log!(LOG::SERIAL, "Factory test write");
                    });
                },
            );

            //register port|6 read8
            io.register_read8(
                self.port | 6,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.uart0_mut().map_or(0, |uart| {
                        dbg_log!(
                            LOG::SERIAL,
                            "read modem status: {:#X}",
                            uart.modem_status
                        );
                        return uart.lsr;
                    })
                },
            );

            //register port|6 write8
            io.register_write8(
                self.port | 6,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, _out_byte: u8| {
                    dev.uart0_mut().map(|_uart| {
                        dbg_log!(LOG::SERIAL, "Unkown register write (base+6)");
                    });
                },
            );

            //register port|7 read8
            io.register_read8(
                self.port | 7,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.uart0_mut().map_or(0, |uart| {
                        return uart.scratch_register;
                    })
                },
            );

            //register port|7 write8
            io.register_write8(
                self.port | 7,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, out_byte: u8| {
                    dev.uart0_mut().map(|uart| {
                        uart.scratch_register = out_byte;
                    });
                },
            );
        });
    }

    fn clear_interrupt(&mut self, line: u8) {
        self.ints &= !(1 << line);
        self.check_interrupt();
    }

    fn check_interrupt(&mut self) {
        if (self.ints as u16 & ((1 as u16) << UART_IIR_CTI)) > 0 && (self.ier & UART_IER_RDI) > 0 {
            self.iir = UART_IIR_CTI;
            self.store.cpu_mut().map(|cpu| {
                cpu.device_raise_irq(self.irq);
            });
        } else if (self.ints & (1 << UART_IIR_THRI)) > 0 && (self.ier & UART_IER_THRI) > 0 {
            self.iir = UART_IIR_THRI;
            self.store.cpu_mut().map(|cpu| {
                cpu.device_raise_irq(self.irq);
            });
        } else if (self.ints & (1 << UART_IIR_MSI)) > 0 && (self.ier & UART_IER_MSI) > 0 {
            self.iir = UART_IIR_MSI;
            self.store.cpu_mut().map(|cpu| {
                cpu.device_raise_irq(self.irq);
            });
        } else {
            self.iir = UART_IIR_NO_INT;
            self.store.cpu_mut().map(|cpu| {
                cpu.device_raise_irq(self.irq);
            });
        }
    }

    #[inline]
    fn data_received(&mut self, data: u8) {
        dbg_log!(LOG::SERIAL, "input: {:#X}", data);
        self.input.push(data);

        self.lsr |= UART_LSR_DATA_READY;
        self.throw_interrupt(UART_IIR_CTI);
    }

    #[inline]
    fn throw_interrupt(&mut self, line: u8) {
        self.ints |= 1 << line;
        self.check_interrupt();
    }

    fn write_data(&mut self, out_byte: u8) {
        
        if self.line_control & DLAB > 0 {
            dbg_log!(LOG::SERIAL, "data return : {:#X}", self.line_control);
            self.baud_rate = self.baud_rate & !0xFF | out_byte as u16;

            return;
        }

        dbg_log!(LOG::SERIAL, "data: {:#X}", out_byte);

        self.throw_interrupt(UART_IIR_THRI);

        if out_byte == 0xFF {
            return;
        }
        self.store.bus_mut().map(|bus| {
            bus.send(
                &format!("serial{}-output-char", self.com),
                BusData::U8(out_byte),
            )
        });

        self.current_line.push(out_byte);

        if out_byte == b'\n' {
            //TODO const line = String.fromCharCode.apply("", this.current_line).trimRight().replace(/[\x00-\x08\x0b-\x1f\x7f\x80-\xff]/g, "");
            let line = unsafe { std::str::from_utf8_unchecked(&self.current_line) };
            dbg_log!(LOG::E, "SERIAL: {}", line);
            self.store.bus_mut().map(|bus| {
                bus.send(
                    &format!("serial{}-output-line", self.com),
                    BusData::String(line.into()),
                );
            });
            self.current_line.clear();
        }
    }
}
