mod ext {
    extern "C" {
        pub fn mmap_read8(addr: u32) -> i32;
        pub fn mmap_read16(addr: u32) -> i32;
        pub fn mmap_read32(addr: u32) -> i32;

        pub fn mmap_write8(addr: u32, value: i32);
        pub fn mmap_write16(addr: u32, value: i32);
        pub fn mmap_write32(addr: u32, value: i32);
        pub fn mmap_write64(addr: u32, v0: i32, v1: i32);
        pub fn mmap_write128(addr: u32, v0: i32, v1: i32, v2: i32, v3: i32);
    }
}

use cpu::cpu::reg128;
use cpu::global_pointers::memory_size;
use cpu::vga;
use page::Page;

use std::alloc;
use std::ptr;

#[allow(non_upper_case_globals)]
pub static mut mem8: *mut u8 = ptr::null_mut();

#[no_mangle]
pub fn allocate_memory(size: u32) -> u32 {
    unsafe {
        dbg_assert!(mem8.is_null());
    };
    dbg_log!("Allocate memory size={}m", size >> 20);
    let layout = alloc::Layout::from_size_align(size as usize, 0x1000).unwrap();
    let ptr = unsafe { alloc::alloc(layout) as u32 };
    unsafe {
        mem8 = ptr as *mut u8;
    };
    ptr
}

#[no_mangle]
pub unsafe fn zero_memory(size: u32) { ptr::write_bytes(mem8, 0, size as usize); }

#[allow(non_upper_case_globals)]
pub static mut vga_mem8: *mut u8 = ptr::null_mut();
#[allow(non_upper_case_globals)]
pub static mut vga_memory_size: u32 = 0;

#[no_mangle]
pub fn svga_allocate_memory(size: u32) -> u32 {
    unsafe {
        dbg_assert!(vga_mem8.is_null());
    };
    let layout = alloc::Layout::from_size_align(size as usize, 0x1000).unwrap();
    let ptr = unsafe { alloc::alloc(layout) as u32 };
    dbg_assert!(
        size & (1 << 12 << 6) == 0,
        "size not aligned to dirty_bitmap"
    );
    unsafe {
        vga_mem8 = ptr as *mut u8;
        vga_memory_size = size;
        vga::dirty_bitmap.resize((size >> 12 >> 6) as usize, 0);
    };
    ptr
}

#[no_mangle]
pub fn in_mapped_range(addr: u32) -> bool {
    return addr >= 0xA0000 && addr < 0xC0000 || addr >= unsafe { *memory_size };
}

pub const VGA_LFB_ADDRESS: u32 = 0xE0000000;
pub fn in_svga_lfb(addr: u32) -> bool {
    addr >= VGA_LFB_ADDRESS && addr <= unsafe { VGA_LFB_ADDRESS + (vga_memory_size - 1) }
}

#[no_mangle]
pub fn read8(addr: u32) -> i32 {
    if in_mapped_range(addr) {
        if in_svga_lfb(addr) {
            unsafe { *vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) as i32 }
        }
        else {
            unsafe { ext::mmap_read8(addr) }
        }
    }
    else {
        read8_no_mmap_check(addr)
    }
}
pub fn read8_no_mmap_check(addr: u32) -> i32 { unsafe { *mem8.offset(addr as isize) as i32 } }

#[no_mangle]
pub fn read16(addr: u32) -> i32 {
    if in_mapped_range(addr) {
        if in_svga_lfb(addr) {
            unsafe {
                ptr::read_unaligned(vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) as *const u16)
                    as i32
            }
        }
        else {
            unsafe { ext::mmap_read16(addr) }
        }
    }
    else {
        read16_no_mmap_check(addr)
    }
}
pub fn read16_no_mmap_check(addr: u32) -> i32 {
    unsafe { ptr::read_unaligned(mem8.offset(addr as isize) as *const u16) as i32 }
}

#[no_mangle]
pub fn read32s(addr: u32) -> i32 {
    if in_mapped_range(addr) {
        if in_svga_lfb(addr) {
            unsafe {
                ptr::read_unaligned(vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) as *const i32)
            } // XXX
        }
        else {
            unsafe { ext::mmap_read32(addr) }
        }
    }
    else {
        read32_no_mmap_check(addr)
    }
}
pub fn read32_no_mmap_check(addr: u32) -> i32 {
    unsafe { ptr::read_unaligned(mem8.offset(addr as isize) as *const i32) }
}

pub unsafe fn read64s(addr: u32) -> i64 {
    if in_mapped_range(addr) {
        if in_svga_lfb(addr) {
            ptr::read_unaligned(vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) as *const i64)
        }
        else {
            ext::mmap_read32(addr) as i64 | (ext::mmap_read32(addr + 4) as i64) << 32
        }
    }
    else {
        ptr::read_unaligned(mem8.offset(addr as isize) as *const i64)
    }
}

