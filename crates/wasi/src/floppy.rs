#![allow(non_snake_case)]
#![allow(unused)]
use crate::{log::Module, ContextTrait, Dev, StoreT, CMOS_FLOPPY_DRIVE_TYPE};

const DEBUG: bool = false;

type NetxCommand = fn(&mut FloppyController);

pub(crate) struct FloppyController {
    store: StoreT,
    dor: u8,
    last_head: u8,
    last_cylinder: u8,
    response_index: u8,
    response_length: u8,
    bytes_expecting: u8,
    receiving_index: u8,
    number_of_heads: u8,
    sectors_per_track: u8,
    response_data: Vec<u8>,
    next_command: NetxCommand,
    fda_image: Option<Vec<u8>>,
    fdb_image: Option<Vec<u8>>,
    receiving_command: Vec<u8>,
}

fn empty(_: &mut FloppyController) {}

impl FloppyController {
    pub fn new(store: StoreT) -> Self {
        let response_data = Vec::new();
        let receiving_command = Vec::new();
        Self {
            store,
            response_data,
            fda_image: None,
            fdb_image: None,
            receiving_command,
            dor: 0,
            last_head: 0,
            last_cylinder: 0,
            response_index: 0,
            response_length: 0,
            receiving_index: 0,
            number_of_heads: 0,
            bytes_expecting: 0,
            sectors_per_track: 0,
            next_command: empty,
        }
    }

    pub fn init(&mut self) {
        if self.fda_image.is_none() {
            self.store.rtc_mut().map(|rtc| {
                rtc.cmos_write(CMOS_FLOPPY_DRIVE_TYPE, 4 << 4);
            });
        } else {
            //TODO:
        }
        self.store.io_mut().map(|io| {
            io.register_read8(
                0x3F0,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.fdc().map_or(0, |fdc| fdc.port3F0_read()),
            );

            io.register_read8(
                0x3F2,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.fdc().map_or(0, |fdc| fdc.port3F2_read()),
            );

            io.register_read8(
                0x3F4,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.fdc().map_or(0, |fdc| fdc.port3F4_read()),
            );

            io.register_read8(
                0x3F5,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.fdc_mut().map_or(0, |fdc| fdc.port3F5_read()),
            );

            io.register_read8(
                0x3F7,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.fdc_mut().map_or(0, |fdc| fdc.port3F7_read()),
            );

            io.register_write8(
                0x3F2,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, val: u8| {
                    dev.fdc_mut().map(|fdc| fdc.port3F2_write(val));
                },
            );

            io.register_write8(
                0x3F5,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, val: u8| {
                    dev.fdc_mut().map(|fdc| fdc.port3F5_write(val));
                },
            );
        });
    }

    #[inline]
    fn port3F0_read(&self) -> u8 {
        dbg_log!(Module::FLOPPY, "3F0 read");
        return 0;
    }

    #[inline]
    fn port3F2_read(&self) -> u8 {
        dbg_log!(Module::FLOPPY, "read 3F2: DOR");
        return 0;
    }

    #[inline]
    fn port3F7_read(&self) -> u8 {
        dbg_log!(Module::FLOPPY, "read 3F7");
        return 0;
    }

    #[inline]
    fn port3F4_read(&self) -> u8 {
        dbg_log!(Module::FLOPPY, "3F4 read");

        let mut return_byte = 0x80;

        if self.response_index < self.response_length {
            return_byte |= 0x40 | 0x10;
        }

        if (self.dor & 8) == 0 {
            return_byte |= 0x20;
        }

        return return_byte;
    }

    #[inline]
    fn port3F5_read(&mut self) -> u8 {
        if self.response_index < self.response_length {
            let rs = self.response_data[self.response_index as usize];
            dbg_log!(Module::FLOPPY, "3F5 read: {}", rs);
            self.store.cpu_mut().map(|cpu| {
                cpu.device_lower_irq(6);
            });
            self.response_index += 1;
            return rs;
        } else {
            dbg_log!(Module::FLOPPY, "3F5 read, empty");
            return 0xFF;
        }
    }

    #[inline]
    fn port3F2_write(&mut self, value: u8) {
        if (value & 4) == 4 && (self.dor & 4) == 0 {
            // reset
            self.store.cpu_mut().map(|cpu| {
                cpu.device_raise_irq(6);
            });
        }

        dbg_log!(Module::FLOPPY, "start motors: {:#X}", value >> 4);
        dbg_log!(Module::FLOPPY, "enable dma: {}", !!(value & 8));
        dbg_log!(Module::FLOPPY, "reset fdc: {}", !!(value & 4));
        dbg_log!(Module::FLOPPY, "drive select: {}", (value & 3));
        dbg_log!(Module::FLOPPY, "DOR = {:#X}", value);

        self.dor = value;
    }

