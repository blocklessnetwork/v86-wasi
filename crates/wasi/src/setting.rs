use std::path::Path;

use json::JsonValue;

#[allow(unused_imports)]
use crate::consts::*;
use crate::LOG;

pub struct Setting {
    pub(crate) cmdline: Option<String>,
    pub(crate) hda_file: Option<String>,
    pub(crate) wasm_file: Option<String>,
    pub(crate) bios_file: Option<String>,
    pub(crate) cdrom_file: Option<String>,
    pub(crate) initrd_file: Option<String>,
    pub(crate) logger_file: Option<String>,
    pub(crate) bzimage_file: Option<String>,
    pub(crate) vga_bios_file: Option<String>,
    pub(crate) tun_addr: Option<String>,
    pub(crate) tun_netmask: Option<String>,
    pub(crate) tun_ether_addr: Option<String>,
    pub(crate) vga_memory_size: u32,
    pub(crate) ws_port: u32,
    pub(crate) memory_size: u32,
    pub(crate) log_mask: u32,
    pub(crate) fast_boot: bool,
}

impl Setting {
    pub fn new() -> Self {
        Self {
            log_mask: 0,
            cmdline: None,
            hda_file: None,
            tun_addr: None,
            bios_file: None,
            wasm_file: None,
            fast_boot: false,
            cdrom_file: None,
            tun_netmask: None,
            logger_file: None,
            initrd_file: None,
            bzimage_file: None,
            vga_bios_file: None,
            tun_ether_addr: None,
            ws_port: 8081,
            vga_memory_size: 8 * 1024 * 1024,
            memory_size: 128 * 1024 * 1024,
        }
    }

    pub fn load_from_str(json_str: &str) -> Self {
        let setting_obj = json::parse(json_str).unwrap();
        let mut setting = Self::new();
        setting.cdrom_file = setting_obj["cdrom"].as_str().map(|s| s.into());
        setting.bzimage_file = setting_obj["bzimage_file"].as_str().map(|s| s.into());
        setting.bios_file = setting_obj["bios_file"].as_str().map(|s| s.into());
        setting.vga_bios_file = setting_obj["vga_bios_file"].as_str().map(|s| s.into());
        setting.wasm_file = setting_obj["wasm_file"].as_str().map(|s| s.into());
        setting.wasm_file = setting_obj["wasm_file"].as_str().map(|s| s.into());
        setting.ws_port = setting_obj["ws_port"].as_u32().unwrap_or(8081);
        setting.vga_memory_size = setting_obj["vga_memory_size"].as_u32().unwrap_or( 8 * 1024 * 1024);
        if setting_obj["cmdline"].is_array() {
            setting.cmdline = match setting_obj["cmdline"] {
                json::JsonValue::Array(ref arr) => {
                    let cmdline: Vec<String> = arr.iter()
                        .map(|s| s.as_str().unwrap().into()).collect();
                    Some(cmdline.join("\n"))
                }
                _ => None,
            };
        }
        Self::load_logger(&mut setting, &setting_obj);
        Self::load_tunnel(&mut setting, &setting_obj);
        setting
    }

    pub fn load_from_file(f: impl AsRef<Path>) -> Self {
        let data = std::fs::read(f).unwrap();
        let json_str = std::str::from_utf8(&data).unwrap();
        Self::load_from_str(json_str)
    }

    fn load_logger(setting: &mut Setting, setting_obj: &JsonValue) {
        if !setting_obj["logger"].is_null() {
            let log_f = setting_obj["logger"]["log_file"].as_str().map(|s| s.into());
            setting.logger_file = log_f;

            if let json::JsonValue::Array(ref arr) = setting_obj["logger"]["log_module"] {
                arr.iter().for_each(|s| {
                    s.as_str().map(|s| {
                        setting.log_mask |= LOG::from_str(s).bit_mask();
                    });
                });
            }
        }
    }

    fn load_tunnel(setting: &mut Setting, setting_obj: &JsonValue) {
        if !setting_obj["tun"].is_null() {
            let tun_addr = setting_obj["tun"]["address"].as_str().map(|s| s.into());
            setting.tun_addr = tun_addr;
            let netmask = setting_obj["tun"]["netmask"].as_str().map(|s| s.into());
            setting.tun_netmask = netmask;
            let ether_address = setting_obj["tun"]["ether_address"].as_str().map(|s| s.into());
            setting.tun_ether_addr = ether_address;
        }
    }


    #[inline]
    pub fn logger_file(&self) -> Option<&String> {
        self.logger_file.as_ref()
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
    pub fn wasm_file(&self) -> Option<&String> {
        self.wasm_file.as_ref()
    }

    #[inline]
    pub fn tun_addr(&self) -> Option<&String> {
        self.tun_addr.as_ref()
    }

    #[inline]
    pub fn tun_netmask(&self) -> Option<&String> {
        self.tun_netmask.as_ref()
    }

    #[inline]
    pub fn tun_ether_addr(&self) -> Option<&String> {
        self.tun_ether_addr.as_ref()
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

    #[inline]
    pub fn log_mask(&self) -> u32 {
        self.log_mask
    }
}
