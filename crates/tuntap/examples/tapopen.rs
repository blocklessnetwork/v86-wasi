use tuntap::{Configuration, Tap};

fn main () {
    let mut cfg = Configuration::new();
    cfg.address("192.168.0.1");
    cfg.broadcast("192.168.0.5");
    cfg.netmask("255.255.254.0");
    let tap = Tap::new(cfg).unwrap();
    println!("tap addr {:?} ", tap.address());
    println!("tap broadcast {:?} ", tap.broadcast());
    println!("tap netmask {:?} ", tap.netmask());
}