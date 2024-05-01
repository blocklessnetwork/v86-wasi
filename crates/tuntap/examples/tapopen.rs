use tuntap::{IntoAddress, Configuration, Tap};

fn main () {
    let mut cfg = Configuration::new();
    cfg.address("192.168.0.1");
    let tap = Tap::new(cfg).unwrap();
}