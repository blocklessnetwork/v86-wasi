use std::fmt::{Display, Write};

pub enum Module {
    EMPTY,
    BIOS,
    DEBUG,
    SERIAL,
    CPU,
    PIC,
    PCI,
    DMA,
    VGA,
    RTC,
    PS2,
    IO,
}

impl Module {
    pub(crate) fn display(&self) -> bool {
        match *self {
            Self::EMPTY => true,
            Self::CPU => true,
            Self::PIC => true,
            Self::PCI => true,
            Self::IO => true,
            Self::VGA => false,
            Self::RTC => true,
            Self::DMA => true,
            Self::DEBUG => true,
            Self::BIOS => true,
            Self::SERIAL => true,
            Self::PS2 => true,
        }
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::EMPTY => f.write_str(""),
            Self::CPU => f.write_str("CPU"),
            Self::PIC => f.write_str("PIC"),
            Self::PCI => f.write_str("PCI"),
            Self::IO => f.write_str("IO"),
            Self::VGA => f.write_str("VGA"),
            Self::RTC => f.write_str("RTC"),
            Self::DMA => f.write_str("DMA"),
            Self::DEBUG => f.write_str("DEBUG"),
            Self::BIOS => f.write_str("BIOS"),
            Self::SERIAL => f.write_str("SERIAL"),
            Self::PS2 => f.write_str("PS2"),
        }
    }
}

macro_rules! dbg_log {
    ($m: expr, $fmt:expr, $($arg:tt)*) => {
        if $m.display() {
            let values = format!($fmt, $($arg)*);
            println!("[{:5}] {}", $m, &values);
        }
    };

    ($m: expr, $fmt:expr) => {
        if $m.display() {
            let values = format!($fmt);
            println!("[{:5}] {}", $m, &values);
        }
    };
}
