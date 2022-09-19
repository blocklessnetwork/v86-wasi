use crate::{dev::OptionRom, log::Module, CPU};

use std::{rc::Rc, slice, ops::Add};

const LINUX_BOOT_HDR_SETUP_SECTS: u16 = 0x1F1;
const LINUX_BOOT_HDR_SYSSIZE: u16 = 0x1F4;
const LINUX_BOOT_HDR_VIDMODE: u16 = 0x1FA;
const LINUX_BOOT_HDR_BOOT_FLAG: u16 = 0x1FE;
const LINUX_BOOT_HDR_HEADER: u16 = 0x202;
const LINUX_BOOT_HDR_VERSION: u16 = 0x206;
const LINUX_BOOT_HDR_TYPE_OF_LOADER: u16 = 0x210;
const LINUX_BOOT_HDR_LOADFLAGS: u16 = 0x211;
const LINUX_BOOT_HDR_CODE32_START: u16 = 0x214;
const LINUX_BOOT_HDR_RAMDISK_IMAGE: u16 = 0x218;
const LINUX_BOOT_HDR_RAMDISK_SIZE: u16 = 0x21C;
const LINUX_BOOT_HDR_HEAP_END_PTR: u16 = 0x224;
const LINUX_BOOT_HDR_CMD_LINE_PTR: u16 = 0x228;
const LINUX_BOOT_HDR_INITRD_ADDR_MAX: u16 = 0x22C;
const LINUX_BOOT_HDR_KERNEL_ALIGNMENT: u16 = 0x230;
const LINUX_BOOT_HDR_RELOCATABLE_KERNEL: u16 = 0x234;
const LINUX_BOOT_HDR_MIN_ALIGNMENT: u16 = 0x235;
const LINUX_BOOT_HDR_XLOADFLAGS: u16 = 0x236;
const LINUX_BOOT_HDR_CMDLINE_SIZE: u16 = 0x238;
const LINUX_BOOT_HDR_PAYLOAD_OFFSET: u16 = 0x248;
const LINUX_BOOT_HDR_PAYLOAD_LENGTH: u16 = 0x24C;
const LINUX_BOOT_HDR_PREF_ADDRESS: u16 = 0x258;
const LINUX_BOOT_HDR_INIT_SIZE: u16 = 0x260;

const LINUX_BOOT_HDR_CHECKSUM1: u16 = 0xAA55;
const LINUX_BOOT_HDR_CHECKSUM2: u32 = 0x53726448;

const LINUX_BOOT_HDR_TYPE_OF_LOADER_NOT_ASSIGNED: u8 = 0xFF;

const LINUX_BOOT_HDR_LOADFLAGS_LOADED_HIGH: u8 = 1 << 0;
const LINUX_BOOT_HDR_LOADFLAGS_QUIET_FLAG: u8 = 1 << 5;
const LINUX_BOOT_HDR_LOADFLAGS_KEEP_SEGMENTS: u8 = 1 << 6;
const LINUX_BOOT_HDR_LOADFLAGS_CAN_USE_HEAPS: u8 = 1 << 7;

