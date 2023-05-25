use crate::StoreT;

pub(crate) struct VirtIOQueueOption {
    size_supported: u16,
    notify_offset: u16,
}

pub(crate) struct VirtIOptionCommon {
    initial_port: u16,
    queues: Vec<VirtIOQueueOption>,
    features: Vec<u8>,
}

pub(crate) struct VirtIOptions {
    name: String,
    pci_id: u16,
    device_id: u16,
    subsystem_device_id: u16,
    common: Vec<VirtIOptionCommon>,
}

pub(crate) struct VirtIO {
    store: StoreT,
    pci_id: u16,
    device_id: u16,
}

impl VirtIO {
    pub fn new(store: StoreT, options: VirtIOptions) -> Self {
        Self {
            store,
            pci_id: options.pci_id,
            device_id: options.device_id,
        }
    }

    pub fn init(&mut self) {
    }
}