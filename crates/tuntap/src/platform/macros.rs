
macro_rules! set_ifname {
    ($req: expr, $name: ident) => {
        if !$name.is_empty() {
            let mut ifname: [i8; IFNAMSIZ as _] = [0; IFNAMSIZ as _];
            for (i, c) in $name.as_bytes().iter().enumerate() {
                ifname[i] = *c as _;
            }
            $req = ifname;
        }
    };
}

macro_rules! get_ifname {
    ($req: expr, $name: ident) => {(
        for i in 0..IFNAMSIZ as _ {
            let c =  $req[i] as u8 as char;
            if c != '\0' {
                $name.push(c)
            }
        }
    )};
}

/// syscall macro wrapper.
macro_rules! syscall {
    ($call: expr) => {
        unsafe {
            let n = $call as i32;
            if n < 0 {
                return Err(Error::Io(io::Error::last_os_error()));
            }
            n
        }
    };
}