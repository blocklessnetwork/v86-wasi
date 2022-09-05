use std::rc::Weak;

use wasmtime::Store;

use crate::{pci::PICBar, Emulator};

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
    bpp: u8,
}

pub(crate) struct VGAScreen {
    store: Weak<Store<Emulator>>,
    vga_memory_size: u32,
    cursor_address: u32,
    cursor_scanline_start: u8,
    cursor_scanline_end: u8,
    max_cols: u32,
    max_rows: u32,
    screen_width: u32,
    screen_height: u32,
    virtual_width: u32,
    virtual_height: u32,
    layers: Vec<Layer>,
    start_address: u32,
    start_address_latched: u32,
    crtc: Vec<u8>,
    crtc_mode: u8,
    horizontal_display_enable_end: u32,
    horizontal_blank_start: u32,
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
    svga_width: u8,
    svga_height: u8,
    svga_enabled: bool,
    svga_bpp: u8,
    svga_bank_offset: u8,
    svga_offset: u8,
    pci_space: Vec<u8>,
    pci_id: u8,
    pci_bars: Vec<PICBar>,
    pci_rom_size: u32,
    pci_rom_address: u32,
    name: &'static str,
    stats: VGAStats,
    index_crtc: u8,
    dac_color_index_write: u8,
    dac_color_index_read: u8,
    dac_state: u8,
    dac_map: Vec<u8>,
    attribute_controller_index: u32,
    palette_source: u8,
    attribute_mode: u8,
    color_plane_enable: u8,
    horizontal_panning: u8,
    color_select: u8,
    sequencer_index: i32,
    plane_write_bm: u8,
    sequencer_memory_mode: u8,
    clocking_mode: u8,
    graphics_index: i32,
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
    vga_memory: Vec<u8>,
    plane0: Vec<u8>,
    plane1: Vec<u8>,
    plane2: Vec<u8>,
    plane3: Vec<u8>,
    pixel_buffer: Vec<u8>,
}

