#![allow(unused)]
use std::mem;
use crate::{io::IOps, utils::*, Dev, Emulator, ContextTrait, IO, log::Module, StoreT};

const PCI_CONFIG_ADDRESS: u32 = 0xCF8;

const PCI_CONFIG_DATA: u32 = 0xCFC;

const PAM0: u8 = 0x10;

pub(crate) struct PCIBar {
    pub size: u32,
    pub original_bar: i32,
    pub entries: Vec<IOps>,
}

pub(crate) trait PCIDevice {
    fn pci_id(&self) -> u8;

    fn name(&self) -> &str;

    fn pci_rom_size(&self) -> u32;

    fn pci_rom_address(&self) -> u32;

    fn pci_space(&self) -> &[u8];

    fn pci_bars(&self) -> &[Option<PCIBar>];

    fn pci_bars_mut(&mut self) -> &mut [Option<PCIBar>];
}

struct Space([u8; 4 * 64]);

impl Space {
    #[inline]
    fn byte_length(&self) -> u32 {
        64 * 4
    }

    fn new() -> Self {
        Self([0; 4 * 64])
    }

    fn set(&mut self, val: &[u8]) {
        self.0[0..val.len()].copy_from_slice(val)
    }

    #[inline]
    fn copy_to_i32s(&self, idx: usize, to: &mut [i32]) {
        copy_to_i32s(&self.0[idx * 4..], to);
    }

    #[inline]
    fn read_i32(&self, idx: usize) -> i32 {
        read_i32(&self.0, idx)
    }

    #[inline]
    fn read_u32(&self, idx: usize) -> u32 {
        read_u32(&self.0, idx)
    }

    #[inline]
    fn read_i16(&self, idx: usize) -> i16 {
        read_i16(&self.0, idx)
    }

    #[inline]
    fn read_u16(&self, idx: usize) -> u16 {
        read_u16(&self.0, idx)
    }

    #[inline]
    fn read_i8(&self, idx: usize) -> i8 {
        self.0[idx] as i8
    }

    #[inline]
    fn read_u8(&self, idx: usize) -> u8 {
        self.0[idx]
    }

    #[inline]
    fn write_i32(&mut self, idx: usize, v: i32) {
        write_i32(&mut self.0, idx, v);
    }

    #[inline]
    fn write_u32(&mut self, idx: usize, v: u32) {
        write_u32(&mut self.0, idx, v);
    }

    #[inline]
    fn write_i16(&mut self, idx: usize, v: i16) {
        write_i16(&mut self.0, idx, v);
    }

    #[inline]
    fn write_u16(&mut self, idx: usize, v: u16) {
        write_u16(&mut self.0, idx, v);
    }

    #[inline]
    fn write_i8(&mut self, idx: usize, v: i8) {
        self.0[idx] = v as u8;
    }

    #[inline]
    fn write_u8(&mut self, idx: usize, v: u8) {
        self.0[idx] = v;
    }
}

impl Default for Space {
    fn default() -> Self {
        Space::new()
    }
}

pub(crate) struct PCI {
    store: StoreT,
    pci_addr: [u8; 4],
    pci_value: [u8; 4],
    pci_response: [u8; 4],
    pci_status: [u8; 4],
    devices: [Option<Box<dyn PCIDevice>>; 256],
    device_spaces: [Option<Space>; 256],
    isa_bridge_id: u8,
}

impl PCI {
    pub fn new(store: StoreT) -> Self {
        const INIT_DEV: Option<Box<dyn PCIDevice>> = None;
        const INIT_SPACE: Option<Space> = None;
        let devices = [INIT_DEV; 256];
        let device_spaces = [INIT_SPACE; 256];
        Self {
            pci_addr: [0; 4],
            pci_value: [0; 4],
            pci_response: [0; 4],
            pci_status: [0; 4],
            devices,
            device_spaces,
            store,
            isa_bridge_id: 0,
        }
    }

    #[inline]
    fn pci_addr32(&self) -> u32 {
        u32::from_le_bytes(self.pci_addr)
    }

