use std::{io::{Read, Write}, ops::Deref};

use winapi::um::winnt::HANDLE;

use super::ffi;

pub struct Fd(HANDLE);

impl Fd {
    pub fn new(handle: HANDLE) -> Self {
        Fd(handle)
    }
}

impl Deref for Fd {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Read for Fd {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        ffi::read_file(self.0, buf)
    }
}

impl Write for Fd {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        ffi::write_file(self.0, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}