use std::process;
use crate::{Result, Error};

fn exec_netsh(args: &[&str]) -> Result<()> {
    match process::Command::new("netsh")
        .args(args)
        .stderr(process::Stdio::null())
        .stdout(process::Stdio::null())
        .status() {
        Ok(res) => {
            if res.success() {
                Ok(())
            } else {
                Err(Error::NetshExecError)
            }
        }
        Err(e) => Err(Error::Io(e))
    }
}

pub fn set_interface_name(name: &str, newname: &str) -> Result<()> {
    exec_netsh(&["int", "set", "int", "name=", name, "newname=", newname])
}
