#[macro_use]
mod log;

use std::slice;
use std::rc::Weak;

const ALL_DEBUG: bool = true;
const LOG_ALL_IO: bool = false;

use bus::BUS;
use wasmtime::*;
use dma::DMA;
use io::IO;
use mem::add_mem_to_linker;
use pci::PCI;
use pic::PIC;
use rtc::RTC;
pub(crate) mod consts;
mod cpu;
mod debug;
mod dev;
mod bus;
mod dma;
mod vga;
mod io;
mod mem;
mod pci;
mod pic;
mod rtc;
mod emulator;
mod setting;
pub use consts::*;
pub use cpu::CPU;
pub use emulator::Emulator;
pub use setting::*;

pub use dev::Dev;

macro_rules! copy_impl {
    ($name: ident, $type: ty, $l: literal) => {
        pub fn $name(src: &[u8], dst: &mut [$type]) {
            let mut bs = [0u8; $l];
            for i in 0..dst.len() {
                let start = i * $l;
                let end = start + $l;
                bs.copy_from_slice(&src[start..end]);
                let t: $type = <$type>::from_le_bytes(bs);
                dst[i] = t;
            }
        }
    };
}


macro_rules! read_impl {
    ($name: ident, $type: ty, $l: literal) => {
        pub fn $name(src: &[u8], idx: usize) -> $type {
            let mut bs = [0u8; $l];
            bs.copy_from_slice(&src[idx * $l..(idx * $l + $l)]);
            <$type>::from_le_bytes(bs)
        }
    };
}

macro_rules! write_impl {
    ($name: ident, $type: ty, $l: literal) => {
        pub fn $name(src: &mut [u8], idx: usize, v: $type) {
            let bs = v.to_le_bytes();
            let dst = &mut src[idx * $l..(idx * $l + $l)];
            dst.copy_from_slice(&bs);
        }
    };
}

pub(crate) mod utils {
    copy_impl!(copy_to_i32s, i32, 4);

    read_impl!(read_i32, i32, 4);
    read_impl!(read_u32, u32, 4);
    read_impl!(read_u16, u16, 2);
    read_impl!(read_i16, i16, 2);
    write_impl!(write_i32, i32, 4);
    write_impl!(write_u32, u32, 4);
    write_impl!(write_u16, u16, 2);
    write_impl!(write_i16, i16, 2);
}

trait EmulatorTrait {
    fn cpu_mut(&self) -> Option<&mut CPU>;
    fn cpu(&self) -> Option<&CPU>;
    fn rtc_mut(&self) -> Option<&mut RTC>;
    fn io_mut(&self) -> Option<&mut IO>;
    fn dma_mut(&self) -> Option<&mut DMA>;
    fn pic_mut(&self) -> Option<&mut PIC>;
    fn pic(&self) -> Option<&PIC>;
    fn io(&self) -> Option<&IO>;
    fn bus_mut(&self) -> Option<&mut BUS>;
    fn bus(&self) -> Option<&BUS>;
    fn pci_mut(&self) -> Option<&mut PCI>;
    fn pci(&self) -> Option<&PCI>;
    fn emulator(&self) -> &Emulator;
    fn emulator_mut(&self) -> &mut Emulator;
}

impl EmulatorTrait for Weak<Store<Emulator>> {
    #[inline]
    fn pic_mut(&self) -> Option<&mut PIC> {
        self.emulator_mut().pic_mut()
    }

    #[inline]
    fn pic(&self) -> Option<&PIC> {
        self.emulator_mut().pic()
    }

    #[inline]
    fn pci_mut(&self) -> Option<&mut PCI> {
        self.emulator_mut().pci_mut()
    }

    #[inline]
    fn bus_mut(&self) -> Option<&mut BUS> {
        self.emulator_mut().bus_mut()
    }
    
    #[inline]
    fn bus(&self) -> Option<&BUS> {
        self.emulator_mut().bus()
    }

    #[inline]
    fn pci(&self) -> Option<&PCI> {
        self.emulator().pci()
    }

    #[inline]
    fn cpu_mut(&self) -> Option<&mut CPU> {
        let emu = self.emulator_mut();
        emu.cpu_mut()
    }

    #[inline]
    fn cpu(&self) -> Option<&CPU> {
        let emu = self.emulator();
        emu.cpu()
    }

    #[inline]
    fn io_mut(&self) -> Option<&mut IO> {
        self.emulator_mut().io_mut()
    }

    #[inline]
    fn io(&self) -> Option<&IO> {
        self.emulator().io()
    }

    #[inline]
    fn dma_mut(&self) -> Option<&mut DMA> {
        self.emulator_mut().dma_mut()
    }

    #[inline]
    fn emulator_mut(&self) -> &mut Emulator {
        unsafe { (*(self.as_ptr() as *mut Store<_>)).data_mut() }
    }

    #[inline]
    fn emulator(&self) -> &Emulator {
        unsafe { (*(self.as_ptr() as *mut Store<_>)).data() }
    }

    #[inline]
    fn rtc_mut(&self) -> Option<&mut RTC> {
        let emu = self.emulator();
        emu.cpu_mut().map(|cpu| &mut cpu.rtc)
    }
}

