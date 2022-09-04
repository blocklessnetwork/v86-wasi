const VGA_BANK_SIZE: u32 = 64 * 1024;

const MAX_XRES: u32 = 2560;

const MAX_YRES: u32 = 1600;

const MAX_BPP: u8 = 32;

//var VGA_LFB_ADDRESS = 0xFE000000; 
// set by seabios
const VGA_LFB_ADDRESS: u32 = 0xE0000000;

const VGA_PIXEL_BUFFER_START: u32 = 4 * VGA_BANK_SIZE;

/**
 * @const
 * Equals the maximum number of pixels for non svga.
 * 8 pixels per byte.
 */
const VGA_PIXEL_BUFFER_SIZE: u32 = 8 * VGA_BANK_SIZE;

/** @const */
const VGA_MIN_MEMORY_SIZE: u32 = VGA_PIXEL_BUFFER_START + VGA_PIXEL_BUFFER_SIZE;

const VGA_HOST_MEMORY_SPACE_START: &[u32] = &[0xA0000,0xA0000,0xB0000,0xB8000];
const VGA_HOST_MEMORY_SPACE_SIZE: &[u32] = &[
    0x20000, // 128K
    0x10000, // 64K
    0x8000, // 32K
    0x8000, // 32K
];