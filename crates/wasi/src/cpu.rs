#![allow(unused)]
#![allow(non_snake_case)]
// Resources:
// https://pdos.csail.mit.edu/6.828/2006/readings/i386/toc.htm
// https://www-ssl.intel.com/content/www/us/en/processors/architectures-software-developer-manuals.html
// http://ref.x86asm.net/geek32.html

use std::{
    rc::{Rc, Weak},
    task::Poll,
};

use crate::{
    bus::BUS,
    consts::*,
    debug::Debug,
    dev::OptionRom,
    dma::DMA,
    floppy::FloppyController,
    io::{MMapFn, MemAccess, MemAccessTrait, IO},
    kernel::load_kernel,
    log::LOG,
    pci::PCI,
    pic::PIC,
    pit::PIT,
    ps2::PS2,
    rtc::RTC,
    uart::UART,
    vga::VGAScreen,
    ContextTrait, Dev, Emulator, StoreT, FLAG_INTERRUPT, MMAP_BLOCK_SIZE, TIME_PER_FRAME, ne2k::Ne2k, ide::{self, IDEDevice}, storage::SyncFileBuffer,
};
use chrono::Duration;
use wasmtime::{AsContextMut, Instance, Memory, Store, TypedFunc};

struct IOMap {
    mem8: Option<MemAccess<u8>>,
    mem32s: Option<MemAccess<i32>>,
    memory_size_io: MemAccess<u32>,
    segment_is_null_io: MemAccess<u8>,
    segment_offsets_io: MemAccess<i32>,
    segment_limits_io: MemAccess<u32>,
    //Wheter or not in protected mode
    protected_mode_io: MemAccess<i32>,
    //Interrupt descriptor table register
    idtr_size_io: MemAccess<i32>,
    idtr_offset_io: MemAccess<i32>,
    //global descriptor table register
    gdtr_size_io: MemAccess<i32>,
    gdtr_offset_io: MemAccess<i32>,
    tss_size_32_io: MemAccess<i32>,
    //whether or not a page fault occured
    page_fault_io: MemAccess<u32>,
    // current privilege level
    cr_io: MemAccess<i32>,
    cpl_io: MemAccess<u8>,
    // current operand/address size
    is_32_io: MemAccess<i32>,
    stack_size_32_io: MemAccess<i32>,
    // Was the last instruction a hlt?
    in_hlt_io: MemAccess<u8>,
    last_virt_eip_io: MemAccess<i32>,
    eip_phys_io: MemAccess<i32>,
    sysenter_cs_io: MemAccess<i32>,
    sysenter_eip_io: MemAccess<i32>,
    prefixes_io: MemAccess<i32>,
    flags_io: MemAccess<i32>,
    /**
     * bitmap of flags which are not updated in the flags variable
     * changed by arithmetic instructions, so only relevant to arithmetic flags
     */
    flags_changed_io: MemAccess<i32>,
    //enough infos about the last arithmetic operation to compute eflags
    last_op1_io: MemAccess<i32>,
    last_op_size_io: MemAccess<i32>,
    last_result_io: MemAccess<i32>,
    current_tsc_io: MemAccess<u32>,
    instruction_pointer_io: MemAccess<i32>,
    previous_ip_io: MemAccess<i32>,
    // configured by guest
    apic_enabled_io: MemAccess<u8>,
    // configured when the emulator starts (changes bios initialisation)
    acpi_enabled_io: MemAccess<u8>,
}

impl IOMap {
    #[inline]
    fn mem8_write_slice(&mut self, store: impl AsContextMut, offset: usize, bs: &[u8]) {
        self.mem8
            .as_mut()
            .map(|mem8| mem8.write_slice(store, offset, bs));
    }

    #[inline]
    fn mem8_read_slice(&mut self, store: impl AsContextMut, offset: usize, bs: &mut [u8]) {
        self.mem8
            .as_mut()
            .map(|mem8| mem8.read_slice(store, offset, bs));
    }

    #[inline]
    fn mem8_read(&mut self, store: impl AsContextMut, idx: u32) -> u8 {
        self.mem8.as_ref().map_or(0, |mem8| mem8.read(store, idx))
    }

    #[inline]
    fn mem8_write(&mut self, store: impl AsContextMut, idx: u32, val: u8) {
        self.mem8.as_mut().map(|mem8| mem8.write(store, idx, val));
    }

