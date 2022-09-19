#![allow(unused)]
use std::collections::HashMap;

use crate::{Dev, StoreT};

pub(crate) enum BusData {
    None,
    String(String),
    Bool(bool),
    U8(u8),
    U32(u32),
    Vec(Vec<u8>),
    U8Tuple(u8, u8),
    U16Tuple(u16, u16),
    MouseEvent(u8, u8, u8),
    PcspeakerUpdate(u8, u16),
    ScreenPutChar(u16, u16, u8, i32, i32),
    ScreenSetSizeGraphical(u32, u32, u32, u32, u16),
}

impl BusData {
    #[inline]
    pub fn map_string<F>(&self, f: F)
    where
        F: FnOnce(&str),
    {
        match self {
            &BusData::String(ref s) => f(s),
            _ => {}
        }
    }

    #[inline]
    pub fn map_bool<F>(&self, f: F)
    where
        F: FnOnce(bool),
    {
        match self {
            &BusData::Bool(b) => f(b),
            _ => {}
        }
    }

    #[inline]
    pub fn map_u8<F>(&self, f: F)
    where
        F: FnOnce(u8),
    {
        match self {
            &BusData::U8(b) => f(b),
            _ => {}
        }
    }

    #[inline]
    pub fn map_u8tuple<F>(&self, f: F)
    where
        F: FnOnce(u8, u8),
    {
        match self {
            &BusData::U8Tuple(a, b) => f(a, b),
            _ => {}
        }
    }

    #[inline]
    pub fn map_u16tuple<F>(&self, f: F)
    where
        F: FnOnce(u16, u16),
    {
        match self {
            &BusData::U16Tuple(a, b) => f(a, b),
            _ => {}
        }
    }

    #[inline]
    pub fn map_mouse_event<F>(&self, f: F)
    where
        F: FnOnce(u8, u8, u8),
    {
        match self {
            &BusData::MouseEvent(a, b, c) => f(a, b, c),
            _ => {}
        }
    }

    #[inline]
    pub fn map_pc_speaker_update<F>(&self, f: F)
    where
        F: FnOnce(u8, u16),
    {
        match self {
            &BusData::PcspeakerUpdate(a, b) => f(a, b),
            _ => {}
        }
    }

    #[inline]
    pub fn map_screen_put_char<F>(&self, f: F)
    where
        F: FnOnce(u16, u16, u8, i32, i32),
    {
        match self {
            &BusData::ScreenPutChar(a, b, c, d, e) => f(a, b, c, d, e),
            _ => {}
        }
    }

    #[inline]
    pub fn map_screen_set_size_graphical<F>(&self, f: F)
    where
        F: FnOnce(u32, u32, u32, u32, u16),
    {
        match self {
            &BusData::ScreenSetSizeGraphical(a, b, c, d, e) => f(a, b, c, d, e),
            _ => {}
        }
    }
}

pub(crate) type BusCall = fn(dev: &Dev, data: &BusData);

struct BusController {
    listeners: HashMap<String, Vec<*const ()>>,
    store: StoreT,
    dev: Dev,
}

impl BusController {
    fn new(store: StoreT) -> Self {
        Self {
            dev: Dev::Emulator(store.clone()),
            listeners: HashMap::new(),
            store,
        }
    }

    fn register(&mut self, name: &str, call: BusCall) {
        let call = call as *const ();
        match self.listeners.get_mut(name) {
            Some(q) => {
                q.push(call);
            }
            None => {
                self.listeners.insert(String::from(name), vec![call]);
            }
        };
    }

    fn unregister(&mut self, name: &str, call: BusCall) {
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

    pub fn send(&mut self, name: &str, data: BusData) {
        match self.listeners.get(name) {
            Some(v) => {
                v.iter().for_each(|call| {
                    let call = *call;
                    let call = unsafe { std::mem::transmute::<_, BusCall>(call) };
                    call(&self.dev, &data);
                });
            }
            None => {}
        }
    }
}

pub(crate) struct BUS(BusController);

impl BUS {
    pub fn new(store: StoreT) -> Self {
        Self(BusController::new(store))
    }

    #[inline]
    pub fn register(&mut self, name: &str, call: BusCall) {
        self.0.register(name, call);
    }

    #[inline]
    pub fn send(&mut self, name: &str, data: BusData) {
        self.0.send(name, data);
    }
}
