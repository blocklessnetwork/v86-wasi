use lazy_static::lazy_static;
use wasmtime::{Memory, AsContextMut, AsContext};
use std::{collections::HashMap, ops::Add, marker::PhantomData};

use crate::Dev;

pub(crate) trait MemAccessTrait<T> {
    fn read(&self, store: impl AsContext, idx: u32) -> T;
    fn write(&mut self, store: impl AsContextMut, idx: u32, v: T);
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
            fn read(&self,store: impl AsContext, idx: u32) -> $t {
                let mut d = [0u8; $s];
                self.mem.read(store, self.offset.add(idx as usize), &mut d).unwrap();
                let rs: $t = <$t>::from_le_bytes(d);
                return rs;
            }
        
            fn write(&mut self, store: impl AsContextMut, idx: u32, v: $t) {
                let d = v.to_le_bytes();
                self.mem.write(store, self.offset.add(idx as usize), &d).unwrap();
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
}

impl IO {
    fn empty_read8(_: &Dev, _: u32) -> u8 {
        0xFF
    }

    fn empty_read16(_: &Dev, _: u32) -> u16 {
        0xFFFF
    }

    fn empty_read32(_: &Dev, _: u32) -> u32 {
        0xFFFF_FFFF
    }

    fn empty_write8(_: &Dev, _: u32, _: u8) {}

    fn empty_write16(_: &Dev, _: u32, _: u16) {}

    fn empty_write32(_: &Dev, _: u32, _: u32) {}

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

    pub fn new() -> IO {
        let mut v = Vec::new();
        for _ in 0..PORTS_SIZE {
            v.push(IO::default_iops());
        }
        IO { ports: v }
    }

    pub fn register_read(&mut self, port: u32, dev: Dev, r8: Rd8Fn, r16: Rd16Fn, r32: Rd32Fn) {
        let iops = &mut self.ports[port as usize];
        iops.read8 = r8;
        iops.read16 = r16;
        iops.read32 = r32;
        iops.dev = dev;
    }

    pub fn register_write(&mut self, port: u32, dev: Dev, w8: Wr8Fn, w16: Wr16Fn, w32: Wr32Fn) {
        let iops = &mut self.ports[port as usize];
        iops.write8 = w8;
        iops.write16 = w16;
        iops.write32 = w32;
        iops.dev = dev;
    }

    pub fn io_port_read8(&self, port: u32) -> u8 {
        let iops = &self.ports[port as usize];
        if iops.read8 as *const () == Self::empty_read8 as *const () {
            debug!(
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
        if iops.read16 as *const () == Self::empty_read16 as *const () {
            debug!(
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
        if iops.read32 as *const () == Self::empty_read32 as *const () {
            debug!(
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
        if iops.write8 as *const () == Self::empty_write8 as *const () {
            debug!(
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
        if iops.write16 as *const () == Self::empty_write16 as *const () {
            debug!(
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
        if iops.write32 as *const () == Self::empty_write32 as *const () {
            debug!(
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