pub(crate) fn load_kernel(
    cpu: &mut CPU,
    mut bzimage: Vec<u8>,
    initrd: Option<Vec<u8>>,
    mut cmdline: String,
) -> Option<OptionRom> {
    const KERNEL_HIGH_ADDRESS: u32 = 0x100000;
    const INITRD_ADDRESS: u32 = 64 << 20;
    let bzimage_ptr = bzimage.as_mut_ptr();
    let bzimage8 = unsafe { slice::from_raw_parts_mut(bzimage_ptr, bzimage.len()) };
    const QUIET: bool = false;

    let bzimage16 =
        unsafe { slice::from_raw_parts_mut(bzimage_ptr as *mut u16, bzimage.len() / 2) };
    let bzimage32 =
        unsafe { slice::from_raw_parts_mut(bzimage_ptr as *mut u32, bzimage.len() / 4) };
    let setup_sects = bzimage8[LINUX_BOOT_HDR_SETUP_SECTS as usize];
    if setup_sects > 0 {
        setup_sects
    } else {
        4
    };
    let checksum1 = bzimage16[LINUX_BOOT_HDR_BOOT_FLAG as usize >> 1];
    if checksum1 != LINUX_BOOT_HDR_CHECKSUM1 {
        dbg_log!(Module::E, "Bad checksum1: {:#X}", checksum1);
        return None;
    }

    let checksum2 = bzimage16[LINUX_BOOT_HDR_HEADER as usize >> 1] as u32
        | (bzimage16[LINUX_BOOT_HDR_HEADER as usize + 2 >> 1] as u32) << 16;
    if checksum2 != LINUX_BOOT_HDR_CHECKSUM2 as u32 {
        dbg_log!(Module::E, "Bad checksum2: {:#X}", checksum2);
        return None;
    }
    let protocol = bzimage16[LINUX_BOOT_HDR_VERSION as usize >> 1];
    assert!(protocol >= 0x202);
    let flags = bzimage8[LINUX_BOOT_HDR_LOADFLAGS as usize];
    assert!(flags & LINUX_BOOT_HDR_LOADFLAGS_LOADED_HIGH > 0);
    let flags2 = bzimage16[LINUX_BOOT_HDR_XLOADFLAGS as usize >> 1];
    let initrd_addr_max = bzimage32[(LINUX_BOOT_HDR_INITRD_ADDR_MAX as usize) >> 2];
    let kernel_alignment = bzimage32[(LINUX_BOOT_HDR_KERNEL_ALIGNMENT as usize) >> 2];
    let relocatable_kernel = bzimage8[LINUX_BOOT_HDR_RELOCATABLE_KERNEL as usize];
    let min_alignment = bzimage8[LINUX_BOOT_HDR_MIN_ALIGNMENT as usize];
    let cmdline_size = bzimage32[(LINUX_BOOT_HDR_CMDLINE_SIZE as usize) >> 2];
    let payload_offset = bzimage32[(LINUX_BOOT_HDR_PAYLOAD_OFFSET as usize) >> 2];
    let payload_length = bzimage32[(LINUX_BOOT_HDR_PAYLOAD_LENGTH as usize) >> 2];
    let pref_address = bzimage32[(LINUX_BOOT_HDR_PREF_ADDRESS as usize) >> 2];
    let pref_address_high = bzimage32[(LINUX_BOOT_HDR_PREF_ADDRESS as usize) + 4 >> 2];
    let init_size = bzimage32[(LINUX_BOOT_HDR_INIT_SIZE as usize) >> 2];
    dbg_log!(Module::E, "kernel boot protocol version: {:#X}", protocol);
    dbg_log!(Module::E, "flags= {:#X}  xflags={:#X}", flags, flags2);
    dbg_log!(
        Module::E,
        "code32_start={:#X}",
        bzimage32[(LINUX_BOOT_HDR_CODE32_START as usize) >> 2]
    );
    dbg_log!(Module::E, "initrd_addr_max={:#X}", initrd_addr_max);
    dbg_log!(Module::E, "kernel_alignment={:#X}", kernel_alignment);
    dbg_log!(Module::E, "relocatable={}", relocatable_kernel);
    dbg_log!(Module::E, "min_alignment={:#X}", min_alignment);
    dbg_log!(Module::E, "cmdline max={:#X}", cmdline_size);
    dbg_log!(
        Module::E,
        "payload offset={:#X} size={:#X}",
        payload_offset,
        payload_length
    );
    dbg_log!(
        Module::E,
        "pref_address={:#X}:{:#X}",
        pref_address_high,
        pref_address
    );
    dbg_log!(Module::E, "init_size={:#X}", init_size);
    let real_mode_segment: u16 = 0x8000;
    let base_ptr = (real_mode_segment as u32) << 4;

    let heap_end = 0xE000;
    let heap_end_ptr = heap_end - 0x200;
    bzimage8[LINUX_BOOT_HDR_TYPE_OF_LOADER as usize] = LINUX_BOOT_HDR_TYPE_OF_LOADER_NOT_ASSIGNED;
    let new_flags = if QUIET {
        flags | LINUX_BOOT_HDR_LOADFLAGS_QUIET_FLAG
    } else {
        flags & !LINUX_BOOT_HDR_LOADFLAGS_QUIET_FLAG
    } & !LINUX_BOOT_HDR_LOADFLAGS_KEEP_SEGMENTS
        | LINUX_BOOT_HDR_LOADFLAGS_CAN_USE_HEAPS;
    bzimage8[LINUX_BOOT_HDR_LOADFLAGS as usize] = new_flags;
    bzimage16[(LINUX_BOOT_HDR_HEAP_END_PTR as usize) >> 1] = heap_end_ptr;
    // should parse the vga=... paramter from cmdline here, but we don't really care
    bzimage16[(LINUX_BOOT_HDR_VIDMODE as usize) >> 1] = 0xFFFF; // normal
    dbg_log!(Module::E, "heap_end_ptr={:#X}", heap_end_ptr);
    cmdline += "\x00";
    assert!(cmdline.len() < cmdline_size as usize);
    let cmd_line_ptr = base_ptr + heap_end as u32;
    dbg_log!(Module::E, "cmd_line_ptr={:#X}", cmd_line_ptr);
    bzimage32[(LINUX_BOOT_HDR_CMD_LINE_PTR as usize) >> 2] = cmd_line_ptr as u32;
    cpu.mem8_write_slice(cmd_line_ptr as usize, cmdline.as_bytes());
    let prot_mode_kernel_start = (setup_sects as usize + 1) * 512;
    dbg_log!(
        Module::E,
        "prot_mode_kernel_start={:#X}",
        prot_mode_kernel_start
    );

    let real_mode_kernel = &bzimage[0..prot_mode_kernel_start];
    let protected_mode_kernel = &bzimage[prot_mode_kernel_start..];

    let mut ramdisk_address = 0;
    let mut ramdisk_size = 0;
    initrd.map(|initrd| {
        ramdisk_address = INITRD_ADDRESS;
        ramdisk_size = initrd.len();
        assert!((KERNEL_HIGH_ADDRESS + protected_mode_kernel.len() as u32) < ramdisk_address);

        cpu.mem8_write_slice(ramdisk_address as usize, &initrd[..]);
    });

    bzimage32[(LINUX_BOOT_HDR_RAMDISK_IMAGE as usize) >> 2] = ramdisk_address;
    bzimage32[(LINUX_BOOT_HDR_RAMDISK_SIZE as usize) >> 2] = ramdisk_size as u32;
    assert!((base_ptr as u32 + real_mode_kernel.len() as u32) < 0xA0000);
    cpu.mem8_write_slice(base_ptr as usize, real_mode_kernel);
    cpu.mem8_write_slice(KERNEL_HIGH_ADDRESS as usize, protected_mode_kernel);

    return Some(OptionRom {
        name: "genroms/kernel.bin".into(),
        data: Rc::new(make_linux_boot_rom(real_mode_segment, heap_end)),
    });
}

