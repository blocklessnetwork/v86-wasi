use std::rc::Weak;

use wasmtime::Store;

use crate::{Emulator, EmulatorTrait, Dev};

pub(crate) struct DMA {
    store: Weak<Store<Emulator>>,
    channel_page: Vec<u8>,
    channel_pagehi: Vec<u8>,
    channel_addr: Vec<u16>,
    channel_addr_init: Vec<u16>,
    channel_count: Vec<u16>,
    channel_count_init: Vec<u16>,
    channel_mask: Vec<u8>,
    channel_mode: Vec<u8>,
    unmask_listeners: Vec<()>,
    lsb_msb_flipflop: u8,

}



impl DMA {

    pub(crate) fn new(store: Weak<Store<Emulator>>) -> Self {
        Self {
            store,
            channel_page: vec![0; 8],
            channel_pagehi: vec![0; 8],
            channel_addr: vec![0; 8],
            channel_addr_init: vec![0; 8],
            channel_count: vec![0; 8],
            channel_count_init: vec![0; 8],
            channel_mask: vec![0; 8],
            channel_mode: vec![0; 8],
            unmask_listeners: Vec::new(),
            lsb_msb_flipflop: 0,
        }
    }

    

    pub(crate) fn init(&mut self) {
        
        self.store.io_mut().map(|io| {
            macro_rules! io_port_addr_write_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_write8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32, val: u8| {
                        dev.dma_mut().map(|dma| {
                            dma.port_addr_write($ch, val as u16);
                        });
                    });
                };
            }

