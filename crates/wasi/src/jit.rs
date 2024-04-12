use std::{sync::mpsc::{Receiver, Sender}, slice, collections::HashMap};

use wasmtime::*;

use crate::{StoreT, ContextTrait};

pub(crate) enum JitMsg {
    Quit,
    JitParams(i32, i32, i32, i32, i32),
    JitResult(i32, i32, i32, Ref)
}

pub(crate) struct JitWorker {
    pub(crate) mem: Memory,
    pub(crate) store: StoreT,
    pub(crate) externs: HashMap<String, Extern>,
    pub(crate) recv: Receiver<JitMsg>,
    pub(crate) sender: Sender<JitMsg>,
}

impl JitWorker {
    pub fn compile(&mut self) {
        loop {
            let (index, start, state_flags, ptr, len) = match self.recv.recv() {
                Ok(JitMsg::JitParams(index, start, state_flags, ptr, len)) => (index, start, state_flags, ptr, len),
                Ok(_) => return,
                Err(_) => return,
            };
            self.store.store_mut().map(|store| {
                let eng = unsafe {
                    std::mem::transmute(store.engine())
                };
                let data_ptr = self.mem.data_ptr(store.as_context_mut());
                let code = unsafe {
                    slice::from_raw_parts_mut(data_ptr.offset(ptr as isize), len as usize)
                };
                let module = {
                    wasmtime::Module::new(eng, code).unwrap()
                };
                let names: Vec<String> = module.imports().map(|i| i.name().into()).collect();
                let externs = self.wasm_externs(names);
                let inst = Instance::new(store.as_context_mut(), &module, &externs).unwrap();
                let func = inst.get_func(store.as_context_mut(), "f");
                assert!(func.is_some());
                let func = Ref::Func(func);
                let rs = JitMsg::JitResult(index, start, state_flags, func);
                self.sender.send(rs).unwrap();
            });
        }
    }

    #[inline]
    pub(crate) fn wasm_externs(&self, names: Vec<String>) -> Vec<Extern> {
        let mut rs = Vec::new();
        for n in names {
            let val = self.externs.get(&n);
            if val.is_some() {
                rs.push(val.unwrap().clone());
            }
        }
        rs
    }
    
}