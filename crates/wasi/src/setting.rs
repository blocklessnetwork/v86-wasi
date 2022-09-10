pub struct Setting {
    pub(crate) bios_file: Option<String>,
    pub(crate) vga_bios_file: Option<String>,
    pub(crate) memory_size: u32,
    pub(crate) vga_memory_size: u32,
    pub(crate) fast_boot: bool,
}

impl Setting {
    pub fn new() -> Self {
        Self {
            bios_file: None,
            vga_bios_file: None,
            vga_memory_size: 8 * 1024 * 1024,
            memory_size: 128 * 1024 * 1024,
            fast_boot: false,
        }
    }

    pub fn bios_file(&mut self, f: String) {
        self.bios_file = Some(f)
    }

    pub fn vga_bios_file(&mut self, f: String) {
        self.vga_bios_file = Some(f)
    }

    pub fn load_vga_bios_file(&self) -> Option<Vec<u8>> {
        self.vga_bios_file
            .as_ref()
            .map(|f| std::fs::read(f).ok())
            .flatten()
    }

    pub fn load_bios_file(&self) -> Option<Vec<u8>> {
        self.bios_file
            .as_ref()
            .map(|f| std::fs::read(f).ok())
            .flatten()
    }
}
