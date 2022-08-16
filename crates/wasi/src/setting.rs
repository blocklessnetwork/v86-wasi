pub struct Setting {
    bios_file: Option<String>
}

impl Setting {

    pub fn new() -> Self {
        Self { bios_file: None }    
    }

    pub fn bios_file(&mut self, f: String) {
        self.bios_file = Some(f)
    }

    pub fn load_bios_file(&self) -> Option<Vec<u8>> {
        self.bios_file.as_ref().map(|f| {
            std::fs::read(f).ok()
        }).flatten()
    }
}