    fn port3F5_write(&mut self, reg_byte: u8) {
        if self.fda_image.is_none() {
            return;
        }

        dbg_log!(Module::FLOPPY, "3F5 write :{:#X}", reg_byte);
        if self.bytes_expecting > 0 {
            self.receiving_command[self.receiving_index as usize] = reg_byte;
            self.receiving_index += 1;
            self.bytes_expecting -= 1;

            if self.bytes_expecting == 0 {
                if DEBUG {
                    let mut log: String = "3F5 command received: ".into();
                    for i in 0..self.receiving_index {
                        log += &format!("{:#X}", self.receiving_command[i as usize]);
                    }
                    dbg_log!(Module::FLOPPY, "{}", log);
                }
                let args: &'static [u8] =
                    unsafe { std::mem::transmute::<_, &[u8]>(&self.receiving_command[..]) };
                (self.next_command)(self);
            }
        } else {
            match reg_byte {
                // TODO
                //case 2:
                //this.next_command = read_complete_track;
                //this.bytes_expecting = 8;
                //break;
                0x03 => {
                    self.next_command = Self::fix_drive_data;
                    self.bytes_expecting = 2;
                }
                0x04 => {
                    self.next_command = Self::check_drive_status;
                    self.bytes_expecting = 1;
                }
                0x05 | 0xC5 => {
                    self.next_command = |f: &mut FloppyController| {
                        f.do_sector(true);
                    };
                    self.bytes_expecting = 8;
                }
                0xE6 => {
                    self.next_command = |f: &mut FloppyController| {
                        f.do_sector(false);
                    };
                    self.bytes_expecting = 8;
                }
                0x07 => {
                    self.next_command = Self::calibrate;
                    self.bytes_expecting = 1;
                }
                0x08 => self.check_interrupt_status(),
                0x4A => {
                    self.next_command = Self::read_sector_id;
                    self.bytes_expecting = 1;
                }
                0x0F => {
                    self.bytes_expecting = 2;
                    self.next_command = Self::seek;
                }
                0x0E => {
                    // dump regs
                    dbg_log!(Module::FLOPPY, "dump registers");
                    self.response_data[0] = 0x80;
                    self.response_index = 0;
                    self.response_length = 1;

                    self.bytes_expecting = 0;
                }

                _ => {
                    assert!(false, "Unimplemented floppy command call {:#X}", reg_byte);
                }
            }

            self.receiving_index = 0;
        }
    }

    #[inline]
    fn read_sector_id(&mut self) {
        let args = &self.receiving_command[..];
        dbg_log!(Module::FLOPPY, "floppy read sector id {:?}", args);
        self.response_index = 0;
        self.response_length = 7;
        self.response_data[0] = 0;
        self.response_data[1] = 0;
        self.response_data[2] = 0;
        self.response_data[3] = 0;
        self.response_data[4] = 0;
        self.response_data[5] = 0;
        self.response_data[6] = 0;
        self.raise_irq();
    }

    #[inline]
    fn check_interrupt_status(&mut self) {
        // do not trigger an interrupt here
        dbg_log!(Module::FLOPPY, "floppy check interrupt status");

        self.response_index = 0;
        self.response_length = 2;

        self.response_data[0] = 1 << 5;
        self.response_data[1] = self.last_cylinder;
    }

    #[inline]
    fn raise_irq(&mut self) {
        if self.dor & 8 > 0 {
            self.store.cpu_mut().map(|cpu| {
                cpu.device_raise_irq(6);
            });
        }
    }

    #[inline]
    fn calibrate(&mut self) {
        dbg_log!(Module::FLOPPY, "floppy calibrate");
        self.raise_irq();
    }

    fn do_sector(&mut self, is_write: bool) {
        let args = &self.receiving_command[..];
        let head = args[2];
        let cylinder = args[1];
        let sector = args[3];
        let sector_size = 128 << args[4];
        let read_count = args[5] - args[3] + 1;

        let read_offset =
            ((head + self.number_of_heads * cylinder) * self.sectors_per_track + sector - 1)
                * sector_size;
        let rw = if is_write { "Write" } else { "Read" };
        dbg_log!(Module::FLOPPY, "Floppy {}", is_write);
        dbg_log!(
            Module::FLOPPY,
            "from {:#X} length {:#X}",
            read_offset,
            read_count * sector_size
        );
        dbg_log!(Module::FLOPPY, "{} / {} / {}", cylinder, head, sector);

        if args[4] == 0 {
            dbg_log!(
                Module::FLOPPY,
                "FDC: sector count is zero, use data length instead"
            );
        }

        if self.fda_image.is_none() {
            return;
        }

        //TODO
        // if(is_write)
        // {
        //     this.dma.do_write(this.fda_image, read_offset, read_count * sector_size, 2, this.done.bind(this, args, cylinder, head, sector));
        // }
        // else
        // {
        //     this.dma.do_read(this.fda_image, read_offset, read_count * sector_size, 2, this.done.bind(this, args, cylinder, head, sector));
        // }
    }

    #[inline]
    fn fix_drive_data(&mut self) {
        let args = &self.receiving_command[..];
        dbg_log!(Module::FLOPPY, "floppy fix drive data {:?}", args);
    }

    #[inline]
    fn check_drive_status(&mut self) {
        let args = &self.receiving_command[..];
        dbg_log!(Module::FLOPPY, "check drive status");

        self.response_index = 0;
        self.response_length = 1;
        self.response_data[0] = 1 << 5;
    }

    #[inline]
    fn seek(&mut self) {
        let args = &self.receiving_command[..];
        dbg_log!(Module::FLOPPY, "seek");
        assert!((args[0] & 3) == 0, "Unhandled seek drive");

        self.last_cylinder = args[1];
        self.last_head = args[0] >> 2 & 1;

        self.raise_irq();
    }
}
