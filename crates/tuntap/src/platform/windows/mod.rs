#![cfg(windows)]
mod fd;
mod dev;
mod ffi;
mod event;

pub const HARDWARE_ID: &str = "tap0901";

/// Encode a string as a utf16 buffer
pub fn encode_utf16(string: &str) -> Vec<u16> {
    use std::iter::once;
    string.encode_utf16().chain(once(0)).collect()
}

/// Decode a string from a utf16 buffer
fn decode_utf16(string: &[u16]) -> String {
    let end = string.iter().position(|b| *b == 0).unwrap_or(string.len());
    String::from_utf16_lossy(&string[..end])
}