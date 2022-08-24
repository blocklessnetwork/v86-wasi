use lazy_static::lazy_static;
use std::{collections::HashMap, marker::PhantomData, ops::Add, rc::Weak};
use wasmtime::{AsContext, AsContextMut, Memory, Store};

use crate::{Dev, Emulator, EmulatorTrait, LOG_ALL_IO, MMAP_BLOCK_BITS, MMAP_BLOCK_SIZE};

pub(crate) trait MemAccessTrait<T> {
    fn read(&self, store: impl AsContext, idx: u32) -> T;
    fn offset(&self) -> usize;
    fn memory(&mut self) -> &mut Memory;
    fn write(&mut self, store: impl AsContextMut, idx: u32, v: T);
    fn write_slice(&mut self, store: impl AsContextMut, off: usize, bs: &[u8]) {
        let offset = self.offset();
        self.memory().write(store, offset + off, bs).unwrap();
    }
    fn read_slice(&mut self, store: impl AsContextMut, off: usize, bs: &mut [u8]) {
        let offset = self.offset();
        self.memory().read(store, offset + off, bs).unwrap();
    }
}

pub struct MemAccess<T> {
    offset: usize,
    len: u32,
    mem: Memory,
    phantom: PhantomData<T>,
}

impl<T> MemAccess<T> {
    pub fn new(offset: usize, len: u32, mem: Memory) -> Self {
        Self {
            offset,
            mem,
            len,
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_mem_access {
    ($t:ty, $s:literal) => {
        impl MemAccessTrait<$t> for MemAccess<$t> {
            #[inline(always)]
            fn offset(&self) -> usize {
                self.offset
            }

            #[inline(always)]
            fn memory(&mut self) -> &mut Memory {
                &mut self.mem
            }

            fn read(&self, store: impl AsContext, idx: u32) -> $t {
                let mut d = [0u8; $s];
                self.mem
                    .read(store, self.offset.add(idx as usize), &mut d)
                    .unwrap();
                let rs: $t = <$t>::from_le_bytes(d);
                return rs;
            }

            fn write(&mut self, store: impl AsContextMut, idx: u32, v: $t) {
                let d = v.to_le_bytes();
                self.mem
                    .write(store, self.offset.add(idx as usize), &d)
                    .unwrap();
            }
        }
    };
}

impl_mem_access!(u8, 1);
impl_mem_access!(u16, 2);
impl_mem_access!(u32, 4);
impl_mem_access!(i8, 1);
impl_mem_access!(i16, 2);
impl_mem_access!(i32, 4);

const PORTS_SIZE: usize = 0x10000;

type Rd8Fn = fn(&Dev, u32) -> u8;
type Rd16Fn = fn(&Dev, u32) -> u16;
type Rd32Fn = fn(&Dev, u32) -> u32;
type Wr8Fn = fn(&Dev, u32, u8);
type Wr16Fn = fn(&Dev, u32, u16);
type Wr32Fn = fn(&Dev, u32, u32);

pub(crate) struct MMapFn {
    pub memory_map_read8: Vec<Rd8Fn>,
    pub memory_map_read32: Vec<Rd32Fn>,
    pub memory_map_write8: Vec<Wr8Fn>,
    pub memory_map_write32: Vec<Wr32Fn>,
}

impl MMapFn {
    pub fn new() -> Self {
        Self {
            memory_map_read8: Vec::new(),
            memory_map_read32: Vec::new(),
            memory_map_write8: Vec::new(),
            memory_map_write32: Vec::new(),
        }
    }

    pub(crate) fn init(&mut self, s: usize) {
        self.memory_map_read8 = vec![IO::empty_read8; s];
        self.memory_map_read32 = vec![IO::empty_read32; s];
        self.memory_map_write8 = vec![IO::empty_write8; s];
        self.memory_map_write32 = vec![IO::empty_write32; s];
    }
}

struct IOps {
    read8: Rd8Fn,
    read16: Rd16Fn,
    read32: Rd32Fn,
    write8: Wr8Fn,
    write16: Wr16Fn,
    write32: Wr32Fn,

    dev: Dev,
}

pub struct IO {
    ports: Vec<IOps>,
    store: Weak<Store<Emulator>>,
}

impl IO {
    fn empty_read8(_: &Dev, p: u32) -> u8 {
        dbg_log!("empty_read8: {}", p);
        0xFF
    }

    fn empty_read16(_: &Dev, p: u32) -> u16 {
        dbg_log!("empty_read16: {}", p);
        0xFFFF
    }

    fn empty_read32(_: &Dev, p: u32) -> u32 {
        dbg_log!("empty_read32: {}", p);
        0xFFFF_FFFF
    }

    fn empty_write8(_: &Dev, p: u32, _: u8) {
        dbg_log!("empty_write8: {}", p);
    }

    fn empty_write16(_: &Dev, p: u32, _: u16) {
        dbg_log!("empty_write16: {}", p);
    }

    fn empty_write32(_: &Dev, p: u32, _: u32) {
        dbg_log!("empty_write32: {}", p);
    }

    fn default_iops() -> IOps {
        IOps {
            read8: Self::empty_read8,
            read16: Self::empty_read16,
            read32: Self::empty_read32,
            write8: Self::empty_write8,
            write16: Self::empty_write16,
            write32: Self::empty_write32,
            dev: Dev::Empty,
        }
    }

    pub fn new(store: Weak<Store<Emulator>>) -> IO {
        let mut v = Vec::new();
        for _ in 0..PORTS_SIZE {
            v.push(IO::default_iops());
        }
        IO { ports: v, store }
    }

    pub fn register_read(&mut self, port: u32, dev: Dev, r8: Rd8Fn, r16: Rd16Fn, r32: Rd32Fn) {
        let iops = &mut self.ports[port as usize];
        iops.read8 = r8;
        iops.read16 = r16;
        iops.read32 = r32;
        iops.dev = dev;
    }

    pub fn register_read8(&mut self, port: u32, dev: Dev, r8: Rd8Fn) {
        let iops = &mut self.ports[port as usize];
        iops.read8 = r8;
        iops.dev = dev;
    }

    pub fn register_write(&mut self, port: u32, dev: Dev, w8: Wr8Fn, w16: Wr16Fn, w32: Wr32Fn) {
        let iops = &mut self.ports[port as usize];
        iops.write8 = w8;
        iops.write16 = w16;
        iops.write32 = w32;
        iops.dev = dev;
    }

    pub fn register_write8(&mut self, port: u32, dev: Dev, w8: Wr8Fn) {
        let iops = &mut self.ports[port as usize];
        iops.write8 = w8;
        iops.dev = dev;
    }

    pub fn io_port_read8(&self, port: u32) -> u8 {
        let iops = &self.ports[port as usize];
        if iops.read8 as *const () == Self::empty_read8 as *const () || LOG_ALL_IO {
            dbg_log!(
                "read8 port  #{:02x} {}",
                port,
                self.get_port_description(port)
            );
        }
        let v = (iops.read8)(&iops.dev, port);
        v
    }

    pub fn io_port_read16(&self, port: u32) -> u16 {
        let iops = &self.ports[port as usize];
        if iops.read16 as *const () == Self::empty_read16 as *const () || LOG_ALL_IO {
            dbg_log!(
                "read16 port  #{:02x} {}",
                port,
                self.get_port_description(port)
            );
        }
        let v = (iops.read16)(&iops.dev, port);
        v
    }

    pub fn io_port_read32(&self, port: u32) -> u32 {
        let iops = &self.ports[port as usize];
        if iops.read32 as *const () == Self::empty_read32 as *const () || LOG_ALL_IO {
            dbg_log!(
                "read32 port #{:02x} {}",
                port,
                self.get_port_description(port)
            );
        }
        let v = (iops.read32)(&iops.dev, port);
        v
    }

    pub fn io_port_write8(&self, port: u32, data: u8) {
        let iops = &self.ports[port as usize];
        if iops.write8 as *const () == Self::empty_write8 as *const () || LOG_ALL_IO {
            dbg_log!(
                "write8 port  #{:02x} <- 0x{:02x} {}",
                port,
                data,
                self.get_port_description(port)
            );
        }
        (iops.write8)(&iops.dev, port, data);
    }

    pub fn io_port_write16(&self, port: u32, data: u16) {
        let iops = &self.ports[port as usize];
        if iops.write16 as *const () == Self::empty_write16 as *const () || LOG_ALL_IO {
            dbg_log!(
                "write16 port  #{:02x} <- 0x{:02x} {}",
                port,
                data,
                self.get_port_description(port)
            );
        }
        (iops.write16)(&iops.dev, port, data);
    }

    pub fn io_port_write32(&self, port: u32, data: u32) {
        let iops = &self.ports[port as usize];
        if iops.write32 as *const () == Self::empty_write32 as *const () || LOG_ALL_IO {
            dbg_log!(
                "write32 port  #{:02x} <- 0x{:02x} {}",
                port,
                data,
                self.get_port_description(port)
            );
        }
        (iops.write32)(&iops.dev, port, data);
    }

    fn get_port_description(&self, addr: u32) -> String {
        DEBUG_PORTS
            .get(&addr)
            .map_or_else(String::default, |s| format!("({})", s))
    }

    pub(crate) fn mmap_register(
        &mut self,
        addr: u32,
        size: usize,
        r8: Rd8Fn,
        w8: Wr8Fn,
        r32: Rd32Fn,
        w32: Wr32Fn,
    ) {
        dbg_log!("mmap_register addr=0x{:x}  size={:x}", addr >> 0, size);
        assert!((addr & MMAP_BLOCK_SIZE as u32).saturating_sub(1) == 0);
        assert!(size > 0 && (size & MMAP_BLOCK_SIZE - 1) == 0);
        let r32 = if r32 as *const () == Self::empty_read8 as *const () {
            Self::mmap_read32_shim
        } else {
            r32
        };

        let w32 = if w32 as *const () == Self::empty_write32 as *const () {
            Self::mmap_write32_shim
        } else {
            w32
        };
        self.store.cpu_mut().map(|cpu| {
            let mut aligned_addr = addr >> MMAP_BLOCK_BITS;
            let mut size = size;
            while size > 0 {
                cpu.mmap_fn.memory_map_read8[aligned_addr as usize] = r8;
                cpu.mmap_fn.memory_map_read32[aligned_addr as usize] = r32;
                cpu.mmap_fn.memory_map_write8[aligned_addr as usize] = w8;
                cpu.mmap_fn.memory_map_write32[aligned_addr as usize] = w32;
                size -= MMAP_BLOCK_SIZE;
                aligned_addr += 1;
            }
        });
    }

    #[inline]
    fn mmap_write32_shim(dev: &Dev, addr: u32, val: u32) {
        let aligned_addr = addr >> MMAP_BLOCK_BITS;
        dev.cpu_mut().map(|cpu| {
            let mmp_fn = cpu.mmap_fn.memory_map_write8[aligned_addr as usize];
            (mmp_fn)(dev, addr, (val & 0xFF) as u8);
            (mmp_fn)(dev, addr, (val >> 8 & 0xFF) as u8);
            (mmp_fn)(dev, addr, (val >> 16 & 0xFF) as u8);
            (mmp_fn)(dev, addr, (val >> 24) as u8);
        });
    }

    #[inline]
    fn mmap_read32_shim(dev: &Dev, addr: u32) -> u32 {
        let aligned_addr = addr >> MMAP_BLOCK_BITS;
        dev.cpu_mut().map_or(0, |cpu| {
            let mmp_fn = cpu.mmap_fn.memory_map_read8[aligned_addr as usize];
            (mmp_fn)(dev, addr) as u32
                | ((mmp_fn)(dev, addr + 1) as u32) << 8
                | ((mmp_fn)(dev, addr + 2) as u32) << 16
                | ((mmp_fn)(dev, addr + 3) as u32) << 24
        })
    }

    pub(crate) fn init(&mut self) {
        let m_size = self.store.cpu_mut().map(|cpu| cpu.read_mem_size()).unwrap();
        self.mmap_register(
            m_size,
            0x100000000 - m_size as usize,
            |_: &Dev, addr: u32| {
                dbg_log!("Read from unmapped memory space, addr=0x{:x}", addr >> 0);
                0xFF
            },
            |_: &Dev, addr: u32, v: u8| {
                dbg_log!(
                    "Write to unmapped memory space, addr=0x{:x} value={:x}",
                    addr >> 0,
                    v
                );
            },
            |_: &Dev, addr: u32| {
                dbg_log!("Read from unmapped memory space, addr=0x{:x}", addr >> 0);
                0xFFFFFFFF
            },
            |_: &Dev, addr: u32, v: u32| {
                dbg_log!(
                    "Write to unmapped memory space, addr=0x{:x} value={:x}",
                    addr >> 0,
                    v
                );
            },
        );
    }
}

lazy_static! {
    static ref DEBUG_PORTS: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0x0004, "PORT_DMA_ADDR_2");
        m.insert(0x0005, "PORT_DMA_CNT_2");
        m.insert(0x000a, "PORT_DMA1_MASK_REG");
        m.insert(0x000b, "PORT_DMA1_MODE_REG");
        m.insert(0x000c, "PORT_DMA1_CLEAR_FF_REG");
        m.insert(0x000d, "PORT_DMA1_MASTER_CLEAR");
        m.insert(0x0020, "PORT_PIC1_CMD");
        m.insert(0x0021, "PORT_PIC1_DATA");
        m.insert(0x0040, "PORT_PIT_COUNTER0");
        m.insert(0x0041, "PORT_PIT_COUNTER1");
        m.insert(0x0042, "PORT_PIT_COUNTER2");
        m.insert(0x0043, "PORT_PIT_MODE");
        m.insert(0x0060, "PORT_PS2_DATA");
        m.insert(0x0061, "PORT_PS2_CTRLB");
        m.insert(0x0064, "PORT_PS2_STATUS");
        m.insert(0x0070, "PORT_CMOS_INDEX");
        m.insert(0x0071, "PORT_CMOS_DATA");
        m.insert(0x0080, "PORT_DIAG");
        m.insert(0x0081, "PORT_DMA_PAGE_2");
        m.insert(0x0092, "PORT_A20");
        m.insert(0x00a0, "PORT_PIC2_CMD");
        m.insert(0x00a1, "PORT_PIC2_DATA");
        m.insert(0x00b2, "PORT_SMI_CMD");
        m.insert(0x00b3, "PORT_SMI_STATUS");
        m.insert(0x00d4, "PORT_DMA2_MASK_REG");
        m.insert(0x00d6, "PORT_DMA2_MODE_REG");
        m.insert(0x00da, "PORT_DMA2_MASTER_CLEAR");
        m.insert(0x00f0, "PORT_MATH_CLEAR");
        m.insert(0x0170, "PORT_ATA2_CMD_BASE");
        m.insert(0x01f0, "PORT_ATA1_CMD_BASE");
        m.insert(0x0278, "PORT_LPT2");
        m.insert(0x02e8, "PORT_SERIAL4");
        m.insert(0x02f8, "PORT_SERIAL2");
        m.insert(0x0374, "PORT_ATA2_CTRL_BASE");
        m.insert(0x0378, "PORT_LPT1");
        m.insert(0x03e8, "PORT_SERIAL3");
        m.insert(0x03f0, "PORT_FD_BASE");
        m.insert(0x03f2, "PORT_FD_DOR");
        m.insert(0x03f4, "PORT_FD_STATUS");
        m.insert(0x03f5, "PORT_FD_DATA");
        m.insert(0x03f6, "PORT_HD_DATA");
        m.insert(0x03f7, "PORT_FD_DIR");
        m.insert(0x03f8, "PORT_SERIAL1");
        m.insert(0x0cf8, "PORT_PCI_CMD");
        m.insert(0x0cf9, "PORT_PCI_REBOOT");
        m.insert(0x0cfc, "PORT_PCI_DATA");
        m.insert(0x0402, "PORT_BIOS_DEBUG");
        m.insert(0x0510, "PORT_QEMU_CFG_CTL");
        m.insert(0x0511, "PORT_QEMU_CFG_DATA");
        m.insert(0xb000, "PORT_ACPI_PM_BASE");
        m.insert(0xb100, "PORT_SMB_BASE");
        m.insert(0x8900, "PORT_BIOS_AP");
        m
    };
}