    fn new(memory: Memory) -> Self {
        let cr_io = MemAccess::new(580, 8, memory);
        let cpl_io = MemAccess::new(612, 1, memory);
        let is_32_io = MemAccess::new(804, 1, memory);
        let flags_io = MemAccess::new(120, 1, memory);
        let in_hlt_io = MemAccess::new(616, 1, memory);
        let last_op1_io = MemAccess::new(96, 1, memory);
        let prefixes_io = MemAccess::new(648, 1, memory);
        let eip_phys_io = MemAccess::new(624, 1, memory);
        let gdtr_size_io = MemAccess::new(572, 1, memory);
        let idtr_size_io = MemAccess::new(564, 1, memory);
        let page_fault_io = MemAccess::new(540, 8, memory);
        let gdtr_offset_io = MemAccess::new(576, 1, memory);
        let memory_size_io = MemAccess::new(812, 1, memory);
        let idtr_offset_io = MemAccess::new(568, 1, memory);
        let last_result_io = MemAccess::new(112, 1, memory);
        let current_tsc_io = MemAccess::new(960, 2, memory);
        let previous_ip_io = MemAccess::new(560, 1, memory);
        let apic_enabled_io = MemAccess::new(548, 1, memory);
        let acpi_enabled_io = MemAccess::new(552, 1, memory);
        let sysenter_cs_io = MemAccess::new(640, 1, memory);
        let sysenter_eip_io = MemAccess::new(644, 1, memory);
        let tss_size_32_io = MemAccess::new(1128, 1, memory);
        let last_op_size_io = MemAccess::new(104, 1, memory);
        let stack_size_32_io = MemAccess::new(808, 1, memory);
        let last_virt_eip_io = MemAccess::new(620, 1, memory);
        let flags_changed_io = MemAccess::new(116, 1, memory);
        let segment_limits_io = MemAccess::new(768, 8, memory);
        let protected_mode_io = MemAccess::new(800, 1, memory);
        let segment_is_null_io = MemAccess::new(724, 8, memory);
        let segment_offsets_io = MemAccess::new(736, 8, memory);
        let instruction_pointer_io = MemAccess::new(556, 1, memory);
        Self {
            memory_size_io,
            segment_is_null_io,
            segment_offsets_io,
            segment_limits_io,
            protected_mode_io,
            idtr_size_io,
            idtr_offset_io,
            gdtr_size_io,
            gdtr_offset_io,
            tss_size_32_io,
            page_fault_io,
            cr_io,
            cpl_io,
            is_32_io,
            stack_size_32_io,
            in_hlt_io,
            last_virt_eip_io,
            eip_phys_io,
            sysenter_cs_io,
            sysenter_eip_io,
            prefixes_io,
            flags_io,
            flags_changed_io,
            last_op1_io,
            last_op_size_io,
            last_result_io,
            current_tsc_io,
            instruction_pointer_io,
            previous_ip_io,
            apic_enabled_io,
            acpi_enabled_io,
            mem8: None,
            mem32s: None,
        }
    }
}

struct VMOpers {
    typed_read8: TypedFunc<u32, i32>,
    typed_read16: TypedFunc<u32, i32>,
    typed_read32s: TypedFunc<u32, i32>,
    typed_write16: TypedFunc<(u32, i32), ()>,
    typed_write32: TypedFunc<(u32, i32), ()>,
    typed_reset_cpu: TypedFunc<(), ()>,
    typed_get_eflags_no_arith: TypedFunc<(), i32>,
    typed_allocate_memory: TypedFunc<u32, u32>,
    typed_do_many_cycles_native: TypedFunc<(), ()>,
    typed_set_tsc: TypedFunc<(u32, u32), ()>,
    typed_pic_call_irq: TypedFunc<i32, ()>,
    typed_rust_init: TypedFunc<(), ()>,
    typed_codegen_finalize_finished: TypedFunc<(i32, i32, i32), ()>,
}

impl VMOpers {
    fn new(inst: &Instance, mut store: impl AsContextMut) -> Self {
        let typed_read8 = inst
            .get_typed_func(store.as_context_mut(), "read8")
            .unwrap();
        let typed_read16 = inst
            .get_typed_func(store.as_context_mut(), "read16")
            .unwrap();
        let typed_read32s = inst
            .get_typed_func(store.as_context_mut(), "read32s")
            .unwrap();
        let typed_write16 = inst
            .get_typed_func(store.as_context_mut(), "write16")
            .unwrap();
        let typed_write32 = inst
            .get_typed_func(store.as_context_mut(), "write32")
            .unwrap();
        let typed_reset_cpu = inst
            .get_typed_func(store.as_context_mut(), "reset_cpu")
            .unwrap();
        let typed_allocate_memory = inst
            .get_typed_func(store.as_context_mut(), "allocate_memory")
            .unwrap();
        let typed_get_eflags_no_arith = inst
            .get_typed_func(store.as_context_mut(), "get_eflags_no_arith")
            .unwrap();
        let typed_do_many_cycles_native = inst
            .get_typed_func(store.as_context_mut(), "do_many_cycles_native")
            .unwrap();
        let typed_set_tsc = inst
            .get_typed_func(store.as_context_mut(), "set_tsc")
            .unwrap();
        let typed_pic_call_irq = inst
            .get_typed_func(store.as_context_mut(), "pic_call_irq")
            .unwrap();
        let typed_rust_init = inst
            .get_typed_func(store.as_context_mut(), "rust_init")
            .unwrap();
        let typed_codegen_finalize_finished = inst
            .get_typed_func(store.as_context_mut(), "codegen_finalize_finished")
            .unwrap();
        Self {
            typed_read8,
            typed_read16,
            typed_set_tsc,
            typed_read32s,
            typed_write16,
            typed_write32,
            typed_reset_cpu,
            typed_rust_init,
            typed_pic_call_irq,
            typed_allocate_memory,
            typed_get_eflags_no_arith,
            typed_do_many_cycles_native,
            typed_codegen_finalize_finished,
        }
    }

