// Resources:
// https://pdos.csail.mit.edu/6.828/2006/readings/i386/toc.htm
// https://www-ssl.intel.com/content/www/us/en/processors/architectures-software-developer-manuals.html
// http://ref.x86asm.net/geek32.html

use crate::{
    io::{MemAccess, MemAccessTrait, IO},
    Emulator, MMAP_BLOCK_SIZE,
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
    typed_allocate_memory: TypedFunc<u32, u32>,
}

impl VMOpers {
    fn new<T>(inst: &Instance, store: &mut Store<T>) -> Self {
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
        Self {
            typed_read8,
            typed_read16,
            typed_read32s,
            typed_write16,
            typed_reset_cpu,
            typed_write32,
            typed_allocate_memory,
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

    fn reset_cpu(&self, store: impl AsContextMut) {
        self.typed_reset_cpu.call(store, ()).unwrap()
    }
}

pub struct CPU {
    memory: Memory,
    inst: Instance,
    iomap: IOMap,
    vm_opers: VMOpers,
    io: IO,
}

impl CPU {
    pub fn new(inst: Instance, mut store: &mut Store<Emulator>) -> Self {
        let memory = inst.get_memory(store.as_context_mut(), "memory").unwrap();
        Self {
            inst,
            memory,
            vm_opers: VMOpers::new(&inst, &mut store),
            iomap: IOMap::new(memory),
            io: IO::new(),
        }
    }

    fn read8(&mut self, store: &mut Store<Emulator>, addr: u32) -> i32 {
        self.vm_opers.read8(store.as_context_mut(), addr)
    }

    fn read16(&mut self, store: &mut Store<Emulator>, addr: u32) -> i32 {
        self.vm_opers.read16(store.as_context_mut(), addr)
    }

    fn read32s(&mut self, store: &mut Store<Emulator>, addr: u32) -> i32 {
        self.vm_opers.read32s(store.as_context_mut(), addr)
    }

    fn allocate_memory(&mut self, store: &mut Store<Emulator>, addr: u32) -> u32 {
        self.vm_opers.allocate_memory(store.as_context_mut(), addr)
    }

    fn write_mem_size(&mut self, store: &mut Store<Emulator>, size: u32) {
        self.iomap
            .memory_size_io
            .write(store.as_context_mut(), 0u32, size as _);
    }

    fn read_mem_size(&mut self, store: &mut Store<Emulator>) -> u32 {
        self.iomap.memory_size_io.read(store.as_context_mut(), 0u32)
    }

    fn reset_cpu(&mut self, store: &mut Store<Emulator>) {
        self.vm_opers.reset_cpu(store.as_context_mut());
    }

    pub(crate) fn create_memory(&mut self, store: &mut Store<Emulator>, size: u32) {
        let max_size = (1_u32 << 31_u32) - MMAP_BLOCK_SIZE as u32;
        let size = if size < 1024 * 1024 {
            1024 * 1024
        } else if size > max_size {
            max_size
        } else {
            size
        };

        assert!((size & MMAP_BLOCK_SIZE as u32 - 1) == 0);
        let ms = self.read_mem_size(store);
        assert!(ms == 0, "Expected uninitialised memory");
        self.write_mem_size(store, size);
        let offset = self.allocate_memory(store, size);
        self.iomap.mem8 = Some(MemAccess::new(offset as _, size, self.memory));
        self.iomap.mem32s = Some(MemAccess::new(offset as _, size >> 2, self.memory));
    }

    pub fn init(&mut self, store: &mut Store<Emulator>) {
        self.create_memory(store, 1024 * 1024);
        self.reset_cpu(store);
    }
}
