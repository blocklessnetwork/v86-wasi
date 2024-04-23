#![allow(unused)]
use crate::{bus::BusData, io::IO, log::LOG, pci::{GenericPCIDevice, PCIBar, PCIDevice}, ContextTrait, Dev, StoreT};

const NE2K_LOG_VERBOSE: bool = true;

const E8390_CMD: u16 = 0x00; /* The command register (for all pages) */

/* Page 0 register offsets. */
const EN0_CLDALO: u8 = 0x01; /* Low byte of current local dma addr RD */
const EN0_STARTPG: u16 = 0x01; /* Starting page of ring bfr WR */
const EN0_CLDAHI: u8 = 0x02; /* High byte of current local dma addr RD */
const EN0_STOPPG: u16 = 0x02; /* Ending page +1 of ring bfr WR */
const EN0_BOUNDARY: u16 = 0x03; /* Boundary page of ring bfr RD WR */
const EN0_TSR: u16 = 0x04; /* Transmit status reg RD */
const EN0_TPSR: u8 = 0x04; /* Transmit starting page WR */
const EN0_NCR: u8 = 0x05; /* Number of collision reg RD */
const EN0_TCNTLO: u16 = 0x05; /* Low byte of tx byte count WR */
const EN0_FIFO: u8 = 0x06; /* FIFO RD */
const EN0_TCNTHI: u16 = 0x06; /* High byte of tx byte count WR */
const EN0_ISR: u16 = 0x07; /* Interrupt status reg RD WR */
const EN0_CRDALO: u8 = 0x08; /* low byte of current remote dma address RD */
const EN0_RSARLO: u16 = 0x08; /* Remote start address reg 0 */
const EN0_CRDAHI: u8 = 0x09; /* high byte, current remote dma address RD */
const EN0_RSARHI: u16 = 0x09; /* Remote start address reg 1 */
const EN0_RCNTLO: u16 = 0x0a; /* Remote byte count reg WR */
const EN0_RCNTHI: u16 = 0x0b; /* Remote byte count reg WR */
const EN0_RSR: u16 = 0x0c; /* rx status reg RD */
const EN0_RXCR: u16 = 0x0c; /* RX configuration reg WR */
const EN0_TXCR: u16 = 0x0d; /* TX configuration reg WR */
const EN0_COUNTER0: u16 = 0x0d; /* Rcv alignment error counter RD */
const EN0_DCFG: u16 = 0x0e; /* Data configuration reg WR */
const EN0_COUNTER1: u16 = 0x0e; /* Rcv CRC error counter RD */
const EN0_IMR: u16 = 0x0f; /* Interrupt mask reg WR */
const EN0_COUNTER2: u16 = 0x0f; /* Rcv missed frame error counter RD */

const NE_DATAPORT: u16 = 0x10; /* NatSemi-defined port window offset. */
const NE_RESET: u16 = 0x1f; /* Issue a read to reset, a write to clear. */

/* Bits in EN0_ISR - Interrupt status register */
const ENISR_RX: u8 = 0x01; /* Receiver, no error */
const ENISR_TX: u8 = 0x02; /* Transmitter, no error */
const ENISR_RX_ERR: u8 = 0x04; /* Receiver, with error */
const ENISR_TX_ERR: u8 = 0x08; /* Transmitter, with error */
const ENISR_OVER: u8 = 0x10; /* Receiver overwrote the ring */
const ENISR_COUNTERS: u8 = 0x20; /* Counters need emptying */
const ENISR_RDC: u8 = 0x40; /* remote dma complete */
const ENISR_RESET: u8 = 0x80; /* Reset completed */
const ENISR_ALL: u8 = 0x3f; /* Interrupts we will enable */

const ENRSR_RXOK: u8 = 0x01; /* Received a good packet */

const START_PAGE: u8 = 0x40;
const START_RX_PAGE: u8 = 0x40 + 12;
const STOP_PAGE: u8 = 0x80;
const PORT: u16 = 0x300;

