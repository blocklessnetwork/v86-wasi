use crate::{bus::BusData, log::Module, ContextTrait, Dev, StoreT};

const OSCILLATOR_FREQ: f64 = 1193.1816666;

pub(crate) struct PIT {
    store: StoreT,
    counter_mode: [u8; 4],
    counter_latch: [u8; 4],
    counter_reload: [u16; 3],
    counter_enabled: [u8; 4],
    counter_next_low: [u8; 4],
    counter_read_mode: [u8; 4],
    counter_start_time: [f64; 3],
    counter_start_value: [u16; 3],
    counter_latch_value: [u16; 3],
}

impl PIT {
    pub fn new(store: StoreT) -> Self {
        Self {
            store,
            counter_mode: [0; 4],
            counter_latch: [0; 4],
            counter_reload: [0; 3],
            counter_enabled: [0; 4],
            counter_next_low: [0; 4],
            counter_read_mode: [0; 4],
            counter_start_time: [0.; 3],
            counter_latch_value: [0; 3],
            counter_start_value: [0; 3],
        }
    }

    pub fn init(&mut self) {
        self.store.io_mut().map(|io| {
            io.register_read8(
                0x61,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.pit().map_or(0, |pit| {
                        let now = dev.microtick();
                        let ref_toggle = ((now as u64) * (1000 * 1000 / 15000)) & 1;
                        let counter2_out = if pit.did_rollover(2, now) { 1 } else { 0 };
                        return (ref_toggle << 4 | counter2_out << 5) as u8;
                    })
                },
            );

            io.register_write8(
                0x61,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.pit()
                        .map(|_pit| {
                            let event = if data & 1 > 0 {
                                "pcspeaker-enable"
                            } else {
                                "pcspeaker-disable"
                            };
                            event
                        })
                        .map(|event| {
                            dev.bus_mut().map(|bus| bus.send(event, BusData::None));
                        });
                },
            );

            io.register_read8(
                0x40,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.pit_mut().map_or(0, |pit| pit.counter_read(0)),
            );
            io.register_read8(
                0x41,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.pit_mut().map_or(0, |pit| pit.counter_read(1)),
            );
            io.register_read8(
                0x42,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.pit_mut().map_or(0, |pit| pit.counter_read(2)),
            );

            io.register_write8(
                0x40,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.pit_mut().map(|pit| {
                        pit.counter_write(0, data);
                    });
                },
            );

