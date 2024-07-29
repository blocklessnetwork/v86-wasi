#[macro_use]
mod macros;

use thiserror::Error;
#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="macos")]
mod macos;

#[derive(Error, Debug)]
pub enum NatError {
    #[error("execute command error.")]
    CommandError,
    #[error("io error. detail: {0}.")]
    IoError(std::io::Error),
    #[error("utf8 code error.")]
    Utf8CodeError,
    #[error("no interface found.")]
    NoInterfaceFound,
    #[error("no stat info.")]
    NoStatError,
}

pub(crate)struct Nat {
    tap_name: String,
}

impl Nat {
    pub fn new(name: &str) -> Self {
        Self {
            tap_name: name.to_string()
        }
    }
}

#[cfg(target_os="windows")]
impl Nat {
    pub fn enable(&self) ->  Result<(), NatError> {
        Ok(())
    }
}

#[cfg(target_os="linux")]
impl Nat {
    pub fn enable(&self) ->  Result<(), NatError> {
        linux::forward(true)?;
        linux::iptable()?;
        Ok(())
    }
}

#[cfg(target_os="macos")]
impl Nat {
    pub fn enable(&self) -> anyhow::Result<()> {
        macos::write_anchors(&self.tap_name)?;
        macos::sysctl(true)?;
        macos::pfctl()?;
        Ok(())
    }
}