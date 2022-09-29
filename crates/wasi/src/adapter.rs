use std::sync::mpsc::{Receiver, Sender};

use crate::{StoreT, ContextTrait, bus::BusData};


pub(crate) struct NetTermAdapter {
    store: StoreT,
    sender: Option<Sender<(u16, BusData)>>,
    sender_rx: Receiver<Sender<(u16, BusData)>>,

}

impl NetTermAdapter {

    pub fn new(store: StoreT, sender_rx: Receiver<Sender<(u16, BusData)>>) -> Self {
        Self {
            store,
            sender_rx,
            sender: None,
        }
    }

    pub fn init(&mut self) {
        self.store.bus_mut().map(|bus| {
            bus.register("screen-set-mode", 
                |s: &StoreT, data: &BusData| Self::try_send(s, 1, data.clone()));
            bus.register("screen-put-char", 
                |s: &StoreT, data: &BusData| Self::try_send(s, 2, data.clone()));
            bus.register("screen-set-size-text", 
                |s: &StoreT, data: &BusData| Self::try_send(s, 3, data.clone()));
            bus.register("screen-update-cursor-scanline", 
                |s: &StoreT, data: &BusData| Self::try_send(s, 4, data.clone()));
        });
    }

    #[inline]
    fn try_recv_sender_from_rx(&mut self) -> Option<&mut Sender<(u16, BusData)>> {
        if self.sender.is_some() {
            return self.sender.as_mut();
        }
        self.sender = match self.sender_rx.try_recv() {
            Ok(s) => Some(s),
            Err(_) => None,
        };
        self.sender.as_mut()
    }

    #[inline]
    fn try_send(s: &StoreT, msg_id: u16, data: BusData) {
        s.net_term_adp_mut().map(|n| n.send(msg_id, data));
    }

    pub fn send(&mut self, msg: u16, d: BusData) {
        let sender = self.try_recv_sender_from_rx();
        if sender.map_or(false, |s| s.send((msg, d)).is_err()) {
            self.sender.take();
        }
    }
}

