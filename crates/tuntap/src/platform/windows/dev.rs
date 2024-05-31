use std::{time, io};

use winapi::shared::ifdef::NET_LUID;

use crate::Configuration;
use crate::{Result, Error};
use super::fd::Fd;
use super::ffi;


winapi::DEFINE_GUID! {
    GUID_NETWORK_ADAPTER,
    0x4d36e972, 0xe325, 0x11ce,
    0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18
}



pub struct Tap {
    luid: NET_LUID,
    fd: Fd,
}

impl Tap {
    pub fn new(_config: Configuration) -> Result<Self> {
        let luid = ffi::create_interface()?;

        // Even after retrieving the luid, we might need to wait
        let start = time::Instant::now();
        let handle = loop {
            // If we surpassed 2 seconds just return
            let now = time::Instant::now();
            if now - start > time::Duration::from_secs(2) {
                return Err(Error::Io(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "Interface timed out",
                )));
            }

            match ffi::open_interface(&luid) {
                Err(_) => {
                    std::thread::yield_now();
                    continue;
                }
                Ok(handle) => break handle,
            };
        };

        Ok(Self { luid, fd: Fd::new(handle) })
    }
}