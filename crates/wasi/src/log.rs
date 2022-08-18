macro_rules! debug {
    ($fmt:expr, $($arg:tt)*) => {
        println!($fmt, $($arg)*);
    };

    ($fmt:expr) => {
        println!($fmt);
    };
}
