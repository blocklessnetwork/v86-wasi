use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError};

use crate::{StoreT, ContextTrait, bus::BusData, log::LOG};

pub(crate) struct NetTermAdapter {
    store: StoreT,
    sender: Option<Sender<(u16, Vec<u8>)>>,
    recv: Option<Receiver<(u16, BusData)>>,
    channel_rx: Receiver<(Receiver<(u16, BusData)>, Sender<(u16, Vec<u8>)>)>,

}

impl NetTermAdapter {

    pub fn new(
        store: StoreT, 
        channel_rx: Receiver<(Receiver<(u16, BusData)>, Sender<(u16, Vec<u8>)>)>
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
                |s: &StoreT, data| Self::try_send(s, 1, data.to_vec()));
            bus.register("screen-put-char", 
                |s: &StoreT, data| Self::try_send(s, 2, data.to_vec()));
            bus.register("screen-set-size-text", 
                |s: &StoreT, data| Self::try_send(s, 3, data.to_vec()));
            bus.register("screen-update-cursor-scanline", 
                |s: &StoreT, data| Self::try_send(s, 4, data.to_vec()));
            bus.register("screen-update-cursor", 
                |s: &StoreT, data| Self::try_send(s, 5, data.to_vec()));
            bus.register("screen-put-sreenchars", 
                |s: &StoreT, data| Self::try_send(s, 6, data.to_vec()));
        });
    }

    #[inline]
    fn try_recv_channel_from_rx(&mut self) {
        if self.sender.is_some() && self.recv.is_some() {
            return;
        }
        (self.recv, self.sender) = match self.channel_rx.try_recv() {
            Ok((r, s)) => (Some(r), Some(s)),
            Err(_) => return,
        };
    }

    pub fn try_recv_from_term(&mut self) {
        self.try_recv_channel_from_rx();
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
    fn try_send(s: &StoreT, msg_id: u16, data: Vec<u8>) {
        s.net_term_adp_mut().map(|n| n.send(msg_id, data));
    }

    #[inline]
    fn send(&mut self, msg: u16, d: Vec<u8>) {
        self.try_recv_channel_from_rx();
        if self.sender.as_mut().map_or(false, |s| {
            match s.try_send((msg, d)) {
                Ok(_) => false,
                Err(TrySendError::Full(_)) => {
                    dbg_log!(LOG::WS, "queue full. ");
                    std::thread::sleep(Duration::from_millis(10));
                    false
                },
                Err(_) => true,
            }
        }) {
            self.clear_channel();
        }
    }
}

