// Resources:
// https://pdos.csail.mit.edu/6.828/2006/readings/i386/toc.htm
// https://www-ssl.intel.com/content/www/us/en/processors/architectures-software-developer-manuals.html
// http://ref.x86asm.net/geek32.html

use std::{rc::Weak, cell::Cell};

use crate::{
    io::{MemAccess, MemAccessTrait, IO},
    Emulator, MMAP_BLOCK_SIZE, Setting, FLAG_INTERRUPT, emulator::InnerEmulator, TIME_PER_FRAME,
};
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

    fn mem8_write_slice(&mut self, store: impl AsContextMut, offset: usize, bs: &[u8]) {
        self.mem8.as_mut().map(|mem8| mem8.write_slice(store, offset, bs));
    }

    fn new(memory: Memory) -> Self {
        let memory_size_io = MemAccess::new(812, 1, memory);
        let segment_is_null_io = MemAccess::new(724, 8, memory);
        let segment_offsets_io = MemAccess::new(736, 8, memory);
        let segment_limits_io = MemAccess::new(768, 8, memory);
        let protected_mode_io = MemAccess::new(800, 1, memory);
        let idtr_size_io = MemAccess::new(564, 1, memory);
        let idtr_offset_io = MemAccess::new(568, 1, memory);
        let gdtr_size_io = MemAccess::new(572, 1, memory);
        let gdtr_offset_io = MemAccess::new(576, 1, memory);
        let tss_size_32_io = MemAccess::new(1128, 1, memory);
        let page_fault_io = MemAccess::new(540, 8, memory);
        let cr_io = MemAccess::new(580, 8, memory);
        let cpl_io = MemAccess::new(612, 1, memory);
        let is_32_io = MemAccess::new(804, 1, memory);
        let stack_size_32_io = MemAccess::new(808, 1, memory);
        let in_hlt_io = MemAccess::new(616, 1, memory);
        let last_virt_eip_io = MemAccess::new(620, 1, memory);
        let eip_phys_io = MemAccess::new(624, 1, memory);
        let sysenter_cs_io = MemAccess::new(640, 1, memory);
        let sysenter_eip_io = MemAccess::new(644, 1, memory);
        let prefixes_io = MemAccess::new(648, 1, memory);
        let flags_io = MemAccess::new(120, 1, memory);
        let flags_changed_io = MemAccess::new(116, 1, memory);
        let last_op1_io = MemAccess::new(96, 1, memory);
        let last_op_size_io = MemAccess::new(104, 1, memory);
        let last_result_io = MemAccess::new(112, 1, memory);
        let current_tsc_io = MemAccess::new(960, 2, memory);
        let instruction_pointer_io = MemAccess::new(556, 1, memory);
        let previous_ip_io = MemAccess::new(560, 1, memory);
        let apic_enabled_io = MemAccess::new(548, 1, memory);
        let acpi_enabled_io = MemAccess::new(552, 1, memory);
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
        Self {
            typed_read8,
            typed_read16,
            typed_set_tsc,
            typed_read32s,
            typed_write16,
            typed_write32,
            typed_reset_cpu,
            typed_allocate_memory,
            typed_get_eflags_no_arith,
            typed_do_many_cycles_native,
        }
    }

    fn read8(&self, store: impl AsContextMut, addr: u32) -> i32 {
        self.typed_read8.call(store, addr).unwrap()
    }

    fn read16(&self, store: impl AsContextMut, addr: u32) -> i32 {
        self.typed_read16.call(store, addr).unwrap()
    }

    fn read32s(&self, store: impl AsContextMut, addr: u32) -> i32 {
        self.typed_read32s.call(store, addr).unwrap()
    }

    fn write16(&self, store: impl AsContextMut, addr: u32, val: i32) {
        self.typed_write16.call(store, (addr, val)).unwrap()
    }

    fn write32(&self, store: impl AsContextMut, addr: u32, val: i32) {
        self.typed_write32.call(store, (addr, val)).unwrap()
    }

    fn allocate_memory(&self, store: impl AsContextMut, size: u32) -> u32 {
        self.typed_allocate_memory.call(store, size).unwrap()
    }

    fn get_eflags_no_arith(&self, store: impl AsContextMut) -> i32 {
        self.typed_get_eflags_no_arith.call(store, ()).unwrap()
    }

    fn do_many_cycles_native(&self, store: impl AsContextMut) {
        self.typed_do_many_cycles_native.call(store, ()).unwrap();
    }

    fn reset_cpu(&self, store: &mut impl AsContextMut) {
        self.typed_reset_cpu.call(store.as_context_mut(), ()).unwrap()
    }

    fn set_tsc(&self, store: &mut impl AsContextMut,low: u32, hig: u32) {
        self.typed_set_tsc.call(store.as_context_mut(), (low, hig)).unwrap()
    }
}

pub struct CPU {
    memory: Memory,
    store: Weak<Store<Emulator>>,
    emulator: Option<Weak<Cell<InnerEmulator>>>,
    inst: Instance,
    iomap: IOMap,
    vm_opers: VMOpers,
    io: IO,
}

