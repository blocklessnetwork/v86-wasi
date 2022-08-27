
macro_rules! dbg_log {
    ($fmt:expr, $($arg:tt)*) => {
        println!($fmt, $($arg)*);
    };

    ($fmt:expr) => {
        println!($fmt);
    };
}
