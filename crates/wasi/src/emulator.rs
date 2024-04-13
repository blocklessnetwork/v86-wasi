#![allow(unused)]
use std::{
    cell::Cell,
    collections::HashMap,
    rc::{Rc, Weak},
    sync::mpsc::{self, Receiver, Sender},
    thread::{JoinHandle, Thread},
    time,
};

use tokio::sync::mpsc::channel;
use wasmtime::{Extern, Instance, Linker, Store, Table};

use crate::{
    io::IO,
    pci::PCI,
    pit::PIT,
    ps2::PS2,
    rtc::RTC,
    bus::BUS,
    dma::DMA,
    ne2k::Ne2k,
    uart::UART,
    ide::IDEDevice,
    vga::VGAScreen,
    log::{self, LOG},
    ws_thr::WsThread,
    adapter::{NetAdapter, NetTermAdapter},
    floppy::FloppyController,
    jit::{JitMsg, JitWorker},
    ContextTrait, Setting, StoreT, CPU, WASM_TABLE_OFFSET, tun_thr::TunThread,
};

pub(crate) struct InnerEmulator {
    start_time: time::Instant,
    setting: Setting,
    cpu: Option<CPU>,
    bus: Option<BUS>,
    table: Option<Table>,
    bios: Option<Vec<u8>>,
    vga_bios: Option<Vec<u8>>,
    jit_tx: Option<Sender<JitMsg>>,
    tick_trigger: Vec<fn(&StoreT)>,
    net_adapter: Option<NetAdapter>,
    jit_result_rx: Option<Receiver<JitMsg>>,
    net_term_adapter: Option<NetTermAdapter>,
    externs: Option<HashMap<String, Extern>>,
}

impl InnerEmulator {
    fn new(setting: Setting) -> Self {
        Self {
            setting,
            cpu: None,
            bus: None,
            bios: None,
            table: None,
            jit_tx: None,
            externs: None,
            vga_bios: None,
            net_adapter: None,
            jit_result_rx: None,
            net_term_adapter: None,
            tick_trigger: Vec::new(),
            start_time: time::Instant::now(),
        }
    }

    #[inline]
    fn register_trigger(&mut self, call: fn(&StoreT)) {
        self.tick_trigger.push(call);
    }

    #[inline]
    fn init(
        &mut self,
        externs: HashMap<String, Extern>,
        table: Table,
        mut inst: Instance,
        store: StoreT,
    ) {
        let store_cl = store.clone().into_raw() as usize;
        let (tx, rx) = mpsc::channel();
        let (rs_tx, rs_rx) = mpsc::channel();
        let mem = store
            .store_mut()
            .map(|store| inst.get_memory(store, "memory"))
            .flatten()
            .unwrap();
        std::thread::spawn(move || {
            let table = table;
            let store: StoreT = unsafe { Weak::from_raw(store_cl as *const Store<Emulator>) };
            let mut jit_worker = JitWorker {
                sender: rs_tx,
                recv: rx,
                externs,
                store,
                mem,
            };
            jit_worker.compile();
        });
        self.jit_tx = Some(tx);
        self.table = Some(table);
        self.jit_result_rx = Some(rs_rx);
        self.bus = Some(BUS::new(store.clone()));

        //tx rx for term adapater
        let (tx, rs) = channel(1);
        std::thread::Builder::new()
            .name("ws thread".to_string())
            .spawn(move || {
                let ws_thr = WsThread::new(tx);
                ws_thr.start();
            });
        
        self.net_term_adapter = Some(NetTermAdapter::new(store.clone(), rs));

        let (tun_tx1, tun_rx1) = crossbeam_channel::bounded(64);
        let (tun_tx2, tun_rx2) = crossbeam_channel::bounded(64);
        store
            .setting()
            .tun_addr()
            .map(String::clone)
            .zip(
                store
                    .setting()
                    .tun_netmask()
                    .map(String::clone)
            )
            .map(|(addr, netmask)| {
                let tun_ether_addess = store.setting()
                    .tun_ether_addr()
                    .map(|addr| addr.clone());
                std::thread::Builder::new()
                    .name("tap thread".to_string())
                    .spawn(move || {
                        let tun_thr = TunThread::new(addr, netmask, tun_ether_addess, tun_tx1, tun_rx2);
                        tun_thr.start();
                    });
            });
        self.net_adapter = Some(NetAdapter::new(store.clone(), tun_rx1, tun_tx2));
        self.cpu = Some(CPU::new(&mut inst, store.clone()));
        self.net_term_adapter.as_mut().map(|t| t.init());
        self.net_adapter.as_mut().map(|t| t.init());

        self.register_trigger(|store| {
            store.emulator_mut().jit_done();
        });

        self.register_trigger(|store| {
            store.net_term_adp_mut().map(|a| a.try_recv_from_term());
        });

        self.register_trigger(|store| {
            store.net_adp_mut().map(|a| a.try_recv_from_tun());
        });
    }

