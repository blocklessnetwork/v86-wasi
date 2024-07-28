use std::{
    fs::{self, OpenOptions}, io::{self, Write}, process::{Command, Stdio}
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
    #[error("no interface found")]
    NoInterfaceFound,
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

macro_rules! io_wrap {
    ($io_op:expr) => {
        $io_op.map_err(|e| NatError::IoError(e))?
    };
}

#[cfg(target_os="linux")]
impl Nat {
    fn forward(enable: bool) -> Result<(), NatError> {
        let mut ip_fwf = io_wrap!(fs::OpenOptions::new()
            .write(true)
            .open("/proc/sys/net/ipv4/ip_forward"));
        let enable = if enable { b"1" } else { b"0" };
        io_wrap!(ip_fwf.write(enable));
        Ok(())
    }

    fn find_active_if() -> Result<String, NatError> {
        let net_path = std::path::Path::new("/sys/class/net");
        let read_dir = io_wrap!(fs::read_dir(&net_path));
        for entry in read_dir {
            let entry = io_wrap!(entry);
            let path = entry.path();
            if path.is_symlink() {
                let path = io_wrap!(fs::read_link(&path));
                let path_str = path.to_str();
                if let Some(s) = path_str {
                    if s.contains("virtual") {
                        continue;
                    }
                    let split = s.split("/");
                    if let Some(s) = split.last() {
                        return Ok(s.into());
                    }
                }
            }
        }
        Err(NatError::NoInterfaceFound)
    }

    fn iptable() -> Result<(), NatError> {
        let active_if = Self::find_active_if()?;
        let mut command = Command::new("iptables");
        command.args(&["-t", "nat", "-A", "POSTROUTING", "-j", "MASQUERADE", "-o", &active_if]);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        let child = io_wrap!(command.spawn());
        let output = io_wrap!(child.wait_with_output());
        if output.status.success() {
            Ok(())
        } else {
            Err(NatError::CommandError)
        }
    }

    pub fn enable(&self) ->  Result<(), NatError> {
        Self::forward(true)?;
        Self::iptable()?;
        Ok(())
    }
}

#[cfg(target_os="macos")]
impl Nat {
    fn sysctl(enable: bool) -> Result<(), NatError> {
        let mut command = Command::new("sysctl");
        let enable = if enable { 1 } else { 0 };
        command.args(&["-w", &format!("net.inet.ip.forwarding={enable}")]);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        let mut child = io_wrap!(command.spawn());
        let exit_code = io_wrap!(child.wait());
        if exit_code.success() {
            Ok(())
        } else {
            Err(NatError::CommandError)
        }
    }

    fn pfctl() -> Result<(), NatError> {
        let mut command = Command::new("pfctl");
        let child = io_wrap!(command.args(&["-f", "/etc/pf.anchors/bls-vm-nat", "-e" ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn());
        let output = io_wrap!(child.wait_with_output());
        if output.status.success() {
            return Ok(());
        } else {
            let out_string = String::from_utf8(output.stderr).map_err(|_| NatError::Utf8CodeError)?;
            if let Some(_) = out_string.find("pf already enabled") {
                return Ok(());
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