pub unsafe fn read128(addr: u32) -> reg128 {
    if in_mapped_range(addr) {
        if in_svga_lfb(addr) {
            ptr::read_unaligned(vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) as *const reg128)
        }
        else {
            reg128 {
                i32: [
                    ext::mmap_read32(addr + 0),
                    ext::mmap_read32(addr + 4),
                    ext::mmap_read32(addr + 8),
                    ext::mmap_read32(addr + 12),
                ],
            }
        }
    }
    else {
        ptr::read_unaligned(mem8.offset(addr as isize) as *const reg128)
    }
}

#[no_mangle]
pub unsafe fn write8(addr: u32, value: i32) {
    if in_mapped_range(addr) {
        mmap_write8(addr, value & 0xFF);
    }
    else {
        ::jit::jit_dirty_page(::jit::get_jit_state(), Page::page_of(addr));
        write8_no_mmap_or_dirty_check(addr, value);
    };
}

pub unsafe fn write8_no_mmap_or_dirty_check(addr: u32, value: i32) {
    *mem8.offset(addr as isize) = value as u8
}

#[no_mangle]
pub unsafe fn write16(addr: u32, value: i32) {
    if in_mapped_range(addr) {
        mmap_write16(addr, value & 0xFFFF);
    }
    else {
        ::jit::jit_dirty_cache_small(addr, addr + 2);
        write16_no_mmap_or_dirty_check(addr, value);
    };
}
pub unsafe fn write16_no_mmap_or_dirty_check(addr: u32, value: i32) {
    ptr::write_unaligned(mem8.offset(addr as isize) as *mut u16, value as u16)
}

#[no_mangle]
pub unsafe fn write32(addr: u32, value: i32) {
    if in_mapped_range(addr) {
        mmap_write32(addr, value);
    }
    else {
        ::jit::jit_dirty_cache_small(addr, addr + 4);
        write32_no_mmap_or_dirty_check(addr, value);
    };
}

pub unsafe fn write32_no_mmap_or_dirty_check(addr: u32, value: i32) {
    ptr::write_unaligned(mem8.offset(addr as isize) as *mut i32, value)
}

pub unsafe fn write64_no_mmap_or_dirty_check(addr: u32, value: u64) {
    ptr::write_unaligned(mem8.offset(addr as isize) as *mut u64, value)
}

pub unsafe fn write128_no_mmap_or_dirty_check(addr: u32, value: reg128) {
    ptr::write_unaligned(mem8.offset(addr as isize) as *mut reg128, value)
}

pub unsafe fn memset_no_mmap_or_dirty_check(addr: u32, value: u8, count: u32) {
    ptr::write_bytes(mem8.offset(addr as isize), value, count as usize);
}

pub unsafe fn memcpy_no_mmap_or_dirty_check(src_addr: u32, dst_addr: u32, count: u32) {
    dbg_assert!(src_addr < *memory_size);
    dbg_assert!(dst_addr < *memory_size);
    ptr::copy(
        mem8.offset(src_addr as isize),
        mem8.offset(dst_addr as isize),
        count as usize,
    )
}

pub unsafe fn memcpy_into_svga_lfb(src_addr: u32, dst_addr: u32, count: u32) {
    dbg_assert!(src_addr < *memory_size);
    dbg_assert!(in_svga_lfb(dst_addr));
    ptr::copy_nonoverlapping(
        mem8.offset(src_addr as isize),
        vga_mem8.offset((dst_addr - VGA_LFB_ADDRESS) as isize),
        count as usize,
    )
}

pub unsafe fn mmap_write8(addr: u32, value: i32) {
    if in_svga_lfb(addr) {
        vga::mark_dirty(addr);
        *vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) = value as u8
    }
    else {
        ext::mmap_write8(addr, value)
    }
}
pub unsafe fn mmap_write16(addr: u32, value: i32) {
    if in_svga_lfb(addr) {
        vga::mark_dirty(addr);
        ptr::write_unaligned(
            vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) as *mut u16,
            value as u16,
        )
    }
    else {
        ext::mmap_write16(addr, value)
    }
}
pub unsafe fn mmap_write32(addr: u32, value: i32) {
    if in_svga_lfb(addr) {
        vga::mark_dirty(addr);
        ptr::write_unaligned(
            vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) as *mut i32,
            value,
        )
    }
    else {
        ext::mmap_write32(addr, value)
    }
}
pub unsafe fn mmap_write64(addr: u32, value: u64) {
    if in_svga_lfb(addr) {
        vga::mark_dirty(addr);
        ptr::write_unaligned(
            vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) as *mut u64,
            value,
        )
    }
    else {
        ext::mmap_write64(addr, value as i32, (value >> 32) as i32)
    }
}
pub unsafe fn mmap_write128(addr: u32, v0: u64, v1: u64) {
    if in_svga_lfb(addr) {
        vga::mark_dirty(addr);
        ptr::write_unaligned(
            vga_mem8.offset((addr - VGA_LFB_ADDRESS) as isize) as *mut u64,
            v0,
        );
        ptr::write_unaligned(
            vga_mem8.offset((addr - VGA_LFB_ADDRESS + 8) as isize) as *mut u64,
            v1,
        )
    }
    else {
        ext::mmap_write128(
            addr,
            v0 as i32,
            (v0 >> 32) as i32,
            v1 as i32,
            (v1 >> 32) as i32,
        )
    }
}
