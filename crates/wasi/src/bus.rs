use std::{collections::HashMap, rc::Weak};

use wasmtime::Store;

use crate::{Emulator, EmulatorTrait};

pub(crate) enum BusData {
    None,
    Bool(bool),
    U8Tuple(u8, u8),
    ScreenSetSizeGraphical(u32, u32, u32, u32, u16),
    ScreenPutChar(u8, u8, u8, i32, i32)
}

pub(crate) type BusCall = fn(store: &Weak<Store<Emulator>>, data: &BusData);

struct BusController {
    listeners: HashMap<&'static str, Vec<*const ()>>,
    store: Weak<Store<Emulator>>,
}

impl BusController {
    fn new(store: Weak<Store<Emulator>>) -> Self {
        Self {
            listeners: HashMap::new(),
            store,
        }
    }

    fn register(&mut self, name: &'static str, call: BusCall) {
        let call = call as *const ();
        match self.listeners.get_mut(name) {
            Some(q) => {
                q.push(call);
            }
            None => {
                self.listeners.insert(name, vec![call]);
            }
        };
    }

    fn unregister(&mut self, name: &'static str, call: BusCall) {
        let call = call as *const ();
        match self.listeners.get_mut(name) {
            Some(q) => match q.binary_search(&call) {
                Ok(i) => {
                    q.remove(i);
                }
                Err(_) => {}
            },
            None => (),
        };
    }

    pub fn send(&mut self, name: &'static str, data: BusData) {
        match self.listeners.get(name) {
            Some(v) => {
                v.iter().for_each(|call| {
                    let call = unsafe { std::mem::transmute::<_, BusCall>(call) };
                    call(&self.store, &data);
                });
            }
            None => {}
        }
    }
}

pub(crate) struct BUS(BusController);

impl BUS {
    pub fn new(store: Weak<Store<Emulator>>) -> Self {
        Self(BusController::new(store))
    }

    #[inline]
    pub fn send(&mut self, name: &'static str, data: BusData) {
        self.0.send(name, data);
    }
}
