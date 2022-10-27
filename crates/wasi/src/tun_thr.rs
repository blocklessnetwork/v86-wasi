use tokio::runtime::Builder;

use crate::ContextTrait;
use tun;


pub struct TunThread {
    address: String,
    netmask: String,
}

impl TunThread {
    pub fn new(address: String, netmask: String) -> Self {
        Self {
            address,
            netmask,
        }
    }

    pub fn start(mut self) {
        let runtime = Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();
        let mut config = tun::Configuration::default();
        
        config
            .address(&self.address)
            .netmask(&self.netmask)
            .up();

        #[cfg(target_os = "linux")]
        config.platform(|config| {
            config.packet_information(true);
        });
        
        runtime.block_on(async move {
            let mut dev = tun::create_as_async(&config).unwrap();
            let (r, w) = dev.get_mut().split();
            tokio::spawn(async {
                    
            });
        });
    }
}