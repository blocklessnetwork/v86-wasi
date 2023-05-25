use std::env;

use v86_wasi::{Setting, run_with_setting};

fn load_setting() -> Setting {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please execute with the configure file");
    }
    Setting::load_from_file(args[1].clone())
}

fn main() {
    let setting = load_setting();
    run_with_setting(setting)
}
