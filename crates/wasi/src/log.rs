use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Once;
use std::time::Instant;
#[allow(unused_imports)]
use crate::consts::*;

static mut LOG_FNAME: Option<String> = None;
static mut LOG_MASK: u32 = 0;
static mut LOG_FILE: Option<File> = None;
static mut LAST_FLUSH: Option<Instant> = None;

#[inline(always)]
pub fn log(record: &[u8]) {
    static mut LOG_FILE_ONCE: Once = Once::new();
    unsafe {
        LOG_FILE_ONCE.call_once(|| {
            LOG_FILE = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(LOG_FNAME.as_ref().map_or("debug.log", |s| s))
                .ok();
            LAST_FLUSH.replace(Instant::now());
        });
        LOG_FILE.as_mut().map(|f| {
            let _ = f.write_all(record);
        });
    }
}

pub fn set_log_file_name(f: Option<String>) {
    unsafe {LOG_FNAME = f};
}

pub fn set_log_mask(m: u32) {
    unsafe {LOG_MASK = m};
}

pub enum LOG {
    E,
    IO,
    WS,
    CPU,
    PIC,
    PCI,
    DMA,
    VGA,
    PIT,
    RTC,
    PS2,
    NET,
    DISK,
    BIOS,
    VIRTIO,
    SERIAL,
    FLOPPY,
}

impl LOG {

    #[inline]
    pub fn from_str(s :&str) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "IO" => LOG::IO,
            "WS" => LOG::WS,
            "CPU" => LOG::CPU,
            "PIC" => LOG::PIC,
            "PCI" => LOG::PCI,
            "DMA" => LOG::DMA,
            "VGA" => LOG::VGA,
            "PIT" => LOG::PIT,
            "RTC" => LOG::RTC,
            "PS2" => LOG::PS2,
            "NET" => LOG::NET,
            "DISK" => LOG::DISK,
            "BIOS" => LOG::BIOS,
            "SERIAL" => LOG::SERIAL,
            "FLOPPY" => LOG::FLOPPY,
            _ => LOG::E,
        }
    }

    #[inline]
    pub(crate) fn bit_mask(&self) -> u32 {
        match *self {
            Self::E => LOG_E,
            Self::IO => LOG_IO,
            Self::CPU => LOG_CPU,
            Self::PIC => LOG_PIC,
            Self::PCI => LOG_PCI,
            Self::PS2 => LOG_PS2,
            Self::NET => LOG_NET,
            Self::VGA => LOG_VGA,
            Self::RTC => LOG_RTC,
            Self::DMA => LOG_DMA,
            Self::PIT => LOG_PIT,
            Self::DISK => LOG_DISK,
            Self::BIOS => LOG_BIOS,
            Self::WS => LOG_WS,
            Self::FLOPPY => LOG_FLOPPY,
            Self::SERIAL => LOG_SERIAL,
        }
    }

    #[inline]
    pub(crate) fn display(&self) -> bool {
        unsafe {self.bit_mask() & LOG_MASK > 0}
    }
}

impl Display for LOG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IO => f.write_str("IO"),
            Self::E => f.write_str("  "),
            Self::CPU => f.write_str("CPU"),
            Self::PIC => f.write_str("PIC"),
            Self::PIT => f.write_str("PIT"),
            Self::NET => f.write_str("NET"),
            Self::PCI => f.write_str("PCI"),
            Self::VGA => f.write_str("VGA"),
            Self::RTC => f.write_str("RTC"),
            Self::DMA => f.write_str("DMA"),
            Self::PS2 => f.write_str("PS2"),
            Self::WS => f.write_str("WS"),
            Self::DISK => f.write_str("DISK"),
            Self::BIOS => f.write_str("BIOS"),
            Self::SERIAL => f.write_str("SERIAL"),
            Self::FLOPPY => f.write_str("FLOPPY"),
        }
    }
}

macro_rules! dbg_log {
    ($m: expr, $fmt:expr, $($arg:tt)*) => {
        if $m.display() {
            let values = format!($fmt, $($arg)*);
            let now = chrono::offset::Local::now();
            let now = now.format("%H:%M:%S");
            let record = format!("{} [{:>5}] {}\n", now, $m, &values);
            crate::log::log(record.as_bytes());
        }

    };

    ($m: expr, $fmt:expr) => {
        if $m.display() {
            let values = format!($fmt);
            let now = chrono::offset::Local::now();
            let now = now.format("%H:%M:%S");
            let record = format!("{} [{:>5}] {}\n", now, $m, &values);
            crate::log::log(record.as_bytes());
        }
    };
}
