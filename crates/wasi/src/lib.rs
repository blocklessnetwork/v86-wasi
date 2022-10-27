#[macro_use]
mod log;

use std::rc::Weak;
use std::slice;

const ALL_DEBUG: bool = true;

type StoreT = Weak<Store<Emulator>>;
use wasmtime::*;

mod io;
mod bus;
mod cpu;
mod dev;
mod dma;
mod jit;
mod mem;
mod pic;
mod pci;
mod ide;
mod pit;
mod ps2;
mod rtc;
mod ne2k;
mod vga;
mod uart;
mod debug;
mod ws_thr;
mod floppy;
mod kernel;
mod adapter;
mod storage;
mod setting;
mod emulator;
pub mod consts;

use io::IO;
use bus::BUS;
use dma::DMA;
use pci::PCI;
use pic::PIC;
use pit::PIT;
use ps2::PS2;
use rtc::RTC;
use ne2k::Ne2k;
use uart::UART;
pub use cpu::CPU;
use ide::IDEDevice;
use vga::VGAScreen;
pub use consts::*;
pub use setting::*;
pub(crate) use log::LOG;
pub use emulator::Emulator;
use adapter::NetTermAdapter;
use floppy::FloppyController;
pub(crate) use storage::FileBuffer;
pub use dev::Dev;

pub use log::set_log_file_name;
pub use log::set_log_mask;

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

trait ContextTrait {
    fn cpu_mut(&self) -> Option<&mut CPU>;
    fn cpu(&self) -> Option<&CPU>;

    fn store(&self) -> Option<&Store<Emulator>>;
    fn store_mut(&self) -> Option<&mut Store<Emulator>>;

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

    fn setting(&self) -> &Setting;

    fn vga(&self) -> Option<&VGAScreen>;
    fn vga_mut(&self) -> Option<&mut VGAScreen>;

    fn uart0_mut(&self) -> Option<&mut UART>;
    fn uart0(&self) -> Option<&UART>;

    fn ps2_mut(&self) -> Option<&mut PS2>;
    fn ps2(&self) -> Option<&PS2>;

    fn fdc_mut(&self) -> Option<&mut FloppyController>;
    fn fdc(&self) -> Option<&FloppyController>;

    fn pit_mut(&self) -> Option<&mut PIT>;
    fn pit(&self) -> Option<&PIT>;

    fn ne2k_mut(&self) -> Option<&mut Ne2k>;
    fn ne2k(&self) -> Option<&Ne2k>;

    fn microtick(&self) -> f64;

    fn ide(&self) -> Option<&IDEDevice>;
    fn ide_mut(&self) -> Option<&mut IDEDevice>;

    fn net_term_adp_mut(&self) -> Option<&mut NetTermAdapter>;
    fn net_term_adp(&self) -> Option<&NetTermAdapter>;
}

impl ContextTrait for StoreT {

    #[inline]
    fn store(&self) -> Option<&Store<Emulator>> {
        unsafe {
            Some(&(*(self.as_ptr())))
        }    
    }

