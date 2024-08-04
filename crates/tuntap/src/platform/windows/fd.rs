use std::{
    io::{Read, Write}, 
    mem::MaybeUninit, 
    ops::Deref, ptr
};

use winapi::um::{minwinbase::OVERLAPPED, winnt::HANDLE};

use super::ffi;

pub struct Fd {
    handle: HANDLE,
    overlapped: Option<OVERLAPPED>,
}

impl Fd {
    pub fn new(handle: HANDLE) -> Self {
        Fd {
            handle,
            overlapped: None,
        }
    }

    pub fn overlapped(&mut self, overlapped: OVERLAPPED) {
        self.overlapped = Some(overlapped);
    }
}

impl Deref for Fd {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Read for Fd {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let overlapped = self.overlapped.as_mut()
            .map(ptr::from_mut)
            .unwrap_or(ptr::null_mut());
        ffi::read_file(self.handle, buf, overlapped)
    }
}

impl Write for Fd {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let overlapped = self.overlapped.as_mut()
            .map(ptr::from_mut)
            .unwrap_or(ptr::null_mut());
        ffi::write_file(self.handle, buf, overlapped)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Drop for Fd {
    fn drop(&mut self) {
        ffi::close_handle(self.handle).unwrap();
    }
}