use std::rc::Rc;

use v86_wasi::{add_x86_to_linker, Emulator, Setting, WASM_TABLE_OFFSET, WASM_TABLE_SIZE};
use wasmtime::*;

fn main() {
    let mut conf = Config::new();
    conf.cache_config_load_default().unwrap();
    let engine = Engine::new(&conf).unwrap();
    let mut setting = Setting::new();
    setting.bzimage_file("arch/buildroot-bzimage.bin".into());
    setting.cmdline("tsc=reliable mitigations=off random.trust_cpu=on".into());
    setting.bios_file("arch/seabios-debug.bin".into());
    setting.vga_bios_file("arch/vgabios-debug.bin".into());
    let mut emulator = Emulator::new(setting);
    let mut store = Store::new(&engine, emulator.clone());
    let module = Module::from_file(&engine, "target/v86-debug.wasm").unwrap();
    let mut linker: Linker<Emulator> = Linker::new(&engine);
    let ty = TableType::new(ValType::FuncRef, WASM_TABLE_SIZE + WASM_TABLE_OFFSET, None);
    let table = Table::new(&mut store, ty, Val::FuncRef(None)).unwrap();
    add_x86_to_linker(&mut linker, table);
    linker.module(&mut store, "", &module).unwrap();
    let inst = linker.instantiate(&mut store, &module).unwrap();
    emulator.start(inst, Rc::downgrade(&Rc::new(store)));
}
