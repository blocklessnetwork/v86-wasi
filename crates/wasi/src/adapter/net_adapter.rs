use crate::{StoreT, bus::BusData, ContextTrait, log::LOG};


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
                            let dst_mac_addr = &v[0..6];
                            let src_mac_addr = &v[6..12];
                            println!("{:#X}:{:#X}:{:#X}:{:#X}:{:#X}:{:#X}", v[0],v[1],v[2],v[3],v[4],v[5]);
                            println!("{:#X}:{:#X}:{:#X}:{:#X}:{:#X}:{:#X}", v[6],v[7],v[8],v[9],v[10],v[11]);
                        }
                        _ => {}
                    }
                }
            );
        });
    }
}
