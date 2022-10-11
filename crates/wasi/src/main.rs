use std::{rc::Rc, collections::HashMap};

use v86_wasi::{add_x86_to_linker, Emulator, Setting, WASM_TABLE_OFFSET, WASM_TABLE_SIZE};
use wasmtime::*;

fn main() {
    let mut conf = Config::new();
    conf.cache_config_load_default().unwrap();
    let engine = Engine::new(&conf).unwrap();
    let mut setting = Setting::new();
    setting.bios_file("arch/seabios-debug.bin".into());
    setting.vga_bios_file("arch/vgabios-debug.bin".into()); 


    setting.bzimage_file("arch/buildroot-bzimage.bin".into());
    setting.cmdline("tsc=reliable mitigations=off random.trust_cpu=on".into());
    
    // setting.cdrom_file("arch/linux4.iso".into()); 
    let mut emulator = Emulator::new(setting);
    let mut store = Store::new(&engine, emulator.clone());
    let module = Module::from_file(&engine, "target/v86-debug.wasm").unwrap();
    let mut linker: Linker<Emulator> = Linker::new(&engine);
    let table = create_table(&mut store);
    add_x86_to_linker(&mut linker, table);
    let inst = linker.instantiate(&mut store, &module).unwrap();
    linker.instance(&mut store, "e", inst).unwrap();
    let mem = inst.get_export(&mut store, "memory").unwrap();
    let mut ex = exports(&mut store, &inst);
    ex.insert("m".into(), mem);
    emulator.start(ex, table, inst, Rc::downgrade(&Rc::new(store)));
    emulator.shutdown();
}

#[inline]
fn create_table(store: &mut Store<Emulator>) -> Table {
    let ty = TableType::new(ValType::FuncRef, WASM_TABLE_SIZE + WASM_TABLE_OFFSET, None);
    let table = Table::new(store.as_context_mut(), ty, Val::FuncRef(None)).unwrap();
    table
}

#[inline]
fn exports(store: &mut Store<Emulator>, inst: &Instance) -> HashMap<String, Extern> {
    inst.exports(store.as_context_mut())
        .map(|m| {
            let n = m.name();
            (n.into(), m.into_extern())
        })
        .collect::<HashMap<String, Extern>>()
}

