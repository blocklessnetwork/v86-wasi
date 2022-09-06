use std::fmt::{Display, Write};

pub enum Module {
    EMPTY,
    DEBUG,
    CPU,
    PIC,
    PCI,
    DMA,
    VGA,
    RTC,
    IO,
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
        }
    }
}

macro_rules! dbg_log {
    ($m: expr, $fmt:expr, $($arg:tt)*) => {
        let values = format!($fmt, $($arg)*);
        println!("[{:<2}] {}", $m, &values);
    };

    ($m: expr, $fmt:expr) => {
        let values = format!($fmt);
        println!("[{:<2}] {}", $m, &values);
    };
}
