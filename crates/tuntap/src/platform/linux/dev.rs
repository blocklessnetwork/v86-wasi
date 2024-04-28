

pub struct Tap {
    fd: Fd,
    name: String,
    _config: Configuration,
}

impl Tap {
    pub fn set_nonblock(&mut self) -> io::Result<()> {
        self.fd.set_nonblock()
    }

    pub fn new(_config: Configuration) -> Result<Self> {
        let (file, idx) = Self::try_open()?;
        let fd = Fd::new(fd)
            .map_err(|_| Error::Io(io::Error::last_os_error()))?;
        unsafe {
            let ctl = Fd::new(libc::socket(AF_INET, SOCK_DGRAM, 0))
                    .map_err(|_| io::Error::last_os_error())?;
            let name = format!("tap{}", idx);
            let cfg = _config.clone();
            let mut tap = Self {
                fd,
                ctl,
                name,
                _config,
            };
            tap.configure(&cfg)?;
            Ok(tap)
        }
    }

    fn try_open() -> Result<File> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun")?;
        Ok(file)
    }

}