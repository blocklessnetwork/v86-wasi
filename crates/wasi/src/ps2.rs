use std::collections::VecDeque;

use crate::{bus::BusData, log::LOG, ContextTrait, Dev, StoreT};

const PS2_LOG_VERBOSE: bool = false;

pub(crate) struct PS2 {
    store: StoreT,
    kbd_buffer: VecDeque<u8>,
    mouse_buffer: VecDeque<u8>,
    resolution: u8,
    scaling2: bool,
    use_mouse: bool,
    sample_rate: u8,
    have_mouse: bool,
    mouse_clicks: u8,
    mouse_delta_x: u8,
    mouse_delta_y: u8,
    next_read_led: bool,
    next_read_rate: bool,
    command_register: u8,
    last_port60_byte: u8,
    next_byte_is_aux: bool,
    next_read_sample: bool,
    next_byte_is_ready: bool,
    enable_mouse_stream: bool,
    read_output_register: bool,
    next_read_resolution: bool,
    controller_output_port: u8,
    read_command_register: bool,
    next_is_mouse_command: bool,
    enable_keyboard_stream: bool,
    next_handle_scan_code_set: bool,
    read_controller_output_port: bool,
}

impl PS2 {
    pub fn new(store: StoreT) -> Self {
        let kbd_buffer = VecDeque::with_capacity(1024);
        let mouse_buffer = VecDeque::with_capacity(1024);

        Self {
            store,
            kbd_buffer,
            mouse_buffer,
            resolution: 4,
            mouse_clicks: 0,
            sample_rate: 100,
            mouse_delta_x: 0,
            mouse_delta_y: 0,
            scaling2: false,
            use_mouse: false,
            have_mouse: true,
            last_port60_byte: 0,
            next_read_led: false,
            next_read_rate: false,
            command_register: 1 | 4,
            next_byte_is_aux: false,
            next_read_sample: false,
            controller_output_port: 0,
            next_byte_is_ready: false,
            enable_mouse_stream: false,
            read_output_register: false,
            next_read_resolution: false,
            read_command_register: false,
            next_is_mouse_command: false,
            enable_keyboard_stream: false,
            next_handle_scan_code_set: false,
            read_controller_output_port: false,
        }
    }

    pub fn init(&mut self) {
        self.store.bus_mut().map(|bus| {
            bus.register("keyboard-code", |s: &StoreT, data: &BusData| match data {
                &BusData::U8(data) => {
                    s.ps2_mut().map(|ps2| ps2.kbd_send_code(data));
                }
                _ => {}
            });

            bus.register("mouse-click", |s: &StoreT, data: &BusData| match data {
                &BusData::MouseEvent(left, middle, right) => {
                    s.ps2_mut().map(|ps2| {
                        ps2.mouse_send_click(left, middle, right);
                    });
                }
                _ => {}
            });

            bus.register("mouse-delta", |s: &StoreT, data: &BusData| match data {
                &BusData::U8Tuple(d1, d2) => {
                    s.ps2_mut().map(|ps2| {
                        ps2.mouse_send_delta(d1, d2);
                    });
                }
                _ => {}
            });
        });

        self.store.io_mut().map(|io| {
            io.register_read8(
                0x60,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| -> u8 { dev.ps2_mut().map_or(0, |ps2| ps2.port60_read()) },
            );

            io.register_read8(
                0x64,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| -> u8 { dev.ps2_mut().map_or(0, |ps2| ps2.port64_read()) },
            );

            io.register_write8(
                0x60,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, val: u8| {
                    dev.ps2_mut().map(|ps2| {
                        ps2.port60_write(val);
                    });
                },
            );

            io.register_write8(
                0x64,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, val: u8| {
                    dev.ps2_mut().map(|ps2| {
                        ps2.port64_write(val);
                    });
                },
            );
        });
    }

    fn port64_read(&mut self) -> u8 {
        // status port

        let mut status_byte = 0x10;

        if self.next_byte_is_ready {
            status_byte |= 0x1;
        }
        if self.next_byte_is_aux {
            status_byte |= 0x20;
        }

        dbg_log!(LOG::PS2, "port 64 read: {:#X}", status_byte);
        return status_byte;
    }

