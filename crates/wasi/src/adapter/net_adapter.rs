use crate::{StoreT, bus::BusData, ContextTrait};


pub(crate) struct NetAdapter {
    store: StoreT,
}

impl NetAdapter {
    pub fn new(store: StoreT) -> Self {
        Self { 
            store
        }
    }

    pub fn init(&mut self) {
        self.store.bus_mut().map(|bus| {
            bus.register("net0-send", 
                |s: &StoreT, data: &BusData| {
                    match data {
                        &BusData::Vec(ref v) => {
                            println!("vec {}", v.len());
                        }
                        _ => {}
                    }
                });
        });
    }
}