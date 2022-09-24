use std::io::{Stdout, stdout};

use crossterm::{terminal, execute, cursor, style::{self, ContentStyle, StyledContent}, queue};

use crate::{ContextTrait, Dev, StoreT};

pub(crate) struct Screen {
    store: StoreT,
    stdout: Stdout,
    is_graphical: bool,
}

impl Screen {
    pub fn new(store: StoreT) -> Self {
        Self {
            store,
            stdout: stdout(),
            is_graphical: false,
        }
    }

    pub fn init(&mut self) {
        self.store.bus_mut().map(|bus| {
            bus.register(
                "screen-set-mode",
                |dev: &Dev, data: &crate::bus::BusData| {
                    dev.screen_mut().map(|screen| {
                        data.map_bool(|b| screen.set_mode(b));
                    });
                },
            );

            bus.register(
                "screen-put-char",
                |dev: &Dev, data: &crate::bus::BusData| {
                    dev.screen_mut().map(|screen| {
                        data.map_screen_put_char(|row, col, chr, bg_color, fg_color| {
                            screen.put_char(row, col, chr, bg_color, fg_color);
                        });
                    });
                },
            );

            bus.register(
                "screen-set-size-text",
                |dev: &Dev, data: &crate::bus::BusData| {
                    dev.screen_mut().map(|screen| {
                        data.map_u16tuple(|row, col| {
                            screen.set_size_text(row, col);
                        });
                    });
                },
            );

            bus.register(
                "screen-update-cursor-scanline",
                |dev: &Dev, data: &crate::bus::BusData| {
                    dev.screen_mut().map(|screen| {
                        data.map_u16tuple(|row, col| {
                            screen.update_cursor(row, col);
                        });
                    });
                },
            );
        });
    }

    #[inline]
    fn set_mode(&mut self, graphical: bool) {
        self.is_graphical = graphical;
    }

    #[inline]
    fn update_cursor(&mut self, start: u16, end: u16) {
        //self.is_graphical = graphical;
    }

    #[inline]
    fn set_size_text(&mut self, start: u16, end: u16) {
    }

    #[inline]
    fn put_char(&mut self, row: u16, col: u16, chr: u8, bg_color: i32, fg_color: i32) {
        let bgc = bg_color.to_le_bytes();
        let fgc = fg_color.to_le_bytes();
        let chr = chr as char;
        let fg_c = style::Color::Rgb { r: fgc[0], g: fgc[1], b: fgc[2] };
        let bg_c = style::Color::Rgb { r: bgc[0], g: bgc[1], b: bgc[2] };
        let mut ct = ContentStyle::new();
        ct.background_color = Some(bg_c);
        ct.foreground_color = Some(fg_c);
        let chr = StyledContent::new(ct, chr);
        let _ = queue!(self.stdout, cursor::Hide, cursor::MoveTo(col, row), style::PrintStyledContent(chr));
    }
}