    fn port60_read(&mut self) -> u8 {
        //dbg_log("port 60 read: " + (buffer[0] || "(none)"));

        self.next_byte_is_ready = false;

        if self.kbd_buffer.len() == 0 && self.mouse_buffer.len() == 0 {
            // should not happen
            dbg_log!(LOG::PS2, "Port 60 read: Empty");
            return self.last_port60_byte;
        }

        if self.next_byte_is_aux {
            self.store.cpu_mut().map(|cpu| {
                cpu.device_lower_irq(12);
            });
            self.last_port60_byte = self.mouse_buffer.pop_front().unwrap();
            dbg_log!(
                LOG::PS2,
                "Port 60 read (mouse): {:#X}",
                self.last_port60_byte
            );
        } else {
            self.store.cpu_mut().map(|cpu| {
                cpu.device_lower_irq(1);
            });
            self.last_port60_byte = self.kbd_buffer.pop_front().unwrap();
            dbg_log!(
                LOG::PS2,
                "Port 60 read (kbd)  : {:#X}",
                self.last_port60_byte
            );
        }

        if self.kbd_buffer.len() > 0 || self.mouse_buffer.len() > 0 {
            self.raise_irq();
        }

        return self.last_port60_byte;
    }

    fn port64_write(&mut self, write_byte: u8) {
        dbg_log!(LOG::PS2, "port 64 write: {:#X}", write_byte);
        match write_byte {
            0x20 => {
                self.kbd_buffer.clear();
                self.mouse_buffer.clear();
                self.kbd_buffer.push_back(self.command_register);
                self.kbd_irq();
            }
            0x60 => self.read_command_register = true,
            0xD1 => self.read_controller_output_port = true,
            0xD3 => self.read_output_register = true,
            0xD4 => self.next_is_mouse_command = true,
            0xA7 => {
                // Disable second port
                dbg_log!(LOG::PS2, "Disable second port");
                self.command_register |= 0x20;
            }
            0xA8 => {
                // Enable second port
                dbg_log!(LOG::PS2, "Enable second port");
                self.command_register &= !0x20;
            }
            0xA9 => {
                // test second ps/2 port
                self.kbd_buffer.clear();
                self.mouse_buffer.clear();
                self.kbd_buffer.push_back(0);
                self.kbd_irq();
            }
            0xAA => {
                self.kbd_buffer.clear();
                self.mouse_buffer.clear();
                self.kbd_buffer.push_back(0x55);
                self.kbd_irq();
            }
            0xAB => {
                // Test first PS/2 port
                self.kbd_buffer.clear();
                self.mouse_buffer.clear();
                self.kbd_buffer.push_back(0);
                self.kbd_irq();
            }
            0xAD => {
                // Disable Keyboard
                dbg_log!(LOG::PS2, "Disable Keyboard");
                self.command_register |= 0x10;
            }
            0xAE => {
                // Enable Keyboard
                dbg_log!(LOG::PS2, "Enable Keyboard");
                self.command_register &= !0x10;
            }
            0xFE => {
                dbg_log!(LOG::PS2, "CPU reboot via PS2");
                self.store.cpu_mut().map(|cpu| {
                    cpu.reboot_internal();
                });
            }
            _ => {
                dbg_log!(
                    LOG::PS2,
                    "port 64: Unimplemented command byte: {:#X}",
                    write_byte
                );
            }
        }
    }

