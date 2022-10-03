pub struct Setting {
    pub(crate) cmdline: Option<String>,
    pub(crate) hda_file: Option<String>,
    pub(crate) bios_file: Option<String>,
    pub(crate) cdrom_file: Option<String>,
    pub(crate) initrd_file: Option<String>,
    pub(crate) bzimage_file: Option<String>,
    pub(crate) vga_bios_file: Option<String>,
    pub(crate) vga_memory_size: u32,
    pub(crate) memory_size: u32,
    pub(crate) fast_boot: bool,
}

impl Setting {
    pub fn new() -> Self {
        Self {
            cmdline: None,
            hda_file: None,
            bios_file: None,
            fast_boot: false,
            cdrom_file: None,
            initrd_file: None,
            bzimage_file: None,
            vga_bios_file: None,
            vga_memory_size: 8 * 1024 * 1024,
            memory_size: 128 * 1024 * 1024,
        }
    }

    #[inline]
    fn load_file(&self, file: Option<&String>) -> Option<Vec<u8>> {
        file.map(|f| std::fs::read(f).ok()).flatten()
    }

    #[inline]
    pub fn bios_file(&mut self, f: String) {
        self.bios_file = Some(f)
    }

    #[inline]
    pub fn bzimage_file(&mut self, f: String) {
        self.bzimage_file = Some(f)
    }

    #[inline]
    pub fn initrd_file(&mut self, f: String) {
        self.initrd_file = Some(f)
    }

    #[inline]
    pub fn cmdline(&mut self, f: String) {
        self.cmdline = Some(f)
    }

    #[inline]
    pub fn hda_file(&mut self, f: String) {
        self.hda_file = Some(f)
    }

    #[inline]
    pub fn cdrom_file(&mut self, f: String) {
        self.cdrom_file = Some(f)
    }

    #[inline]
    pub fn vga_bios_file(&mut self, f: String) {
        self.vga_bios_file = Some(f)
    }

    #[inline]
    pub fn load_hda_file(&self) -> Option<Vec<u8>> {
        self.load_file(self.hda_file.as_ref())
    }

    #[inline]
    pub fn load_cdrom_file(&self) -> Option<Vec<u8>> {
        self.load_file(self.cdrom_file.as_ref())
    }

    #[inline]
    pub fn load_vga_bios_file(&self) -> Option<Vec<u8>> {
        self.load_file(self.vga_bios_file.as_ref())
    }

    #[inline]
    pub fn load_bios_file(&self) -> Option<Vec<u8>> {
        self.load_file(self.bios_file.as_ref())
    }

    #[inline]
    pub fn load_bzimage_file(&self) -> Option<Vec<u8>> {
        self.load_file(self.bzimage_file.as_ref())
    }

    #[inline]
    pub fn load_initrd_file(&self) -> Option<Vec<u8>> {
        self.load_file(self.initrd_file.as_ref())
    }
}