    #[inline]
    fn read8(&self, store: impl AsContextMut, addr: u32) -> i32 {
        self.typed_read8.call(store, addr).unwrap()
    }

    #[inline]
    fn read16(&self, store: impl AsContextMut, addr: u32) -> i32 {
        self.typed_read16.call(store, addr).unwrap()
    }

    #[inline]
    fn read32s(&self, store: impl AsContextMut, addr: u32) -> i32 {
        self.typed_read32s.call(store, addr).unwrap()
    }

    #[inline]
    fn write16(&self, store: impl AsContextMut, addr: u32, val: i32) {
        self.typed_write16.call(store, (addr, val)).unwrap()
    }

    #[inline]
    fn write32(&self, store: impl AsContextMut, addr: u32, val: i32) {
        self.typed_write32.call(store, (addr, val)).unwrap()
    }

    #[inline]
    fn allocate_memory(&self, store: impl AsContextMut, size: u32) -> u32 {
        self.typed_allocate_memory.call(store, size).unwrap()
    }

    #[inline]
    fn get_eflags_no_arith(&self, store: impl AsContextMut) -> i32 {
        self.typed_get_eflags_no_arith.call(store, ()).unwrap()
    }

    #[inline]
    fn do_many_cycles_native(&self, store: impl AsContextMut) {
        self.typed_do_many_cycles_native.call(store, ()).unwrap();
    }

    #[inline]
    fn reset_cpu(&self, store: impl AsContextMut) {
        self.typed_reset_cpu.call(store, ()).unwrap()
    }

    #[inline]
    fn pic_call_irq(&self, store: impl AsContextMut, interrupt_nr: i32) {
        self.typed_pic_call_irq.call(store, interrupt_nr).unwrap();
    }

    #[inline]
    fn set_tsc(&self, store: impl AsContextMut, low: u32, hig: u32) {
        self.typed_set_tsc.call(store, (low, hig)).unwrap()
    }

    #[inline]
    fn rust_init(&self, store: impl AsContextMut) {
        self.typed_rust_init.call(store, ()).unwrap()
    }
}

type TaskFn = (fn(store: &StoreT, u64), u64);

pub struct CPU {
    idle: bool,
    a20_byte: u8,
    iomap: IOMap,
    store: StoreT,
    memory: Memory,
    vm_opers: VMOpers,
    tick_counter: u64,
    fw_pointer: usize,
    fw_value: Rc<Vec<u8>>,
    option_roms: Vec<OptionRom>,
    pub(crate) io: IO,
    pub(crate) dma: DMA,
    pub(crate) pic: PIC,
    pub(crate) pit: PIT,
    pub(crate) pci: PCI,
    pub(crate) ps2: PS2,
    pub(crate) rtc: RTC,
    pub(crate) ne2k: Ne2k,
    pub(crate) uart0: UART,
    pub(crate) debug: Debug,
    pub(crate) mmap_fn: MMapFn,
    pub(crate) vga: VGAScreen,
    pub(crate) fdc: FloppyController,
    pub(crate) ide: Option<IDEDevice>,
    pub(crate) cdrom: Option<IDEDevice>,
}