    fn port60_write(&mut self, write_byte: u8) {
        dbg_log!(LOG::PS2, "port 60 write: {:#X}", write_byte);
        if self.read_command_register {
            self.command_register = write_byte;
            self.read_command_register = false;

            // not sure, causes "spurious ack" in Linux
            //this.kbd_buffer.push_back(0xFA);
            //this.kbd_irq();

            dbg_log!(
                LOG::PS2,
                "Keyboard command register = {:#X}",
                self.command_register
            );
        } else if self.read_output_register {
            self.read_output_register = false;

            self.mouse_buffer.clear();
            self.mouse_buffer.push_back(write_byte);
            self.mouse_irq();
        } else if self.next_read_sample {
            self.next_read_sample = false;
            self.mouse_buffer.clear();
            self.mouse_buffer.push_back(0xFA);

            self.sample_rate = write_byte;
            dbg_log!(LOG::PS2, "mouse sample rate: {:#X}", write_byte);
            if self.sample_rate == 0 {
                dbg_log!(LOG::PS2, "invalid sample rate, reset to 100");
                self.sample_rate = 100;
            }
            self.mouse_irq();
        } else if self.next_read_resolution {
            self.next_read_resolution = false;
            self.mouse_buffer.clear();
            self.mouse_buffer.push_back(0xFA);

            if write_byte > 3 {
                self.resolution = 4;
                dbg_log!(LOG::PS2, "invalid resolution, resetting to 4");
            } else {
                self.resolution = 1 << write_byte;
                dbg_log!(LOG::PS2, "resolution: {}", self.resolution);
            }
            self.mouse_irq();
        } else if self.next_read_led {
            // nope
            self.next_read_led = false;
            self.kbd_buffer.push_back(0xFA);
            self.kbd_irq();
        } else if self.next_handle_scan_code_set {
            self.next_handle_scan_code_set = false;

            self.kbd_buffer.push_back(0xFA);
            self.kbd_irq();

            if write_byte > 0 {
                // set scan code set
            } else {
                self.kbd_buffer.push_back(2);
            }
        } else if self.next_read_rate {
            // nope
            self.next_read_rate = false;
            self.kbd_buffer.push_back(0xFA);
            self.kbd_irq();
        } else if self.next_is_mouse_command {
            self.next_is_mouse_command = false;
            dbg_log!(
                LOG::PS2,
                "Port 60 data register write: {:#X}",
                write_byte
            );

            if !self.have_mouse {
                return;
            }

            // send ack
            self.kbd_buffer.clear();
            self.mouse_buffer.clear();
            self.mouse_buffer.push_back(0xFA);

            match write_byte {
                0xE6 => {
                    // set scaling to 1:1
                    dbg_log!(LOG::PS2, "Scaling 1:1");
                    self.scaling2 = false;
                }
                0xE7 => {
                    // set scaling to 2:1
                    dbg_log!(LOG::PS2, "Scaling 2:1");
                    self.scaling2 = true;
                }
                0xE8 => {
                    // set mouse resolution
                    self.next_read_resolution = true;
                }
                0xE9 => {
                    // status request - send one packet
                    self.send_mouse_packet(0, 0);
                }
                0xEB => {
                    // request single packet
                    dbg_log!(LOG::PS2, "unimplemented request single packet");
                    self.send_mouse_packet(0, 0);
                }
                0xF2 => {
                    //  MouseID Byte
                    self.mouse_buffer.push_back(0);
                    self.mouse_buffer.push_back(0);
                    self.mouse_delta_y = 0;
                    self.mouse_clicks = 0;
                    self.mouse_delta_x = 0;
                }
                0xF3 => {
                    // sample rate
                    self.next_read_sample = true;
                }
                0xF4 => {
                    // enable streaming
                    self.enable_mouse_stream = true;
                    self.use_mouse = true;
                    self.store.bus_mut().map(|bus| {
                        bus.send("mouse-enable", BusData::Bool(true));
                    });
                    self.mouse_clicks = 0;
                    self.mouse_delta_x = 0;
                    self.mouse_delta_y = 0;
                }
                0xF5 => {
                    // disable streaming
                    self.enable_mouse_stream = false;
                }
                0xF6 => {
                    // set defaults
                    self.enable_mouse_stream = false;
                    self.sample_rate = 100;
                    self.scaling2 = false;
                    self.resolution = 4;
                }
                0xFF => {
                    // reset, send completion code
                    dbg_log!(LOG::PS2, "Mouse reset");
                    self.mouse_buffer.push_back(0xAA);
                    self.mouse_buffer.push_back(0);

                    self.use_mouse = true;
                    self.store.bus_mut().map(|bus| {
                        bus.send("mouse-enable", BusData::Bool(true));
                    });
                    self.enable_mouse_stream = false;
                    self.sample_rate = 100;
                    self.scaling2 = false;
                    self.resolution = 4;

                    self.mouse_clicks = 0;
                    self.mouse_delta_x = 0;
                    self.mouse_delta_y = 0;
                }

                _ => {
                    dbg_log!(
                        LOG::PS2,
                        "Unimplemented mouse command: {:#X}",
                        write_byte
                    );
                }
            }
            self.mouse_irq();
        } else if self.read_controller_output_port {
            self.read_controller_output_port = false;
            self.controller_output_port = write_byte;
            // If we ever want to implement A20 masking, here is where
            // we should turn the masking off if the second bit is on
        } else {
            dbg_log!(
                LOG::PS2,
                "Port 60 data register write: {:#X}",
                write_byte
            );

            // send ack
            self.mouse_buffer.clear();
            self.kbd_buffer.clear();
            self.kbd_buffer.push_back(0xFA);

            match write_byte {
                0xED => self.next_read_led = true,
                // get/set scan code set
                0xF0 => self.next_handle_scan_code_set = true,
                0xF2 => {
                    // identify
                    self.kbd_buffer.push_back(0xAB);
                    self.kbd_buffer.push_back(83);
                }
                //  Set typematic rate and delay
                0xF3 => self.next_read_rate = true,
                0xF4 => {
                    // enable scanning
                    dbg_log!(LOG::PS2, "kbd enable scanning");
                    self.enable_keyboard_stream = true;
                }
                0xF5 => {
                    // disable scanning
                    dbg_log!(LOG::PS2, "kbd disable scanning");
                    self.enable_keyboard_stream = false;
                }
                0xF6 => {}
                0xFF => {
                    self.kbd_buffer.clear();
                    self.kbd_buffer.push_back(0xFA);
                    self.kbd_buffer.push_back(0xAA);
                    self.kbd_buffer.push_back(0);
                }
                _ => {
                    dbg_log!(
                        LOG::PS2,
                        "Unimplemented keyboard command: {:#X}",
                        write_byte
                    );
                }
            }
            self.kbd_irq();
        }
    }

