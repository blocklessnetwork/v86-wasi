#[macro_use]
mod log;

use core::slice;

use mem::add_mem_to_linker;
use wasmtime::*;
mod consts;
mod cpu;
mod dev;
mod emulator;
mod io;
mod mem;
mod rtc;
mod setting;
pub use setting::*;
pub use consts::*;
pub use cpu::CPU;
pub use emulator::Emulator;

pub use dev::Dev;

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
                emu.cpu_mut().map_or(0, |cpu| {
                    cpu.io.io_port_read8(port as _) as _
                })
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "io_port_read16",
            move |mut caller: Caller<'_, Emulator>, port: i32| -> i32 {
                let emu = caller.data_mut();
                emu.cpu_mut().map_or(0, |cpu| {
                    cpu.io.io_port_read16(port as _) as _
                })
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "io_port_read32",
            move |mut caller: Caller<'_, Emulator>, port: i32| -> i32 {
                let emu = caller.data_mut();
                emu.cpu_mut().map_or(0, |cpu| {
                    cpu.io.io_port_read32(port as _) as _
                })
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
            move |mut _caller: Caller<'_, Emulator>, _off: i32, _v: i32| {
                panic!("env io_port_write16 call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "io_port_write32",
            move |mut _caller: Caller<'_, Emulator>, _off: i32, _v: i32| {
                panic!("env io_port_write32 call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_read8",
            move |mut _caller: Caller<'_, Emulator>, _off: i32| -> i32 {
                panic!("env mmap_read8 call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_read16",
            move |mut _caller: Caller<'_, Emulator>, _off: i32| -> i32 {
                panic!("env mmap_read16 call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_read32",
            move |mut _caller: Caller<'_, Emulator>, _off: i32| -> i32 {
                panic!("env mmap_read32 call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_write8",
            move |mut _caller: Caller<'_, Emulator>, _off: i32, _v: i32| {
                panic!("env mmap_write8 call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_write16",
            move |mut _caller: Caller<'_, Emulator>, _off: i32, _v: i32| {
                panic!("env mmap_write16 call.");
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "env",
            "mmap_write32",
            move |mut _caller: Caller<'_, Emulator>, _off: i32, _v: i32| {
                panic!("env mmap_write32 call.");
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