            macro_rules! io_port_count_write_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_write8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32, val: u8| {
                        dev.dma_mut().map(|dma| {
                            dma.port_count_write($ch, val as u16);
                        });
                    });
                };
            }

            macro_rules! io_port_addr_read_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_read8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32| {
                        dev.dma_mut().map_or(0, |dma| {
                            dma.port_addr_read($ch)
                        })
                    });
                };
            }
            macro_rules! io_port_count_read_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_read8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32| {
                        dev.dma_mut().map_or(0, |dma| {
                            dma.port_count_read($ch)
                        })
                    });
                };
            }

            macro_rules! io_port_page_write_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_write8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32, val: u8| {
                        dev.dma_mut().map(|dma| {
                            dma.port_page_write($ch, val);
                        });
                    });
                };
            }

            macro_rules! io_port_page_read_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_read8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32| {
                        dev.dma_mut().map_or(0, |dma| {
                            dma.port_page_read($ch)
                        })
                    });
                };
            }

            macro_rules! io_port_pagehi_write_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_write8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32, val: u8| {
                        dev.dma_mut().map(|dma| {
                            dma.port_pagehi_write($ch, val);
                        });
                    });
                };
            }

            macro_rules! io_port_pagehi_read_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_read8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32| {
                        dev.dma_mut().map_or(0, |dma| {
                            dma.port_pagehi_read($ch)
                        })
                    });
                };
            }

            macro_rules! io_port_singlemask_write_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_write8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32, val: u8| {
                        dev.dma_mut().map(|dma| {
                            dma.port_singlemask_write($ch, val);
                        });
                    });
                };
            }

            macro_rules! io_port_multimask_write_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_write8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32, val: u8| {
                        dev.dma_mut().map(|dma| {
                            dma.port_multimask_write($ch, val);
                        });
                    });
                };
            }
            macro_rules! io_port_multimask_read_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_read8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32| {
                        dev.dma_mut().map_or(0, |dma| {
                            dma.port_multimask_read($ch)
                        })
                    });
                };
            }
            macro_rules! io_port_mode_write_bind {
                ($addr: literal, $ch: literal) => {
                    io.register_write8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32, val: u8| {
                        dev.dma_mut().map(|dma| {
                            dma.port_mode_write($ch, val);
                        });
                    });
                };
            }
            macro_rules! io_portc_write_bind {
                ($addr: literal) => {
                    io.register_write8($addr, Dev::Emulator(self.store.clone()), |dev: &Dev, _addr: u32, val: u8| {
                        dev.dma_mut().map(|dma| {
                            dma.portc_write(val);
                        });
                    });
                };
            }
            io_port_addr_write_bind!(0x00, 0);
            io_port_addr_write_bind!(0x02, 1);
            io_port_addr_write_bind!(0x04, 2);
            io_port_addr_write_bind!(0x06, 3);
            io_port_count_write_bind!(0x01, 0);
            io_port_count_write_bind!(0x03, 1);
            io_port_count_write_bind!(0x05, 2);
            io_port_count_write_bind!(0x07, 3);

            io_port_addr_read_bind!(0x00, 0);
            io_port_addr_read_bind!(0x02, 1);
            io_port_addr_read_bind!(0x04, 2);
            io_port_addr_read_bind!(0x06, 3);
            io_port_count_read_bind!(0x01, 0);
            io_port_count_read_bind!(0x03, 1);
            io_port_count_read_bind!(0x05, 2);
            io_port_count_read_bind!(0x07, 3);
            
            io_port_addr_write_bind!(0xC0, 4);
            io_port_addr_write_bind!(0xC4, 5);
            io_port_addr_write_bind!(0xC8, 6);
            io_port_addr_write_bind!(0xCC, 7);
            io_port_count_write_bind!(0xC2, 4);
            io_port_count_write_bind!(0xC6, 5);
            io_port_count_write_bind!(0xCA, 6);
            io_port_count_write_bind!(0xCE, 7);

            io_port_addr_read_bind!(0xC0, 4);
            io_port_addr_read_bind!(0xC4, 5);
            io_port_addr_read_bind!(0xC8, 6);
            io_port_addr_read_bind!(0xCC, 7);
            io_port_count_read_bind!(0xC2, 4);
            io_port_count_read_bind!(0xC6, 5);
            io_port_count_read_bind!(0xCA, 6);
            io_port_count_read_bind!(0xCE, 7);

            io_port_page_write_bind!(0x87, 0);
            io_port_page_write_bind!(0x83, 1);
            io_port_page_write_bind!(0x81, 2);
            io_port_page_write_bind!(0x82, 3);
            io_port_page_write_bind!(0x8F, 4);
            io_port_page_write_bind!(0x8B, 5);
            io_port_page_write_bind!(0x89, 6);
            io_port_page_write_bind!(0x8A, 7);

            io_port_page_read_bind!(0x87, 0);
            io_port_page_read_bind!(0x83, 1);
            io_port_page_read_bind!(0x81, 2);
            io_port_page_read_bind!(0x82, 3);
            io_port_page_read_bind!(0x8F, 4);
            io_port_page_read_bind!(0x8B, 5);
            io_port_page_read_bind!(0x89, 6);
            io_port_page_read_bind!(0x8A, 7);

            io_port_pagehi_write_bind!(0x487, 0);
            io_port_pagehi_write_bind!(0x483, 1);
            io_port_pagehi_write_bind!(0x481, 2);
            io_port_pagehi_write_bind!(0x482, 3);
            io_port_pagehi_write_bind!(0x48B, 5);
            io_port_pagehi_write_bind!(0x489, 6);
            io_port_pagehi_write_bind!(0x48A, 7);

            io_port_pagehi_read_bind!(0x487, 0);
            io_port_pagehi_read_bind!(0x483, 1);
            io_port_pagehi_read_bind!(0x481, 2);
            io_port_pagehi_read_bind!(0x482, 3);
            io_port_pagehi_read_bind!(0x48B, 5);
            io_port_pagehi_read_bind!(0x489, 6);
            io_port_pagehi_read_bind!(0x48A, 7);

            io_port_singlemask_write_bind!(0x0A, 0);
            io_port_singlemask_write_bind!(0xD4, 4);
            io_port_multimask_write_bind!(0x0F, 0);
            io_port_multimask_write_bind!(0xDE, 4);

            io_port_multimask_read_bind!(0x0F, 0);
            io_port_multimask_read_bind!(0xDE, 4);
            
            io_port_mode_write_bind!(0x0B, 0);
            io_port_mode_write_bind!(0xD6, 4);

            io_portc_write_bind!(0x0C);
            io_portc_write_bind!(0xD8);
        });
    }

    fn portc_write (&mut self, data_byte: u8) {
        dbg_log!("flipflop reset");
        self.lsb_msb_flipflop = 0;
    }

    fn port_multimask_write(&mut self, channel_offset: usize, data_byte: u8) {
        dbg_log!("multichannel mask write: 0x{:x}", data_byte);
        for i in 0..4 {
            self.update_mask(channel_offset + i, data_byte & (1 << i));
        }
    }

    fn port_multimask_read(&self, channel_offset: usize) -> u8 {
        let mut value = 0;
        value |= self.channel_mask[channel_offset + 0];
        value |= self.channel_mask[channel_offset + 1] << 1;
        value |= self.channel_mask[channel_offset + 2] << 2;
        value |= self.channel_mask[channel_offset + 3] << 3;
        dbg_log!("multichannel mask read: 0x{:x}", value);
        value
    }

    fn port_mode_write(&mut self, channel_offset: usize, data_byte: u8) {
        let channel = (data_byte & 0x3) as usize + channel_offset;
        dbg_log!("mode write [{}] = 0x{:0x}", channel, data_byte);
        self.channel_mode[channel] = data_byte;
    }

    fn port_singlemask_write(&mut self, channel_offset: usize, data_byte: u8) {
        let channel = (data_byte & 0x3) as usize + channel_offset;
        let value = if data_byte & 0x4 > 0 {
            1
        } else {
            0
        };
        dbg_log!("singlechannel mask write [{}] = {}", channel, value);
        self.update_mask(channel, value);
    }

    fn update_mask(&mut self, channel: usize, value: u8) {
        if self.channel_mask[channel] != value {
            self.channel_mask[channel] = value;
            if value == 0 {
                dbg_log!("firing on_unmask({})", channel);
                self.unmask_listeners.iter().map(|v| {
                    //TODO
                });
            }
        }
    }


    fn port_pagehi_write(&mut self, channel: usize, data_byte: u8) {
        dbg_log!("pagehi write [{}] = 0x{:x}", channel, data_byte);
        self.channel_pagehi[channel] = data_byte;
    }

    fn  port_pagehi_read(&self, channel: usize) -> u8 {
        dbg_log!("pagehi read [{}]", channel);
        self.channel_pagehi[channel]
    }

    fn port_page_write(&mut self, channel: usize, data_byte: u8) {
        dbg_log!("page write [{}] = 0x{:x}", channel, data_byte);
        self.channel_page[channel] = data_byte;
    }

    fn  port_page_read(&self, channel: usize) -> u8 {
        dbg_log!("page read [{}]", channel);
        self.channel_page[channel]
    }

    fn port_addr_write(&mut self, channel: usize, data_byte: u16) {
        dbg_log!("addr write [{}] = 0x{:x}", channel ,data_byte);
        self.channel_addr[channel] =
            self.flipflop_get(self.channel_addr[channel], data_byte, false);
        self.channel_addr_init[channel] =
            self.flipflop_get(self.channel_addr_init[channel], data_byte, true);
    }

    fn port_count_write(&mut self, channel: usize, data_byte: u16) {
        dbg_log!("count write [{}] = 0x{:x}", channel ,data_byte);
        self.channel_count[channel] =
            self.flipflop_get(self.channel_count[channel], data_byte, false);
        self.channel_count_init[channel] =
            self.flipflop_get(self.channel_count_init[channel], data_byte, true);
    }

    fn port_addr_read(&mut self, channel: usize) -> u8 {
        dbg_log!("addr read [{}] -> 0x{:x}", channel, self.channel_addr[channel]);
        self.flipflop_read(self.channel_addr[channel])
    }

    fn port_count_read(&mut self, channel: usize) -> u8 {
        dbg_log!("count read [{}] -> 0x{:x}", channel, self.channel_count[channel]);
        self.flipflop_read(self.channel_count[channel])
    }

    fn flipflop_get(&mut self, old_dword: u16, new_byte: u16, continuing: bool) -> u16 {
        if !continuing {
            self.lsb_msb_flipflop ^= 1;
        }
        if self.lsb_msb_flipflop > 0 {
            old_dword & !0xFF | new_byte
        } else {
            old_dword & !0xFF00 | new_byte << 8
        }
    }

    fn flipflop_read(&mut self, dword: u16) -> u8 {
        self.lsb_msb_flipflop ^= 1;
        if self.lsb_msb_flipflop > 0 {
            (dword & 0xFF) as u8
        } else {
            ((dword >> 8) & 0xFF) as u8
        }
    }

}