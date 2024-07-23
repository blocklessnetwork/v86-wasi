use std::{
    fs::OpenOptions, 
    io::{self, Read, Write}, 
    process::Command
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NatError {
    #[error("execute command error.")]
    CommandError,
    #[error("io error. detail: {0}.")]
    IoError(io::Error),
    #[error("utf8 code error.")]
    Utf8CodeError,
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

#[cfg(target_os="macos")]
impl Nat {
    fn sysctl(enable: bool) -> Result<(), NatError> {
        let mut command = Command::new("sysctl");
        let enable = if enable { 1 } else { 0 };
        command.args(&["-w", &format!("net.inet.ip.forwarding={enable}")]);
        let mut child = command.spawn().map_err(|e| NatError::IoError(e))?;
        let exit_code = child.wait().map_err(|e| NatError::IoError(e))?;
        if exit_code.success() {
            Ok(())
        } else {
            Err(NatError::CommandError)
        }
    }

    fn pfctl() -> Result<(), NatError> {
        let mut command = Command::new("pfctl");
        command.args( &["-f", "/etc/pf.anchors/bls-vm-nat", "-e"]);
        let mut child = command.spawn().map_err(|e| NatError::IoError(e))?;
        let exit_code = child.wait().map_err(|e| NatError::IoError(e))?;
        if exit_code.success() {
            return Ok(());
        } else {
            if let Some(mut output) = child.stdout {
                let mut out_string = String::new();
                output.read_to_string(&mut out_string).unwrap();
                if let Some(_) = out_string.find("pfctl: pf already enabled") {
                    return Ok(());
                }
            }
        }
        Err(NatError::CommandError)
    }


    pub fn enable(&self) -> anyhow::Result<()> {
        let mut pfctl = OpenOptions::new()
            .write(true)
            .create(true)
            .open("/etc/pf.anchors/bls-vm-nat")?;
        let cmd = format!("nat on en0 from {}:network to any -> (en0)\n", &self.tap_name);
        pfctl.write_all(cmd.as_bytes())?;
        Self::sysctl(true)?;
        Self::pfctl()?;
        Ok(())
    }
}