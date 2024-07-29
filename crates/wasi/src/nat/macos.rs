use std::{
    fs::OpenOptions, io::Write, process::{Command, Stdio}
};
use super::NatError;

pub(crate) fn sysctl(enable: bool) -> Result<(), NatError> {
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

pub(crate) fn pfctl() -> Result<(), NatError> {
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

/// write the archors file
pub(crate) fn write_anchors(name: &str) {
    let mut pfctl = OpenOptions::new()
        .write(true)
        .create(true)
        .open("/etc/pf.anchors/bls-vm-nat")?;
    let cmd = format!("nat on en0 from {name}:network to any -> (en0)\n");
    pfctl.write_all(cmd.as_bytes())?;
}
