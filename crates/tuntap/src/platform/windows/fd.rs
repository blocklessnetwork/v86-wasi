use std::{
    io::{Read, Write},
    ops::Deref, ptr
};

use winapi::um::{minwinbase::OVERLAPPED, winnt::HANDLE};

use super::ffi;

pub struct Fd {
    handle: HANDLE,
    read_overlapped: *mut OVERLAPPED,
    write_overlapped: *mut OVERLAPPED,
}

impl Fd {
    pub fn new(
        handle: HANDLE,
        read_overlapped: *mut OVERLAPPED,
        write_overlapped: *mut OVERLAPPED,
    ) -> Self {
        Fd {
            handle,
            read_overlapped,
            write_overlapped,
        }
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
        ffi::read_file(self.handle, buf, self.read_overlapped)
    }
}

impl Write for Fd {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        ffi::write_file(self.handle, buf, self.write_overlapped)
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