    #[inline]
    pub(crate) fn microtick(&self) -> f64 {
        (self.start_time.elapsed().as_micros() as f64) / 1000.
    }

    fn start(&mut self, store: StoreT) {
        self.cpu.as_mut().map(|c| {
            c.init();
            let mut t = c.main_loop();
            loop {
                self.tick_trigger.iter().for_each(|cb| cb(&store));
                t = c.next_tick(t as u64);
                if t > 0f64 {
                    let sleep_time = if t > 1f64 {
                        time::Duration::from_millis(t as u64)
                    } else {
                        //adjust the sleep time 
                        time::Duration::from_micros((1000f64) as u64)
                    };
                    std::thread::sleep(sleep_time);
                }
            }
        });
    }
}

#[derive(Clone)]
pub struct Emulator {
    inner: Rc<Cell<InnerEmulator>>,
}

impl Emulator {
    pub fn new(setting: Setting) -> Self {
        let inner = Rc::new(Cell::new(InnerEmulator::new(setting)));
        Emulator { inner: inner }
    }

    #[inline]
    pub fn microtick(&self) -> f64 {
        self.inner().microtick()
    }

    #[inline]
    pub fn jit_done(&mut self) {
        self.inner_mut().jit_result_rx.as_mut().map(|rx| loop {
            let (index, start, state_flags, func) = match rx.try_recv() {
                Ok(JitMsg::JitResult(index, start, state_flags, func)) => {
                    (index, start, state_flags, func)
                }
                Ok(_) => return,
                Err(_) => return,
            };
            self.cpu_mut().map(|cpu| {
                cpu.codegen_finalize_finished(index, start, state_flags);
                cpu.store_mut().map(|store| {
                    self.inner_mut().table.map(|table| {
                        table
                            .set(store, WASM_TABLE_OFFSET + index as u32, func)
                            .unwrap();
                    })
                });
            });
        });
    }

    #[inline]
    pub(crate) fn jit(&mut self, msg: JitMsg) {
        self.inner_mut().jit_tx.as_mut().map(|tx| {
            let _ = tx.send(msg);
        });
    }

    pub fn start(
        &mut self,
        externs: HashMap<String, Extern>,
        table: Table,
        inst: Instance,
        store: StoreT,
    ) {
        self.inner_mut().init(externs, table, inst, store.clone());
        self.inner_mut().start(store);
        println!("exit.");
    }

