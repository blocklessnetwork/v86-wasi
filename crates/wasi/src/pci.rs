use std::{mem, rc::Weak};

use wasmtime::Store;

use crate::{io::IOps, Emulator, EmulatorTrait, IO};

const PCI_CONFIG_ADDRESS: u32 = 0xCF8;
const PCI_CONFIG_DATA: u32 = 0xCFC;

pub(crate) struct PICBar {
    size: u32,
    original_bar: i32,
    entries: Vec<IOps>,
}

trait PCIDevice {
    fn pci_id(&self) -> u8;

    fn name(&self) -> &str;

    fn pci_rom_size(&self) -> u32;

    fn pci_rom_address(&self) -> u32;

    fn pci_space(&self) -> &[u8];

    fn pci_space_mut(&self) -> &mut [u8];

    fn pci_bars(&self) -> &[Option<PICBar>];

    fn pci_bars_mut(&mut self) -> &mut [Option<PICBar>];
}

struct Space([u8; 4 * 64]);

macro_rules! copy_impl {
    ($name: ident, $type: ty, $l: literal) => {
        fn $name(src: &[u8], dst: &mut [$type]) {
            let mut bs = [0u8; $l];
            for i in 0..src.len() / $l {
                let start = i * $l;
                let mut end = start + $l;
                if end > src.len() {
                    end = src.len();
                }
                bs.copy_from_slice(&src[start..end]);
                let t: $type = <$type>::from_le_bytes(bs);
                dst[i] = t;
            }
        }
    };
}

macro_rules! read_impl {
    ($name: ident, $type: ty, $l: literal) => {
        fn $name(src: &[u8], idx: usize) -> $type {
            let mut bs = [0u8; $l];
            bs.copy_from_slice(&src[idx * $l..(idx * $l + $l)]);
            <$type>::from_le_bytes(bs)
        }
    };
}

macro_rules! write_impl {
    ($name: ident, $type: ty, $l: literal) => {
        fn $name(src: &mut [u8], idx: usize, v: $type) {
            let bs = v.to_le_bytes();
            let dst = &mut src[idx * $l..(idx * $l + $l)];
            dst.copy_from_slice(&bs);
        }
    };
}

copy_impl!(copy_to_i32s, i32, 4);

read_impl!(read_i32, i32, 4);
read_impl!(read_u32, u32, 4);
read_impl!(read_u16, u16, 2);
read_impl!(read_i16, i16, 2);
write_impl!(write_i32, i32, 4);
write_impl!(write_u32, u32, 4);
write_impl!(write_u16, u16, 2);
write_impl!(write_i16, i16, 2);

impl Space {
    fn new() -> Self {
        Self([0; 4 * 64])
    }

    fn set(&mut self, val: &[u8]) {
        self.0.copy_from_slice(val)
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
    store: Weak<Store<Emulator>>,
    pci_addr: [u8; 4],
    pci_value: [u8; 4],
    pci_response: [u8; 4],
    pci_status: [u8; 4],
    devices: [Option<Box<dyn PCIDevice>>; 256],
    device_spaces: [Option<Space>; 256],
}

impl PCI {
    fn new(store: Weak<Store<Emulator>>) -> Self {
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
        }
    }

    fn register_device(&mut self, mut dev: impl PCIDevice + 'static) {
        let device_id: usize = dev.pci_id() as _;
        dbg_log!("PCI register bdf=0x{:x} ({})", device_id, dev.name());
        assert!(self.devices[device_id].is_some());
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
            "PCI: Expected 32-bit write, got 8-bit (addr: 0x{:02x})",
            addr
        );

        dbg_log!(
            "PCI write8 dev= 0x{:02x} ({}) addr=0x{:04x} value=0x{:02x}",
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
                "Warning: PCI: Expected 32-bit write, got 16-bit (addr: 0x{:x})",
                addr
            );
            return;
        }

        assert!(
            !(addr >= 0x30 && addr < 0x34),
            "PCI: Expected 32-bit write, got 16-bit (addr: 0x{:x})",
            addr,
        );

        dbg_log!(
            "PCI writ16 dev=0x{:02x} ({}) addr=0x{:0x} value=0x{:0x}",
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
            let bar = device.pci_bars()[bar_nr as usize].as_ref();
            // dbg_log!("BAR" + bar_nr + " exists=" + (bar ? "y" : "n") + " changed to " +
            //        h(written >>> 0) + " dev=" + h(bdf >> 3, 2) + " (" + device.name + ") ", LOG_PCI);
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
                            dbg_log!("Warning: Changing memory bar not supported, ignored");
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
            dbg_log!("BAR effective value: {:x}", sp_val >> 0);
        } else if addr == 0x30 {
            dbg_log!(
                "PCI write rom address dev={:02x}, ({}) value={:x}",
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
                "PCI write dev={:x} ({}) addr={:x} value={:x}",
                bdf >> 3,
                device.name(),
                addr,
                written
            );
        } else {
            dbg_log!(
                "PCI write dev={:x} ({}) addr={:x} value={:x}",
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

    fn set_io_bars(&self, bar: &PICBar, from: u32, to: u32) {
        let count = bar.size;
        dbg_log!("Move io bars: from={:x} to={:x} count={}", from, to, count);
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
                        "Warning: Bad IO bar: Source not mapped, port=0x{:x}",
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
                        "Warning: Bad IO bar: Target already mapped, port={:x}",
                        to + i
                    );
                }
            }
        });
    }
}