impl VGAScreen {
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
            dbg_log!("vga read outside memory space: addr:{:#X}", addr);
            return 0;
        }
        let i_addr: usize = addr as usize;
        self.latch_dword = self.plane0[i_addr] as u32;
        self.latch_dword |= (self.plane1[i_addr] << 8) as u32;
        self.latch_dword |= (self.plane2[i_addr] << 16) as u32;
        self.latch_dword |= (self.plane3[i_addr] << 24) as u32;

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
            return self.vga_memory[(plane << 16) as usize | addr as usize];
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
                "vga write outside memory space: addr:{:#X}, value:{:#X}",
                addr,
                value
            );
            return;
        }

        if self.graphical_mode {
            //TODO self.vga_memory_write_graphical(addr, value);
        } else {
            if !(self.plane_write_bm & 0x3 > 0) {
                // Ignore writes to font planes.
                return;
            }
            //TODO self.vga_memory_write_text_mode(addr, value);
        }
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
                //TODO this.bus.send("screen-put-char", [row, col, chr,
                //self.vga256_palette[color >> 4 & 0xF], this.vga256_palette[color & 0xF]]);
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
            //TODO this.bus.send("screen-clear");
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

        let mut split_screen_row = self.scan_line_to_screen_row(1 + self.line_compare);
        split_screen_row = split_screen_row.min(self.screen_height);

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
                buffer_height: split_screen_row,
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
                buffer_height: split_buffer_height,
            });
            x += self.virtual_width as i32;
            y += 1;
        }
    }

    fn complete_redraw(&mut self) {
        dbg_log!("complete redraw");
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
            _ =>{}
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
            pixel_addr &= (self.crtc_mode << 13) as u32 | !0x6000;

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
        dbg_log!("complete replot");

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
        let mut vertical_scans = (1 + self.vertical_display_enable_end).min(self.vertical_blank_start);

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

            let height = vertical_scans / (1 + ((self.max_scan_line as u32) & 0x1F)) | 0;

            if horizontal_characters > 0 && height > 0 {
                self.set_size_text(horizontal_characters, height);
            }
        }
    }

    fn set_size_text(&mut self, cols_count: u32, rows_count: u32) {
        self.max_cols = cols_count;
        self.max_rows = rows_count;
        //RODOthis.bus.send("screen-set-size-text", [cols_count, rows_count]);
    }

    fn set_size_graphical(
        &mut self,
        width: u32,
        height: u32,
        bpp: u8,
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

            //TODO self.bus.send("screen-set-size-graphical", [width, height, virtual_width, virtual_height, bpp]);
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
            dbg_log!("attribute controller index register: {:#X}", value);
            self.attribute_controller_index = (value & 0x1F) as u32;
            dbg_log!(
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
                        dbg_log!("3C0 / attribute mode control: {:#X}", value);
                        if self.attribute_mode != value {
                            let previous_mode = self.attribute_mode;
                            self.attribute_mode = value;
                            let is_graphical = (value & 0x1) > 0;
                            if !self.svga_enabled && self.graphical_mode != is_graphical {
                                self.graphical_mode = is_graphical;
                                //TODO this.bus.send("screen-set-mode", this.graphical_mode);
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
                        dbg_log!("3C0 / color plane enable: {:#X}", value);
                        if self.color_plane_enable != value {
                            self.color_plane_enable = value;

                            // Data stored in image buffer are invalidated
                            self.complete_redraw();
                        }
                    }
                    0x13 => {
                        dbg_log!("3C0 / horizontal panning: {:#X}", value);
                        if self.horizontal_panning != value {
                            self.horizontal_panning = value & 0xF;
                            self.update_layers();
                        }
                    }
                    0x14 => {
                        dbg_log!("3C0 / color select: {:#X}", value);
                        if self.color_select != value {
                            self.color_select = value;

                            // Data stored in image buffer are invalidated
                            self.complete_redraw();
                        }
                    }
                    _ => {
                        dbg_log!(
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

    fn update_cursor_scanline (&mut self) {
        //TODOthis.bus.send("screen-update-cursor-scanline", [this.cursor_scanline_start, this.cursor_scanline_end]);
    }

    fn port3CF_write(&mut self, value: u8) {
        match self.graphics_index {
            0 => {
                self.planar_setreset = value;
                dbg_log!("plane set/reset: {:#X}", value);
            }
            1 => {
                self.planar_setreset_enable = value;
                dbg_log!("plane set/reset enable: {:#X}", value);
            }
            2 => {
                self.color_compare = value;
                dbg_log!("color compare: {:#X}", value);
            }
            3 => {
                self.planar_rotate_reg = value;
                dbg_log!("plane rotate: {:#X}", value);
            }
            4 => {
                self.plane_read = value;
                dbg_log!("plane read: {:#X}", value);
            }
            5 => {
                let previous_planar_mode = self.planar_mode;
                self.planar_mode = value;
                dbg_log!("planar mode: {:#X}", value);
                if (previous_planar_mode ^ value) & 0x60 > 0 {
                    // Shift mode modified. Pixel buffer invalidated
                    self.complete_replot();
                }
            }
            6 => {
                dbg_log!("miscellaneous graphics register: {:#X}", value);
                if self.miscellaneous_graphics_register != value {
                    self.miscellaneous_graphics_register = value;
                    self.update_vga_size();
                }
            }
            7 => {
                self.color_dont_care = value;
                dbg_log!("color don't care: {:#X}", value);
            }
            8 => {
                self.planar_bitmap = value;
                dbg_log!("planar bitmap: {:#X}", value);
            }
            _ => {
                dbg_log!(
                    "3CF / graphics write {:#X}: {:#X}", 
                    self.graphics_index, 
                    value
                );
            }
        }
    }

    fn port3CF_read(&self) -> u8 {
        dbg_log!("3CF / graphics read {:X}", self.graphics_index);
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
        dbg_log!("3D4 / crtc index: {:#X}", register);
        self.index_crtc = register;
    }

    fn port3D4_read(&self) -> u8 {
        dbg_log!("3D4 read / crtc index: {:#X}", self.index_crtc);
        self.index_crtc
    }

    fn update_cursor(&mut self) {
        let mut row = (self.cursor_address - self.start_address) / self.max_cols | 0;
        let col = (self.cursor_address - self.start_address) % self.max_cols;
    
        row = (self.max_rows - 1).min(row);
        //TODO this.bus.send("screen-update-cursor", [row, col]);
    }

    fn port3D5_write(&mut self, value: u8) {
        match self.index_crtc {
            0x1 => {
                dbg_log!("3D5 / hdisp enable end write: {:#X}", value);
                if self.horizontal_display_enable_end != value as u32 {
                    self.horizontal_display_enable_end = value as u32;
                    self.update_vga_size();
                }
            }
            0x2 => {
                if self.horizontal_blank_start != value as u32 {
                    self.horizontal_blank_start = value as u32;
                    self.update_vga_size();
                }
            }
            0x7 => {
                dbg_log!("3D5 / overflow register write: {:#X}", value);
                let previous_vertical_display_enable_end = self.vertical_display_enable_end;
                self.vertical_display_enable_end &= 0xFF;
                let value: u32 = value as u32;
                self.vertical_display_enable_end |= (value << 3 & 0x200) | (value << 7 & 0x100);
                if previous_vertical_display_enable_end != self.vertical_display_enable_end {
                    self.update_vga_size();
                }
                self.line_compare = (self.line_compare & 0x2FF) | (value << 4 & 0x100);

                let previous_vertical_blank_start = self.vertical_blank_start;
                self.vertical_blank_start = (self.vertical_blank_start & 0x2FF) | (value << 5 & 0x100);
                if previous_vertical_blank_start != self.vertical_blank_start {
                    self.update_vga_size();
                }
                self.update_layers();
            }
            0x8 => {
                dbg_log!("3D5 / preset row scan write: {:#X}", value);
                self.preset_row_scan = value;
                self.update_layers();
            }
            0x9 => {
                dbg_log!("3D5 / max scan line write: {:#X}", value);
                self.max_scan_line = value;
                let value: u32 = value as u32;
                self.line_compare = (self.line_compare & 0x1FF) | (value << 3 & 0x200);

                let previous_vertical_blank_start = self.vertical_blank_start;
                self.vertical_blank_start = (self.vertical_blank_start & 0x1FF) | (value << 4 & 0x200);
                if previous_vertical_blank_start != self.vertical_blank_start {
                    self.update_vga_size();
                }

                self.update_layers();
            }
            0xA => {
                dbg_log!("3D5 / cursor scanline start write: {:#X}", value);
                self.cursor_scanline_start = value;
                self.update_cursor_scanline();
            }
            0xB => {
                dbg_log!("3D5 / cursor scanline end write: #{:#X}", value);
                self.cursor_scanline_end = value;
                self.update_cursor_scanline();
            }
            0xC => {
                if (self.start_address >> 8 & 0xFF) as u8 != value  {
                    let value: u32 = value as u32;
                    self.start_address = self.start_address & 0xff | value << 8;
                    self.update_layers();
                    if !self.crtc_mode &  0x3 > 0 {
                        // Address substitution implementation depends on the
                        // starting row and column, so the pixel buffer is invalidated.
                        self.complete_replot();
                    }
                }
                dbg_log!("3D5 / start addr hi write: {:#X} -> {:#04X}", value, self.start_address);
            }
            0xD => {
                if (self.start_address & 0xFF) as u8 != value {
                    self.start_address = self.start_address & 0xff00 | value as u32;
                    self.update_layers();
                    if !self.crtc_mode &  0x3 >0 {
                        // Address substitution implementation depends on the
                        // starting row and column, so the pixel buffer is invalidated.
                        self.complete_replot();
                    }
                }
                dbg_log!("3D5 / start addr lo write: {:#X} -> {:#04X}", value, self.start_address);
            }
            0xE => {
                dbg_log!("3D5 / cursor address hi write: {:#X}", value);
                self.cursor_address = self.cursor_address & 0xFF | (value as u32) << 8;
                self.update_cursor();
            }
            0xF => {
                dbg_log!("3D5 / cursor address lo write: {:#X}", value);
                self.cursor_address = self.cursor_address & 0xFF00 | (value as u32);
                self.update_cursor();
            }
            0x12 => {
                dbg_log!("3D5 / vdisp enable end write: {:#X}", value);
                if (self.vertical_display_enable_end & 0xFF) as u8 != value {
                    self.vertical_display_enable_end = (self.vertical_display_enable_end & 0x300) | (value as u32);
                    self.update_vga_size();
                }
            }
            0x13 => {
                dbg_log!("3D5 / offset register write: {:#X}", value);
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
                dbg_log!("3D5 / underline location write: {:#X}", value);
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
                dbg_log!("3D5 / vertical blank start write: {:#X}", value);
                if(self.vertical_blank_start & 0xFF) as u8 != value {
                    self.vertical_blank_start = (self.vertical_blank_start & 0x300) | value as u32;
                    self.update_vga_size();
                }
            }
            0x17 => {
                dbg_log!("3D5 / crtc mode write: {:#X}", value);
                if self.crtc_mode != value {
                    let previous_mode = self.crtc_mode;

                    self.crtc_mode = value;
                    self.update_vga_size();

                    if(previous_mode ^ value) & 0x43 > 0 {
                        // Word/byte addressing changed or address substitution changed.
                        // Pixel buffer invalidated.
                        self.complete_replot();
                    }
                }
            }
            0x18 => {
                dbg_log!("3D5 / line compare write: {:#X}", value);
                self.line_compare = (self.line_compare & 0x300) | value as u32;
                self.update_layers();
            }
            _ =>  {
                if self.index_crtc < self.crtc.len() as u8 {
                    self.crtc[self.index_crtc as usize] = value as u8;
                }
                dbg_log!("3D5 / CRTC write {:#X}:  {:#X}", self.index_crtc, value);
            }
        }
    }

}
