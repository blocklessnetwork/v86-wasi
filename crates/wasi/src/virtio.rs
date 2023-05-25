use crate::{StoreT, ContextTrait};

// Size (bytes) of the virtq_desc struct per queue size.
const VIRTIO_PCI_VENDOR_ID: u16 = 0x1AF4;
// Size (bytes) of the virtq_avail struct per queue size.
const VIRTQ_AVAIL_ENTRYSIZE: u32 = 2;
// Size (bytes) of the virtq_desc struct per queue size.
const VIRTQ_USED_ENTRYSIZE: u32 = 8;


struct VirtQueue {
    store: StoreT,
    size: u32,
    mask: u32,
    enabled: bool,
    size_supported: u32,
    notify_offset: u32,
    desc_addr: u32,
    avail_addr: u32,
    avail_last_idx: u32,
    used_addr: u32,
    num_staged_replies: u32,
}

impl VirtQueue {
    fn reset(&mut self) {
        self.enabled = false;
        self.desc_addr = 0;
        self.avail_addr = 0;
        self.avail_last_idx = 0;
        self.used_addr = 0;
        self.num_staged_replies = 0;
        self.set_size(self.size_supported);
    }

    fn set_size(&mut self, size: u32) {
        assert!((size & size - 1) == 0, "VirtQueue size must be power of 2 or zero");
        assert!(size <= self.size_supported, "VirtQueue size must be within supported size");
        self.size = size;
        self.mask = size - 1;
    }

    fn enable(&mut self) {
        assert!(self.is_configured(), "VirtQueue must be configured before enabled");
        self.enabled = true;
    }

    fn is_configured(&self) -> bool {
        return self.desc_addr > 0 && self.avail_addr > 0 && self.used_addr > 0;
    }

    fn count_requests(&self) -> u32 {
        assert!(self.avail_addr > 0, "VirtQueue addresses must be configured before use");
        return (self.avail_get_idx() - self.avail_last_idx) & self.mask;
    }

    fn avail_get_idx(&self) -> u32 {
        self.store.cpu_mut().map(|cpu| {
            cpu.read16(self.avail_addr + 2)
        }).unwrap() as _
    }

    fn avail_get_entry(&self, i: u32) -> u32 {
        self.store.cpu_mut().map(|cpu| {
            cpu.read16(self.avail_addr + 4 + VIRTQ_AVAIL_ENTRYSIZE * i)
        }).unwrap() as _
    }

    fn avail_get_used_event(&self) -> u32 {
        self.store.cpu_mut().map(|cpu| {
            cpu.read16(self.avail_addr + 4 + VIRTQ_AVAIL_ENTRYSIZE * self.size)
        }).unwrap() as _
    }

    fn used_get_flags(&self) -> u32  {
        self.store.cpu_mut().map(|cpu| {
            cpu.read16(self.used_addr)
        }).unwrap() as _
    }

    fn used_set_flags(&self, value: i32)  {
        self.store.cpu_mut().map(|cpu| {
            cpu.write16(self.used_addr, value);
        });
    }

    fn used_get_idx(&self) -> u32 {
        self.store.cpu_mut().map(|cpu| {
            cpu.read16(self.used_addr + 2)
        }).unwrap() as _
    }

    fn used_set_idx(&self, value: i32)  {
        self.store.cpu_mut().map(|cpu| {
            cpu.write16(self.used_addr + 2, value);
        });
    }
    
    fn used_set_entry(&self, i: u32, desc_idx: i32, length_written: i32)  {
        self.store.cpu_mut().map(|cpu| {
            cpu.write32(self.used_addr + 4 + VIRTQ_USED_ENTRYSIZE* i, desc_idx);
            cpu.write32(self.used_addr + 8 + VIRTQ_USED_ENTRYSIZE* i, length_written);
        });
    }

    fn used_set_avail_event(&self, value: i32) {
        self.store.cpu_mut().map(|cpu| {
            cpu.write16(self.used_addr + 4 + VIRTQ_USED_ENTRYSIZE * self.size, value);
        });
    }
    


}

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
    queues: Vec<VirtQueue>,
    options: VirtIOptions,
}

impl VirtIO {
    pub fn new(store: StoreT, options: VirtIOptions) -> Self {
        Self {
            store,
            pci_id: options.pci_id,
            device_id: options.device_id,
            queues: Vec::new(),
            options,
        }
    }

    pub fn init(&mut self) {
        let pci_space: Vec<u8> = vec![
            (VIRTIO_PCI_VENDOR_ID & 0xFF) as u8, (VIRTIO_PCI_VENDOR_ID >> 8) as _,  
            (self.device_id & 0xFF) as _, (self.device_id >> 8) as _,
             // Command
            0x07, 0x05,
            // Status - enable capabilities list
            0x10, 0x00,
            // Revision ID
            0x01,
            // Prof IF, Subclass, Class code
            0x00, 0x02, 0x00,
            // Cache line size
            0x00,
            // Latency Timer
            0x00,
            // Header Type
            0x00,
            // Built-in self test
            0x00,
            // BAR0
            0x01, 0xa8, 0x00, 0x00,
            // BAR1
            0x00, 0x10, 0xbf, 0xfe,
            // BAR2
            0x00, 0x00, 0x00, 0x00,
            // BAR3
            0x00, 0x00, 0x00, 0x00,
            // BAR4
            0x00, 0x00, 0x00, 0x00,
            // BAR5
            0x00, 0x00, 0x00, 0x00,
            // CardBus CIS pointer
            0x00, 0x00, 0x00, 0x00,
            // Subsystem vendor ID
            (VIRTIO_PCI_VENDOR_ID & 0xFF) as _, (VIRTIO_PCI_VENDOR_ID >> 8) as _,
            // Subsystem ID
            (self.options.subsystem_device_id & 0xFF) as _, (self.options.subsystem_device_id >> 8) as _,
            // Expansion ROM base address
            0x00, 0x00, 0x00, 0x00,
            // Capabilities pointer
            0x40,
            // Reserved
            0x00, 0x00, 0x00,
            // Reserved
            0x00, 0x00, 0x00, 0x00,
            // Interrupt line
            0x00,
            // Interrupt pin
            0x01,
            // Min grant
            0x00,
            // Max latency
            0x00,
        ];
        
    }
}