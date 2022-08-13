// Resources:
// https://pdos.csail.mit.edu/6.828/2006/readings/i386/toc.htm
// https://www-ssl.intel.com/content/www/us/en/processors/architectures-software-developer-manuals.html
// http://ref.x86asm.net/geek32.html

use crate::io::{IO, MemAccess};
use wasmtime::{Memory, Instance, Store, AsContextMut, TypedFunc};

struct IOMap {
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
        }
    }
}

struct VMOpers {
    typed_read8: TypedFunc<u32, i32>,
    typed_read16: TypedFunc<u32, i32>,
    typed_read32s: TypedFunc<u32, i32>,
    typed_write16: TypedFunc<(u32, i32), ()>,
    typed_write32: TypedFunc<(u32, i32), ()>,
}

impl VMOpers {

    fn new<T>(inst: &Instance, store: &mut Store<T>) -> Self {
        let typed_read8 = inst.get_typed_func(store.as_context_mut(), "read8").unwrap();
        let typed_read16 = inst.get_typed_func(store.as_context_mut(), "read16").unwrap();
        let typed_read32s = inst.get_typed_func(store.as_context_mut(), "read32s").unwrap();
        let typed_write16 = inst.get_typed_func(store.as_context_mut(), "write16").unwrap();
        let typed_write32 = inst.get_typed_func(store.as_context_mut(), "write32").unwrap();
        Self {
            typed_read8,
            typed_read16,
            typed_read32s,
            typed_write16, 
            typed_write32, 
        }
    }

    fn read8(&self, store: impl AsContextMut,addr: u32) -> i32 {
        self.typed_read8.call(store, addr).unwrap()
    }

    fn read16(&self, store: impl AsContextMut,addr: u32) -> i32 {
        self.typed_read16.call(store, addr).unwrap()
    }

    fn read32s(&self, store: impl AsContextMut,addr: u32) -> i32 {
        self.typed_read32s.call(store, addr).unwrap()
    }

    fn write16(&self, store: impl AsContextMut,addr: u32, val: i32) {
        self.typed_write16.call(store, (addr, val)).unwrap()
    }

    fn write32(&self, store: impl AsContextMut,addr: u32, val: i32) {
        self.typed_write32.call(store, (addr, val)).unwrap()
    }
}



pub(crate) struct CPU<S:'static> {
    memory: Memory,
    store: Store<S>,
    inst: Instance,
    iomap: IOMap,
    io: IO,
}

impl<T> CPU<T> {
    pub fn new(mut store: Store<T>, inst: Instance) -> Self {
        let memory = inst.get_memory(store.as_context_mut(), "memory").unwrap();
        Self {
            inst,
            memory,
            store,
            iomap: IOMap::new(memory),
            io: IO::new(),
        }
    }

    pub(crate) fn create_memory(size: u32) {
        let size = if size < 1024 * 1024 {
            1024 * 1024
        } else {
            size
        };


    }

    pub fn init(&mut self) {

    }
}
