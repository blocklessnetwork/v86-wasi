use std::ffi;

use v86_wasi::{Setting, self};

#[no_mangle]
pub extern "C" fn run_v86_wasi(conf :*const ffi::c_char, len: ffi::c_int) -> ffi::c_int {
    let conf: &str = unsafe { 
        let data: &[u8] = std::slice::from_raw_parts(conf as _, len as _);
        std::str::from_utf8_unchecked(data)
    };
    let setting = Setting::load_from_str(conf);
    v86_wasi::run_with_setting(setting);
    0
}