            io.register_write8(
                0x41,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.pit_mut().map(|pit| {
                        pit.counter_write(1, data);
                    });
                },
            );

            io.register_write8(
                0x42,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.pit_mut().map(|pit| {
                        pit.counter_write(2, data);
                    });
                },
            );

            io.register_write8(
                0x43,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.pit_mut().map(|pit| {
                        pit.port43_write(data);
                    });
                },
            );
        });
    }

    pub fn timer(&mut self, now: f64, no_irq: bool) -> f64 {
        let mut time_to_next_interrupt = 100.;

        // counter 0 produces interrupts
        if !no_irq {
            if self.counter_enabled[0] > 0 && self.did_rollover(0, now) {
                self.counter_start_value[0] = self.get_counter_value(0, now);
                self.counter_start_time[0] = now;

                dbg_log!(
                    Module::PIT,
                    "pit interrupt. new value: {}",
                    self.counter_start_value[0]
                );

                // This isn't strictly correct, but it's necessary since browsers
                // may sleep longer than necessary to trigger the else branch below
                // and clear the irq
                self.store.cpu_mut().map(|cpu| {
                    cpu.device_lower_irq(0);
                    cpu.device_raise_irq(0);
                });

                let mode = self.counter_mode[0];

                if mode == 0 {
                    self.counter_enabled[0] = 0;
                }
            } else {
                self.store.cpu_mut().map(|cpu| {
                    cpu.device_lower_irq(0);
                });
            }

            if self.counter_enabled[0] > 0 {
                let diff = now - self.counter_start_time[0];
                let diff_in_ticks = (diff * OSCILLATOR_FREQ).floor() as u16;
                let ticks_missing: f64 = (self.counter_start_value[0] - diff_in_ticks) as f64; // XXX: to simplify
                time_to_next_interrupt = ticks_missing / OSCILLATOR_FREQ;
            }
        }
        return time_to_next_interrupt;
    }

    fn did_rollover(&self, i: u8, now: f64) -> bool {
        let i = i as usize;
        let diff = now - self.counter_start_time[i];

        if diff < 0.0 {
            // should only happen after restore_state
            dbg_log!(
                Module::PIT,
                "Warning: PIT timer difference is negative, resetting (timer {})",
                i
            );
            return true;
        }
        let diff_in_ticks = (diff * OSCILLATOR_FREQ).floor() as f64;
        return (self.counter_start_value[i] as f64) < diff_in_ticks;
    }

    fn counter_read(&mut self, i: u8) -> u8 {
        let i = i as usize;
        let latch = self.counter_latch[i];

        if latch > 0 {
            self.counter_latch[i] -= 1;

            if latch == 2 {
                return (self.counter_latch_value[i] & 0xFF) as u8;
            } else {
                return (self.counter_latch_value[i] >> 8) as u8;
            }
        } else {
            let next_low = self.counter_next_low[i];

            if self.counter_mode[i] == 3 {
                self.counter_next_low[i] ^= 1;
            }
            let tick = self.store.microtick();
            let value = self.get_counter_value(i as u8, tick);

            if next_low > 0 {
                return (value & 0xFF) as u8;
            } else {
                return (value >> 8) as u8;
            }
        }
    }

    fn counter_write(&mut self, i: u8, value: u8) {
        let i = i as usize;
        if self.counter_next_low[i] > 0 {
            self.counter_reload[i] = self.counter_reload[i] & !0xFF | value as u16;
        } else {
            self.counter_reload[i] = self.counter_reload[i] & 0xFF | (value as u16) << 8;
        }

        if self.counter_read_mode[i] != 3 || self.counter_next_low[i] == 0 {
            if self.counter_reload[i] == 0 {
                self.counter_reload[i] = 0xFFFF;
            }

            // depends on the mode, should actually
            // happen on the first tick
            self.counter_start_value[i] = self.counter_reload[i];

            self.counter_enabled[i] = 1;

            self.counter_start_time[i] = self.store.microtick();

            let ms = if self.counter_reload[i] > 0 {
                self.counter_reload[i] as u32
            } else {
                0x10000
            } as f64
                / OSCILLATOR_FREQ;
            dbg_log!(
                Module::PIT,
                "counter{} reload={:#X} tick={}ms",
                i,
                self.counter_reload[i],
                ms,
            );
        }

        if self.counter_read_mode[i] == 3 {
            self.counter_next_low[i] ^= 1;
        }
        let cm = self.counter_mode[2];
        let cr = self.counter_reload[2];
        self.store.bus_mut().map(|bus| {
            bus.send("pcspeaker-update", BusData::PcspeakerUpdate(cm, cr));
        });
    }

    fn get_counter_value(&mut self, i: u8, now: f64) -> u16 {
        let i = i as usize;
        if self.counter_enabled[i] == 0 {
            return 0;
        }

        let diff = now - self.counter_start_time[i];
        let diff_in_ticks = (diff * OSCILLATOR_FREQ).floor();

        let mut value = self.counter_start_value[i] - diff_in_ticks as u16;

        dbg_log!(
            Module::PIT,
            "diff={} dticks={} value={} reload={}",
            diff,
            diff_in_ticks,
            value,
            self.counter_reload[i]
        );

        let reload = self.counter_reload[i];

        if value >= reload {
            dbg_log!(
                Module::PIT,
                "Warning: Counter{} value {} is larger than reload {}",
                i,
                value,
                reload
            );
            value %= reload;
        } else if value > i16::MAX as u16 {
            value = value % reload + reload;
        }
        return value;
    }

    fn port43_write(&mut self, reg_byte: u8) {
        let mut mode = reg_byte >> 1 & 7;
        let binary_mode = reg_byte & 1;
        let i = (reg_byte >> 6 & 3) as usize;
        let read_mode = reg_byte >> 4 & 3;

        if i == 1 {
            dbg_log!(Module::PIT, "Unimplemented timer1");
        }

        if i == 3 {
            dbg_log!(Module::PIT, "Unimplemented read back");
            return;
        }

        if read_mode == 0 {
            // latch
            self.counter_latch[i] = 2;
            let value = self.get_counter_value(i as u8, self.store.microtick());
            dbg_log!(Module::PIT, "latch: {}", value);
            self.counter_latch_value[i] = if value > 0 { value - 1 } else { 0 };
            return;
        }

        if mode >= 6 {
            // 6 and 7 are aliased to 2 and 3
            mode &= !4;
        }

        dbg_log!(
            Module::PIT,
            "Control: mode={} ctr={} read_mode={} bcd={}",
            mode,
            i,
            read_mode,
            binary_mode
        );

        if read_mode == 1 {
            // msb
            self.counter_next_low[i] = 0;
        } else if read_mode == 2 {
            // lsb
            self.counter_next_low[i] = 1;
        } else {
            // first lsb then msb
            self.counter_next_low[i] = 1;
        }

        if i == 0 {
            self.store.cpu_mut().map(|cpu| {
                cpu.device_lower_irq(0);
            });
        }

        if mode == 0 {
        } else if mode == 3 || mode == 2 {
            // what is the difference
        } else {
            dbg_log!(Module::PIT, "Unimplemented counter mode: {:#X}", mode);
        }

        self.counter_mode[i] = mode;
        self.counter_read_mode[i] = read_mode;
        let cm = self.counter_mode[2];
        let cr = self.counter_reload[2];
        self.store.bus_mut().map(|bus| {
            bus.send("pcspeaker-update", BusData::PcspeakerUpdate(cm, cr));
        });
    }
}
