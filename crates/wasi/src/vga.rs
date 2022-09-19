#![allow(unused)]
#![allow(non_snake_case)]
use std::slice;

use crate::{
    bus::BusData,
    io::IO,
    log::Module,
    pci::{PCIBar, PCIDevice},
    ContextTrait, Dev, StoreT,
};

const VGA_BANK_SIZE: u32 = 64 * 1024;

const MAX_XRES: u16 = 2560;

const MAX_YRES: u16 = 1600;

const MAX_BPP: u8 = 32;

//var VGA_LFB_ADDRESS = 0xFE000000;
// set by seabios
const VGA_LFB_ADDRESS: u32 = 0xE0000000;

const VGA_PIXEL_BUFFER_START: u32 = 4 * VGA_BANK_SIZE;

/**
 * Equals the maximum number of pixels for non svga.
 * 8 pixels per byte.
 */
const VGA_PIXEL_BUFFER_SIZE: u32 = 8 * VGA_BANK_SIZE;

const VGA_MIN_MEMORY_SIZE: u32 = VGA_PIXEL_BUFFER_START + VGA_PIXEL_BUFFER_SIZE;

const VGA_HOST_MEMORY_SPACE_START: &[u32] = &[0xA0000, 0xA0000, 0xB0000, 0xB8000];

const VGA_HOST_MEMORY_SPACE_SIZE: &[u32] = &[
    0x20000, // 128K
    0x10000, // 64K
    0x8000,  // 32K
    0x8000,  // 32K
];

struct Layer {
    screen_x: i32,
    screen_y: i32,
    buffer_x: u32,
    buffer_y: u32,
    buffer_width: u32,
    buffer_height: u32,
}

struct VGAStats {
    is_graphical: bool,
    res_x: u32,
    res_y: u32,
    bpp: u16,
}

pub(crate) struct VGAScreen {
    store: StoreT,
    vga_memory_size: u32,
    cursor_address: u32,
    cursor_scanline_start: u8,
    cursor_scanline_end: u8,
    max_cols: u16,
    max_rows: u16,
    screen_width: u32,
    screen_height: u32,
    virtual_width: u32,
    virtual_height: u32,
    layers: Vec<Layer>,
    start_address: u32,
    start_address_latched: u32,
    crtc: Vec<u8>,
    crtc_mode: u8,
    horizontal_display_enable_end: u8,
    dispi_index: u16,
    horizontal_blank_start: u8,
    vertical_display_enable_end: u32,
    vertical_blank_start: u32,
    underline_location_register: u8,
    preset_row_scan: u8,
    offset_register: u8,
    line_compare: u32,
    graphical_mode_is_linear: bool,
    graphical_mode: bool,
    vga256_palette: Vec<i32>,
    latch_dword: u32,
    svga_width: u16,
    svga_height: u16,
    svga_enabled: bool,
    svga_bpp: u16,
    svga_bank_offset: u32,
    svga_offset: u32,
    dispi_enable_value: u16,
    pci_space: &'static [u8],
    pci_id: u16,
    pci_bars: Vec<Option<PCIBar>>,
    pci_rom_size: u32,
    pci_rom_address: u32,
    name: &'static str,
    stats: VGAStats,
    index_crtc: u8,
    dac_color_index_write: u32,
    dac_color_index_read: u8,
    dac_state: u8,
    dac_map: Vec<u8>,
    attribute_controller_index: u32,
    palette_source: u8,
    attribute_mode: u8,
    color_plane_enable: u8,
    horizontal_panning: u8,
    color_select: u8,
    sequencer_index: u8,
    plane_write_bm: u8,
    sequencer_memory_mode: u8,
    clocking_mode: u8,
    graphics_index: u8,
    plane_read: u8,
    planar_mode: u8,
    planar_rotate_reg: u8,
    planar_bitmap: u8,
    planar_setreset: u8,
    planar_setreset_enable: u8,
    miscellaneous_graphics_register: u8,
    color_compare: u8,
    color_dont_care: u8,
    max_scan_line: u8,
    miscellaneous_output_register: u8,
    port_3DA_value: u8,
    diff_addr_min: u32,
    diff_addr_max: u32,
    diff_plot_min: u32,
    diff_plot_max: u32,
    svga_memory: Vec<u8>,
    vga_memory: &'static mut [u8],
    svga_memory16: &'static mut [u16],
    svga_memory32: &'static mut [u32],
    plane0: &'static mut [u8],
    plane1: &'static mut [u8],
    plane2: &'static mut [u8],
    plane3: &'static mut [u8],
    pixel_buffer: &'static mut [u8],
}

impl VGAScreen {
    pub fn new(store: StoreT, vga_memory_size: u32) -> Self {
        let cursor_address = 0;
        let cursor_scanline_start = 0xE;
        let cursor_scanline_end = 0xF;
        //Number of columns in text mode
        let max_cols = 80;
        //Number of rows in text mode
        let max_rows = 25;
        //Width in pixels in graphical mode
        let screen_width = 0;
        //Height in pixels in graphical mode
        let screen_height = 0;
        //Logical width in pixels of virtual buffer available for panning
        let virtual_width = 0;
        //Logical height in pixels of virtual buffer available for panning
        let virtual_height = 0;
        //The rectangular fragments of the image buffer, and their destination
        //locations, to be drawn every screen_fill_buffer during VGA modes.
        let layers = Vec::new();
        //video memory start address
        let start_address = 0;
        //Start address - a copy of start_address that only gets updated
        //during VSync, used for panning and page flipping
        let start_address_latched = 0;
        let crtc_mode = 0;
        let horizontal_display_enable_end = 0;
        let horizontal_blank_start = 0;
        let vertical_display_enable_end = 0;
        let vertical_blank_start = 0;
        let underline_location_register = 0;
        let preset_row_scan = 0;
        let offset_register = 0;
        let line_compare = 0;
        let graphical_mode_is_linear = true;
        let graphical_mode = false;
        let latch_dword = 0;
        let svga_width = 0;
        let svga_height = 0;
        let svga_enabled = false;
        let svga_bpp = 32;
        let svga_bank_offset = 0;
        let svga_offset = 0;
        let pci_rom_size = 0x10000;
        let pci_rom_address = 0xFEB00000;
        let name = "vga";
        let stats = VGAStats {
            is_graphical: false,
            res_x: 0,
            res_y: 0,
            bpp: 0,
        };
        let index_crtc = 0;
        let dac_color_index_write = 0;
        let dac_color_index_read = 0;
        let dac_state = 0;
        let attribute_controller_index = 0xFFFF_FFFF;
        let palette_source = 0x20;
        let attribute_mode = 0;
        let color_plane_enable = 0;
        let horizontal_panning = 0;
        let color_select = 0;
        let sequencer_index = 0xFF;
        let plane_write_bm = 0xF;
        let sequencer_memory_mode = 0;
        let clocking_mode = 0;
        let graphics_index = 0xFF;
        // value 0-3, which plane to read
        let plane_read = 0;
        let planar_mode = 0;
        let planar_rotate_reg = 0;
        let planar_bitmap = 0xFF;
        let planar_setreset = 0;
        let planar_setreset_enable = 0;
        let miscellaneous_graphics_register = 0;
        let color_compare = 0;
        let color_dont_care = 0;
        let max_scan_line = 0;
        let miscellaneous_output_register = 0xff;
        let port_3DA_value = 0xFF;
        let crtc = vec![0u8; 0x19];
        let dispi_index = 0xFFFF;
        let dispi_enable_value = 0;
        let diff_addr_min = vga_memory_size;
        let diff_addr_max = 0;
        let diff_plot_min = vga_memory_size;
        let diff_plot_max = 0;
        let vga256_palette = vec![0i32; 256];
        let pci_id = 0x12 << 3;
        let pci_bars = vec![Some(PCIBar {
            size: vga_memory_size,
            original_bar: 0,
            entries: Vec::new(),
        })];
        let pci_space: &[u8] = &[
            0x34,0x12,0x11,0x11,0x03,0x01,0x00,0x00,0x02,0x00,0x00,0x03,0x00,0x00,0x00,0x00,0x08,
            (VGA_LFB_ADDRESS >> 8) as u8,(VGA_LFB_ADDRESS >> 16) as u8,(VGA_LFB_ADDRESS >> 24) as u8,
            0x00,0x00,0x00,0x00,0x00,0x00,0xbf,0xfe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0xf4,0x1a,0x00,0x11,0x00,0x00,0xbe,0xfe,0x00,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        ];
        let dac_map = vec![0u8; 0x10];
        let mut svga_memory = vec![0u8; vga_memory_size as usize];
        let vga_memory = unsafe {
            let ptr = svga_memory.as_mut_ptr();
            slice::from_raw_parts_mut(ptr, 4 * VGA_BANK_SIZE as usize)
        };
        let svga_memory16 = unsafe {
            let ptr = svga_memory.as_mut_ptr() as *mut u16;
            slice::from_raw_parts_mut(ptr, svga_memory.len() / 2 as usize)
        };
        let svga_memory32 = unsafe {
            let ptr = svga_memory.as_mut_ptr() as *mut u32;
            slice::from_raw_parts_mut(ptr, svga_memory.len() / 3 as usize)
        };
        let plane0 = unsafe {
            let ptr = svga_memory.as_mut_ptr();
            slice::from_raw_parts_mut(ptr.add(0 * VGA_BANK_SIZE as usize), VGA_BANK_SIZE as usize)
        };
        let plane1 = unsafe {
            let ptr = svga_memory.as_mut_ptr();
            slice::from_raw_parts_mut(ptr.add(1 * VGA_BANK_SIZE as usize), VGA_BANK_SIZE as usize)
        };
        let plane2 = unsafe {
            let ptr = svga_memory.as_mut_ptr();
            slice::from_raw_parts_mut(ptr.add(2 * VGA_BANK_SIZE as usize), VGA_BANK_SIZE as usize)
        };
        let plane3 = unsafe {
            let ptr = svga_memory.as_mut_ptr();
            slice::from_raw_parts_mut(ptr.add(3 * VGA_BANK_SIZE as usize), VGA_BANK_SIZE as usize)
        };
        let pixel_buffer = unsafe {
            let ptr = svga_memory.as_mut_ptr();
            slice::from_raw_parts_mut(
                ptr.add(VGA_PIXEL_BUFFER_START as usize),
                VGA_PIXEL_BUFFER_SIZE as usize,
            )
        };
        VGAScreen {
            store,
            name,
            crtc,
            stats,
            layers,
            pci_bars,
            dac_map,
            plane0,
            plane1,
            plane2,
            plane3,
            pixel_buffer,
            max_cols,
            vga_memory,
            svga_memory,
            svga_memory16,
            svga_memory32,
            pci_id,
            max_rows,
            pci_space,
            crtc_mode,
            vga256_palette,
            line_compare,
            graphical_mode,
            preset_row_scan,
            latch_dword,
            svga_width,
            svga_height,
            svga_enabled,
            svga_bpp,
            svga_bank_offset,
            svga_offset,
            offset_register,
            screen_width,
            screen_height,
            start_address,
            virtual_width,
            virtual_height,
            cursor_address,
            vga_memory_size,
            cursor_scanline_end,
            pci_rom_size,
            pci_rom_address,
            cursor_scanline_start,
            start_address_latched,
            horizontal_display_enable_end,
            horizontal_blank_start,
            vertical_display_enable_end,
            vertical_blank_start,
            underline_location_register,
            graphical_mode_is_linear,
            index_crtc,
            dac_color_index_write,
            dac_color_index_read,
            dac_state,
            attribute_controller_index,
            palette_source,
            attribute_mode,
            color_plane_enable,
            horizontal_panning,
            color_select,
            sequencer_index,
            plane_write_bm,
            sequencer_memory_mode,
            clocking_mode,
            graphics_index,
            plane_read,
            planar_mode,
            planar_rotate_reg,
            planar_bitmap,
            planar_setreset,
            planar_setreset_enable,
            miscellaneous_graphics_register,
            color_compare,
            color_dont_care,
            max_scan_line,
            miscellaneous_output_register,
            port_3DA_value,
            dispi_index,
            dispi_enable_value,
            diff_addr_min,
            diff_addr_max,
            diff_plot_min,
            diff_plot_max,
        }
    }