fn make_linux_boot_rom(real_mode_segment: u16, heap_end: u16) -> Vec<u8> {
    // This rom will be executed by seabios after its initialisation
    // It sets up segment registers, the stack and calls the kernel real mode entry point

    const SIZE: u16 = 0x200;

    let mut data8 = vec![0u8; 0x100];
    unsafe {
        *(data8.as_mut_ptr() as *mut u16) = 0xAA55;
    }

    data8[2] = (SIZE / 0x200) as u8;

    let mut i = 3;

    data8[i] = 0xFA; // cli
    i += 1;
    data8[i] = 0xB8; // mov ax, real_mode_segment
    i += 1;
    data8[i] = real_mode_segment as u8;
    i += 1;
    data8[i] = (real_mode_segment >> 8) as u8;
    i += 1;
    data8[i] = 0x8E; // mov es, ax
    i += 1;
    data8[i] = 0xC0;
    i += 1;
    data8[i] = 0x8E; // mov ds, ax
    i += 1;
    data8[i] = 0xD8;
    i += 1;
    data8[i] = 0x8E; // mov fs, ax
    i += 1;
    data8[i] = 0xE0;
    i += 1;
    data8[i] = 0x8E; // mov gs, ax
    i += 1;
    data8[i] = 0xE8;
    i += 1;
    data8[i] = 0x8E; // mov ss, ax
    i += 1;
    data8[i] = 0xD0;
    i += 1;
    data8[i] = 0xBC; // mov sp, heap_end
    i += 1;
    data8[i] = (heap_end >> 0) as u8;
    i += 1;
    data8[i] = (heap_end >> 8) as u8;
    i += 1;
    data8[i] = 0xEA; // jmp (real_mode_segment+0x20):0x0
    i += 1;
    data8[i] = 0x00;
    i += 1;
    data8[i] = 0x00;
    i += 1;
    data8[i] = (real_mode_segment + 0x20) as u8;
    i += 1;
    data8[i] = ((real_mode_segment + 0x20) >> 8) as u8;
    i += 1;

    assert!((i as u16) < SIZE);

    let checksum_index = i;
    data8[checksum_index] = 0;

    let mut checksum: u8 = 0;

    for i in 0..data8.len() {
        checksum += data8[i];
    }

    data8[checksum_index] = !checksum + 1;

    return data8;
}