pub fn add_x86_to_linker(linker: &mut Linker<Emulator>) {
    linker
        .func_wrap(
            "env",
            "log_from_wasm",
            move |mut caller: Caller<'_, Emulator>, off: u32, len: u32| {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(m)) => m,
                    _ => {
                        return Err(Trap::new("missing required memory export"));
                    }
                };

                let (mem, _ctx) = mem.data_and_store_mut(&mut caller);
                unsafe {
                    let ptr = mem.as_ptr().offset(off as _);
                    let sl = slice::from_raw_parts(ptr, len as _);
                    println!("{}", std::str::from_utf8_unchecked(sl));
                }
                Ok(())
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "console_log_from_wasm",
            move |mut caller: Caller<'_, Emulator>, off: u32, len: u32| {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(m)) => m,
                    _ => {
                        return Err(Trap::new("missing required memory export"));
                    }
                };

                let (mem, _ctx) = mem.data_and_store_mut(&mut caller);
                unsafe {
                    let ptr = mem.as_ptr().offset(off as _);
                    let sl = slice::from_raw_parts(ptr, len as _);
                    eprintln!("{}", std::str::from_utf8_unchecked(sl));
                }
                Ok(())
            },
        )
        .unwrap();

    linker
        .func_wrap("env", "abort", move |mut _caller: Caller<'_, Emulator>| {
            panic!("env abort call.");
        })
        .unwrap();

    linker
        .func_wrap("env", "hlt_op", move |mut _caller: Caller<'_, Emulator>| {
            panic!("env hlt_op call.");
        })
        .unwrap();

    linker
        .func_wrap(
            "env",
            "get_rand_int",
            move |mut _caller: Caller<'_, Emulator>| -> i32 {
                panic!("env get_rand_int call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "cpu_exception_hook",
            move |mut _caller: Caller<'_, Emulator>, _i: i32| -> i32 {
                panic!("env cpu_exception_hook call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "microtick",
            move |caller: Caller<'_, Emulator>| -> f64 {
                let emu = caller.data();
                emu.microtick()
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "dbg_trace_from_wasm",
            move |mut _caller: Caller<'_, Emulator>| {
                panic!("env dbg_trace_from_wasm call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "pic_acknowledge",
            move |mut _caller: Caller<'_, Emulator>| {
                panic!("env pic_acknowledge call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "io_port_read8",
            move |mut caller: Caller<'_, Emulator>, port: i32| -> i32 {
                let emu = caller.data_mut();
                emu.cpu_mut()
                    .map_or(0, |cpu| cpu.io.io_port_read8(port as _) as _)
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "io_port_read16",
            move |mut caller: Caller<'_, Emulator>, port: i32| -> i32 {
                let emu = caller.data_mut();
                emu.cpu_mut()
                    .map_or(0, |cpu| cpu.io.io_port_read16(port as _) as _)
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "io_port_read32",
            move |mut caller: Caller<'_, Emulator>, port: i32| -> i32 {
                let emu = caller.data_mut();
                emu.cpu_mut()
                    .map_or(0, |cpu| cpu.io.io_port_read32(port as _) as _)
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "io_port_write8",
            move |mut caller: Caller<'_, Emulator>, port: i32, data: i32| {
                let emu = caller.data_mut();
                emu.cpu_mut().map(|cpu| {
                    cpu.io.io_port_write8(port as _, data as _);
                });
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "io_port_write16",
            move |mut caller: Caller<'_, Emulator>, port: i32, data: i32| {
                let emu = caller.data_mut();
                emu.cpu_mut().map(|cpu| {
                    cpu.io.io_port_write16(port as _, data as _);
                });
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "io_port_write32",
            move |mut caller: Caller<'_, Emulator>, port: i32, data: i32| {
                let emu = caller.data_mut();
                emu.cpu_mut().map(|cpu| {
                    cpu.io.io_port_write32(port as _, data as _);
                });
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_read8",
            move |mut caller: Caller<'_, Emulator>, addr: i32| -> i32 {
                let emu = caller.data_mut();
                emu.cpu_mut()
                    .map_or(0, |cpu| cpu.mmap_read8(addr as u32) as i32)
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_read16",
            move |mut caller: Caller<'_, Emulator>, addr: i32| -> i32 {
                let emu = caller.data_mut();
                emu.cpu_mut()
                    .map_or(0, |cpu| cpu.mmap_read16(addr as u32) as i32)
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_read32",
            move |mut caller: Caller<'_, Emulator>, addr: i32| -> i32 {
                let emu = caller.data_mut();
                emu.cpu_mut()
                    .map_or(0, |cpu| cpu.mmap_read32(addr as u32) as i32)
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_write8",
            move |mut caller: Caller<'_, Emulator>, addr: i32, v: i32| {
                let emu = caller.data_mut();
                emu.cpu_mut().map(|cpu| {
                    cpu.mmap_write8(addr as u32, v as u8);
                });
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_write16",
            move |mut caller: Caller<'_, Emulator>, addr: i32, v: i32| {
                let emu = caller.data_mut();
                emu.cpu_mut().map(|cpu| {
                    cpu.mmap_write16(addr as u32, v as u16);
                });
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_write32",
            move |mut caller: Caller<'_, Emulator>, addr: i32, v: i32| {
                let emu = caller.data_mut();
                emu.cpu_mut().map(|cpu| {
                    cpu.mmap_write32(addr as u32, v as u32);
                });
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_write64",
            move |mut _caller: Caller<'_, Emulator>, _off: i32, _v0: i32, _v1: i32| {
                panic!("env mmap_write64 call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_write128",
            move |mut _caller: Caller<'_, Emulator>,
                  _off: i32,
                  _v0: i32,
                  _v1: i32,
                  _v2: i32,
                  _v3: i32| {
                panic!("env mmap_write128 call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "codegen_finalize",
            move |mut _caller: Caller<'_, Emulator>,
                  _i: i32,
                  _addr: u32,
                  _f: i32,
                  _ptr: i32,
                  _l: i32| {
                panic!("env codegen_finalize call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "jit_clear_func",
            move |mut _caller: Caller<'_, Emulator>, _i: i32| {
                panic!("env jit_clear_func call.");
            },
        )
        .unwrap();

    add_mem_to_linker(linker);
}
