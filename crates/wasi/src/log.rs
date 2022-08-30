pub(crate) enum Module {
    EMPTY,
    CPU,
    PIC,
    PCI,
    IO,
}

macro_rules! dbg_log {
    ($fmt:expr, $($arg:tt)*) => {
        println!($fmt, $($arg)*);
    };

    ($fmt:expr) => {
        println!($fmt);
    };
}