    pub fn init(&mut self) {
        self.store.io_mut().map(|io| {
            io.register_write8(
                0x3C0,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3C0_write(v);
                    });
                },
            );
            io.register_read(
                0x3C0,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3C0_read8()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3C0_read16()),
                IO::empty_read32,
            );
            io.register_read8(
                0x3C1,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3C1_read()),
            );
            io.register_write8(
                0x3C2,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3C2_write(v);
                    });
                },
            );
            io.register_write_consecutive(
                0x3C4,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3C4_write(v);
                    });
                },
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3C5_write(v);
                    });
                },
                IO::empty_write8,
                IO::empty_write8,
            );

            io.register_read8(
                0x3C4,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3C4_read()),
            );

            io.register_read8(
                0x3C5,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3C5_read()),
            );

            io.register_write_consecutive(
                0x3CE,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3CE_write(v);
                    });
                },
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3CF_write(v);
                    });
                },
                IO::empty_write8,
                IO::empty_write8,
            );

            io.register_read8(
                0x3CE,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3CE_read()),
            );

            io.register_read8(
                0x3CF,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3CF_read()),
            );

            io.register_write8(
                0x3C7,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3C7_write(v);
                    });
                },
            );

            io.register_read8(
                0x3C7,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3C7_read()),
            );

            io.register_write8(
                0x3C8,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3C8_write(v);
                    });
                },
            );

            io.register_read8(
                0x3C8,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3C8_read()),
            );

            io.register_write8(
                0x3C9,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3C9_write(v);
                    });
                },
            );

            io.register_read8(
                0x3C9,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3C9_read()),
            );

            io.register_read8(
                0x3CC,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3CC_read()),
            );

            io.register_write_consecutive(
                0x3D4,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3D4_write(v);
                    });
                },
                |dev: &Dev, _addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.port3D5_write(v);
                    });
                },
                IO::empty_write8,
                IO::empty_write8,
            );

            io.register_read8(
                0x3D4,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3D4_read()),
            );

            io.register_read(
                0x3D5,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3D5_read()),
                |dev: &Dev, _addr: u32| {
                    dev.vga_mut().map_or(0, |vga| {
                        dbg_log!(Module::VGA, "Warning: 16-bit read from 3D5");
                        vga.port3D5_read() as u16
                    })
                },
                IO::empty_read32,
            );

            io.register_read8(
                0x3CA,
                Dev::Emulator(self.store.clone()),
                |_dev: &Dev, _addr: u32| {
                    dbg_log!(Module::VGA, "3CA read");
                    return 0;
                },
            );

            io.register_read8(
                0x3DA,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3DA_read()),
            );

            io.register_read8(
                0x3BA,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port3DA_read()),
            );

            io.register_write(
                0x1CE,
                Dev::Emulator(self.store.clone()),
                IO::empty_write8,
                |dev: &Dev, _addr: u32, v: u16| {
                    dev.vga_mut().map(|vga| {
                        vga.port1CE_write(v);
                    });
                },
                IO::empty_write32,
            );

            io.register_write(
                0x1CF,
                Dev::Emulator(self.store.clone()),
                IO::empty_write8,
                |dev: &Dev, _addr: u32, v: u16| {
                    dev.vga_mut().map(|vga| {
                        vga.port1CF_write(v);
                    });
                },
                IO::empty_write32,
            );

            io.register_read(
                0x1CF,
                Dev::Emulator(self.store.clone()),
                IO::empty_read8,
                |dev: &Dev, _addr: u32| dev.vga_mut().map_or(0, |vga| vga.port1CF_read()),
                IO::empty_read32,
            );

            io.mmap_register(
                0xA0000,
                0x20000,
                |dev: &Dev, addr: u32| dev.vga_mut().map_or(0, |vga| vga.vga_memory_read(addr)),
                |dev: &Dev, addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.vga_memory_write(addr, v);
                    });
                },
                IO::empty_read32,
                IO::empty_write32,
            );
            io.mmap_register(
                VGA_LFB_ADDRESS,
                self.vga_memory_size as usize,
                |dev: &Dev, addr: u32| dev.vga_mut().map_or(0, |vga| vga.svga_memory_read8(addr)),
                |dev: &Dev, addr: u32, v: u8| {
                    dev.vga_mut().map(|vga| {
                        vga.svga_memory_write8(addr, v);
                    });
                },
                |dev: &Dev, addr: u32| dev.vga_mut().map_or(0, |vga| vga.svga_memory_read32(addr)),
                |dev: &Dev, addr: u32, v: u32| {
                    dev.vga_mut().map(|vga| {
                        vga.svga_memory_write32(addr, v);
                    });
                },
            );
        });
        self.store.pci_mut().map(|pci| {
            pci.register_device(VGADev(self.store.clone()));
        });
    }

    #[inline]
    fn port3CC_read(&self) -> u8 {
        dbg_log!(Module::VGA, "3CC read");
        self.miscellaneous_output_register
    }

    #[inline]
    fn port3C9_read(&mut self) -> u8 {
        dbg_log!(Module::VGA, "3C9 read");
        let index = self.dac_color_index_read / 3 | 0;
        let offset = self.dac_color_index_read % 3;
        let color = self.vga256_palette[index as usize];

        self.dac_color_index_read += 1;
        return ((color >> (2 - offset) * 8 & 0xFF) / 255 * 63 | 0) as u8;
    }

    fn port3C9_write(&mut self, color_byte: u8) {
        let index = (self.dac_color_index_write / 3 | 0) as usize;
        let offset = self.dac_color_index_write % 3;
        let mut color = self.vga256_palette[index];
        let color_byte: i32 = ((color_byte as i32) & 0x3F) * 255 / 63 | 0;
        if offset == 0 {
            color = color & !0xFF0000 | (color_byte as i32) << 16;
        } else if offset == 1 {
            color = color & !0xFF00 | (color_byte as i32) << 8;
        } else {
            color = color & !0xFF | color_byte as i32;
            dbg_log!(
                Module::VGA,
                "dac set color, index={:#X} value={:#X}",
                index,
                color
            );
        }

        if self.vga256_palette[index] != color {
            self.vga256_palette[index] = color;
            self.complete_redraw();
        }
        self.dac_color_index_write += 1;
    }

    #[inline]
    fn port3C8_read(&self) -> u8 {
        (self.dac_color_index_write / 3 & 0xFF) as u8
    }

    #[inline]
    fn port3C8_write(&mut self, index: u8) {
        self.dac_color_index_write = (index as u32) * 3;
        self.dac_state |= 0x3;
    }

    #[inline]
    fn port3C7_read(&self) -> u8 {
        // prepared to accept reads or writes
        self.dac_state
    }

    #[inline]
    fn port3C7_write(&mut self, index: u8) {
        // index for reading the DAC
        dbg_log!(Module::VGA, "3C7 write: {:#X}", index);
        self.dac_color_index_read = index * 3;
        self.dac_state &= 0x0;
    }

    #[inline]
    fn port3CE_read(&self) -> u8 {
        self.graphics_index
    }

    #[inline]
    fn port3CE_write(&mut self, value: u8) {
        self.graphics_index = value;
    }

    fn port3CF_write(&mut self, value: u8) {
        match self.graphics_index {
            0 => {
                self.planar_setreset = value;
                dbg_log!(Module::VGA, "plane set/reset: {:#X}", value);
            }
            1 => {
                self.planar_setreset_enable = value;
                dbg_log!(Module::VGA, "plane set/reset enable: {:#X}", value);
            }
            2 => {
                self.color_compare = value;
                dbg_log!(Module::VGA, "color compare: {:#X}", value);
            }
            3 => {
                self.planar_rotate_reg = value;
                dbg_log!(Module::VGA, "plane rotate: {:#X}", value);
            }
            4 => {
                self.plane_read = value;
                dbg_log!(Module::VGA, "plane read: {:#X}", value);
            }
            5 => {
                let previous_planar_mode = self.planar_mode;
                self.planar_mode = value;
                dbg_log!(Module::VGA, "planar mode: {:#X}", value);
                if (previous_planar_mode ^ value) & 0x60 > 0 {
                    // Shift mode modified. Pixel buffer invalidated
                    self.complete_replot();
                }
            }
            6 => {
                dbg_log!(Module::VGA, "miscellaneous graphics register: {:#X}", value);
                if self.miscellaneous_graphics_register != value {
                    self.miscellaneous_graphics_register = value;
                    self.update_vga_size();
                }
            }
            7 => {
                self.color_dont_care = value;
                dbg_log!(Module::VGA, "color don't care: {:#X}", value);
            }
            8 => {
                self.planar_bitmap = value;
                dbg_log!(Module::VGA, "planar bitmap: {:#X}", value);
            }
            _ => {
                dbg_log!(
                    Module::VGA,
                    "3CF / graphics write {:#X}: {:#X}",
                    self.graphics_index,
                    value
                );
            }
        }
    }

    #[inline]
    fn port3C5_read(&self) -> u8 {
        dbg_log!(
            Module::VGA,
            "3C5 / sequencer read {:#X}",
            self.sequencer_index
        );

        match self.sequencer_index {
            0x01 => self.clocking_mode,
            0x02 => self.plane_write_bm,
            0x04 => self.sequencer_memory_mode,
            0x06 => 0x12,
            _ => 0,
        }
    }

    #[inline]
    fn port3C4_read(&self) -> u8 {
        self.sequencer_index
    }

    #[inline]
    fn port3C4_write(&mut self, value: u8) {
        self.sequencer_index = value;
    }

    #[inline]
    fn port3C2_write(&mut self, value: u8) {
        dbg_log!(
            Module::VGA,
            "3C2 / miscellaneous output register = {:#X}",
            value
        );
        self.miscellaneous_output_register = value;
    }

    fn port3C1_read(&self) -> u8 {
        if self.attribute_controller_index < 0x10 {
            dbg_log!(
                Module::VGA,
                "3C1 / internal palette read: {:#X} -> {:#X}",
                self.attribute_controller_index,
                self.dac_map[self.attribute_controller_index as usize],
            );
            return self.dac_map[self.attribute_controller_index as usize] & 0xFF;
        }

        match self.attribute_controller_index {
            0x10 => {
                dbg_log!(
                    Module::VGA,
                    "3C1 / attribute mode read: {:#X}",
                    self.attribute_mode
                );
                self.attribute_mode
            }
            0x12 => {
                dbg_log!(
                    Module::VGA,
                    "3C1 / color plane enable read: {:#X}",
                    self.color_plane_enable
                );
                self.color_plane_enable
            }
            0x13 => {
                dbg_log!(
                    Module::VGA,
                    "3C1 / horizontal panning read: {:#X}",
                    self.horizontal_panning
                );
                self.horizontal_panning
            }
            0x14 => {
                dbg_log!(
                    Module::VGA,
                    "3C1 / color select read: {:#X}",
                    self.color_select
                );
                self.color_select
            }
            _ => {
                dbg_log!(
                    Module::VGA,
                    "3C1 / attribute controller read {:#X}",
                    self.attribute_controller_index
                );
                0xFF
            }
        }
    }

    #[inline]
    fn port3C0_read8(&self) -> u8 {
        dbg_log!(Module::VGA, "3C0 read");
        let result = self.attribute_controller_index as u8 | self.palette_source;
        return result;
    }

    #[inline]
    fn port3C0_read16(&self) -> u16 {
        dbg_log!(Module::VGA, "3C0 read16");
        return (self.port3C0_read8() as u16) & 0xFF | ((self.port3C1_read() as u16) << 8 & 0xFF00);
    }

    #[inline]
    fn svga_memory_read8(&self, addr: u32) -> u8 {
        return self.svga_memory[(addr & 0xFFFFFFF) as usize];
    }
    #[inline]
    fn svga_memory_read32(&self, mut addr: u32) -> u32 {
        let addr: usize = (addr & 0xFFFFFFF) as usize;
        if addr & 3 > 0 {
            return self.svga_memory[addr] as u32
                | (self.svga_memory[addr + 1] as u32) << 8
                | (self.svga_memory[addr + 2] as u32) << 16
                | (self.svga_memory[addr + 3] as u32) << 24;
        } else {
            return self.svga_memory32[addr >> 2];
        }
    }

    #[inline]
    fn svga_memory_write8(&mut self, mut addr: u32, value: u8) {
        addr &= 0xFFFFFFF;
        self.svga_memory[addr as usize] = value;

        self.diff_addr_min = if addr < self.diff_addr_min {
            addr
        } else {
            self.diff_addr_min
        };
        self.diff_addr_max = if addr > self.diff_addr_max {
            addr
        } else {
            self.diff_addr_max
        };
    }

    fn svga_memory_write32(&mut self, mut addr: u32, value: u32) {
        addr &= 0xFFFFFFF;
        self.diff_addr_min = if addr < self.diff_addr_min {
            addr
        } else {
            self.diff_addr_min
        };
        self.diff_addr_max = if addr + 3 > self.diff_addr_max {
            addr + 3
        } else {
            self.diff_addr_max
        };
        let addr: usize = addr as usize;
        self.svga_memory[addr] = value as u8;
        self.svga_memory[addr + 1] = (value >> 8) as u8;
        self.svga_memory[addr + 2] = (value >> 16) as u8;
        self.svga_memory[addr + 3] = (value >> 24) as u8;
    }

    fn vga_memory_read(&mut self, mut addr: u32) -> u8 {
        if self.svga_enabled && self.graphical_mode_is_linear {
            addr -= 0xA0000;
            addr |= self.svga_bank_offset as u32;

            return self.svga_memory[addr as usize];
        }

        let memory_space_select = self.miscellaneous_graphics_register >> 2 & 0x3;
        addr -= VGA_HOST_MEMORY_SPACE_START[memory_space_select as usize];

        // VGA chip only decodes addresses within the selected memory space.
        if addr >= VGA_HOST_MEMORY_SPACE_SIZE[memory_space_select as usize] as u32 {
            dbg_log!(
                Module::VGA,
                "vga read outside memory space: addr:{:#X}",
                addr
            );
            return 0;
        }
        let i_addr: usize = addr as usize;
        self.latch_dword = self.plane0[i_addr] as u32;
        self.latch_dword |= (self.plane1[i_addr] as u32) << 8;
        self.latch_dword |= (self.plane2[i_addr] as u32) << 16;
        self.latch_dword |= (self.plane3[i_addr] as u32) << 24;

        if self.planar_mode & 0x08 > 0 {
            // read mode 1
            let mut reading = 0xFF;
            macro_rules! reading_proc {
                ($mask: literal, $e: expr) => {
                    if self.color_dont_care & $mask > 0 {
                        let mask = if self.color_compare & $mask > 0 {
                            0xFF
                        } else {
                            0x00
                        };
                        reading &= $e ^ !mask;
                    }
                };
            }
            reading_proc!(0x1, self.plane0[i_addr]);
            reading_proc!(0x2, self.plane1[i_addr]);
            reading_proc!(0x4, self.plane2[i_addr]);
            reading_proc!(0x8, self.plane3[i_addr]);
            return reading;
        } else {
            // read mode 0

            let mut plane = self.plane_read;
            if !self.graphical_mode {
                // We currently put all text data linearly
                plane = 0;
            } else if self.sequencer_memory_mode & 0x8 > 0 {
                // Chain 4
                plane = (addr & 0x3) as u8;
                addr &= !0x3;
            } else if self.planar_mode & 0x10 > 0 {
                // Odd/Even host read
                plane = (addr & 0x1) as u8;
                addr &= !0x1;
            }
            return self.vga_memory[(plane as usize) << 16 | addr as usize];
        }
    }

    fn vga_memory_write(&mut self, mut addr: u32, value: u8) {
        if self.svga_enabled && self.graphical_mode && self.graphical_mode_is_linear {
            // vbe banked mode
            addr -= 0xA0000;
            self.vga_memory_write_graphical_linear(addr, value);
            return;
        }

        let memory_space_select = self.miscellaneous_graphics_register >> 2 & 0x3;
        addr -= VGA_HOST_MEMORY_SPACE_START[memory_space_select as usize] as u32;

        if addr >= VGA_HOST_MEMORY_SPACE_SIZE[memory_space_select as usize] as u32 {
            dbg_log!(
                Module::VGA,
                "vga write outside memory space: addr:{:#X}, value:{:#X}",
                addr,
                value
            );
            return;
        }

        if self.graphical_mode {
            self.vga_memory_write_graphical(addr, value);
        } else {
            if !(self.plane_write_bm & 0x3 > 0) {
                // Ignore writes to font planes.
                return;
            }
            self.vga_memory_write_text_mode(addr, value);
        }
    }

    fn vga_memory_write_text_mode(&mut self, addr: u32, value: u8) {
        let memory_start = (addr >> 1) - self.start_address;
        let row = memory_start / self.max_cols as u32 | 0;
        let col = memory_start % self.max_cols as u32;
        let chr;
        let color;

        // XXX: Should handle 16 bit write if possible
        if addr & 1 > 0 {
            color = value;
            chr = self.vga_memory[addr as usize & !1];
        } else {
            chr = value;
            color = self.vga_memory[addr as usize | 1];
        }
        self.store.bus_mut().map(|bus| {
            bus.send(
                "screen-put-char",
                BusData::ScreenPutChar(
                    row as u16,
                    col as u16,
                    chr,
                    self.vga256_palette[(color as usize) >> 4 & 0xF],
                    self.vga256_palette[(color as usize) & 0xF],
                ),
            );
        });
        self.vga_memory[addr as usize] = value;
    }

    #[inline]
    fn vga_memory_write_graphical_linear(&mut self, mut addr: u32, value: u8) {
        addr |= self.svga_bank_offset as u32;
        self.diff_addr_min = if addr < self.diff_addr_min {
            addr
        } else {
            self.diff_addr_min
        };
        self.diff_addr_max = if addr > self.diff_addr_max {
            addr
        } else {
            self.diff_addr_max
        };
        self.svga_memory[addr as usize] = value;
    }

    #[inline]
    fn apply_feed(&self, data_byte: u8) -> u32 {
        let data_byte = data_byte as u32;
        let mut dword = data_byte;
        dword |= data_byte << 8;
        dword |= data_byte << 16;
        dword |= data_byte << 24;
        dword
    }

    #[inline]
    fn apply_expand(&self, data_byte: u8) -> u32 {
        let mut dword: u32 = if data_byte & 0x1 > 0 { 0xFF } else { 0x00 };
        dword |= if data_byte & 0x2 > 0 { 0xFF } else { 0x00 } << 8;
        dword |= if data_byte & 0x4 > 0 { 0xFF } else { 0x00 } << 16;
        dword |= if data_byte & 0x8 > 0 { 0xFF } else { 0x00 } << 24;
        return dword;
    }

    #[inline]
    fn apply_rotate(&self, data_byte: u8) -> u8 {
        let data_byte: u32 = data_byte as u32;
        let wrapped = data_byte | (data_byte << 8);
        let count = self.planar_rotate_reg & 0x7;
        let shifted = wrapped >> count;
        return (shifted & 0xFF) as u8;
    }

    #[inline]
    fn apply_setreset(&self, mut data_dword: u32, enable_dword: u32) -> u32 {
        let setreset_dword = self.apply_expand(self.planar_setreset);
        data_dword |= enable_dword & setreset_dword;
        data_dword &= !enable_dword | setreset_dword;
        return data_dword;
    }

    #[inline]
    fn apply_logical(&self, data_dword: u32, latch_dword: u32) -> u32 {
        match self.planar_rotate_reg & 0x18 {
            0x08 => data_dword & latch_dword,
            0x10 => data_dword | latch_dword,
            0x18 => data_dword ^ latch_dword,
            _ => data_dword,
        }
    }

    #[inline]
    fn apply_bitmask(&self, data_dword: u32, bitmask_dword: u32) -> u32 {
        let mut plane_dword = bitmask_dword & data_dword;
        plane_dword |= !bitmask_dword & self.latch_dword;
        plane_dword
    }

    #[inline]
    fn text_mode_redraw(&self) {
        let mut addr = (self.start_address << 1) as usize;
        let mut chr = 0;
        let mut color = 0;

        for row in 0..self.max_rows {
            for col in 0..self.max_cols {
                chr = self.vga_memory[addr];
                color = self.vga_memory[addr | 1];
                self.store.bus_mut().map(|bus| {
                    bus.send(
                        "screen-put-char",
                        BusData::ScreenPutChar(
                            row,
                            col,
                            chr,
                            self.vga256_palette[(color >> 4 & 0xF) as usize],
                            self.vga256_palette[(color & 0xF) as usize],
                        ),
                    );
                });
                addr += 2;
            }
        }
    }

    #[inline]
    fn vga_bytes_per_line(&self) -> u8 {
        let mut bytes_per_line = self.offset_register << 2;
        if self.underline_location_register & 0x40 > 0 {
            bytes_per_line <<= 1;
        } else if self.crtc_mode & 0x40 > 0 {
            bytes_per_line >>= 1;
        }
        bytes_per_line as u8
    }

    fn scan_line_to_screen_row(&self, mut scan_line: u32) -> u32 {
        // Double scanning. The clock to the row scan counter is halved
        // so it is not affected by the memory address bit substitutions below
        if self.max_scan_line & 0x80 > 0 {
            scan_line >>= 1;
        }

        // Maximum scan line, aka scan lines per character row
        // This is the number of repeats - 1 for graphic modes
        let repeat_factor = 1 + (self.max_scan_line & 0x1F);
        scan_line = (scan_line as f32 / repeat_factor as f32).ceil() as u32;

        // Odd and Even Row Scan Counter
        // Despite repeated address counter values, because bit 13 of the shifted
        // address is substituted with bit 0 of the row scan counter, a different
        // display buffer address is generated instead of repeated
        // Assumes maximum scan line register is set to 2 or 4.
        // Note: can't assert this as register values may not be fully programmed.
        if (self.crtc_mode & 0x1) == 0 {
            scan_line <<= 1;
        }

        // Undo effects of substituted bit 14
        // Assumes maximum scan line register is set to 2 or 4
        // Note: can't assert this as register values may not be fully programmed.
        // Other maximum scan line register values would result in weird addressing
        // anyway
        if (self.crtc_mode & 0x2) == 0 {
            scan_line <<= 1;
        }

        return scan_line;
    }

    fn update_layers(&mut self) {
        if !self.graphical_mode {
            self.text_mode_redraw();
        }

        if self.svga_enabled {
            self.layers.clear();
            return;
        }

        if self.virtual_width == 0 || self.screen_width == 0 {
            // Avoid division by zero
            return;
        }

        if self.palette_source == 0 || self.clocking_mode & 0x20 > 0 {
            // Palette source and screen disable bits = draw nothing
            // See http://www.phatcode.net/res/224/files/html/ch29/29-05.html#Heading6
            // and http://www.osdever.net/FreeVGA/vga/seqreg.htm#01
            self.layers.clear();
            self.store.bus_mut().map(|bus| {
                bus.send("screen-clear", BusData::None);
            });

            return;
        }

        let start_addr = self.start_address_latched;

        let mut pixel_panning = self.horizontal_panning;
        if self.attribute_mode & 0x40 > 0 {
            pixel_panning >>= 1;
        }

        let byte_panning = self.preset_row_scan >> 5 & 0x3;
        let pixel_addr_start = self.vga_addr_to_pixel(start_addr + byte_panning as u32);

        let start_buffer_row = pixel_addr_start / self.virtual_width | 0;
        let start_buffer_col = pixel_addr_start % self.virtual_width + pixel_panning as u32;

        let split_screen_row = self.scan_line_to_screen_row(1 + self.line_compare);
        let split_screen_row = split_screen_row.min(self.screen_height as u32);

        let split_buffer_height = self.screen_height - split_screen_row;

        self.layers.clear();
        let mut x = -(start_buffer_col as i32);
        let mut y = 0;
        while x < self.screen_width as i32 {
            self.layers.push(Layer {
                screen_x: x,
                screen_y: 0,
                buffer_x: 0,
                buffer_y: start_buffer_row + y,
                buffer_width: self.virtual_width,
                buffer_height: split_screen_row as u32,
            });
            x += self.virtual_width as i32;
            y += 1;
        }

        let mut start_split_col = 0;
        if (self.attribute_mode & 0x20) == 0 {
            // Pixel panning mode. Allow panning for the lower split screen
            start_split_col =
                (self.vga_addr_to_pixel(byte_panning as u32) + pixel_panning as u32) as i32;
        }
        let mut x = -(start_buffer_col as i32);
        let mut y = 0;
        while x < self.screen_width as i32 {
            self.layers.push(Layer {
                screen_x: x,
                screen_y: split_screen_row as i32,
                buffer_x: 0,
                buffer_y: y,
                buffer_width: self.virtual_width,
                buffer_height: split_buffer_height as u32,
            });
            x += self.virtual_width as i32;
            y += 1;
        }
    }

    fn complete_redraw(&mut self) {
        dbg_log!(Module::VGA, "complete redraw");
        if self.graphical_mode {
            self.diff_addr_min = 0;

            if self.svga_enabled {
                self.diff_addr_max = self.vga_memory_size;
            } else {
                self.diff_addr_max = VGA_PIXEL_BUFFER_SIZE;
            }
        } else {
            self.text_mode_redraw();
        }
    }

    fn vga_memory_write_graphical(&mut self, mut addr: u32, mut value: u8) {
        let mut plane_dword = 0;
        let write_mode = self.planar_mode & 3;
        let mut bitmask = self.apply_feed(self.planar_bitmap);
        let setreset_dword = self.apply_expand(self.planar_setreset);
        let setreset_enable_dword = self.apply_expand(self.planar_setreset_enable);

        // Write modes - see http://www.osdever.net/FreeVGA/vga/graphreg.htm#05
        match write_mode {
            0 => {
                value = self.apply_rotate(value);
                plane_dword = self.apply_feed(value);
                plane_dword = self.apply_setreset(plane_dword, setreset_enable_dword);
                plane_dword = self.apply_logical(plane_dword, self.latch_dword);
                plane_dword = self.apply_bitmask(plane_dword, bitmask);
            }
            1 => {
                plane_dword = self.latch_dword;
            }
            2 => {
                plane_dword = self.apply_expand(value);
                plane_dword = self.apply_logical(plane_dword, self.latch_dword);
                plane_dword = self.apply_bitmask(plane_dword, bitmask);
            }
            3 => {
                value = self.apply_rotate(value);
                bitmask &= self.apply_feed(value);
                plane_dword = setreset_dword;
                plane_dword = self.apply_bitmask(plane_dword, bitmask);
            }
            _ => {}
        }

        let mut plane_select = 0xF;

        match self.sequencer_memory_mode & 0xC {
            // Odd/Even (aka chain 2)
            0x0 => {
                plane_select = 0x5 << (addr & 0x1);
                addr &= !0x1;
            }

            // Chain 4
            // Note: FreeVGA may have mistakenly stated that this bit field is
            // for system read only, yet the IBM Open Source Graphics Programmer's
            // Reference Manual explicitly states "both read and write".
            0x8 | 0xC => {
                plane_select = 1 << (addr & 0x3);
                addr &= !0x3;
            }
            _ => {}
        }

        // Plane masks take precedence
        // See: http://www.osdever.net/FreeVGA/vga/seqreg.htm#02
        plane_select &= self.plane_write_bm;

        if plane_select & 0x1 > 0 {
            self.plane0[addr as usize] = ((plane_dword >> 0) & 0xFF) as u8;
        }
        if plane_select & 0x2 > 0 {
            self.plane1[addr as usize] = ((plane_dword >> 8) & 0xFF) as u8;
        }
        if plane_select & 0x4 > 0 {
            self.plane2[addr as usize] = ((plane_dword >> 16) & 0xFF) as u8;
        }
        if plane_select & 0x8 > 0 {
            self.plane3[addr as usize] = ((plane_dword >> 24) & 0xFF) as u8;
        }
        let pixel_addr = self.vga_addr_to_pixel(addr);
        self.partial_replot(pixel_addr, pixel_addr + 7);
    }

    fn vga_addr_shift_count(&self) -> u8 {
        // Count in multiples of 0x40 for convenience
        // Left shift 2 for word mode - 2 bytes per dot clock
        let mut shift_count = 0x80;

        // Left shift 3 for byte mode - 1 byte per dot clock
        shift_count += !self.underline_location_register & self.crtc_mode & 0x40;

        // Left shift 1 for doubleword mode - 4 bytes per dot clock
        shift_count -= self.underline_location_register & 0x40;

        // But shift one less if PEL width mode - 2 dot clocks per pixel
        shift_count -= self.attribute_mode & 0x40;

        return (shift_count >> 6) as u8;
    }

    fn partial_replot(&mut self, min: u32, max: u32) {
        if min < self.diff_plot_min {
            self.diff_plot_min = min;
        }
        if max > self.diff_plot_max {
            self.diff_plot_max = max;
        }

        self.partial_redraw(min, max);
    }

    fn partial_redraw(&mut self, min: u32, max: u32) {
        if min < self.diff_addr_min {
            self.diff_addr_min = min;
        }
        if max > self.diff_addr_max {
            self.diff_addr_max = max;
        }
    }

    fn vga_addr_to_pixel(&self, addr: u32) -> u32 {
        let shift_count = self.vga_addr_shift_count();

        // Undo effects of substituted bits 13 and 14
        // Assumptions:
        //  - max_scan_line register is set to the values shown below
        //  - Each scan line stays within the offset alignment
        //  - No panning and no page flipping after drawing
        if !self.crtc_mode & 0x3 > 0 {
            let mut pixel_addr = addr - self.start_address;

            // Remove substituted bits
            pixel_addr &= (self.crtc_mode as u32) << 13 | !0x6000;

            // Convert to 1 pixel per address
            pixel_addr <<= shift_count;

            // Decompose address
            let mut row = pixel_addr / self.virtual_width | 0;
            let col = pixel_addr % self.virtual_width;

            match self.crtc_mode & 0x3 {
                0x2 => {
                    // Alternating rows using bit 13
                    // Assumes max scan line = 1
                    row = row << 1 | (addr >> 13 & 0x1);
                }
                0x1 => {
                    // Alternating rows using bit 14
                    // Assumes max scan line = 3
                    row = row << 1 | (addr >> 14 & 0x1);
                }
                0x0 => {
                    // Cycling through rows using bit 13 and 14
                    // Assumes max scan line = 3
                    row = row << 2 | (addr >> 13 & 0x3);
                }
                _ => {}
            }

            // Reassemble address
            row * self.virtual_width + col + (self.start_address << shift_count)
        } else {
            // Convert to 1 pixel per address
            addr << shift_count
        }
    }

    fn complete_replot(&mut self) {
        dbg_log!(Module::VGA, "complete replot");

        if !self.graphical_mode || self.svga_enabled {
            return;
        }

        self.diff_plot_min = 0;
        self.diff_plot_max = VGA_PIXEL_BUFFER_SIZE;

        self.complete_redraw();
    }

    fn update_vga_size(&mut self) {
        if self.svga_enabled {
            return;
        }

        let horizontal_characters =
            (1 + self.horizontal_display_enable_end).min(self.horizontal_blank_start);
        let mut vertical_scans =
            (1 + self.vertical_display_enable_end).min(self.vertical_blank_start);

        if horizontal_characters == 0 || vertical_scans == 0 {
            // Don't update if width or height is zero.
            // These happen when registers are not fully configured yet.
            return;
        }

        if self.graphical_mode {
            let mut screen_width: u32 = (horizontal_characters as u32) << 3;

            // Offset is half the number of bytes/words/dwords (depending on clocking mode)
            // of display memory that each logical line occupies.
            // However, the number of pixels latched, regardless of addressing mode,
            // should always 8 pixels per character clock (except for 8 bit PEL width, in which
            // case 4 pixels).
            let mut virtual_width = (self.offset_register as u32) << 4;

            // Pixel Width / PEL Width / Clock Select
            if self.attribute_mode & 0x40 > 0 {
                screen_width >>= 1;
                virtual_width >>= 1;
            }

            let screen_height = self.scan_line_to_screen_row(vertical_scans as u32);

            // The virtual buffer height is however many rows of data that can fit.
            // Previously drawn graphics outside of current memory address space can
            // still be drawn by setting start_address. The address at
            // VGA_HOST_MEMORY_SPACE_START[memory_space_select] is mapped to the first
            // byte of the frame buffer. Verified on some hardware.
            // Depended on by: Windows 98 start screen
            let available_bytes = VGA_HOST_MEMORY_SPACE_SIZE[0];

            let virtual_height =
                (available_bytes as f32 / self.vga_bytes_per_line() as f32).ceil() as u32;

            self.set_size_graphical(
                screen_width,
                screen_height,
                8,
                virtual_width,
                virtual_height,
            );

            self.update_vertical_retrace();
            self.update_layers();
        } else {
            if self.max_scan_line & 0x80 > 0 {
                // Double scanning means that half of those scan lines
                // are just repeats
                vertical_scans >>= 1;
            }

            let height = (vertical_scans / (1 + ((self.max_scan_line as u32) & 0x1F)) | 0) as u16;

            if horizontal_characters > 0 && height > 0 {
                self.set_size_text(horizontal_characters as u16, height);
            }
        }
    }

    fn set_size_text(&mut self, cols_count: u16, rows_count: u16) {
        self.max_cols = cols_count;
        self.max_rows = rows_count;
        self.store.bus_mut().map(|bus| {
            bus.send(
                "screen-set-size-text",
                BusData::U16Tuple(cols_count, rows_count),
            );
        });
    }

    fn set_size_graphical(
        &mut self,
        width: u32,
        height: u32,
        bpp: u16,
        virtual_width: u32,
        virtual_height: u32,
    ) {
        let needs_update = !self.stats.is_graphical
            || self.stats.bpp != bpp
            || self.screen_width != width
            || self.screen_height != height
            || self.virtual_width != virtual_width
            || self.virtual_height != virtual_height;

        if needs_update {
            self.screen_width = width;
            self.screen_height = height;
            self.virtual_width = virtual_width;
            self.virtual_height = virtual_height;

            self.stats.bpp = bpp;
            self.stats.is_graphical = true;
            self.stats.res_x = width;
            self.stats.res_y = height;
            self.store.bus_mut().map(|bus| {
                bus.send(
                    "screen-set-size-graphical",
                    BusData::ScreenSetSizeGraphical(
                        width,
                        height,
                        virtual_width,
                        virtual_height,
                        bpp,
                    ),
                );
            });
        }
    }

    #[inline]
    fn update_vertical_retrace(&mut self) {
        // Emulate behaviour during VSync/VRetrace
        self.port_3DA_value |= 0x8;
        if self.start_address_latched != self.start_address {
            self.start_address_latched = self.start_address;
            self.update_layers();
        }
    }

    fn port3C0_write(&mut self, value: u8) {
        if self.attribute_controller_index == 0xFFFF_FFFF {
            dbg_log!(
                Module::VGA,
                "attribute controller index register: {:#X}",
                value
            );
            self.attribute_controller_index = (value & 0x1F) as u32;
            dbg_log!(
                Module::VGA,
                "attribute actual index: {:#X}",
                self.attribute_controller_index
            );

            if self.palette_source != (value & 0x20) {
                // A method of blanking the screen.
                // See http://www.phatcode.net/res/224/files/html/ch29/29-05.html#Heading6
                self.palette_source = value & 0x20;
                self.update_layers();
            }
        } else {
            if self.attribute_controller_index < 0x10 {
                dbg_log!(
                    Module::VGA,
                    "internal palette: {:#X} -> {:#X}",
                    self.attribute_controller_index,
                    value
                );
                self.dac_map[self.attribute_controller_index as usize] = value;
                if (self.attribute_mode & 0x40) == 0 {
                    self.complete_redraw();
                }
            } else {
                match self.attribute_controller_index as u32 {
                    0x10 => {
                        dbg_log!(Module::VGA, "3C0 / attribute mode control: {:#X}", value);
                        if self.attribute_mode != value {
                            let previous_mode = self.attribute_mode;
                            self.attribute_mode = value;
                            let is_graphical = (value & 0x1) > 0;
                            if !self.svga_enabled && self.graphical_mode != is_graphical {
                                self.graphical_mode = is_graphical;
                                self.store.bus_mut().map(|bus| {
                                    bus.send("screen-set-mode", BusData::Bool(self.graphical_mode));
                                });
                            }

                            if (previous_mode ^ value) & 0x40 > 0 {
                                // PEL width changed. Pixel Buffer now invalidated
                                self.complete_replot();
                            }

                            self.update_vga_size();

                            // Data stored in image buffer are invalidated
                            self.complete_redraw();
                        }
                    }
                    0x12 => {
                        dbg_log!(Module::VGA, "3C0 / color plane enable: {:#X}", value);
                        if self.color_plane_enable != value {
                            self.color_plane_enable = value;

                            // Data stored in image buffer are invalidated
                            self.complete_redraw();
                        }
                    }
                    0x13 => {
                        dbg_log!(Module::VGA, "3C0 / horizontal panning: {:#X}", value);
                        if self.horizontal_panning != value {
                            self.horizontal_panning = value & 0xF;
                            self.update_layers();
                        }
                    }
                    0x14 => {
                        dbg_log!(Module::VGA, "3C0 / color select: {:#X}", value);
                        if self.color_select != value {
                            self.color_select = value;

                            // Data stored in image buffer are invalidated
                            self.complete_redraw();
                        }
                    }
                    _ => {
                        dbg_log!(
                            Module::VGA,
                            "3C0 / attribute controller write {:#X}: {:#X}",
                            self.attribute_controller_index,
                            value
                        );
                    }
                }
            }
            self.attribute_controller_index = 0xFFFF_FFFF;
        }
    }

    #[inline]
    fn update_cursor_scanline(&mut self) {
        self.store.bus_mut().map(|bus| {
            bus.send(
                "screen-update-cursor-scanline",
                BusData::U8Tuple(self.cursor_scanline_start, self.cursor_scanline_end),
            )
        });
    }

    fn port3C5_write(&mut self, value: u8) {
        match self.sequencer_index {
            0x01 => {
                dbg_log!(Module::VGA, "clocking mode: {:#X}", value);
                let previous_clocking_mode = self.clocking_mode;
                self.clocking_mode = value;
                if (previous_clocking_mode ^ value) & 0x20 > 0 {
                    // Screen disable bit modified
                    self.update_layers();
                }
            }
            0x02 => {
                dbg_log!(Module::VGA, "plane write mask: {:#X}", value);
                self.plane_write_bm = value;
            }
            0x04 => {
                dbg_log!(Module::VGA, "sequencer memory mode: {:#X}", value);
                self.sequencer_memory_mode = value;
            }
            _ => {
                dbg_log!(
                    Module::VGA,
                    "3C5 / sequencer write {:#X}: {:#X}",
                    self.sequencer_index,
                    value
                );
            }
        }
    }

    #[inline]
    fn port3CF_read(&self) -> u8 {
        dbg_log!(Module::VGA, "3CF / graphics read {:X}", self.graphics_index);
        match self.graphics_index {
            0 => self.planar_setreset,
            1 => self.planar_setreset_enable,
            2 => self.color_compare,
            3 => self.planar_rotate_reg,
            4 => self.plane_read,
            5 => self.planar_mode,
            6 => self.miscellaneous_graphics_register,
            7 => self.color_dont_care,
            8 => self.planar_bitmap,
            _ => 0,
        }
    }

    fn port3D4_write(&mut self, register: u8) {
        dbg_log!(Module::VGA, "3D4 / crtc index: {:#X}", register);
        self.index_crtc = register;
    }

    fn port3D4_read(&self) -> u8 {
        dbg_log!(Module::VGA, "3D4 read / crtc index: {:#X}", self.index_crtc);
        self.index_crtc
    }

    fn update_cursor(&mut self) {
        let mut row =
            ((self.cursor_address - self.start_address) / self.max_cols as u32 | 0) as u16;
        let col = ((self.cursor_address - self.start_address) % self.max_cols as u32) as u16;

        row = (self.max_rows - 1).min(row);
        self.store.bus_mut().map(|bus| {
            bus.send("screen-update-cursor", BusData::U16Tuple(row, col));
        });
    }

    fn port3D5_write(&mut self, value: u8) {
        match self.index_crtc {
            0x1 => {
                dbg_log!(Module::VGA, "3D5 / hdisp enable end write: {:#X}", value);
                if self.horizontal_display_enable_end != value {
                    self.horizontal_display_enable_end = value;
                    self.update_vga_size();
                }
            }
            0x2 => {
                if self.horizontal_blank_start != value {
                    self.horizontal_blank_start = value;
                    self.update_vga_size();
                }
            }
            0x7 => {
                dbg_log!(Module::VGA, "3D5 / overflow register write: {:#X}", value);
                let previous_vertical_display_enable_end = self.vertical_display_enable_end;
                self.vertical_display_enable_end &= 0xFF;
                let value: u32 = value as u32;
                self.vertical_display_enable_end |= (value << 3 & 0x200) | (value << 7 & 0x100);
                if previous_vertical_display_enable_end != self.vertical_display_enable_end {
                    self.update_vga_size();
                }
                self.line_compare = (self.line_compare & 0x2FF) | (value << 4 & 0x100);

                let previous_vertical_blank_start = self.vertical_blank_start;
                self.vertical_blank_start =
                    (self.vertical_blank_start & 0x2FF) | (value << 5 & 0x100);
                if previous_vertical_blank_start != self.vertical_blank_start {
                    self.update_vga_size();
                }
                self.update_layers();
            }
            0x8 => {
                dbg_log!(Module::VGA, "3D5 / preset row scan write: {:#X}", value);
                self.preset_row_scan = value;
                self.update_layers();
            }
            0x9 => {
                dbg_log!(Module::VGA, "3D5 / max scan line write: {:#X}", value);
                self.max_scan_line = value;
                let value: u32 = value as u32;
                self.line_compare = (self.line_compare & 0x1FF) | (value << 3 & 0x200);

                let previous_vertical_blank_start = self.vertical_blank_start;
                self.vertical_blank_start =
                    (self.vertical_blank_start & 0x1FF) | (value << 4 & 0x200);
                if previous_vertical_blank_start != self.vertical_blank_start {
                    self.update_vga_size();
                }

                self.update_layers();
            }
            0xA => {
                dbg_log!(
                    Module::VGA,
                    "3D5 / cursor scanline start write: {:#X}",
                    value
                );
                self.cursor_scanline_start = value;
                self.update_cursor_scanline();
            }
            0xB => {
                dbg_log!(
                    Module::VGA,
                    "3D5 / cursor scanline end write: #{:#X}",
                    value
                );
                self.cursor_scanline_end = value;
                self.update_cursor_scanline();
            }
            0xC => {
                if (self.start_address >> 8 & 0xFF) as u8 != value {
                    let value: u32 = value as u32;
                    self.start_address = self.start_address & 0xff | value << 8;
                    self.update_layers();
                    if !self.crtc_mode & 0x3 > 0 {
                        // Address substitution implementation depends on the
                        // starting row and column, so the pixel buffer is invalidated.
                        self.complete_replot();
                    }
                }
                dbg_log!(
                    Module::VGA,
                    "3D5 / start addr hi write: {:#X} -> {:#04X}",
                    value,
                    self.start_address
                );
            }
            0xD => {
                if (self.start_address & 0xFF) as u8 != value {
                    self.start_address = self.start_address & 0xff00 | value as u32;
                    self.update_layers();
                    if !self.crtc_mode & 0x3 > 0 {
                        // Address substitution implementation depends on the
                        // starting row and column, so the pixel buffer is invalidated.
                        self.complete_replot();
                    }
                }
                dbg_log!(
                    Module::VGA,
                    "3D5 / start addr lo write: {:#X} -> {:#04X}",
                    value,
                    self.start_address
                );
            }
            0xE => {
                dbg_log!(Module::VGA, "3D5 / cursor address hi write: {:#X}", value);
                self.cursor_address = self.cursor_address & 0xFF | (value as u32) << 8;
                self.update_cursor();
            }
            0xF => {
                dbg_log!(Module::VGA, "3D5 / cursor address lo write: {:#X}", value);
                self.cursor_address = self.cursor_address & 0xFF00 | (value as u32);
                self.update_cursor();
            }
            0x12 => {
                dbg_log!(Module::VGA, "3D5 / vdisp enable end write: {:#X}", value);
                if (self.vertical_display_enable_end & 0xFF) as u8 != value {
                    self.vertical_display_enable_end =
                        (self.vertical_display_enable_end & 0x300) | (value as u32);
                    self.update_vga_size();
                }
            }
            0x13 => {
                dbg_log!(Module::VGA, "3D5 / offset register write: {:#X}", value);
                if self.offset_register != value {
                    self.offset_register = value;
                    self.update_vga_size();

                    if !self.crtc_mode & 0x3 > 0 {
                        // Address substitution implementation depends on the
                        // virtual width, so the pixel buffer is invalidated.
                        self.complete_replot();
                    }
                }
            }
            0x14 => {
                dbg_log!(Module::VGA, "3D5 / underline location write: {:#X}", value);
                if self.underline_location_register != value {
                    let previous_underline = self.underline_location_register;

                    self.underline_location_register = value;
                    self.update_vga_size();

                    if (previous_underline ^ value) & 0x40 > 0 {
                        // Doubleword addressing changed. Pixel buffer invalidated.
                        self.complete_replot();
                    }
                }
            }
            0x15 => {
                dbg_log!(
                    Module::VGA,
                    "3D5 / vertical blank start write: {:#X}",
                    value
                );
                if (self.vertical_blank_start & 0xFF) as u8 != value {
                    self.vertical_blank_start = (self.vertical_blank_start & 0x300) | value as u32;
                    self.update_vga_size();
                }
            }
            0x17 => {
                dbg_log!(Module::VGA, "3D5 / crtc mode write: {:#X}", value);
                if self.crtc_mode != value {
                    let previous_mode = self.crtc_mode;

                    self.crtc_mode = value;
                    self.update_vga_size();

                    if (previous_mode ^ value) & 0x43 > 0 {
                        // Word/byte addressing changed or address substitution changed.
                        // Pixel buffer invalidated.
                        self.complete_replot();
                    }
                }
            }
            0x18 => {
                dbg_log!(Module::VGA, "3D5 / line compare write: {:#X}", value);
                self.line_compare = (self.line_compare & 0x300) | value as u32;
                self.update_layers();
            }
            _ => {
                if self.index_crtc < self.crtc.len() as u8 {
                    self.crtc[self.index_crtc as usize] = value as u8;
                }
                dbg_log!(
                    Module::VGA,
                    "3D5 / CRTC write {:#X}:  {:#X}",
                    self.index_crtc,
                    value
                );
            }
        }
    }

    fn port3D5_read(&self) -> u8 {
        dbg_log!(Module::VGA, "3D5 read {:#X}", self.index_crtc);

        match self.index_crtc {
            0x1 => self.horizontal_display_enable_end,
            0x2 => self.horizontal_blank_start,
            0x7 => {
                ((self.vertical_display_enable_end >> 7 & 0x2)
                    | (self.vertical_blank_start >> 5 & 0x8)
                    | (self.line_compare >> 4 & 0x10)
                    | (self.vertical_display_enable_end >> 3 & 0x40)) as u8
            }
            0x8 => self.preset_row_scan as u8,
            0x9 => self.max_scan_line,
            0xA => self.cursor_scanline_start,
            0xB => self.cursor_scanline_end,
            0xC => (self.start_address & 0xFF) as u8,
            0xD => (self.start_address >> 8) as u8,
            0xE => (self.cursor_address >> 8) as u8,
            0xF => (self.cursor_address & 0xFF) as u8,
            0x12 => (self.vertical_display_enable_end & 0xFF) as u8,
            0x13 => self.offset_register,
            0x14 => self.underline_location_register,
            0x15 => (self.vertical_blank_start & 0xFF) as u8,
            0x17 => self.crtc_mode,
            0x18 => (self.line_compare & 0xFF) as u8,
            _ => {
                if self.index_crtc < self.crtc.len() as u8 {
                    self.crtc[self.index_crtc as usize]
                } else {
                    0
                }
            }
        }
    }

    fn svga_bytes_per_line(&self) -> u32 {
        let bits = if self.svga_bpp == 15 {
            16
        } else {
            self.svga_bpp
        };
        return (self.svga_width * bits) as u32 / 8;
    }

    fn port3DA_read(&mut self) -> u8 {
        dbg_log!(Module::VGA, "3DA read - status 1 and clear attr index");

        let value = self.port_3DA_value;

        // Status register, bit 3 set by update_vertical_retrace
        // during screen-fill-buffer
        if !self.graphical_mode {
            // But screen-fill-buffer may not get triggered in text mode
            // so toggle it manually here
            if self.port_3DA_value & 1 > 0 {
                self.port_3DA_value ^= 8;
            }
            self.port_3DA_value ^= 1;
        } else {
            self.port_3DA_value ^= 1;
            self.port_3DA_value &= 1;
        }
        self.attribute_controller_index = 0xFFFF_FFFF;
        return value;
    }

    fn port1CE_write(&mut self, value: u16) {
        self.dispi_index = value;
    }

    fn port1CF_write(&mut self, value: u16) {
        dbg_log!(
            Module::VGA,
            "1CF / dispi write {:#X}: {:#X}",
            self.dispi_index,
            value
        );
        match self.dispi_index {
            1 => {
                self.svga_width = value;
                if self.svga_width > MAX_XRES {
                    dbg_log!(
                        Module::VGA,
                        "svga_width reduced from {} to {}",
                        self.svga_width,
                        MAX_XRES
                    );
                    self.svga_width = MAX_XRES;
                }
            }
            2 => {
                self.svga_height = value;
                if self.svga_height > MAX_YRES {
                    dbg_log!(
                        Module::VGA,
                        "svga_height reduced from {} to {}",
                        self.svga_height,
                        MAX_YRES
                    );
                    self.svga_height = MAX_YRES;
                }
            }
            3 => {
                self.svga_bpp = value;
            }
            4 => {
                // enable, options
                self.svga_enabled = (value & 1) == 1;
                self.dispi_enable_value = value;
            }
            5 => {
                self.svga_bank_offset = (value as u32) << 16;
            }
            9 => {
                // y offset
                self.svga_offset = value as u32 * self.svga_bytes_per_line();
                dbg_log!(
                    Module::VGA,
                    "SVGA offset: {:#X} y={:#X}",
                    self.svga_offset,
                    value
                );
                self.complete_redraw();
            }
            _ => {}
        }

        if self.svga_enabled && (self.svga_width == 0 || self.svga_height == 0) {
            dbg_log!(
                Module::VGA,
                "SVGA: disabled because of invalid width/height: {} x{}",
                self.svga_width,
                self.svga_height
            );
            self.svga_enabled = false;
        }

        assert!(self.svga_bpp != 4, "unimplemented svga bpp: 4");
        assert!(self.svga_bpp != 15, "unimplemented svga bpp: 15");
        assert!(
            self.svga_bpp == 4
                || self.svga_bpp == 8
                || self.svga_bpp == 15
                || self.svga_bpp == 16
                || self.svga_bpp == 24
                || self.svga_bpp == 32,
            "unexpected svga bpp: {}",
            self.svga_bpp
        );

        dbg_log!(
            Module::VGA,
            "SVGA: enabled={}, {}x{}x{}",
            self.svga_enabled,
            self.svga_width,
            self.svga_height,
            self.svga_bpp
        );

        if self.svga_enabled && self.dispi_index == 4 {
            self.set_size_graphical(
                self.svga_width as u32,
                self.svga_height as u32,
                self.svga_bpp,
                self.svga_width as u32,
                self.svga_height as u32,
            );
            self.store.bus_mut().map(|bus| {
                bus.send("screen-set-mode", BusData::Bool(true));
            });
            self.graphical_mode = true;
            self.graphical_mode_is_linear = true;
        }

        if !self.svga_enabled {
            self.svga_bank_offset = 0;
        }

        self.update_layers();
    }

    fn port1CF_read(&self) -> u16 {
        dbg_log!(Module::VGA, "1CF / dispi read {:#X}", self.dispi_index);
        return self.svga_register_read(self.dispi_index);
    }

    fn svga_register_read(&self, n: u16) -> u16 {
        match n {
            0 => {
                // id
                0xB0C0u16
            }
            1 => {
                if self.dispi_enable_value & 2 > 0 {
                    MAX_XRES
                } else {
                    self.svga_width
                }
            }
            2 => {
                if self.dispi_enable_value & 2 > 0 {
                    MAX_YRES
                } else {
                    self.svga_height
                }
            }
            3 => {
                if self.dispi_enable_value & 2 > 0 {
                    MAX_BPP as u16
                } else {
                    self.svga_bpp
                }
            }
            4 => self.dispi_enable_value as u16,

            5 => (self.svga_bank_offset >> 16) as u16,
            6 => {
                // virtual width
                if self.screen_width > 0 {
                    self.screen_width as u16
                } else {
                    1u16 // seabios/windows98 divide exception
                }
            }
            8 => {
                // x offset
                0
            }
            0x0A => (self.vga_memory_size / VGA_BANK_SIZE | 0) as u16,
            _ => 0xFF,
        }
    }
}

struct VGADev(StoreT);

impl PCIDevice for VGADev {
    #[inline]
    fn pci_id(&self) -> u16 {
        self.0.vga().map_or(0, |vga| vga.pci_id)
    }

    #[inline]
    fn name(&self) -> &str {
        self.0.vga().map_or("vga", |vga| vga.name)
    }

    #[inline]
    fn pci_rom_size(&self) -> u32 {
        self.0.vga().map_or(0, |vga| vga.pci_rom_size)
    }

    #[inline]
    fn pci_rom_address(&self) -> u32 {
        self.0.vga().map_or(0, |vga| vga.pci_rom_address)
    }

    #[inline]
    fn pci_space(&self) -> &[u8] {
        self.0.vga().map(|vga| vga.pci_space).unwrap()
    }

    #[inline]
    fn pci_bars(&self) -> &[Option<PCIBar>] {
        self.0.vga().map(|vga| &vga.pci_bars).unwrap()
    }

    #[inline]
    fn pci_bars_mut(&mut self) -> &mut [Option<PCIBar>] {
        self.0.vga_mut().map(|vga| &mut vga.pci_bars).unwrap()
    }
}
