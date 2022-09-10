use std::rc::Rc;

use v86_wasi::{add_x86_to_linker, Emulator, Setting};
use wasmtime::*;

fn main() {
    let mut conf = Config::new();
    conf.cache_config_load_default().unwrap();
    let engine = Engine::new(&conf).unwrap();
    let mut setting = Setting::new();
    setting.bios_file("arch/seabios-debug.bin".into());
    setting.vga_bios_file("arch/vgabios-debug.bin".into());
    let mut emulator = Emulator::new(setting);
    let mut store = Store::new(&engine, emulator.clone());
    let module = Module::from_file(&engine, "target/v86-debug.wasm").unwrap();
    let mut linker: Linker<Emulator> = Linker::new(&engine);
    add_x86_to_linker(&mut linker);
    linker.module(&mut store, "", &module).unwrap();
    let inst = linker.instantiate(&mut store, &module).unwrap();
    emulator.start(inst, Rc::downgrade(&Rc::new(store)));
}
