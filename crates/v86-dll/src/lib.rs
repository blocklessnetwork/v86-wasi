use std::ffi;

#[no_mangle]
pub extern "C" fn start_v8(conf :*const ffi::c_char, conf_len: ffi::c_int) -> ffi::c_int {
    let data: &[i8] = unsafe {std::slice::from_raw_parts(conf as _, conf_len as _)};
    
    0
}