impl CPU {
    #[inline]
    pub(crate) fn store_mut(&self) -> Option<&'static mut Store<Emulator>> {
        if self.store.weak_count() == 0 {
            None
        } else {
            Some(unsafe { &mut *(self.store.as_ptr() as *mut Store<Emulator>) })
        }
    }

    pub fn new(inst: &mut Instance, store: StoreT) -> Self {
        let s = unsafe { &mut *(store.as_ptr() as *mut Store<Emulator>) };
        let memory = inst.get_memory(s.as_context_mut(), "memory").unwrap();
        let rtc = RTC::new(store.clone());
        let pit = PIT::new(store.clone());
        let bus = BUS::new(store.clone());
        let ps2 = PS2::new(store.clone());
        let fdc = FloppyController::new(store.clone());
        let vga_mem_size = store.setting().vga_memory_size;
        let vga = VGAScreen::new(store.clone(), vga_mem_size);
        let uart0 = UART::new(store.clone(), 0x3F8);
        let ne2k = Ne2k::new(store.clone());
        let ide = None;
        let cdrom = None;
        Self {
            ps2,
            rtc,
            pit,
            fdc,
            vga,
            ide,
            ne2k,
            uart0,
            cdrom,
            memory,
            idle: true,
            a20_byte: 0,
            fw_pointer: 0,
            tick_counter: 0,
            store: store.clone(),
            mmap_fn: MMapFn::new(),
            option_roms: Vec::new(),
            iomap: IOMap::new(memory),
            io: IO::new(store.clone()),
            pic: PIC::new(store.clone()),
            pci: PCI::new(store.clone()),
            dma: DMA::new(store.clone()),
            fw_value: Rc::new(Vec::new()),
            debug: Debug::new(store.clone()),
            vm_opers: VMOpers::new(&inst, s),
        }
    }

    #[inline]
    pub fn mmap_read8(&mut self, addr: u32) -> u8 {
        let mfn = self
            .mmap_fn
            .memory_map_read8
            .get(&(addr >> MMAP_BLOCK_BITS))
            .unwrap();
        let dev = Dev::Emulator(self.store.clone());
        (mfn)(&dev, addr)
    }

    #[inline]
    pub fn mmap_read16(&mut self, addr: u32) -> u16 {
        let mfn = self
            .mmap_fn
            .memory_map_read8
            .get(&(addr >> MMAP_BLOCK_BITS))
            .unwrap();
        let dev = Dev::Emulator(self.store.clone());
        let value = mfn(&dev, addr) as u16 | (mfn(&dev, addr + 1 | 0) as u16) << 8;
        value
    }

    #[inline]
    pub fn mmap_read32(&mut self, addr: u32) -> u32 {
        let mfn = self
            .mmap_fn
            .memory_map_read32
            .get(&(addr >> MMAP_BLOCK_BITS))
            .unwrap();
        let dev = Dev::Emulator(self.store.clone());
        mfn(&dev, addr)
    }

    #[inline]
    pub fn mmap_write8(&mut self, addr: u32, value: u8) {
        let mfn = self
            .mmap_fn
            .memory_map_write8
            .get(&(addr >> MMAP_BLOCK_BITS))
            .unwrap();
        let dev = Dev::Emulator(self.store.clone());
        (mfn)(&dev, addr, value);
    }

    #[inline]
    pub fn mmap_write16(&mut self, addr: u32, value: u16) {
        let mfn = self
            .mmap_fn
            .memory_map_write8
            .get(&(addr >> MMAP_BLOCK_BITS))
            .unwrap();
        let dev = Dev::Emulator(self.store.clone());
        mfn(&dev, addr, (value & 0xFF) as u8);
        mfn(&dev, addr + 1|0, (value >> 8) as u8);
    }

    #[inline]
    pub fn mmap_write32(&mut self, addr: u32, value: u32) {
        let mfn = self
            .mmap_fn
            .memory_map_write32
            .get(&(addr >> MMAP_BLOCK_BITS))
            .unwrap();
        let dev = Dev::Emulator(self.store.clone());
        mfn(&dev, addr, value)
    }

    #[inline]
    pub(crate) fn write16(&mut self, addr: u32, value: i32) {
        self.store_mut()
            .map(|store| self.vm_opers.write16(store, addr, value));
    }

    #[inline]
    pub(crate) fn write32(&mut self, addr: u32, value: i32) {
        self.store_mut()
            .map(|store| self.vm_opers.write32(store, addr, value));
    }

    #[inline]
    pub(crate) fn read8(&mut self, addr: u32) -> i32 {
        self.store_mut()
            .map_or(0, |store| self.vm_opers.read8(store, addr))
    }

    #[inline]
    pub(crate) fn read16(&mut self, addr: u32) -> i32 {
        self.store_mut()
            .map_or(0, |store| self.vm_opers.read16(store, addr))
    }

    pub(crate) fn read32s(&mut self, addr: u32) -> i32 {
        self.store_mut()
            .map_or(0, |store| self.vm_opers.read32s(store, addr))
    }

    fn read_slice(&mut self, val: &mut [u8], offset: usize) {
        self.store_mut()
            .map(|store| self.memory.read(store, offset, val).unwrap());
    }

    #[inline]
    fn allocate_memory(&mut self, addr: u32) -> u32 {
        self.store_mut()
            .map_or(0, |store| self.vm_opers.allocate_memory(store, addr))
    }

    #[inline]
    fn rust_init(&mut self) {
        self.store_mut()
            .map(|store| self.vm_opers.rust_init(store));
    }

    #[inline]
    fn write_mem_size(&mut self, size: u32) {
        self.store_mut().map(|s| {
            self.iomap.memory_size_io.write(s, 0u32, size as _);
        });
    }

    #[inline]
    pub fn pic_call_irq(&mut self, interrupt_nr: i32) {
        self.store_mut().map(|store| {
            self.vm_opers
                .pic_call_irq(store.as_context_mut(), interrupt_nr);
        });
    }

    #[inline]
    fn do_many_cycles_native(&mut self) {
        self.store_mut().map(|store| {
            self.vm_opers.do_many_cycles_native(store);
        });
    }

    #[inline]
    pub(crate) fn read_mem_size(&mut self) -> u32 {
        self.store_mut()
            .map_or(0, |store| self.iomap.memory_size_io.read(store, 0u32))
    }

    #[inline]
    fn reset_cpu(&mut self) {
        self.store_mut().map(|store| {
            self.vm_opers.reset_cpu(store);
        });
    }

    #[inline]
    fn set_tsc(&mut self, low: u32, hig: u32) {
        self.store_mut().map(|store| {
            self.vm_opers.set_tsc(store, low, hig);
        });
    }

    pub(crate) fn create_memory(&mut self, size: u32) {
        let max_size = (1_u32 << 31_u32) - MMAP_BLOCK_SIZE as u32;
        let size = if size < 1024 * 1024 {
            1024 * 1024
        } else if size > max_size {
            max_size
        } else {
            size
        };

        assert!((size & MMAP_BLOCK_SIZE as u32 - 1) == 0);
        let ms = self.read_mem_size();
        assert!(ms == 0, "Expected uninitialised memory");
        self.write_mem_size(size);
        let offset = self.allocate_memory(size);
        self.iomap.mem8 = Some(MemAccess::new(offset as _, size, self.memory));
        self.iomap.mem32s = Some(MemAccess::new(offset as _, size >> 2, self.memory));
    }

    #[inline]
    pub fn mem8_write_slice(&mut self, idx: usize, s: &[u8]) {
        self.store_mut().map(|store| {
            self.iomap.mem8_write_slice(store, idx, s);
        });
    }

    #[inline]
    pub fn mem8_write(&mut self, idx: u32, s: u8) {
        self.store_mut().map(|store| {
            self.iomap.mem8_write(store, idx, s);
        });
    }

    #[inline]
    pub fn mem8_read_slice(&mut self, idx: usize, s: &mut [u8]) {
        self.store_mut().map(|store| {
            self.iomap.mem8_write_slice(store, idx, s);
        });
    }

    fn load_bios(&mut self) {
        let bios = self
            .emulator_mut()
            .map(|emu| emu.setting().load_bios_file())
            .flatten();
        let bios = bios.expect("Warning: No BIOS");
        let offset = 0x100000 - bios.len();
        dbg_log!(LOG::CPU, "load bois to: {}", offset);
        // load bios
        self.store_mut().map(|store| {
            self.iomap.mem8_write_slice(store, offset, &bios);
        });
        self.store.emulator_mut().set_bios(bios);
        // load vga bios
        let vga_bios = self
            .emulator_mut()
            .map(|emu| emu.setting().load_vga_bios_file())
            .flatten();
        if vga_bios.is_some() {
            let vga_bios = vga_bios.unwrap();
            self.store_mut().map(|store| {
                // older versions of seabios
                self.iomap.mem8_write_slice(store, 0xC0000, &vga_bios);
            });
            self.store.emulator_mut().set_vga_bios(vga_bios);
            // newer versions of seabios
            self.io.mmap_register(
                0xFEB00000,
                0x100000,
                |dev: &Dev, addr: u32| {
                    let mut addr: usize = addr as usize;
                    dev.vga_bios().map_or(0, |b| {
                        addr = (addr - 0xFEB00000) | 0;
                        if addr < b.len() {
                            b[addr]
                        } else {
                            0
                        }
                    })
                },
                |dev: &Dev, addr: u32, v: u8| {
                    assert!(false, "Unexpected write to VGA rom");
                },
                IO::empty_read32,
                IO::empty_write32,
            );
        }
        // seabios expects the bios to be mapped to 0xFFF00000 also
        self.io.mmap_register(
            0xFFF00000,
            0x100000,
            |dev: &Dev, addr: u32| -> u8 {
                let addr = addr & 0xFFFFF;
                dev.cpu_mut().map_or(0, |cpu| {
                    cpu.store_mut().map_or(0, |s| cpu.iomap.mem8_read(s, addr))
                })
            },
            |dev: &Dev, addr: u32, v: u8| {
                let addr = addr & 0xFFFFF;
                dev.cpu_mut().map(|cpu| {
                    cpu.store_mut().map(|s| cpu.iomap.mem8_write(s, addr, v));
                });
            },
            IO::empty_read32,
            IO::empty_write32,
        );
        #[cfg(feature = "check_bios")]
        self.check_bios(&bios, offset);
    }

    fn check_bios(&mut self, bios: &[u8], off: usize) {
        let mut in_m = vec![0; bios.len()];
        self.read_slice(&mut in_m, off);
        assert!(in_m == bios);
    }

    fn init_io(&mut self) {
        let sz: usize = self.read_mem_size() as _;
        self.mmap_fn.init(sz);
        self.io.init();
    }

    #[inline]
    fn load_kernel(&mut self) {
        let setting = self.store.setting();
        if setting.bzimage_file.is_some() {
            let bzimage = setting.load_bzimage_file().expect("can't load the bzimage file.");
            let initrd = setting.load_initrd_file();
            let cmd = setting.cmdline.clone().unwrap_or_default();
            load_kernel(self, bzimage, initrd, cmd)
                .map(|option_roms| self.option_roms.push(option_roms));
        }
    }

    #[inline]
    fn cdrom_init(&mut self) {
        let setting = self.store.setting();
        if setting.cdrom_file.is_some() {
            let cdrom_buf = setting.load_cdrom_file().expect("can't load the cd rom file.");
            let sync_file_buf = Box::new(SyncFileBuffer::new(self.store.clone(), cdrom_buf));
            let cdrom_ide = IDEDevice::new(self.store.clone(), Some(sync_file_buf), None, true, 0);
            self.cdrom = Some(cdrom_ide);
            self.cdrom.as_mut().map(|cdrom| cdrom.init());
        }
    }

    pub fn init(&mut self) {
        self.rust_init();
        self.set_tsc(0, 0);
        let memory_size = self
            .emulator_mut()
            .map_or(0, |emu| emu.setting().memory_size);
        self.create_memory(memory_size);
        self.debug.init();
        self.init_io();
        self.pci.init();
        self.pic.init();
        
        self.reset_cpu();
        self.load_bios();
        self.uart0.init();
        self.ide.as_mut().map(|ide| ide.init());
        self.load_kernel();
        

        self.io.register_read8(
            0x511,
            Dev::Emulator(self.store.clone()),
            |dev: &Dev, _: u32| {
                dev.cpu_mut().map_or(0, |cpu| {
                    if cpu.fw_pointer < cpu.fw_value.len() {
                        let rs = cpu.fw_value[cpu.fw_pointer];
                        cpu.fw_pointer += 1;
                        rs
                    } else {
                        assert!(false, "config port: Read past value");
                        0
                    }
                })
            },
        );

        self.io.register_write8(
            0x80,
            Dev::Emulator(self.store.clone()),
            |dev: &Dev, _: u32, value: u8| {},
        );

        self.io.register_write(
            0x510,
            Dev::Emulator(self.store.clone()),
            IO::empty_write8,
            |dev: &Dev, _: u32, value: u16| {
                dev.cpu_mut().map(|cpu| {
                    dbg_log!(LOG::E, "bios config port, index={:#X}", value);
                    let vi32 = |i: i32| -> Rc<Vec<u8>> { Rc::new(Vec::from(i.to_le_bytes())) };
                    cpu.fw_pointer = 0;
                    if value == FW_CFG_SIGNATURE {
                        // Pretend to be qemu (for seabios)
                        cpu.fw_value = vi32(FW_CFG_SIGNATURE_QEMU as i32);
                    } else if value == FW_CFG_ID {
                        cpu.fw_value = vi32(0);
                    } else if value == FW_CFG_RAM_SIZE {
                        cpu.fw_value = vi32(cpu.read_mem_size() as i32);
                    } else if value == FW_CFG_NB_CPUS {
                        cpu.fw_value = vi32(1);
                    } else if value == FW_CFG_MAX_CPUS {
                        cpu.fw_value = vi32(1);
                    } else if value == FW_CFG_NUMA {
                        cpu.fw_value = Rc::new(vec![0; 16]);
                    } else if value == FW_CFG_FILE_DIR {
                        let buffer_size = 4 + 64 * cpu.option_roms.len();
                        let mut buffer8 = vec![0u8; buffer_size * 4];
                        let buffer32 = unsafe {
                            std::slice::from_raw_parts_mut(
                                buffer8.as_mut_ptr() as *mut i32,
                                buffer_size,
                            )
                        };
                        buffer32[0] = (cpu.option_roms.len() as i32).to_be();
                        for i in 0..cpu.option_roms.len() {
                            let rom = &cpu.option_roms[i];
                            let name = rom.name.as_bytes();
                            let data = &rom.data;
                            let file_struct_ptr = 4 + 64 * i;
                            assert!((FW_CFG_FILE_START as usize + i) < 0x10000);
                            buffer32[file_struct_ptr + 0 >> 2] = (data.len() as i32).to_be();
                            buffer32[file_struct_ptr + 4 >> 2] =
                                (FW_CFG_FILE_START + i as u16).to_be() as i32;
                            assert!(name.len() < 64 - 8);
                            let start = file_struct_ptr + 8;
                            let end = start + name.len();
                            buffer8[start..end].copy_from_slice(name);
                        }
                        cpu.fw_value = Rc::new(buffer8);
                    } else if value >= FW_CFG_CUSTOM_START && value < FW_CFG_FILE_START {
                        cpu.fw_value = vi32(0);
                    } else if value >= FW_CFG_FILE_START
                        && value - FW_CFG_FILE_START < cpu.option_roms.len() as u16
                    {
                        let i = value - FW_CFG_FILE_START;
                        cpu.fw_value = cpu.option_roms[i as usize].data.clone();
                    } else {
                        dbg_log!(LOG::E, "Warning: Unimplemented fw index: {:#X}", value);
                        cpu.fw_value = vi32(0);
                    }
                });
            },
            IO::empty_write32,
        );

        self.io
            .register_read8(0xB3, Dev::Empty, |_: &Dev, _: u32| -> u8 {
                dbg_log!(LOG::CPU, "port 0xB3 read");
                0
            });
        self.io.register_read8(
            0x92,
            Dev::Emulator(self.store.clone()),
            |d: &Dev, _: u32| -> u8 { d.cpu_mut().map_or(0, |cpu| cpu.a20_byte) },
        );
        self.io.register_write8(
            0x92,
            Dev::Emulator(self.store.clone()),
            |d: &Dev, _: u32, v: u8| {
                d.cpu_mut().map(|cpu| cpu.a20_byte = v);
            },
        );

        self.rtc.init();
        //TODO device loading
        self.fill_cmos();

        self.pit.init();
        self.dma.init();
        self.vga.init();
        self.ne2k.init();
        self.ps2.init();
        self.fdc.init();
        self.cdrom_init();
    }

    #[inline]
    fn in_hlt(&mut self) -> bool {
        self.store_mut()
            .map_or(false, |store| {
                self.iomap.in_hlt_io.read(store, 0) > 0
            })
    }

    #[inline]
    pub fn next_tick(&mut self, t: u64) -> f64 {
        self.tick_counter += 1;
        let tick = self.tick_counter;
        self.idle = true;
        self.cpu_yield(t, tick)
    }

    #[inline]
    fn cpu_yield(&mut self, t: u64, tick: u64) -> f64 {
        self.do_tick()
    }

    #[inline]
    fn do_tick(&mut self) -> f64 {
        self.idle = false;
        self.main_run()
    }

    pub fn handle_irqs(&mut self) {
        if self.has_interrupt() {
            self.pic_acknowledge();
        }
    }

    #[inline]
    pub fn pic_acknowledge(&mut self) {
        self.pic.acknowledge_irq();
    }

    #[inline]
    fn has_interrupt(&self) -> bool {
        (self.get_eflags_no_arith() & (FLAG_INTERRUPT as i32)) != 0
    }

    #[inline]
    fn emulator_mut(&self) -> Option<&mut Emulator> {
        self.store_mut().map(|store| store.data_mut())
    }

    #[inline]
    fn microtick(&self) -> f64 {
        self.emulator_mut().map_or(0f64, |e| e.microtick())
    }

    #[inline]
    pub fn hlt_op(&mut self) {
        if !self.has_interrupt() {
            self.store.bus_mut().map(|bus| {
                bus.send("cpu-event-halt", crate::bus::BusData::None);
            });
        }
        self.store_mut()
            .map(|store| self.iomap.in_hlt_io.write(store, 0, 1));
        self.hlt_loop();
    }

    #[inline]
    fn hlt_loop(&mut self) -> f64 {
        if self.has_interrupt() {
            let s = self.run_hardware_timers(self.microtick());
            self.handle_irqs();
            s
        } else {
            100f64
        }
    }

    #[inline]
    fn run_hardware_timers(&mut self, now: f64) -> f64 {
        //TODO:
        let pit_time = self.pit.timer(now, false);
        let rtc_time = self.rtc.timer(now) as f64;
        100f64.min(rtc_time).min(pit_time)
    }

    #[inline]
    fn get_eflags_no_arith(&self) -> i32 {
        self.store_mut()
            .map_or(0, |store| self.vm_opers.get_eflags_no_arith(store))
    }

    #[inline]
    fn do_many_cycles(&mut self) {
        self.do_many_cycles_native();
        //TODO:
    }

    pub fn main_run(&mut self) -> f64 {
        if self.in_hlt() {
            let t = self.hlt_loop();
            if self.in_hlt() {
                return t;
            }
        }
        let start = self.microtick();
        let mut now = start;
        while now - start < TIME_PER_FRAME as _ {
            self.do_many_cycles();
            now = self.microtick();
            let t = self.run_hardware_timers(now);
            self.handle_irqs();
            if self.in_hlt() {
                return t;
            }
        }
        return 0f64;
    }

    pub fn fill_cmos(&mut self) {
        let boot_order: u32 = 0x213;
        // Used by seabios to determine the boot order
        //   Nibble
        //   1: FloppyPrio
        //   2: HDPrio
        //   3: CDPrio
        //   4: BEVPrio
        // bootflag 1, high nibble, lowest priority
        // Low nibble: Disable floppy signature check (1)
        self.rtc
            .cmos_write(CMOS_BIOS_BOOTFLAG1, 1 | ((boot_order >> 4) & 0xF0) as u8);

        // bootflag 2, both nibbles, high and middle priority
        self.rtc
            .cmos_write(CMOS_BIOS_BOOTFLAG2, (boot_order & 0xFF) as u8);

        self.rtc.cmos_write(CMOS_MEM_BASE_LOW, (640 & 0xFF) as u8);
        self.rtc.cmos_write(CMOS_MEM_BASE_HIGH, (640 >> 8) as u8);
        let mut memory_above_1m = 0; // in k

        let memory_size = self.read_mem_size();
        if memory_size >= 1024 * 1024 {
            memory_above_1m = (memory_size - 1024 * 1024) >> 10;
            memory_above_1m = memory_above_1m.min(0xFFFF);
        }
        self.rtc
            .cmos_write(CMOS_MEM_OLD_EXT_LOW, (memory_above_1m & 0xFF) as u8);
        self.rtc
            .cmos_write(CMOS_MEM_OLD_EXT_HIGH, (memory_above_1m >> 8 & 0xFF) as u8);
        self.rtc
            .cmos_write(CMOS_MEM_EXTMEM_LOW, (memory_above_1m & 0xFF) as u8);
        self.rtc
            .cmos_write(CMOS_MEM_EXTMEM_HIGH, (memory_above_1m >> 8 & 0xFF) as u8);
        let mut memory_above_16m = 0; // in 64k blocks
        let memory_size = self.read_mem_size();
        if memory_size >= 16 * 1024 * 1024 {
            memory_above_16m = (memory_size - 16 * 1024 * 1024) >> 16;
            memory_above_16m = memory_above_16m.min(0xFFFF);
        }
        self.rtc
            .cmos_write(CMOS_MEM_EXTMEM2_LOW, (memory_above_16m & 0xFF) as u8);
        self.rtc
            .cmos_write(CMOS_MEM_EXTMEM2_HIGH, (memory_above_16m >> 8 & 0xFF) as u8);

        // memory above 4G (not supported by this emulator)
        self.rtc.cmos_write(CMOS_MEM_HIGHMEM_LOW, 0);
        self.rtc.cmos_write(CMOS_MEM_HIGHMEM_MID, 0);
        self.rtc.cmos_write(CMOS_MEM_HIGHMEM_HIGH, 0);

        self.rtc.cmos_write(CMOS_EQUIPMENT_INFO, 0x2F);

        self.rtc.cmos_write(CMOS_BIOS_SMP_COUNT, 0);

        //TODO: fast boot

        let fast_boot = self
            .emulator_mut()
            .map_or(false, |emu| emu.setting().fast_boot);
        if fast_boot {
            self.rtc.cmos_write(0x3f, 0x01);
        }
    }

    pub fn device_raise_irq(&mut self, i: u8) {
        self.pic.set_irq(i);
        //TODO
    }

    pub fn device_lower_irq(&mut self, i: u8) {
        self.pic.clear_irq(i);
        //TODO:
    }

    pub fn reboot_internal(&mut self) {
        self.reset_cpu();
        self.fw_value = Rc::new(Vec::new());
        //TODO:
        // if(this.devices.virtio)
        // {
        //     this.devices.virtio.reset();
        // }
        self.load_bios();
    }

    #[inline]
    pub fn codegen_finalize_finished(&mut self, index: i32, phys_addr: i32, state_flags: i32) {
        self.store_mut().map(|store| {
            self.vm_opers.typed_codegen_finalize_finished.call(store, (index, phys_addr, state_flags));
        });
    }

}
