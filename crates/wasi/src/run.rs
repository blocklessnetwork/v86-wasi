use std::{collections::HashMap, rc::Rc};

use wasmtime::*;

use crate::{
    Setting, 
    set_log_file_name, 
    set_log_mask, 
    Emulator, 
    add_x86_to_linker, 
    WASM_TABLE_SIZE, 
    WASM_TABLE_OFFSET
};

pub fn run_with_setting(setting: Setting) {
    let logger_path = setting.logger_file().map(|f| f.clone());
    set_log_file_name(logger_path);
    set_log_mask(setting.log_mask());
    let mut conf = Config::new();
    conf.cache_config_load_default().unwrap();
    let engine = Engine::new(&conf).unwrap();
    let wasm_file = setting.wasm_file().map(|s| s.clone()).unwrap();
    let mut emulator = Emulator::new(setting);
    let mut store = Store::new(&engine, emulator.clone());
    let module = Module::from_file(&engine, wasm_file).unwrap();
    let mut linker: Linker<Emulator> = Linker::new(&engine);
    let table = create_table(&mut store);
    add_x86_to_linker(&mut linker, &mut store, table);
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
    let ty = TableType::new(RefType::FUNCREF, WASM_TABLE_SIZE + WASM_TABLE_OFFSET, None);
    let table = Table::new(store.as_context_mut(), ty, Ref::Func(None)).unwrap();
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