pub(crate) struct Ne2k {
    store: StoreT,
    port: u16,
    rcnt: u16,
    cr: u8,
    tsr: u8,
    imr: u8,
    isr: u8,
    tpsr: u8,
    rxcr: u8,
    rsar: u16,
    dcfg: u8,
    txcr: u8,
    tcnt: u16,
    curpg: u8,
    pstop: u8,
    pstart: u8,
    boundary: u8,
    mac: Vec<u8>,
    mar: Vec<u8>,
    pci_id : u16,
    memory: Vec<u8>,
}

impl Ne2k {
    pub fn new(store: StoreT) -> Self {
        let pci_id = 0x05 << 3;
        let mac = vec![0x00, 0x22, 0x15,rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()];
        let mut memory = vec![0u8; 256 * 0x80];
        let mar = vec![0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF];
        let cr = 1;
        let imr = 0;
        let tsr = 1;
        let isr = 0;
        let txcr = 0;
        let tpsr = 0;
        let tcnt = 0;
        let rcnt = 0;
        let dcfg = 0;
        let rsar = 0;
        let rxcr = 0;
        let pstop = STOP_PAGE;
        let pstart = START_PAGE;
        let curpg = START_RX_PAGE;
        let boundary = START_RX_PAGE;
        for i in 0..6 {
            memory[i<<1] = mac[i];
            memory[i<<1|1] = mac[i];
        }
        // the PROM signature of 0x57, 0x57 is also doubled
        // resulting in setting the 4 bytes at the end, 28, 29, 30 and 31 to 0x57
        memory[14 << 1] = 0x57;
        memory[14 << 1 | 1] = 0x57;
        memory[15 << 1] = 0x57;
        memory[15 << 1 | 1] = 0x57;
        dbg_log!(
            LOG::NET, 
            "Mac: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
        );

        Self {
            cr,
            mar,
            mac,
            isr,
            imr,
            tsr,
            dcfg,
            rsar,
            tcnt,
            tpsr,
            txcr,
            rcnt,
            rxcr,
            store,
            curpg, 
            pstop,
            pstart,
            memory,
            pci_id,
            boundary,
            port: PORT,
        }
    }

    pub fn init(&mut self) {
        self.store.io_mut().map(|io| {
            io.register_read8(
                (self.port | E8390_CMD) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        dbg_log!(LOG::NET, "Read cmd");
                        ne2k.cr
                    })
                }
            );

