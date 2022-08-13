use v86_wasi::{add_x86_to_linker, CPU};
use wasmtime::*;

fn main() {
    let mut conf = Config::new();
    conf.cache_config_load_default().unwrap();
    let engine = Engine::new(&conf).unwrap();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(&engine, "target/v86-debug.wasm").unwrap();
    let mut linker: Linker<()> = Linker::new(&engine);
    add_x86_to_linker(&mut linker);
    linker.module(&mut store, "", &module).unwrap();
    let inst = linker.instantiate(&mut store, &module).unwrap();
    // let mem = inst.get_memory(store.as_context_mut(), "memory").unwrap();
    // println!("{}", mem.data_size(store.as_context()));
    // let func = inst
    //     .get_typed_func::<(), (), _>(&mut store, "start")
    //     .unwrap();
    // func.call(store.as_context_mut(), ()).unwrap();
    CPU::new(inst, store).init();
    
}
