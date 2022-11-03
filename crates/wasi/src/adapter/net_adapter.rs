use crossbeam_channel::{Receiver, Sender, TryRecvError};

use crate::{StoreT, bus::BusData, ContextTrait};


pub(crate) struct NetAdapter {
    store: StoreT,
    rx: Option<Receiver<Vec<u8>>>,
    tx: Sender<Vec<u8>>,
}

impl NetAdapter {
    pub fn new(store: StoreT, rx: Receiver<Vec<u8>>, tx: Sender<Vec<u8>>) -> Self {
        Self { 
            rx: Some(rx),
            tx,
            store,
        }
    }

    pub fn try_recv_from_tun(&mut self) {
        let rm_bool = self.rx.as_mut().map_or(false, |rx| {
            match rx.try_recv() {
                Ok(d) => {
                    self.store.bus_mut().map(|bus| {
                        bus.send("net0-receive", BusData::Vec(d));
                    });
                    false
                },
                Err(TryRecvError::Empty) => false,
                Err(TryRecvError::Disconnected) => true,
            }
        });
        if rm_bool {
            self.rx.take();
        }
    }

    pub fn init(&mut self) {
        self.store.bus_mut().map(|bus| {
            bus.register("net0-send", 
                |s: &StoreT, data: &BusData| {
                    s.net_adp_mut().map(|n| n.tx.send(data.to_vec()));
                }
            );
        });
    }
}