            io.register_write8(
                (self.port | E8390_CMD) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        ne2k.write_e8390_cmd(data_byte);
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_COUNTER0) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 1 {
                            dbg_log!(LOG::NET, "Read mar5");
                            return ne2k.mar[5];
                        } else {
                            dbg_log!(LOG::NET, "Read counter0 pg={pg}");
                            return 0;
                        }
                    })
                }
            );

            io.register_read(
                (self.port | EN0_COUNTER1) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 1 {
                            dbg_log!(LOG::NET, "Read mar6");
                            return ne2k.mar[6];
                        } else {
                            dbg_log!(LOG::NET, "Read8 counter1 pg={pg}");
                            return 0;
                        }
                    })
                },
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        dbg_log!(LOG::NET, "Read16 counter1 pg={pg}");
                        return 0;
                    })
                },
                IO::empty_read32
            );

            io.register_read8(
                (self.port | EN0_COUNTER2) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 1 {
                            dbg_log!(LOG::NET, "Read mar7");
                            return ne2k.mar[7];
                        } else {
                            dbg_log!(LOG::NET, "Read counter2 pg={pg}");
                            return 0;
                        }
                    })
                }
            );

            io.register_read8(
                (self.port | NE_RESET) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k_mut().map_or(0, |ne2k| {
                        let _pg = ne2k.get_page();
                        dbg_log!(LOG::NET, "Read reset");
                        ne2k.do_interrupt(ENISR_RESET);
                        return 0;
                    })
                }
            );

            io.register_write8(
                (self.port | NE_RESET) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        dbg_log!(LOG::NET, "Write reset: {data_byte:#X}");
                        //this.isr &= ~ENISR_RESET;
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_STARTPG) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            return ne2k.pstart;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read pg1/01 (mac[0])");
                            return ne2k.mac[0];
                        } else if pg == 2 {
                            return ne2k.pstart;
                        } else {
                            dbg_log!(LOG::NET, "Read pg{pg}/01");
                            assert!(false);
                            return 0;
                        }
                    })        
                }
            );

            io.register_write8(
                (self.port | EN0_STARTPG) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "start page: {data_byte:#X}");
                            ne2k.pstart = data_byte;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "mac[0] = {data_byte:#X}");
                            ne2k.mac[0] = data_byte;
                        } else if pg == 3 {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg3/01 (9346CR): {:#X}", data_byte);
                        } else {
                            dbg_log!(LOG::NET, "Write pg{}/01: {:#X}", pg, data_byte);
                            assert!(false);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_STOPPG) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            return ne2k.pstop;
                        } else if pg == 1{
                            dbg_log!(LOG::NET, "Read pg1/02 (mac[1])");
                            return ne2k.mac[1];
                        } else if pg == 2 {
                            return ne2k.pstop;
                        } else {
                            dbg_log!(LOG::NET, "Read pg{}/02", pg);
                            assert!(false);
                            return 0;
                        }
                    })        
                }
            );
            
            io.register_write8(
                (self.port | EN0_STOPPG) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let mut data_byte = data_byte;
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "stop page: {:#X}", data_byte);
                            if data_byte > (ne2k.memory.len() >> 8) as u8 {
                                data_byte = (ne2k.memory.len() >> 8) as u8;
                                dbg_log!(LOG::NET, "XXX: Adjusting stop page to {:#X}", data_byte);
                            }
                            ne2k.pstop = data_byte;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "mac[1] = {:#X}", data_byte);
                            ne2k.mac[1] = data_byte;
                        } else {
                            dbg_log!(LOG::NET, "Write pg{}/02: {:#X}", pg, data_byte);
                            assert!(false);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_ISR) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Read isr: {:#X}", ne2k.isr);
                            ne2k.isr
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read curpg: {:#X}", ne2k.curpg);
                            ne2k.curpg
                        } else {
                            assert!(false);
                            0
                        }
                    })        
                }
            );

            io.register_write8(
                (self.port | EN0_ISR) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            // acknowledge interrupts where bit is set
                            dbg_log!(LOG::NET, "Write isr: {:#X}", data_byte);
                            ne2k.isr &= !data_byte;
                            ne2k.update_irq();
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Write curpg: {:#X}", data_byte);
                            ne2k.curpg = data_byte;
                        } else {
                            assert!(false);
                        }
                    });
                }
            );

            io.register_write8(
                (self.port | EN0_TXCR) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            ne2k.txcr = data_byte;
                            dbg_log!(LOG::NET, "Write tx config: {:#X}", data_byte);
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg{}/0d {:#X}", pg ,data_byte);
                        }
                    });
                }
            );

            io.register_write8(
                (self.port | EN0_DCFG) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write data configuration: {:#X}", data_byte);
                            ne2k.dcfg = data_byte;
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg{}/0e {:#X}", pg, data_byte);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_RCNTLO) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Read pg0/0a");
                            0x50
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read mar2");
                            return ne2k.mar[2];
                        } else {
                            assert!(false, "TODO");
                            0
                        }
                    })        
                }
            );

            io.register_write8(
                (self.port | EN0_RCNTLO) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write remote byte count low: {:#2X}", data_byte);
                            ne2k.rcnt = ne2k.rcnt & 0xFF00 | (data_byte & 0xFF) as u16;
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg{}/0a {:#X}", pg, data_byte);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_RCNTHI) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Read pg0/0b");
                            return 0x43;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read mar3");
                            return ne2k.mar[3];
                        } else {
                            assert!(false, "TODO");
                            return 0;
                        }
                    })        
                }
            );

            io.register_write8(
                (self.port | EN0_RCNTHI) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write remote byte count high: {:#X}", data_byte);
                            ne2k.rcnt = ne2k.rcnt & 0xFF | (data_byte as u16) << 8 & 0xFF00;
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg{}/0b {:#X}", pg, data_byte);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_RSARLO) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Read remote start address low");
                            return (ne2k.rsar & 0xFF) as u8;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read mar0");
                            return ne2k.mar[0];
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Read pg{}/08", pg);
                            assert!(false);
                            0
                        }
                    })        
                }
            );

            io.register_write8(
                (self.port | EN0_RSARLO) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write remote start address low: {:#X}", data_byte);
                            ne2k.rsar = ne2k.rsar & 0xFF00 | (data_byte as u16) & 0xFF;
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg{}/08 {:#X}", pg , data_byte);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_RSARHI) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Read remote start address high");
                            return (ne2k.rsar >> 8 & 0xFF) as u8;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read mar1");
                            return ne2k.mar[1];
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Read pg{}/09", pg);
                            assert!(false);
                            0
                        }
                    })        
                }
            );

            io.register_write8(
                (self.port | EN0_RSARHI) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write remote start address low: {:#X}", data_byte);
                            ne2k.rsar = ne2k.rsar & 0xFF | (data_byte as u16) << 8 & 0xFF00;
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg{}/09 {:#X}", pg, data_byte);
                        }
                    });
                }
            );

            io.register_write8(
                (self.port | EN0_IMR) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write interrupt mask register: {:#X} isr={:#X}", data_byte, ne2k.isr);
                            ne2k.imr = data_byte;
                            ne2k.update_irq();
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg{}/0f {:#X}", pg,  data_byte);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_BOUNDARY) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Read boundary: {:#X}", ne2k.boundary);
                            return ne2k.boundary;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read pg1/03 (mac[2])");
                            return ne2k.mac[2];
                        } else if pg == 3 {
                            dbg_log!(LOG::NET, "Unimplemented: Read pg3/03 (CONFIG0)");
                            return 0;
                        } else {
                            dbg_log!(LOG::NET, "Read pg{}/03", pg);
                            assert!(false);
                            return 0;
                        }
                    })        
                }
            );

            io.register_write8(
                (self.port | EN0_BOUNDARY) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write boundary: {:#X}", data_byte);
                            ne2k.boundary = data_byte;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "mac[2] = {:#X}", data_byte);
                            ne2k.mac[2] = data_byte;
                        } else {
                            dbg_log!(LOG::NET, "Write pg{}/03: {:#X}", pg, data_byte);
                            assert!(false);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_TSR) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            return ne2k.tsr;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read pg1/04 (mac[3])");
                            return ne2k.mac[3];
                        } else {
                            dbg_log!(LOG::NET, "Read pg{}/04", pg);
                            assert!(false);
                            return 0;
                        }
                    })        
                }
            );

            io.register_write8(
                (self.port | EN0_TSR) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write tpsr: {:#X}", data_byte);
                            ne2k.tpsr = data_byte;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "mac[3] = {:#X}", data_byte);
                            ne2k.mac[3] = data_byte;
                        } else {
                            dbg_log!(LOG::NET, "Write pg{}/04: {:#X}", pg, data_byte);
                            assert!(false);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_TCNTLO) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Unimplemented: Read pg0/05 (NCR: Number of Collisions Register)");
                            return 0;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read pg1/05 (mac[4])");
                            return ne2k.mac[4];
                        } else if pg == 3 {
                            dbg_log!(LOG::NET, "Unimplemented: Read pg3/05 (CONFIG2)");
                            return 0;
                        } else {
                            dbg_log!(LOG::NET, "Read pg{}/05", pg);
                            assert!(false);
                            return 0;
                        }
                    })        
                }
            );

            io.register_write8(
                (self.port | EN0_TCNTLO) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write tcnt low: {:#X}", data_byte);
                            ne2k.tcnt = ne2k.tcnt & !0xFF | data_byte as u16;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "mac[4] = {:#X}", data_byte);
                            ne2k.mac[4] = data_byte;
                        } else if pg == 3 {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg3/05 (CONFIG2): {:#X}", data_byte);
                        } else {
                            dbg_log!(LOG::NET, "Write pg{}/05: {:#X}", pg, data_byte);
                            assert!(false);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_TCNTHI) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            assert!(false, "TODO");
                            return 0;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "Read pg1/06 (mac[5])");
                            return ne2k.mac[5];
                        } else if pg == 3 {
                            dbg_log!(LOG::NET, "Unimplemented: Read pg3/06 (CONFIG3)");
                            return 0;
                        } else {
                            dbg_log!(LOG::NET, "Read pg{}/06", pg);
                            assert!(false);
                            return 0;
                        }
                    }) 
                }
            );

            io.register_write8(
                (self.port | EN0_TCNTHI) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "Write tcnt high: {:#X}", data_byte);
                            ne2k.tcnt = ne2k.tcnt & 0xFF | (data_byte as u16) << 8;
                        } else if pg == 1 {
                            dbg_log!(LOG::NET, "mac[5] = {:#X}", data_byte);
                            ne2k.mac[5] = data_byte;
                        } else if pg == 3 {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg3/06 (CONFIG3): {:#X}", data_byte);
                        } else {
                            dbg_log!(LOG::NET, "Write pg{}/06: {:#X}", pg, data_byte);
                            assert!(false);
                        }
                    });
                }
            );

            io.register_read8(
                (self.port | EN0_RSR) as u32, 
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32| {
                    dev.ne2k().map_or(0, |ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            return 1 | 1 << 3; // receive status ok
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Read pg{}/0c", pg);
                            assert!(false);
                            return 0;
                        }
                    }) 
                }
            );

            io.register_write8(
                (self.port | EN0_RXCR) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        let pg = ne2k.get_page();
                        if pg == 0 {
                            dbg_log!(LOG::NET, "RX configuration reg write: {:#X}", data_byte);
                            ne2k.rxcr = data_byte;
                        } else {
                            dbg_log!(LOG::NET, "Unimplemented: Write pg{}/0c: {:#X}", pg, data_byte);
                        }
                    });
                }
            );

            io.register_read(
                (self.port | NE_DATAPORT | 0) as u32, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ne2k_mut().map_or(0, |ne2k| {
                        ne2k.data_port_read8()
                    })
                },

                |dev: &Dev, _addr: u32| {
                    dev.ne2k_mut().map_or(0, |ne2k| {
                        ne2k.data_port_read16()
                    })
                },
                |dev: &Dev, _addr: u32| {
                    dev.ne2k_mut().map_or(0, |ne2k| {
                        ne2k.data_port_read32()
                    })
                },
            );

            io.register_write(
                (self.port | NE_DATAPORT) as u32,  
                Dev::Emulator(self.store.clone()), 
                |dev: &Dev, _addr: u32, data_byte: u8| {
                    dev.ne2k_mut().map(|ne2k| {
                        ne2k.data_port_write16(data_byte as u16);
                    });
                },
                |dev: &Dev, _addr: u32, data_byte: u16| {
                    dev.ne2k_mut().map(|ne2k| {
                        ne2k.data_port_write16(data_byte);
                    });
                },
                |dev: &Dev, _addr: u32, data_byte: u32| {
                    dev.ne2k_mut().map(|ne2k| {
                        ne2k.data_port_write32(data_byte);
                    });
                }
            );
        });

        self.store.bus_mut().map(|bus| {
            bus.register("net0-receive", |s: &StoreT, data: &BusData| {
                s.ne2k_mut().map(|ne2k| ne2k.receive(data.to_vec()));
            });
        });
        
        let dev_name = "ne2k";

        let pci_space = vec![
            0xec, 0x10, 0x29, 0x80, 0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00,
            (PORT & 0xFF) as u8 | 1, (PORT >> 8) as u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00, 0x00,0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
            0x00, 0xf4, 0x1a, 0x00, 0x11, 0x00, 0x00, 0xb8, 0xfe, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x01, 0x00, 0x00,
        ];

        let pci_dev = GenericPCIDevice::new(
            self.pci_id,
            pci_space,
            vec![Some(PCIBar::new(32))],
            dev_name,
        );
        self.store.pci_mut().map(|pci| {
            pci.register_device(pci_dev);
        });
    }

    #[inline]
    fn data_port_read8(&mut self) -> u8 {
        (self.data_port_read16() & 0xFF) as u8
    }

    #[inline]
    fn data_port_read16(&mut self) -> u16 {
        if self.dcfg & 1 > 0{
            return self.data_port_read() as u16 | (self.data_port_read() as u16) << 8;
        } else {
            return self.data_port_read() as u16;
        }
    }

    #[inline]
    fn data_port_read32(&mut self) -> u32 {
        self.data_port_read() as u32 | (self.data_port_read() as u32) << 8 |
       (self.data_port_read() as u32) << 16 | (self.data_port_read() as u32) << 24
    }

    fn data_port_read(&mut self) -> u8 {
        let mut data = 0;

        if self.rsar < ((STOP_PAGE as u16) << 8) {
            data = self.memory[self.rsar as usize];
        }

        if NE2K_LOG_VERBOSE {
            dbg_log!(
                LOG::NET,
                "Read data port: data={:#X}  rsar={:#X}  rcnt={:#X}",
                data,
                self.rsar,
                self.rcnt
            );
        }

        self.rsar += 1;
        self.rcnt -= 1;

        if self.rsar >= ((self.pstop as u16)  << 8) {
            self.rsar += ((self.pstart - self.pstop) as u16) << 8;
        }

        if self.rcnt == 0 {
            self.do_interrupt(ENISR_RDC);
        }
        return data;
    }

    fn receive(&mut self, data: Vec<u8>) {
        // called from the adapter when data is received over the network

        if self.cr & 1 > 0 {
            // stop bit set
            return;
        }

        self.store.bus_mut().map(|bus| {
            bus.send("eth-receive-end", BusData::U32(data.len() as u32));
        });

        if self.rxcr & 0x10 > 0 {
            // promiscuous
        } else if(self.rxcr & 4 > 0 &&
                data[0] == 0xFF && data[1] == 0xFF && data[2] == 0xFF &&
                data[3] == 0xFF && data[4] == 0xFF && data[5] == 0xFF) {
            // broadcast
        } else if (self.rxcr & 8) > 0 && (data[0] & 1) == 1 {
            // multicast
            // XXX
            return;
        } else if(data[0] == self.mac[0] && data[1] == self.mac[1] &&
                data[2] == self.mac[2] && data[3] == self.mac[3] &&
                data[4] == self.mac[4] && data[5] == self.mac[5]) {
        } else {
            return;
        }

        let packet_length = 60.max(data.len());

        let offset = (self.curpg as usize) << 8;
        let total_length = packet_length + 4;
        let data_start = offset + 4;
        let mut next = self.curpg as usize + 1 + (total_length >> 8);

        let end = offset + total_length;

        let needed = 1 + (total_length >> 8);

        // boundary == curpg interpreted as ringbuffer empty
        let available = if self.boundary > self.curpg {
            self.boundary - self.curpg 
        } else {
            self.pstop - self.curpg + self.boundary - self.pstart
        };

        if (available as usize) < needed &&
            self.boundary != 0 // XXX: ReactOS sets this to 0 initially and never updates it unless it receives a packet
        {
            dbg_log!(LOG::NET, 
                "Buffer full, dropping packet pstart={:#X} pstop={:#X} curpg={:#X} needed={:#X} boundary={:#X} available={:#X}",
                self.pstart,
                self.pstop,
                self.curpg,
                needed,
                self.boundary,
                available,
            );

            return;
        }

        if end > ((self.pstop as usize) << 8) {
            // Shouldn't happen because at this size it can't cross a page,
            // so we can skip filling with zeroes
            assert!(data.len() >= 60);

            let cut = ((self.pstop as usize) << 8) - data_start as usize;
            assert!(((self.pstop as usize) << 8) >= data_start as usize);

            //this.memory.set(data.subarray(0, cut), data_start);
            let start = data_start as usize;
            let end = start + cut;
            self.memory[start..end].copy_from_slice(&data[0..cut]);
            // this.memory.set(data.subarray(cut), this.pstart << 8);
            let start = (self.pstart as usize) << 8;
            let end = start + (data.len() - cut);
            self.memory[start..end].copy_from_slice(&data[cut..]);
            dbg_log!(LOG::NET, "rcv cut={:#X}", cut);
        } else {
            //this.memory.set(data, data_start);
            let start = data_start as usize;
            let end = start + data.len();
            self.memory[start..end].copy_from_slice(&data);

            if data.len() < 60 {
                let start = start + data.len();
                let end = start + 60;
                self.memory[start..end].fill(0);
            }
        }

        if next >= self.pstop as usize {
            next += self.pstart as usize - self.pstop as usize;
        }
        let offset = offset as usize;
        // write packet header
        self.memory[offset] = ENRSR_RXOK; // status
        self.memory[offset + 1] = next as u8;
        self.memory[offset + 2] = total_length as u8;
        self.memory[offset + 3] = (total_length >> 8) as u8;

        self.curpg = next as u8;

        dbg_log!(LOG::NET, 
            "rcv offset={:#X} len={:#X} next={:#X}",
            offset,
            total_length,
            next
        );
        self.do_interrupt(ENISR_RX);
    }

    #[inline]
    fn data_port_write32(&mut self, data: u32) {
        self.data_port_write(data as u16);
        self.data_port_write((data >> 8) as u16);
        self.data_port_write((data >> 16) as u16);
        self.data_port_write((data >> 24) as u16);
    }

    #[inline]
    fn data_port_write16(&mut self, data: u16) {
        self.data_port_write(data);
        if self.dcfg & 1 > 0 {
            self.data_port_write(data >> 8);
        }
    }

    fn data_port_write(&mut self, data_byte: u16) {
        if NE2K_LOG_VERBOSE {
            dbg_log!(LOG::NET, 
                "Write data port: data={:#X} rsar={:#X} rcnt={:#X}",
                data_byte & 0xFF,
                self.rsar,
                self.rcnt,
            );
        }

        if self.rsar <= 0x10 || self.rsar >= ((START_PAGE as u16) << 8) && self.rsar < ((STOP_PAGE as u16)  << 8) {
            self.memory[self.rsar as usize] = data_byte as u8;
        }

        self.rsar += 1;
        self.rcnt -= 1;

        if self.rsar >= (self.pstop as u16) << 8 {
            self.rsar += ((self.pstart - self.pstop)as u16) << 8;
        }

        if self.rcnt == 0 {
            self.do_interrupt(ENISR_RDC);
        }
    }

    #[inline]
    fn get_page(&self) -> u8 {
        return self.cr >> 6 & 3;
    }

    #[inline]
    fn write_e8390_cmd(&mut self, data_byte: u8) {
        self.cr = data_byte;
        dbg_log!(LOG::NET, "Write command: {:#X} newpg={:#X} txcr={:#X}", data_byte, self.cr >> 6, self.txcr);
        if self.cr & 1 > 0 {
            return;
        }
        if (data_byte & 0x18) > 0 && self.rcnt == 0 {
            self.do_interrupt(ENISR_RDC);
        }

        if data_byte & 4 > 0 {
            let start = (self.tpsr as usize) << 8;
            let end = start + self.tcnt as usize;
            let len = self.store.bus_mut().map_or(0, |bus| {
                let data = &self.memory[start..end];
                let data: Vec<u8> = Vec::from(data);
                let len = data.len() as u32;
                bus.send("net0-send", BusData::Vec(data));
                bus.send("eth-transmit-end", BusData::U32(len));
                len
            });
            self.cr &= !4;
            self.do_interrupt(ENISR_TX);
            dbg_log!(LOG::NET, "Command: Transfer. length={:#X}", len);
        }
    }

    #[inline]
    fn do_interrupt(&mut self, ir_mask: u8) {
        dbg_log!(LOG::NET, "Do interrupt {:#X}", ir_mask);
        self.isr |= ir_mask;
        self.update_irq();
    }

    #[inline]
    fn update_irq(&mut self) {
        if (self.imr & self.isr) > 0 {
            self.store.pci_mut().map(|pci| {
                pci.raise_irq(self.pci_id);
            });
        } else {
            self.store.pci_mut().map(|pci| {
                pci.lower_irq(self.pci_id);
            });
        }
    }
}


