use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Once;
use std::time::Instant;

static LOG_FNAME: &str = "debug.log";
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
                .open(LOG_FNAME)
                .ok();
            LAST_FLUSH.replace(Instant::now());
        });
        LOG_FILE.as_mut().map(|f| {
            let _ = f.write_all(record);
        });
    }
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
    SERIAL,
    FLOPPY,
}

impl LOG {
    pub(crate) fn display(&self) -> bool {
        match *self {
            Self::E => true,
            Self::IO => true,
            Self::CPU => true,
            Self::PIC => false,
            Self::PCI => true,
            Self::PS2 => false,
            Self::NET => true,
            Self::VGA => false,
            Self::RTC => true,
            Self::DMA => true,
            Self::PIT => false,
            Self::DISK => true,
            Self::BIOS => true,
            Self::WS => true,
            Self::FLOPPY => true,
            Self::SERIAL => true,
        }
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
            let now = chrono::offset::Utc::now();
            let now = now.format("%H:%M:%S");
            let record = format!("{} [{:>5}] {}\n", now, $m, &values);
            crate::log::log(record.as_bytes());
        }
    };
}