impl CPU {
    fn store_mut(&self) -> Option<&'static mut Store<Emulator>> {
        if self.store.weak_count() == 0 {
            None
        } else {
            Some(unsafe{&mut *(self.store.as_ptr() as *mut Store<Emulator>)})
        }
    }

    pub fn new(inst: Instance, store: Weak<Store<Emulator>>) -> Self {
        let s = unsafe{&mut *(store.as_ptr() as *mut Store<Emulator>)};
        let memory = inst.get_memory(s.as_context_mut(), "memory").unwrap();
        Self {
            inst,
            store,
            emulator: None,
            memory,
            vm_opers: VMOpers::new(&inst, s),
            iomap: IOMap::new(memory),
            io: IO::new(),
        }
    }

    pub(crate) fn set_emulator(&mut self, emu: Option<Weak<Cell<InnerEmulator>>>) {
        self.emulator = emu;
    }

    fn read8(&mut self, addr: u32) -> i32 {
        self.store_mut().map_or(0, |store| {
            self.vm_opers.read8(store, addr)
        })
    }

    fn read16(&mut self, addr: u32) -> i32 {
        self.store_mut().map_or(0, |store| {
            self.vm_opers.read16(store, addr)
        })
    }

    fn read32s(&mut self, addr: u32) -> i32 {
        self.store_mut().map_or(0, |store| {
            self.vm_opers.read32s(store, addr)
        })
    }

    fn read_slice(&mut self, val: &mut [u8], offset: usize) {
        self.store_mut().map(|store| {
            self.memory.read(store, offset, val).unwrap()
        });
    }

    fn allocate_memory(&mut self, addr: u32) -> u32 {
        self.store_mut().map_or(0, |store| {
            self.vm_opers.allocate_memory(store, addr)
        })
    }

    fn write_mem_size(&mut self, size: u32) {
        self.store_mut().map(|s| {
            self.iomap
                .memory_size_io
                .write(s, 0u32, size as _);
        });
    }

    fn do_many_cycles_native(&mut self) {
        self.store_mut().map(|store| {
            self.vm_opers.do_many_cycles_native(store);
        });
    }

    fn read_mem_size(&mut self) -> u32 {
        self.store_mut().map_or(0, |store| {
            self.iomap.memory_size_io.read(store, 0u32)
        })
    }

    fn reset_cpu(&mut self) {
        self.store_mut().map(|store| {
            self.vm_opers.reset_cpu(store);
        });
    }

    fn set_tsc(&mut self,low: u32, hig: u32) {
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

    fn load_bios(&mut self, setting: &Setting) {
        let bios = setting.load_bios_file().expect("Warning: No BIOS");
        let offset = 0x100000 - bios.len();
        debug!("load bois to: {}", offset);
        self.store_mut().map(|store| {
            self.iomap.mem8_write_slice(store, offset, &bios);
        });
        #[cfg(feature="check_bios")]
        self.check_bios(&bios, offset);
        
    }

    fn check_bios(&mut self, bios: &[u8], off: usize) {
        let mut in_m = vec![0; bios.len()];
        self.read_slice(&mut in_m, off);
        assert!(in_m == bios);
    }

    pub fn init(&mut self,  setting: &Setting) {
        self.set_tsc(0, 0);
        self.create_memory(1024 * 1024 * 64);
        self.reset_cpu();
        self.load_bios(setting);
    }

    fn in_hlt(&mut self) -> bool {
        self.store_mut().map_or(false, |store| {
            self.iomap.in_hlt_io.read(store, 1) > 0
        })
    }

    fn handle_irqs(&mut self) {
        if self.has_interrupt() {
            //TODO: 
        }
    }

    #[inline]
    fn has_interrupt(&mut self) -> bool {
        (self.get_eflags_no_arith() & (FLAG_INTERRUPT as i32)) != 0
    }

    fn emulator_mut(&self) -> Option<&mut InnerEmulator> {
        self.emulator.as_ref().map(|e| {
            if e.weak_count() == 0 {
                None
            } else {
                unsafe {
                    let e = (*e.as_ptr()).as_ptr();
                    Some(&mut *e)
                }
            }
        }).flatten()
    }

    fn microtick(&self) -> f64 {
        self.emulator_mut().map_or(0f64, |e| e.microtick())
    }

    fn hlt_loop(&mut self) -> i32 {
        if self.has_interrupt() {
            let s = self.run_hardware_timers(self.microtick());
            self.handle_irqs();
            s
        } else {
            100
        }
    }

    fn run_hardware_timers(&self, now: f64) -> i32 {
        //TODO:
        100
    }

    fn get_eflags_no_arith(&self) -> i32 {
        self.store_mut().map_or(0, |store| {
            self.vm_opers.get_eflags_no_arith(store)
        })
    }

    fn do_many_cycles(&mut self) {
        self.do_many_cycles_native();
        //TODO:
    }

    pub fn main_run(&mut self) -> i32 {
        if self.in_hlt() {
            let t = self.hlt_loop();
            if self.in_hlt() {
                return t;
            }
        }
        let start  = self.microtick();
        let mut now = start;
        while now - start < TIME_PER_FRAME as _ {
            self.do_many_cycles();
            now = self.microtick();
            let t = self.run_hardware_timers(now);
            self.handle_irqs();
            if self.in_hlt() {
                return t
            }
        }
        return 0;
    }
}
