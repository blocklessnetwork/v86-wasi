use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::{StoreT, ContextTrait, bus::BusData};


pub(crate) struct NetTermAdapter {
    store: StoreT,
    sender: Option<Sender<(u16, BusData)>>,
    recv: Option<Receiver<(u16, BusData)>>,
    channel_rx: Receiver<(Receiver<(u16, BusData)>, Sender<(u16, BusData)>)>,

}

impl NetTermAdapter {

    pub fn new(
        store: StoreT, 
        channel_rx: Receiver<(Receiver<(u16, BusData)>, Sender<(u16, BusData)>)>
    ) -> Self {
        Self {
            store,
            channel_rx,
            recv: None,
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
    fn try_recv_channel_from_rx(&mut self) {
        if self.sender.is_some() {
            return;
        }
        (self.recv, self.sender) = match self.channel_rx.try_recv() {
            Ok((r, s)) => (Some(r), Some(s)),
            Err(_) => return,
        };
    }

    pub fn try_recv_from_term(&mut self) {
        let clear = self.recv.as_mut().map_or(false, |r| {
            let mut clear = false;
            loop {
                match r.try_recv() {
                    Ok((msg_id, bus_data)) => {
                        if msg_id == 0x0100 {
                            self.store.clone().bus_mut().map(|bus| {
                                bus.send("keyboard-code", bus_data);
                            });
                        }
                    },
                    Err(TryRecvError::Empty) => break,
                    Err(_) => {
                        clear = true;
                        break;
                    }
                }
            }
            clear
        });
        if clear {
            self.clear_channel();
        }
    }

    #[inline]
    fn clear_channel(&mut self) {
        self.sender.take();
        self.recv.take();
    }

    #[inline]
    fn try_send(s: &StoreT, msg_id: u16, data: BusData) {
        s.net_term_adp_mut().map(|n| n.send(msg_id, data));
    }

    #[inline]
    pub fn send(&mut self, msg: u16, d: BusData) {
        let sender = self.try_recv_channel_from_rx();
        if self.sender.as_mut().map_or(false, |s| s.send((msg, d)).is_err()) {
            self.clear_channel();
        }
    }
}