    fn kbd_send_code(&mut self, code: u8) {
        if self.enable_keyboard_stream {
            dbg_log!(LOG::PS2, "adding kbd code: {:#X}", code);
            self.kbd_buffer.push_back(code);
            //TODO: this is error, should be self.raise_irq(); 
            self.kbd_irq();
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
            dbg_log!(LOG::PS2, "Mouse irq");

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
            dbg_log!(LOG::PS2, "Keyboard irq");

            // Pulse the irq line
            // Note: can't lower immediately after rising, so lower before rising
            // http://www.os2museum.com/wp/ibm-ps2-model-50-keyboard-controller/
            self.store.cpu_mut().map(|cpu| {
                cpu.device_lower_irq(1);
                cpu.device_raise_irq(1);
            });
        }
    }

    fn mouse_send_click(&mut self, left: u8, middle: u8, right: u8) {
        if !self.have_mouse || self.use_mouse {
            return;
        }

        self.mouse_clicks = left | right << 1 | middle << 2;

        if self.enable_mouse_stream {
            self.send_mouse_packet(0, 0);
        }
    }

    fn mouse_send_delta(&mut self, delta_x: u8, delta_y: u8) {
        if !self.have_mouse || !self.use_mouse {
            return;
        }

        // note: delta_x or delta_y can be floating point numbers

        let factor = self.resolution * self.sample_rate / 80;

        self.mouse_delta_x += delta_x * factor;
        self.mouse_delta_y += delta_y * factor;

        if self.enable_mouse_stream {
            let change_x = self.mouse_delta_x | 0;
            let change_y = self.mouse_delta_y | 0;

            if change_x > 0 || change_y > 0 {
                //TODOvar now = Date.now();

                //if(now - this.last_mouse_packet < 1000 / this.sample_rate)
                //{
                //    // TODO: set timeout
                //    return;
                //}

                self.mouse_delta_x -= change_x;
                self.mouse_delta_y -= change_y;

                self.send_mouse_packet(change_x, change_y);
            }
        }
    }

    fn send_mouse_packet(&mut self, dx: u8, dy: u8) {
        let delta_x = dx;
        let delta_y = dy;
        let dy: u8 = if dy > 127 { 1 } else { 0 };
        let dx: u8 = if dx > 127 { 1 } else { 0 };
        let info_byte = dy << 5 | dx << 4 | 1 << 3 | self.mouse_clicks;

        //TODO this.last_mouse_packet = Date.now();

        //if(this.scaling2)
        //{
        //    // only in automatic packets, not 0xEB requests
        //    delta_x = this.apply_scaling2(delta_x);
        //    delta_y = this.apply_scaling2(delta_y);
        //}

        self.mouse_buffer.push_back(info_byte);
        self.mouse_buffer.push_back(delta_x);
        self.mouse_buffer.push_back(delta_y);

        if PS2_LOG_VERBOSE {
            dbg_log!(
                LOG::PS2,
                "adding mouse packets: {} {} {}",
                info_byte,
                dx,
                dy
            );
        }

        self.raise_irq();
    }
}