    pub fn init(&mut self) {
        self.store.io_mut().map(|io| {
            io.register_write(
                PCI_CONFIG_DATA,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, _port: u32, w8: u8| {
                    dev.pci_mut().map(|pci| {
                        pci.pci_write8(pci.pci_addr32(), w8);
                    });
                },
                |dev: &Dev, _port: u32, w16: u16| {
                    dev.pci_mut().map(|pci| {
                        pci.pci_write16(pci.pci_addr32(), w16);
                    });
                },
                |dev: &Dev, _port: u32, w32: u32| {
                    dev.pci_mut().map(|pci| {
                        pci.pci_write32(pci.pci_addr32(), w32);
                    });
                },
            );

            io.register_write8(
                PCI_CONFIG_DATA + 1,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, _port: u32, w8: u8| {
                    dev.pci_mut().map(|pci| {
                        pci.pci_write8(pci.pci_addr32() + 1 | 0, w8);
                    });
                },
            );

            io.register_write8(
                PCI_CONFIG_DATA + 2,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, _port: u32, w8: u8| {
                    dev.pci_mut().map(|pci| {
                        pci.pci_write8(pci.pci_addr32() + 2 | 0, w8);
                    });
                },
            );

            io.register_write8(
                PCI_CONFIG_DATA + 3,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, _port: u32, w8: u8| {
                    dev.pci_mut().map(|pci| {
                        pci.pci_write8(pci.pci_addr32() + 3 | 0, w8);
                    });
                },
            );
            io.register_read_consecutive(
                PCI_CONFIG_DATA,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, _: u32| dev.pci().map_or(0, |pci| pci.pci_response[0]),
                |dev: &Dev, _: u32| dev.pci().map_or(0, |pci| pci.pci_response[1]),
                |dev: &Dev, _: u32| dev.pci().map_or(0, |pci| pci.pci_response[2]),
                |dev: &Dev, _: u32| dev.pci().map_or(0, |pci| pci.pci_response[3]),
            );
            io.register_read_consecutive(
                PCI_CONFIG_ADDRESS,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, _: u32| dev.pci().map_or(0, |pci| pci.pci_status[0]),
                |dev: &Dev, _: u32| dev.pci().map_or(0, |pci| pci.pci_status[1]),
                |dev: &Dev, _: u32| dev.pci().map_or(0, |pci| pci.pci_status[2]),
                |dev: &Dev, _: u32| dev.pci().map_or(0, |pci| pci.pci_status[3]),
            );
            io.register_write_consecutive(
                PCI_CONFIG_ADDRESS,
                crate::Dev::Emulator(self.store.clone()),
                |dev: &Dev, _: u32, data: u8| {
                    dev.pci_mut().map(|pci| pci.pci_addr[0] = data & 0xFC);
                },
                |dev: &Dev, _: u32, data: u8| {
                    dev.cpu_mut().map(|cpu| {
                        if cpu.pci.pci_addr[1] & 0x06 == 0x02 && data & 0x06 == 0x06 {
                            dbg_log!(Module::PCI, "CPU reboot via PCI");
                            cpu.reboot_internal();
                            return;
                        }
                        cpu.pci.pci_addr[1] = data
                    });
                },
                |dev: &Dev, _: u32, data: u8| {
                    dev.pci_mut().map(|pci| pci.pci_addr[2] = data);
                },
                |dev: &Dev, _: u32, data: u8| {
                    dev.pci_mut().map(|pci| {
                        pci.pci_addr[3] = data;
                        pci.pci_query();
                    });
                },
            );
        });
        let host_bridge = GenericPCIDevice::new(
            0,
            vec![
                // 00:00.0 Host bridge: Intel Corporation 440FX - 82441FX PMC [Natoma] (rev 02)
                0x86, 0x80, 0x37, 0x12, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, PAM0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
            vec![],
            "82441FX PMC",
        );
        self.register_device(host_bridge);
        let isa_bridge = GenericPCIDevice::new(
            1 << 3,
            vec![
                // 00:01.0 ISA bridge: Intel Corporation 82371SB PIIX3 ISA [Natoma/Triton II]
                0x86, 0x80, 0x00, 0x70, 0x07, 0x00, 0x00, 0x02, 0x00, 0x00, 0x01, 0x06, 0x00, 0x00,
                0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
            vec![],
            "82371SB PIIX3 ISA",
        );
        self.isa_bridge_id = isa_bridge.pci_id;
        self.register_device(isa_bridge);
    }

    pub fn register_device(&mut self, mut dev: impl PCIDevice + 'static) {
        let device_id: usize = dev.pci_id() as _;
        dbg_log!(Module::PCI, "PCI register bdf=0x{:x} ({})", device_id, dev.name());
        assert!(self.devices[device_id].is_none());
        assert!(dev.pci_space().len() >= 64);
        assert!(device_id < self.devices.len());
        let mut space = Space::new();
        space.set(dev.pci_space());
        let mut bar_space = [0i32; 6];
        space.copy_to_i32s(4, &mut bar_space);
        for i in 0..dev.pci_bars().len() {
            let bar = &mut dev.pci_bars_mut()[i];
            let mut bar = match bar {
                &mut None => continue,
                &mut Some(ref mut s) => s,
            };
            let bar_base = bar_space[i];
            let ty = bar_base & 1;
            bar.original_bar = bar_base;
            if ty == 0 {
                // memory, not needed currently
            } else {
                assert!(ty == 1);
                let port = bar_base & !1;
                bar.entries.clear();
                for j in 0..bar.size {
                    self.store.io().map(|io| {
                        bar.entries
                            .push(io.ports[port as usize + j as usize].clone())
                    });
                }
            }
        }
        self.device_spaces[device_id] = Some(space);
        self.devices[device_id] = Some(Box::new(dev));
    }

    fn pci_write8(&mut self, address: u32, written: u8) {
        let bdf = address >> 8 & 0xFFFF;
        let addr = address & 0xFF;
        let index: usize = bdf as usize;
        if self.device_spaces[index].is_none() {
            return;
        }
        let device = self.devices[index].as_ref();
        assert!(
            !(addr >= 0x10 && addr < 0x2C || addr >= 0x30 && addr < 0x34),
            "PCI: Expected 32-bit write, got 8-bit (addr: {:#02X})",
            addr
        );

        dbg_log!(
            Module::PCI, 
            "PCI write8 dev={:#02x} ({}) addr={:#04x} value={:#02X}",
            bdf >> 3,
            device.unwrap().name(),
            addr,
            written
        );
        self.device_spaces[index].as_mut().map(|s| {
            s.write_u8(addr as usize, written);
        });
    }

    fn pci_write16(&mut self, address: u32, written: u16) {
        assert!((address & 1) == 0);

        let bdf = address >> 8 & 0xFFFF;
        let addr = address & 0xFF;
        let index: usize = bdf as usize;
        if self.device_spaces[index].is_none() {
            return;
        }
        let device = self.devices[index].as_ref();
        if addr >= 0x10 && addr < 0x2C {
            // Bochs bios
            dbg_log!(
                Module::PCI, 
                "Warning: PCI: Expected 32-bit write, got 16-bit (addr: {:#X})",
                addr
            );
            return;
        }

        assert!(
            !(addr >= 0x30 && addr < 0x34),
            "PCI: Expected 32-bit write, got 16-bit (addr: {:#X})",
            addr,
        );

        dbg_log!(
            Module::PCI, 
            "PCI writ16 dev=0x{:02x} ({}) addr={:#x} value={:#X}",
            bdf,
            device.unwrap().name(),
            addr,
            written,
        );
        self.device_spaces[index].as_mut().map(|s| {
            s.write_u16((addr >> 1) as usize, written);
        });
    }

    fn pci_write32(&mut self, address: u32, written: u32) {
        assert!((address & 3) == 0);
        let bdf = address >> 8 & 0xFFFF;
        let addr = address & 0xFF;
        let index = bdf as usize;

        if self.device_spaces[index].is_none() {
            return;
        }
        let device = self.devices[index].as_ref().unwrap();
        if addr >= 0x10 && addr < 0x28 {
            let bar_nr = addr - 0x10 >> 2;
            let bar = if device.pci_bars().len() > bar_nr as usize {
                device.pci_bars()[bar_nr as usize].as_ref()
            } else {
                None
            };
            let bar_yn = if bar.is_some() { "y" } else { "n" };
            dbg_log!(
                Module::PCI, 
                "BAR {} exists={} changed to 0x{:#x} dev=0x{:#02x} ({})",
                bar_nr,
                bar_yn,
                written,
                bdf >> 3,
                device.name(),
            );
            if bar.is_some() {
                let mut bar = bar.unwrap();
                assert!(
                    (bar.size & bar.size - 1) == 0,
                    "bar size should be power of 2"
                );

                let space_addr = addr >> 2;
                let typ = self.device_spaces[index]
                    .as_mut()
                    .map(|s| s.read_u32(space_addr as usize))
                    .unwrap()
                    & 1;
                let mut written = written;
                // size check
                if (written | 3 | bar.size as u32 - 1) == 0xFFFF_FFFF {
                    written = !(bar.size as u32 - 1) | typ;
                    if typ == 0 {
                        self.device_spaces[index].as_mut().map(|s| {
                            s.write_u32(space_addr as usize, written);
                        });
                    }
                } else {
                    if typ == 0 {
                        // memory
                        let original_bar = bar.original_bar as u32;

                        if (written & !0xF) != (original_bar & !0xF) {
                            // seabios
                            dbg_log!(Module::PCI, "Warning: Changing memory bar not supported, ignored");
                        }

                        // changing isn't supported yet, reset to default
                        self.device_spaces[index].as_mut().map(|s| {
                            s.write_u32(space_addr as usize, original_bar);
                        });
                    }
                }

                if typ == 1 {
                    // io
                    assert!(typ == 1);
                    let sp_val = self.device_spaces[index]
                        .as_ref()
                        .map(|s| s.read_u32(space_addr as usize))
                        .unwrap();
                    let from = sp_val & 0xfffffffe & 0xFFFF;
                    let to = written & 0xfffffffe & 0xFFFF;
                    dbg_log!(
                        Module::PCI, 
                        "io bar changed from {:x} to {:x} size={}",
                        from >> 0,
                        to >> 0,
                        bar.size
                    );
                    self.set_io_bars(&mut bar, from, to);
                    self.device_spaces[index].as_mut().map(|s| {
                        s.write_u32(space_addr as usize, written | 1);
                    });
                }
            } else {
                self.device_spaces[index].as_mut().map(|s| {
                    s.write_u32((addr >> 2) as usize, 0);
                });
            }
            let sp_val = self.device_spaces[index]
                .as_ref()
                .map(|s| s.read_u32((addr >> 2) as usize))
                .unwrap();
            dbg_log!(Module::PCI, "BAR effective value: {:x}", sp_val >> 0);
        } else if addr == 0x30 {
            dbg_log!(
                Module::PCI, 
                "PCI write rom address dev=0x{:02x}, ({}) value=0x{:X}",
                bdf >> 3,
                device.name(),
                written >> 0
            );
            let dev_size = device.pci_rom_size();
            if dev_size > 0 {
                if (written | 0x7FF) == (0xFFFFFFFF | 0) {
                    self.device_spaces[index].as_mut().map(|s| {
                        s.write_i32((addr >> 2) as usize, -(dev_size as i32) | 0);
                    });
                } else {
                    let rom_addr = device.pci_rom_address();
                    self.device_spaces[index]
                        .as_mut()
                        .map(|s| s.write_u32((addr >> 2) as usize, rom_addr | 0));
                }
            } else {
                self.device_spaces[index].as_mut().map(|s| {
                    s.write_i32((addr >> 2) as usize, 0);
                });
            }
        } else if addr == 0x04 {
            dbg_log!(
                Module::PCI, 
                "PCI write dev={:#X} ({}) addr={:#X} value={:#X}",
                bdf >> 3,
                device.name(),
                addr,
                written
            );
        } else {
            dbg_log!(
                Module::PCI, 
                "PCI write dev={:#X} ({}) addr={:#X} value={:#X}",
                bdf >> 3,
                device.name(),
                addr,
                written
            );
            self.device_spaces[index].as_mut().map(|s| {
                s.write_u32((addr >> 2) as usize, written);
            });
        }
    }

    fn pci_query(&mut self) {
        let dbg_line = "query";

        // Bit | .31                     .0
        // Fmt | EBBBBBBBBDDDDDFFFRRRRRR00

        let bdf = (self.pci_addr[2] as u16) << 8 | self.pci_addr[1] as u16;
        let addr = self.pci_addr[0] & 0xFC;
        //devfn = bdf & 0xFF,
        //bus = bdf >> 8,
        let dev = bdf >> 3 & 0x1F;
        //fn = bdf & 7,
        let enabled = self.pci_addr[3] >> 7;

        let dbg_line = format!(
            "query enabled={} bdf={:#04X} dev={:#02X} addr={:#X}",
            enabled, bdf, dev, addr
        );

        let device = self.device_spaces[bdf as usize].as_ref();

        if device.is_some() {
            let device = device.unwrap();
            self.pci_status = (0x80000000u32 | 0).to_le_bytes();
            let mut respone32 = 0u32;
            if (addr as u32) < device.byte_length() {
                respone32 = device.read_u32((addr >> 2) as usize);
                self.pci_response = respone32.to_le_bytes();
            } else {
                // required by freebsd-9.1
                self.pci_response = [0; 4];
            }
            let mut dbg_line = format!(
                "{} {:#08X} -> {:#08X}",
                dbg_line,
                self.pci_addr32() >> 0,
                respone32 >> 0
            );
            if (addr as u32) >= device.byte_length() {
                dbg_line = format!("{} (undef)", dbg_line);
            }
            let dev = self.devices[bdf as usize].as_ref().unwrap();
            dbg_log!(Module::PCI, "{} ({})", dbg_line, dev.name());
        } else {
            self.pci_response = (0xFFFF_FFFFu32).to_le_bytes(); //-1i32
            self.pci_status = [0; 4];
        }
    }

    fn set_io_bars(&self, bar: &PCIBar, from: u32, to: u32) {
        let count = bar.size;
        dbg_log!(
            Module::PCI, 
            "Move io bars: from={:#X} to={:#X} count={}",
            from,
            to,
            count
        );
        self.store.io_mut().map(|io| {
            for i in 0..count {
                let empty_iops = crate::io::IO::default_iops();
                let old_idx = (from + i) as usize;
                let old_entry = mem::replace(&mut io.ports[old_idx], empty_iops);

                if old_entry.read8 as *const () == IO::empty_read8 as *const ()
                    && old_entry.read16 as *const () == IO::empty_read16 as *const ()
                    && old_entry.read32 as *const () == IO::empty_read32 as *const ()
                    && old_entry.write32 as *const () == IO::empty_write32 as *const ()
                    && old_entry.write16 as *const () == IO::empty_write16 as *const ()
                    && old_entry.write8 as *const () == IO::empty_write8 as *const ()
                {
                    dbg_log!(
                        Module::PCI, 
                        "Warning: Bad IO bar: Source not mapped, port=0x{:#X}",
                        from + i
                    );
                }
                let to_idx = (to + i) as usize;
                let entry = bar.entries[i as usize].clone();
                let empty_entry = mem::replace(&mut io.ports[to_idx], entry);

                if empty_entry.read8 as *const () == IO::empty_read8 as *const ()
                    || empty_entry.read16 as *const () == IO::empty_read16 as *const ()
                    || empty_entry.read32 as *const () == IO::empty_read32 as *const ()
                    || empty_entry.write8 as *const () == IO::empty_write8 as *const ()
                    || empty_entry.write16 as *const () == IO::empty_write16 as *const ()
                    || empty_entry.write32 as *const () == IO::empty_write32 as *const ()
                {
                    // These can fail if the os maps an io port in multiple bars (indicating a bug)
                    // XXX: Fails during restore_state
                    dbg_log!(
                        Module::PCI, 
                        "Warning: Bad IO bar: Target already mapped, port={:#X}",
                        to + i
                    );
                }
            }
        });
    }

    #[inline]
    fn isa_bridge_space_read8(&self, idx: usize) -> u8 {
        let space = self.device_spaces[self.isa_bridge_id as usize].as_ref();
        space.map_or(0, |s| s.read_u8(idx))
    }

    fn raise_irq(&self, pci_id: u8) {
        let space = self.device_spaces[pci_id as usize].as_ref();
        assert!(space.is_some());
        let space = space.unwrap();
        let val = space.read_u32((0x3C >> 2) as usize);
        let pin = (val >> 8 & 0xFF) - 1;
        let device = (pci_id >> 3) - 1 & 0xFF;
        let parent_pin = pin + device as u32 & 3;
        let irq = self.isa_bridge_space_read8((0x60 + parent_pin) as usize);

        //dbg_log("PCI raise irq " + h(irq) + " dev=" + h(device, 2) +
        //        " (" + this.devices[pci_id].name + ")", LOG_PCI);
        self.store.cpu_mut().map(|cpu| {
            cpu.device_raise_irq(irq);
        });
    }
}

pub(crate) struct GenericPCIDevice {
    pci_id: u8,
    pci_space: Vec<u8>,
    pci_bars: Vec<Option<PCIBar>>,
    name: String,
    pci_rom_size: u32,
    pci_rom_address: u32,
}

impl GenericPCIDevice {
    pub fn new(pci_id: u8, pci_space: Vec<u8>, pci_bars: Vec<Option<PCIBar>>, name: &str) -> Self {
        Self {
            pci_id,
            pci_space,
            pci_bars,
            name: name.into(),
            pci_rom_size: 0,
            pci_rom_address: 0,
        }
    }
}

impl PCIDevice for GenericPCIDevice {
    fn pci_id(&self) -> u8 {
        self.pci_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn pci_rom_size(&self) -> u32 {
        self.pci_rom_size
    }

    fn pci_rom_address(&self) -> u32 {
        self.pci_rom_address
    }

    fn pci_space(&self) -> &[u8] {
        &self.pci_space
    }

    fn pci_bars(&self) -> &[Option<PCIBar>] {
        &self.pci_bars
    }

    fn pci_bars_mut(&mut self) -> &mut [Option<PCIBar>] {
        &mut self.pci_bars
    }
}