    #[inline]
    fn store_mut(&self) -> Option<&mut Store<Emulator>>  {
        unsafe {
            Some(&mut (*(self.as_ptr() as *mut Store<_>)))
        }
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
    fn rtc_mut(&self) -> Option<&mut RTC> {
        let emu = self.emulator();
        emu.cpu_mut().map(|cpu| &mut cpu.rtc)
    }

    #[inline]
    fn io_mut(&self) -> Option<&mut IO> {
        self.emulator_mut().io_mut()
    }

    #[inline]
    fn dma_mut(&self) -> Option<&mut DMA> {
        self.emulator_mut().dma_mut()
    }

    #[inline]
    fn pic_mut(&self) -> Option<&mut PIC> {
        self.emulator_mut().pic_mut()
    }

    #[inline]
    fn pic(&self) -> Option<&PIC> {
        self.emulator_mut().pic()
    }

    #[inline]
    fn io(&self) -> Option<&IO> {
        self.emulator().io()
    }

    #[inline]
    fn bus_mut(&self) -> Option<&mut BUS> {
        self.emulator_mut().bus_mut()
    }

    #[inline]
    fn bus(&self) -> Option<&BUS> {
        self.emulator().bus()
    }

    #[inline]
    fn ide(&self) -> Option<&IDEDevice> {
        self.emulator().ide()
    }

    #[inline]
    fn ide_mut(&self) -> Option<&mut IDEDevice> {
        self.emulator_mut().ide_mut()
    }

    #[inline]
    fn pci_mut(&self) -> Option<&mut PCI> {
        self.emulator_mut().pci_mut()
    }

    #[inline]
    fn pci(&self) -> Option<&PCI> {
        self.emulator().pci()
    }

    #[inline]
    fn microtick(&self) -> f64 {
        self.emulator().microtick()
    }

    #[inline(always)]
    fn emulator(&self) -> &Emulator {
        unsafe { (*(self.as_ptr() as *mut Store<_>)).data() }
    }

    #[inline(always)]
    fn emulator_mut(&self) -> &mut Emulator {
        unsafe { (*(self.as_ptr() as *mut Store<_>)).data_mut() }
    }

    #[inline]
    fn setting(&self) -> &Setting {
        let emu = self.emulator();
        emu.setting()
    }

    #[inline]
    fn vga(&self) -> Option<&VGAScreen> {
        self.cpu().map(|cpu| &cpu.vga)
    }

    #[inline]
    fn vga_mut(&self) -> Option<&mut VGAScreen> {
        self.cpu_mut().map(|cpu| &mut cpu.vga)
    }

    #[inline]
    fn uart0_mut(&self) -> Option<&mut UART> {
        self.emulator_mut().uart0_mut()
    }

    #[inline]
    fn uart0(&self) -> Option<&UART> {
        self.emulator().uart0()
    }

    #[inline]
    fn ps2_mut(&self) -> Option<&mut PS2> {
        self.emulator_mut().ps2_mut()
    }

    #[inline]
    fn ps2(&self) -> Option<&PS2> {
        self.emulator().ps2()
    }

    #[inline]
    fn fdc_mut(&self) -> Option<&mut FloppyController> {
        self.emulator().fdc_mut()
    }

    #[inline]
    fn fdc(&self) -> Option<&FloppyController> {
        self.emulator().fdc()
    }

    #[inline]
    fn pit_mut(&self) -> Option<&mut PIT> {
        self.emulator().pit_mut()
    }

    #[inline]
    fn pit(&self) -> Option<&PIT> {
        self.emulator().pit()
    }

    #[inline]
    fn ne2k_mut(&self) -> Option<&mut Ne2k> {
        self.emulator().ne2k_mut()
    }

    #[inline]
    fn ne2k(&self) -> Option<&Ne2k> {
        self.emulator().ne2k()
    }

    #[inline]
    fn net_term_adp_mut(&self) -> Option<&mut NetTermAdapter> {
        self.emulator_mut().net_term_adp_mut()
    }

    #[inline]
    fn net_term_adp(&self) -> Option<&NetTermAdapter> {
        self.emulator().net_term_adp()
    }
}

pub fn add_x86_to_linker(linker: &mut Linker<Emulator>, table: Table) {
    linker
        .define("env", "__indirect_function_table", table)
        .unwrap();
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
                    dbg_log!(LOG::CPU, "{}", std::str::from_utf8_unchecked(sl));
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
        .func_wrap("env", "hlt_op", move |mut caller: Caller<'_, Emulator>| {
            let emu = caller.data_mut();
            emu.cpu_mut().map(|cpu| {
                cpu.hlt_op();
            });
        })
        .unwrap();

    linker
        .func_wrap(
            "env",
            "get_rand_int",
            move |mut _caller: Caller<'_, Emulator>| -> i32 { rand::random::<i32>() },
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
            move |mut caller: Caller<'_, Emulator>| {
                caller.data_mut().cpu_mut().map(|cpu| {
                    cpu.pic_acknowledge();
                });
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
            move |mut caller: Caller<'_, Emulator>,
                  index: i32,
                  start: i32,
                  state_flags: i32,
                  ptr: i32,
                  len: i32| {
                let emu = caller.data_mut();
                emu.jit(jit::JitMsg::JitParams(index, start, state_flags, ptr, len));
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "jit_clear_func",
            move |mut caller: Caller<'_, Emulator>, index: u32| {
                let func = Val::FuncRef(None);
                let emu: &'static Emulator = unsafe {
                    std::mem::transmute(caller.data())
                };
                let table = emu.wasm_table();
                table.set(caller.as_context_mut(), index, func).unwrap();
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "jit_clear_all_funcs",
            move |mut caller: Caller<'_, Emulator>| {
                let func = Val::FuncRef(None);
                let emu: &'static Emulator = unsafe {
                    std::mem::transmute(caller.data())
                };
                let table = emu.wasm_table();
                for i in 0..WASM_TABLE_SIZE {
                    table.set(caller.as_context_mut(), i + WASM_TABLE_OFFSET, func.clone()).unwrap();
                }
                
            },
        )
        .unwrap();
    mem::add_mem_to_linker(linker);
}
