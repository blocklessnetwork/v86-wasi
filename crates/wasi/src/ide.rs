use crate::StoreT;


const CDROM_SECTOR_SIZE: u32 = 2048;

const HD_SECTOR_SIZE: u32 = 512;

struct IDEInterface {
    store: StoreT,
}

impl IDEInterface {
    fn new(store: StoreT) -> IDEInterface {

        IDEInterface { 
            store 
        }
    }
}

pub(crate) struct IDEDevice {
    
}

impl IDEDevice {
    
}

