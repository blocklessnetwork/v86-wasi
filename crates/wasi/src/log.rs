use std::fmt::Display;

pub enum Module {
    EMPTY,
    BIOS,
    DEBUG,
    SERIAL,
    FLOPPY,
    CPU,
    PIC,
    PCI,
    DMA,
    VGA,
    PIT,
    RTC,
    PS2,
    IO,
}

impl Module {
    pub(crate) fn display(&self) -> bool {
        match *self {
            Self::EMPTY => true,
            Self::FLOPPY => true,
            Self::DEBUG => true,
            Self::CPU => true,
            Self::PIC => false,
            Self::PCI => true,
            Self::IO => false,
            Self::VGA => false,
            Self::RTC => true,
            Self::DMA => true,
            Self::PIT => false,
            Self::BIOS => true,
            Self::SERIAL => true,
            Self::PS2 => false,
        }
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IO => f.write_str("IO"),
            Self::EMPTY => f.write_str(""),
            Self::CPU => f.write_str("CPU"),
            Self::PIC => f.write_str("PIC"),
            Self::PIT => f.write_str("PIT"),
            Self::PCI => f.write_str("PCI"),
            Self::VGA => f.write_str("VGA"),
            Self::RTC => f.write_str("RTC"),
            Self::DMA => f.write_str("DMA"),
            Self::PS2 => f.write_str("PS2"),
            Self::BIOS => f.write_str("BIOS"),
            Self::DEBUG => f.write_str("DEBUG"),
            Self::SERIAL => f.write_str("SERIAL"),
            Self::FLOPPY => f.write_str("FLOPPY"),
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
