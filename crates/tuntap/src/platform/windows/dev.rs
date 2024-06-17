use std::io::{Read, Write};
use std::{io, ptr, time};

use winapi::shared::ifdef::NET_LUID;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::um::winioctl::{
    CTL_CODE, FILE_ANY_ACCESS, FILE_DEVICE_UNKNOWN, METHOD_BUFFERED
};

use crate::{Configuration, Device};
use crate::{Result, Error};
use super::fd::Fd;
use super::{encode_utf16, netsh};
use super::{decode_utf16, ffi};


winapi::DEFINE_GUID! {
    GUID_NETWORK_ADAPTER,
    0x4d36e972, 0xe325, 0x11ce,
    0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18
}

pub struct Tap {
    fd: Fd,
    dev_index: u32,
    is_open: bool,
    read_overlapped: Option<OVERLAPPED>,
    write_overlapped: Option<OVERLAPPED>,
    name: String,
    luid: NET_LUID,
}

impl Tap {

    pub fn open(name: &str) -> Result<Self> {
        let name_string = name.to_string();
        let name = encode_utf16(name);

        let luid = ffi::alias_to_luid(&name)?;
        ffi::check_interface(&luid)?;

        let handle = ffi::open_interface(&luid)?;
        let dev_index = ffi::luid_to_index(&luid).unwrap() as _;
        Ok(Self { 
            luid, 
            dev_index,
            is_open: true,
            fd: Fd::new(handle),
            name: name_string,
            read_overlapped: None,
            write_overlapped: None,
        })
    }

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

        let name = ffi::luid_to_alias(&luid)
            .map(|name| decode_utf16(&name))?;
        let dev_index = ffi::luid_to_index(&luid).unwrap() as _;

        Ok(Self { 
            luid,
            name,
            dev_index,
            is_open: false,
            read_overlapped: None,
            write_overlapped: None,
            fd: Fd::new(handle),
        })
    }
}

impl Device for Tap {
    fn token(&self) -> crate::Token {
        todo!()
    }
    fn set_nonblock(&mut self) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) -> Result<()> {
        let ret = netsh::set_interface_name(&self.name, name);
        if ret.is_ok() {
            self.name = name.to_string();
        }
        ret
    }

    fn enabled(&mut self, value: bool) -> Result<()> {
        let status: u32 = if value { 1 } else { 0 };

        ffi::device_io_control(
            *self.fd,
            CTL_CODE(FILE_DEVICE_UNKNOWN, 6, METHOD_BUFFERED, FILE_ANY_ACCESS),
            &status,
            &mut (),
        )
    }

    fn address(&self) -> Result<std::net::Ipv4Addr> {
        let dev_index = self.dev_index;
        let mut ip = Vec::new();
        ffi::visit_adapters_info(|dev_info| {
            if dev_index == dev_info.Index {
                ip = dev_info
                        .IpAddressList
                        .IpAddress
                        .String
                        .iter()
                        .filter_map(|i| if *i == 0 {None} else {Some(*i as _)})
                        .collect::<Vec<u8>>();
                
            }
        }).unwrap();
        let ip: String = String::from_utf8(ip).unwrap();
        ip.parse().map_err(|_| Error::InvalidAddress)
    }

    fn set_address(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        netsh::set_ip(&self.name, &value.to_string())
    }

    fn broadcast(&self) -> Result<std::net::Ipv4Addr> {
        todo!()
    }

    fn set_broadcast(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        todo!()
    }

    fn netmask(&self) -> Result<std::net::Ipv4Addr> {
        todo!()
    }

    fn set_netmask(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        netsh::set_mask(&self.name, &value.to_string())
    }

    fn mtu(&self) -> Result<i32> {
        let mut mtu = 0;
        ffi::device_io_control(
            *self.fd,
            CTL_CODE(FILE_DEVICE_UNKNOWN, 3, METHOD_BUFFERED, FILE_ANY_ACCESS),
            &(),
            &mut mtu,
        )
        .map(|_| mtu)
    }

    fn set_mtu(&mut self, value: i32) -> Result<()> {
        Ok(())
    }

    fn set_ether_address(&mut self, eth: crate::EtherAddr) -> Result<()> {
        Ok(())
    }

    fn fd(&self) -> &Fd {
        &self.fd
    }
}

impl Read for Tap {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.fd.read(buf)
    }
}

impl Write for Tap {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.fd.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.fd.flush()
    }
}

impl Drop for Tap {
    fn drop(&mut self) {
        if !self.is_open {
            ffi::delete_interface(&self.luid).unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{platform::windows::netsh, Configuration, Device};

    use super::Tap;

    fn test_create() {
        let mut config = Configuration::new();
        let ip = "192.168.0.1";
        config.address(ip);
        let tap = Tap::new(config).unwrap();
        println!("{}", tap.name());
        netsh::set_ip(tap.name(), ip).unwrap();
        assert_eq!(tap.address().unwrap().to_string(), ip);
    }
}