    #[inline]
    pub(crate) fn vga_mut(&self) -> Option<&mut VGAScreen> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.vga)
    }

    #[inline]
    pub(crate) fn pit_mut(&self) -> Option<&mut PIT> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.pit)
    }

    #[inline]
    pub(crate) fn net_adp_mut(&self) -> Option<&mut NetAdapter> {
        self.inner_mut().net_adapter.as_mut()
    }

    #[inline]
    pub(crate) fn net_adp(&self) -> Option<&NetAdapter> {
        self.inner().net_adapter.as_ref()
    }

    #[inline]
    pub(crate) fn pit(&self) -> Option<&PIT> {
        self.inner_mut().cpu.as_mut().map(|cpu| &cpu.pit)
    }

    #[inline]
    pub(crate) fn uart0_mut(&self) -> Option<&mut UART> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.uart0)
    }

    #[inline]
    pub(crate) fn uart0(&self) -> Option<&UART> {
        self.inner_mut().cpu.as_mut().map(|cpu| &cpu.uart0)
    }

    #[inline]
    pub(crate) fn ps2_mut(&self) -> Option<&mut PS2> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.ps2)
    }

    #[inline]
    pub(crate) fn ps2(&self) -> Option<&PS2> {
        self.inner_mut().cpu.as_mut().map(|cpu| &cpu.ps2)
    }

    #[inline]
    pub fn bios(&self) -> Option<&Vec<u8>> {
        self.inner_mut().bios.as_ref()
    }

    #[inline]
    pub fn set_bios(&self, b: Vec<u8>) {
        self.inner_mut().bios = Some(b);
    }

    #[inline]
    pub fn vga_bios(&self) -> Option<&Vec<u8>> {
        self.inner_mut().vga_bios.as_ref()
    }

    #[inline]
    pub fn set_vga_bios(&self, b: Vec<u8>) {
        self.inner_mut().vga_bios = Some(b);
    }

    #[inline]
    pub fn inner_strong_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }

    #[inline]
    pub(crate) fn cpu_mut(&self) -> Option<&mut CPU> {
        self.inner_mut().cpu.as_mut()
    }

    #[inline]
    pub(crate) fn bus_mut(&self) -> Option<&mut BUS> {
        self.inner_mut().bus.as_mut()
    }

    #[inline]
    pub(crate) fn bus(&self) -> Option<&BUS> {
        self.inner_mut().bus.as_ref()
    }

    #[inline]
    pub(crate) fn io_mut(&self) -> Option<&mut IO> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.io)
    }

    #[inline]
    pub(crate) fn io(&self) -> Option<&IO> {
        self.inner_mut().cpu.as_ref().map(|cpu| &cpu.io)
    }

    #[inline]
    pub(crate) fn dma_mut(&self) -> Option<&mut DMA> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.dma)
    }

    #[inline]
    pub(crate) fn cpu(&self) -> Option<&CPU> {
        self.inner_mut().cpu.as_ref()
    }

    #[inline]
    pub(crate) fn rtc_mut(&self) -> Option<&mut RTC> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.rtc)
    }

    #[inline]
    pub(crate) fn pci_mut(&self) -> Option<&mut PCI> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.pci)
    }

    #[inline]
    pub(crate) fn pci(&self) -> Option<&PCI> {
        self.inner_mut().cpu.as_mut().map(|cpu| &cpu.pci)
    }

    #[inline]
    fn inner(&self) -> &InnerEmulator {
        unsafe {
            let rc = &(*self.inner);
            &mut (*rc.as_ptr())
        }
    }

    #[inline]
    fn inner_mut(&self) -> &mut InnerEmulator {
        unsafe {
            let rc = &(*self.inner);
            &mut (*rc.as_ptr())
        }
    }

    #[inline]
    pub fn setting(&self) -> &Setting {
        &self.inner().setting
    }

    #[inline]
    pub(crate) fn fdc_mut(&self) -> Option<&mut FloppyController> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.fdc)
    }

    #[inline]
    pub(crate) fn fdc(&self) -> Option<&FloppyController> {
        self.inner_mut().cpu.as_ref().map(|cpu| &cpu.fdc)
    }

    #[inline]
    pub(crate) fn ne2k_mut(&self) -> Option<&mut Ne2k> {
        self.inner_mut().cpu.as_mut().map(|cpu| &mut cpu.ne2k)
    }

    #[inline]
    pub(crate) fn ne2k(&self) -> Option<&Ne2k> {
        self.inner_mut().cpu.as_ref().map(|cpu| &cpu.ne2k)
    }

    #[inline]
    pub(crate) fn ide_mut(&self) -> Option<&mut IDEDevice> {
            self.inner_mut().cpu.as_mut().map(|cpu| {
                if cpu.ide.is_some() {
                    cpu.ide.as_mut()
                } else {
                    cpu.cdrom.as_mut()
                }
            }).flatten()
    }

    #[inline]
    pub(crate) fn ide(&self) -> Option<&IDEDevice> {
        self.inner().cpu.as_ref().map(|cpu| {
            if cpu.ide.is_some() {
                cpu.ide.as_ref()
            } else {
                cpu.cdrom.as_ref()
            }
        }).flatten()
    }

    #[inline]
    pub(crate) fn wasm_table(&self) -> &mut Table {
        self.inner_mut().table.as_mut().unwrap()
    }

    #[inline]
    pub fn shutdown(&self) {
        self.inner_mut().jit_tx.as_mut().map(|tx| {
            let _ = tx.send(JitMsg::Quit);
        });
    }

    #[inline]
    pub(crate) fn net_term_adp_mut(&self) -> Option<&mut NetTermAdapter> {
        self.inner_mut().net_term_adapter.as_mut()
    }

    #[inline]
    pub(crate) fn net_term_adp(&self) -> Option<&NetTermAdapter> {
        self.inner().net_term_adapter.as_ref()
    }

    #[inline]
    pub(crate) fn wasm_externs(&self, names: Vec<String>) -> Vec<Extern> {
        let mut rs = Vec::new();
        self.inner_mut().externs.as_ref().map(|e| {
            for n in names {
                let val = e.get(&n);
                if val.is_some() {
                    rs.push(val.unwrap().clone());
                }
            }
        });
        rs
    }
}
