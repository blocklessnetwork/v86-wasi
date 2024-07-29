
macro_rules! io_wrap {
    ($io_op:expr) => {
        $io_op.map_err(|e| NatError::IoError(e))?
    };
}

#[cfg(any(target_os="linux", target_os="macos"))]
macro_rules! syscall {
    ($ep: expr) => {
        unsafe {
            let n = $ep as i64;
            if n < 0 {
                return Err(NatError::IoError(std::io::Error::last_os_error()));
            }
            n
        }    
    };
}