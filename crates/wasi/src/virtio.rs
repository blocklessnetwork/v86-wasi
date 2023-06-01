#![allow(dead_code)]
use crate::{StoreT, ContextTrait, log::LOG, int_log2, pci::{PCIBar, GenericPCIDevice}, Dev, io::IO};
use std::collections::{HashSet, HashMap};

const DEBUG: bool = false;

const VIRTIO_PCI_VENDOR_ID: u16 = 0x1AF4;
// Identifies vendor-specific PCI capability.
const VIRTIO_PCI_CAP_VENDOR: u8 = 0x09;
// Length (bytes) of VIRTIO_PCI_CAP linked list entry.
const VIRTIO_PCI_CAP_LENGTH: u8 = 16;

const VIRTIO_PCI_CAP_COMMON_CFG: u8 = 1;
const VIRTIO_PCI_CAP_NOTIFY_CFG: u8 = 2;
const VIRTIO_PCI_CAP_ISR_CFG: u8 = 3;
const VIRTIO_PCI_CAP_DEVICE_CFG: u8 = 4;
const VIRTIO_PCI_CAP_PCI_CFG: u8 = 5;

const VIRTIO_STATUS_ACKNOWLEDGE: u8 = 1;
const VIRTIO_STATUS_DRIVER: u8 = 2;
const VIRTIO_STATUS_DRIVER_OK: u8 = 4;
const VIRTIO_STATUS_FEATURES_OK: u8 = 8;
const VIRTIO_STATUS_DEVICE_NEEDS_RESET: u8 = 64;
const VIRTIO_STATUS_FAILED: u8 = 128;

// ISR bits (isr_status values).

const VIRTIO_ISR_QUEUE: u8 = 1;
const VIRTIO_ISR_DEVICE_CFG: u8 = 2;


// Size (bytes) of the virtq_desc struct per queue size.
const VIRTQ_DESC_ENTRYSIZE: u32 = 16;
// Size (bytes) of the virtq_avail struct ignoring ring entries.
const VIRTQ_AVAIL_BASESIZE: u32 = 6;
// Size (bytes) of the virtq_avail struct per queue size.
const VIRTQ_AVAIL_ENTRYSIZE: u32 = 2;
// Size (bytes) of the virtq_desc struct per queue size.
const VIRTQ_USED_ENTRYSIZE: u32 = 8;
// Mask for wrapping the idx field of the virtq_used struct so that the value
// naturally overflows after 65535 (idx is a word).
const VIRTQ_IDX_MASK: u32 = 0xFFFF;

const VIRTIO_F_VERSION_1: u8 = 32;

struct DescTable {
    addr_low: i32,
    addr_high: i32,
    len: i32,
    flags: i32,
    next: i32,
}

pub struct VirtIODeviceSpecificCapabilityOptions {
    initial_port: u16,
    struct_: Vec<VirtIOCapabilityInfoStruct>,
}

pub struct VirtIONotificationCapabilityOptions {
    initial_port: u16,
    single_handler: bool,
    handlers: Vec<StructWrite>,
}

pub struct VirtIOISRCapabilityOptions {
    initial_port: u16,
}

pub struct VirtQueue {
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
    fn new(store: StoreT, options: &VirtQueueOptions) -> Self {
        let size = options.size_supported;
        let size_supported = options.size_supported;
        let mask = size - 1;
        let notify_offset = options.notify_offset;
        Self {
            store,
            size,
            size_supported,
            enabled: false,
            notify_offset,
            desc_addr: 0,
            avail_addr: 0,
            avail_last_idx: 0,
            used_addr: 0,
            num_staged_replies: 0,
            mask
        }
    }

    fn init(&mut self) {
        self.reset();
    }

    fn get_descriptor(&self, table_address: u32, i: u32) -> DescTable {
        let addr_low = self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read32s(table_address + i * VIRTQ_DESC_ENTRYSIZE)
        });
        let addr_high = self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read32s(table_address + i * VIRTQ_DESC_ENTRYSIZE + 4)
        });
        let len = self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read32s(table_address + i * VIRTQ_DESC_ENTRYSIZE + 8)
        });
        let flags = self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read16(table_address + i * VIRTQ_DESC_ENTRYSIZE + 12)
        });
        let next = self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read16(table_address + i * VIRTQ_DESC_ENTRYSIZE + 14)
        });
        DescTable {
            addr_low,
            addr_high,
            len,
            flags,
            next
        }
    }

    fn avail_get_flags(&self) -> i32 {
        let addr = self.avail_addr;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read16(addr)
        })
    }

    fn avail_get_idx(&self) -> i32 {
        let addr = self.avail_addr + 2;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read16(addr)
        })
    }

    fn avail_get_entry(&self, i: u32) -> i32 {
        let addr = self.avail_addr + 4 + VIRTQ_AVAIL_ENTRYSIZE * i;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read16(addr)
        })
    }

    fn avail_get_used_event(&self) -> i32 {
        let addr = self.avail_addr + 4 + VIRTQ_AVAIL_ENTRYSIZE * self.size;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read16(addr)
        })
    }

    fn used_get_flags(&self) -> i32 {
        let addr = self.used_addr;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read16(addr)
        })
    }

    fn used_set_flags(&mut self, value: i32) {
        let addr = self.used_addr;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.write16(addr, value)
        })
    }

    fn used_get_idx(&self) -> i32 {
        let addr = self.used_addr + 2;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.read16(addr)
        })
    }

    fn used_set_idx(&mut self, value: i32) {
        let addr = self.used_addr + 2;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.write16(addr, value)
        })
    }

    fn used_set_entry(&mut self, i: u32, desc_idx: i32, length_written: i32) {
        let desc_idx_addr = self.used_addr + 4 + VIRTQ_USED_ENTRYSIZE * i;
        let length_written_addr = self.used_addr + 8 + VIRTQ_USED_ENTRYSIZE * i;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.write32(desc_idx_addr, desc_idx);
            cpu.write32(length_written_addr, length_written);
        })
    }

    fn used_set_avail_event(&mut self, value: i32) {
        let addr = self.used_addr + 4 + VIRTQ_USED_ENTRYSIZE * self.size;
        self.store.cpu_mut().map_or(0, |cpu| {
            cpu.write16(addr, value)
        })
    }
    

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

    fn has_request(&self) -> bool {
        assert!(self.avail_addr > 0, "VirtQueue addresses must be configured before use");
        return (self.avail_get_idx() & self.mask) != self.avail_last_idx;
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

type StructWrite = fn(StoreT, i32);
type StructRead = fn(StoreT) -> i32;

struct VirtIOCapabilityInfoStruct {
    bytes: u8,
    name: String,
    read: StructRead,
    write: StructWrite,
}

pub(crate) struct VirtIOCapabilityInfo {
    type_: u8,
    bar: u8,
    port: u16,
    use_mmio: bool,
    offset: u32,
    extra: Vec<u8>,
    struct_: Vec<VirtIOCapabilityInfoStruct>,
}

pub(crate) struct VirtQueueOptions {
    size_supported: u32,
    notify_offset: u32,
}

pub(crate) struct VirtIOCommonCapabilityOptions {
    initial_port: u16,
    queues: Vec<VirtQueueOptions>,
    features: Vec<u8>,
    on_driver_ok: fn(StoreT)
}

pub struct VirtIOptions {
    name: String,
    pci_id: u16,
    device_id: u16,
    single_handler: bool,
    subsystem_device_id: u16,
    common: VirtIOCommonCapabilityOptions,
    isr_status: VirtIOISRCapabilityOptions,
    notification: VirtIONotificationCapabilityOptions,
    device_specific: Option<VirtIODeviceSpecificCapabilityOptions>,
}

#[derive(Clone, Copy)]
struct AddrRW {
    read: StructRead,
    write: StructWrite,
}

pub struct VirtIO {
    store: StoreT,
    pci_id: u16,
    name: String,
    device_id: u16,
    isr_status: i32,
    queue_select: i32,
    device_feature_select: i32,
    driver_feature_select: i32,
    device_feature: [u32; 4],
    driver_feature: [u32; 4],
    features_ok: bool,
    device_status: i32,
    config_has_changed: bool,
    config_generation: i32,
    pci_space: Option<Vec<u8>>,
    pci_bars: Option<Vec<PCIBar>>,
    queues: Vec<VirtQueue>,
    options: VirtIOptions,
    addr_rw: HashMap<u32, AddrRW>
}

impl VirtIO {

    pub fn queue_selected_mut(&mut self) -> Option<&mut VirtQueue> {
        self.queues.iter_mut().nth(self.queue_select as _)
    }

    pub fn queue_selected(&self) -> Option<&VirtQueue> {
        self.queues.iter().nth(self.queue_select as _)
    }

    pub fn new(store: StoreT, options: VirtIOptions) -> Self {
        let name = options.name.clone();
        let mut device_feature = [0u32; 4];
        let mut driver_feature = [0u32; 4];
        for f in options.common.features.iter() {
            assert!(*f >= 0,
                "VirtIO device<{name}> feature bit numbers must be non-negative");
            assert!(*f < 128,
                "VirtIO device<{name}> feature bit numbers assumed less than 128 in implementation");
    
            // Feature bits are grouped in 32 bits.
            device_feature[(f >> 5) as usize] |= 1 << (f & 0x1F);
            driver_feature[(f >> 5) as usize] |= 1 << (f & 0x1F);
        }
        assert!(options.common.features.contains(&VIRTIO_F_VERSION_1),
        "VirtIO device<{name}> only non-transitional devices are supported");

        let queues = options.common.queues
            .iter()
            .map(|opt| VirtQueue::new(store.clone(), opt)) 
            .collect::<Vec<_>>();
        let queue_select = 0;

        if DEBUG {
            let offsets = options.common.queues
                .iter()
                .map(|q| {
                    let effective_offset = if options.notification.single_handler {
                        0
                    } else {
                        q.notify_offset as usize
                    };
                    let nth = options.notification.handlers.iter().nth(effective_offset);
                    assert!(nth.is_some(), 
                        "VirtIO device<{name}> every queue's notifier must exist");
                    effective_offset
                })
                .collect::<HashSet<_>>();
            options.notification.handlers.iter().enumerate().for_each(|(i, h)| {
                assert!(offsets.contains(&i),
                    "VirtIO device<{name}> no defined notify handler should be unused");
            });
        }

        let device_id = options.device_id;

        let mut pci_space: Vec<u8> = vec![
            (VIRTIO_PCI_VENDOR_ID & 0xFF) as u8, (VIRTIO_PCI_VENDOR_ID >> 8) as _,  
            (device_id & 0xFF) as _, (device_id >> 8) as _,
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
            (options.subsystem_device_id & 0xFF) as _, (options.subsystem_device_id >> 8) as _,
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
        pci_space.resize(256, 0);
        Self {
            store,
            name,
            device_feature_select: 0, 
            driver_feature_select: 0,
            device_feature,
            driver_feature,
            queue_select,
            pci_id: options.pci_id,
            device_id: options.device_id,
            queues,
            isr_status: 0,
            features_ok: true,
            device_status: 0,
            config_generation: 0,
            addr_rw: HashMap::new(),
            config_has_changed: false,
            pci_space: Some(pci_space),
            pci_bars: None,
            options,
        }
    }
    
    fn create_isr_capability(&self) -> VirtIOCapabilityInfo {
        VirtIOCapabilityInfo {
            type_: VIRTIO_PCI_CAP_ISR_CFG,
            bar: 2,
            port: self.options.isr_status.initial_port,
            use_mmio: false,
            offset: 0,
            extra: vec![],
            struct_: vec![VirtIOCapabilityInfoStruct {
                bytes: 1,
                name: "isr_status".into(),
                read: |store| {
                    store.virtio_mut().map_or(0, |vr| {
                        let isr_status = vr.isr_status;
                        vr.lower_irq();
                        isr_status
                    })
                },
                write: |_store, _v| {
                    //read only
                },
            }]
        }
    }

    fn create_notification_capability(&self) -> VirtIOCapabilityInfo {
        let notify_off_multiplier = if self.options.single_handler {
            assert!(self.options.notification.handlers.len() == 1,
                "VirtIO device<{}> too many notify handlers specified: expected single handler",
                self.name
            );
            0
        } else {
            2
        };
        let notify_struct = self.options.notification.handlers
            .iter()
            .enumerate()
            .map(|(i, handler)| {
                VirtIOCapabilityInfoStruct {
                    bytes: 2,
                    name: format!("notify{i}"),
                    read: |_| 0xFFFF,
                    write: *handler,
                }
            }).collect::<Vec<_>>();
        VirtIOCapabilityInfo {
            type_: VIRTIO_PCI_CAP_NOTIFY_CFG,
            bar: 1,
            port: self.options.notification.initial_port,
            use_mmio: false,
            offset: 0,
            extra: vec![
                (notify_off_multiplier & 0xFF) as u8,
                ((notify_off_multiplier >> 8) & 0xFF) as u8,
                ((notify_off_multiplier >> 16) & 0xFF) as u8,
                (notify_off_multiplier >> 24) as u8,
            ],
            struct_: notify_struct
        }
    }

    fn create_device_specific_capability(&self, options: VirtIODeviceSpecificCapabilityOptions) -> VirtIOCapabilityInfo {
        VirtIOCapabilityInfo {
            type_: VIRTIO_PCI_CAP_DEVICE_CFG,
            bar: 3,
            port: options.initial_port,
            use_mmio: false,
            offset: 0,
            extra: Vec::new(),
            struct_: options.struct_
        }
    }

    fn create_common_capability(&self) -> VirtIOCapabilityInfo {
        let mut rs = Vec::new();
        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "device_feature_select".into(),
            read: |store| {
                store.virtio().map_or(0, |vr| vr.device_feature_select)
            },
            write: |store, data| {
                store.virtio_mut().map(|vr| vr.device_feature_select = data);
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "device_feature".into(),
            read: |store| {
                store.virtio().map_or(0, |vr| vr.device_feature[vr.device_feature_select as usize] as _)
            },
            write: |_store, _v| {
                //read only
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "driver_feature_select".into(),
            read: |store| {
                store.virtio().map_or(0, |vr| vr.driver_feature_select)
            },
            write: |store, data| {
                store.virtio_mut().map(|vr| vr.driver_feature_select = data);
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "driver_feature".into(),
            read: |store| {
                store.virtio().map_or(0, |vr| vr.driver_feature[vr.driver_feature_select as usize] as _)
            },
            write: |store, data| {
                store.virtio_mut().map(|vr| {
                    let supported_feature = vr.device_feature[vr.driver_feature_select as usize];
                    if (vr.driver_feature_select as usize) < vr.driver_feature.len() {
                        // Note: only set subset of device_features is set.
                        // Required in our implementation for is_feature_negotiated().
                        vr.driver_feature[vr.driver_feature_select as usize] = (data as u32) & supported_feature;
                    }
                    // Check that driver features is an inclusive subset of device features.
                    let invalid_bits = data & !(supported_feature as i32);
                    vr.features_ok = vr.features_ok && invalid_bits == 0;
                });
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 2,
            name: "msix_config".into(),
            read: |_store| {
                dbg_log!(LOG::VIRTIO, "No msi-x capability supported.");
                0xFFFF
            },
            write: |_store, _v| {
                dbg_log!(LOG::VIRTIO, "No msi-x capability supported.");
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 2,
            name: "num_queues".into(),
            read: |store| {
                store.virtio().map_or(0, |vr| vr.queues.len() as _)
            },
            write: |_store, _v| {
            },
        };
        rs.push(struct_);


        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 1,
            name: "device_status".into(),
            read: |store| {
                store.virtio().map_or(0, |vr| vr.device_status)
            },
            write: |store, mut data| {
                store.virtio_mut().map(|vr| {
                    if data == 0 {
                        dbg_log!(LOG::VIRTIO, "Reset device<{}>", vr.name);
                        vr.reset();
                    } else if data & (VIRTIO_STATUS_FAILED as i32) > 0 {
                        dbg_log!(LOG::VIRTIO, "Warning: Device<{}> status failed", vr.name);
                    } else {
                        let acknowlege = if data & (VIRTIO_STATUS_ACKNOWLEDGE as i32) > 0 {
                            "ACKNOWLEDGE "
                        } else {
                            ""
                        };
                        let driver = if data & (VIRTIO_STATUS_DRIVER as i32) > 0 {
                            "DRIVER "
                        } else {
                            ""
                        };
                        let driver_ok = if data & (VIRTIO_STATUS_DRIVER_OK as i32) > 0 {
                            "DRIVER_OK "
                        } else {
                            ""
                        };
                        let feature_ok = if data & (VIRTIO_STATUS_FEATURES_OK as i32) > 0 {
                            "FEATURES_OK "
                        } else {
                            ""
                        };
                        let need_rest = if data & (VIRTIO_STATUS_DEVICE_NEEDS_RESET as i32) > 0 {
                            "DEVICE_NEEDS_RESET "
                        } else {
                            ""
                        };
                        dbg_log!(LOG::VIRTIO, 
                            "Device<{}> status: {acknowlege}{driver}{driver_ok}{feature_ok}{need_rest}",
                            vr.name
                        );
                    }

                    if (data & !(vr.device_status as i32) & (VIRTIO_STATUS_DRIVER_OK as i32)) > 0 &&
                        (vr.device_status & VIRTIO_STATUS_DEVICE_NEEDS_RESET as i32) > 0 {
                        // We couldn't notify NEEDS_RESET earlier because DRIVER_OK was not set.
                        // Now it has been set, notify now.
                        vr.notify_config_changes();
                    }

                    // Don't set FEATURES_OK if our device doesn't support requested features.
                    if !vr.features_ok {
                        if DEBUG && (data & VIRTIO_STATUS_FEATURES_OK as i32) > 0 {
                            dbg_log!(LOG::VIRTIO, "Removing FEATURES_OK");
                        }
                        data &= (!VIRTIO_STATUS_FEATURES_OK) as i32;
                    }

                    vr.device_status = data;

                    if (data & !vr.device_status & VIRTIO_STATUS_DRIVER_OK as i32) > 0 {
                        (vr.options.common.on_driver_ok)(store.clone());
                    }
                });
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 1,
            name: "config_generation".into(),
            read: |store| {
                store.virtio().map_or(0, |vr| vr.config_generation as _)
            },
            write: |_store, _| {
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 2,
            name: "queue_select".into(),
            read: |store| {
                store.virtio().map_or(0, |vr| vr.queue_select as _)
            },
            write: |store, data| {
                store.virtio_mut().map(|vr| {
                    vr.queue_select = data;
                    // if vr.queue_select < vr.queues.len() as i32 {
                    //     vr.queues_selected = vr.queues[vr.queue_select as usize] as i32;
                    // }
                });
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 2,
            name: "queue_size".into(),
            read: |store| {
                store.virtio().and_then(|vr| vr.queue_selected())
                    .and_then(|q| Some(q.size as i32)).unwrap_or(0)
            },
            write: |store, mut data| {
                store.virtio_mut().map(|vr| {
                    if vr.queue_selected().is_none() {
                        return;
                    }
                    if (data & data - 1) > 0 {
                        dbg_log!(LOG::VIRTIO, 
                            "Warning: dev<{}> Given queue size was not a power of 2. Rounding up to next power of 2.",
                            vr.name
                        );
                        data = 1 << (int_log2(data - 1) + 1);
                    }
                    let size_supported = vr.queue_selected().map_or(0, |q| q.size_supported as i32);
                    if data > size_supported {
                        dbg_log!(LOG::VIRTIO, 
                            "Warning: dev<{}> Trying to set queue size greater than supported. Clamping to supported size.",
                            vr.name
                        );
                        data = size_supported;
                    }
                    vr.queue_selected_mut().map(|q| {
                        q.set_size(data as u32);
                    });
                });
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 2,
            name: "queue_msix_vector".into(),
            read: |_store| {
                dbg_log!(LOG::VIRTIO, "No msi-x capability supported.");
                0xFFFF
            },
            write: |_store, _data| {
                dbg_log!(LOG::VIRTIO, "No msi-x capability supported.");
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 2,
            name: "queue_enable".into(),
            read: |store| {
                store.virtio().and_then(|vr| {
                    vr.queue_selected().map(|q| {
                        let enabled = if q.enabled {
                            1
                        } else {
                            0
                        };
                        enabled | 0
                    })
                }).unwrap_or(0)
            },
            write: |store, data| {
                store.virtio_mut().map(|vr| {
                    if vr.queue_selected().is_none() {
                        return;
                    }
                    let queue_selected = vr.queue_selected_mut().unwrap();
                    if data == 1 {
                        if queue_selected.is_configured() {
                            queue_selected.enable();
                        } else {
                            dbg_log!(LOG::VIRTIO, "Driver bug: tried enabling unconfigured queue");
                        }
                    } else if data == 0 {
                        dbg_log!(LOG::VIRTIO, "Driver bug: tried writing 0 to queue_enable");
                    }
                });
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 2,
            name: "queue_notify_off".into(),
            read: |store| {
                store.virtio().and_then(|vr| {
                    vr.queue_selected()
                }).map_or(0, |q| q.notify_offset as i32)
            },
            write: |_store, _data| {
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "queue_desc (low dword)".into(),
            read: |store| {
                store.virtio().and_then(|vr| {
                    vr.queue_selected()
                }).map_or(0, |q| q.desc_addr as i32)
            },
            write: |store, data| {
                store.virtio_mut().and_then(|vr| {
                    vr.queue_selected_mut()
                }).map(|q| q.desc_addr = data as u32);
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "queue_desc (high dword)".into(),
            read: |store| {
                0
            },
            write: |_store, _data| {
                dbg_log!(LOG::VIRTIO, "Warning: High dword of 64 bit queue_desc ignored");
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "queue_avail (low dword)".into(),
            read: |store| {
                store.virtio().and_then(|vr| {
                    vr.queue_selected()
                }).map_or(0, |q| q.avail_addr as i32)
            },
            write: |store, data| {
                store.virtio_mut().and_then(|vr| {
                    vr.queue_selected_mut()
                }).map(|q| q.avail_addr = data as u32);
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "queue_avail (high dword)".into(),
            read: |store| {
                0
            },
            write: |_store, _data| {
                dbg_log!(LOG::VIRTIO, "Warning: High dword of 64 bit queue_avail ignored");
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "queue_used (low dword)".into(),
            read: |store| {
                store.virtio().and_then(|vr| {
                    vr.queue_selected()
                }).map_or(0, |q| q.used_addr as i32)
            },
            write: |store, data| {
                store.virtio_mut().and_then(|vr| {
                    vr.queue_selected_mut()
                }).map(|q| q.used_addr = data as u32);
            },
        };
        rs.push(struct_);

        let struct_ = VirtIOCapabilityInfoStruct {
            bytes: 4,
            name: "queue_used (high dword)".into(),
            read: |store| {
                0
            },
            write: |_store, _data| {
                dbg_log!(LOG::VIRTIO, "Warning: High dword of 64 bit queue_used ignored");
            },
        };
        rs.push(struct_);
        VirtIOCapabilityInfo {
            type_: VIRTIO_PCI_CAP_COMMON_CFG,
            bar: 0,
            port: self.options.common.initial_port,
            use_mmio: false,
            offset: 0,
            extra: Vec::new(),
            struct_: rs
        }
    }

    fn notify_config_changes(&mut self) {
        self.config_has_changed = true;

        if self.device_status & VIRTIO_STATUS_DRIVER_OK as i32 > 0 {
            self.raise_irq(VIRTIO_ISR_DEVICE_CFG);
        } else {
            assert!(false,
                "VirtIO device<{}> attempted to notify driver before DRIVER_OK",
                self.name
            );
        }
    }

    fn raise_irq(&mut self, typ: u8) {
        dbg_log!(LOG::VIRTIO, "Raise irq {typ:x}");
        self.isr_status |= typ as i32;
        self.store.pci_mut().map(|pci| {
            pci.raise_irq(self.pci_id);
        });
    }

    fn lower_irq(&mut self) {
        dbg_log!(LOG::VIRTIO, "Lower irq ");
        self.isr_status = 0;
        self.store.pci_mut().map(|pci| {
            pci.raise_irq(self.pci_id);
        });
    }

    pub fn reset(&mut self) {
        self.device_feature_select = 0;
        self.driver_feature_select = 0;
        self.driver_feature.copy_from_slice(&self.device_feature);

        self.features_ok = true;
        self.device_status = 0;

        self.queue_select = 0;

        self.queues.iter_mut().for_each(|que| que.reset());

        self.config_has_changed = false;
        self.config_generation = 0;

        self.lower_irq();
    }

    fn init_capabilities(&mut self, capabilities: Vec<VirtIOCapabilityInfo>) {
        // Next available offset for capabilities linked list.
        let pci_space = self.pci_space.as_mut().unwrap();
        pci_space[0x34] = 0x40;
        let mut cap_next: i32 = pci_space[0x34] as _;
        // Current offset.
        let mut cap_ptr = cap_next;

        let mut pci_bars: Vec<PCIBar> = Vec::new(); 
        for cap in capabilities.iter() {
            let cap_len = VIRTIO_PCI_CAP_LENGTH as i32 + cap.extra.len() as i32;
            cap_ptr = cap_next;

            cap_next = cap_ptr + cap_len;

            assert!(cap_next <= 256,
                "VirtIO device<{}> can't fit all capabilities into 256byte configspace",
                self.name
            );

            assert!(0 <= cap.bar && cap.bar < 6,
                "VirtIO device<{}> capability invalid bar number",
                self.name
            );
            let mut bar_size: u32 = cap.struct_.iter().fold(0u32, |acc, s| acc + (s.bytes as u32));
            bar_size += cap.offset as u32;
            bar_size = if bar_size < 16 {
                16  
            } else { 
                1 << (int_log2((bar_size as i32) - 1) + 1)
            };
            assert!(((cap.port as u32) & (bar_size - 1)) == 0,
                "VirtIO device<{}> capability port should be aligned to pci bar size",
                self.name
            );
            pci_bars[cap.bar as usize] =  PCIBar::new(bar_size);
            let cap_ptr = cap_ptr as usize;
            pci_space[cap_ptr] = VIRTIO_PCI_CAP_VENDOR;
            pci_space[cap_ptr + 1] = cap_next as u8;
            pci_space[cap_ptr + 2] = cap_len as u8;
            pci_space[cap_ptr + 3] = cap.type_ as u8;
            pci_space[cap_ptr + 4] = cap.bar;

            pci_space[cap_ptr + 5] = 0; // Padding.
            pci_space[cap_ptr + 6] = 0; // Padding.
            pci_space[cap_ptr + 7] = 0; // Padding.

            pci_space[cap_ptr + 8] = (cap.offset & 0xFF) as u8;
            pci_space[cap_ptr + 9] = ((cap.offset >> 8) & 0xFF) as u8;
            pci_space[cap_ptr + 10] = ((cap.offset >> 16) & 0xFF) as u8;
            pci_space[cap_ptr + 11] = (cap.offset >> 24) as u8;

            pci_space[cap_ptr + 12] = (bar_size & 0xFF) as u8;
            pci_space[cap_ptr + 13] = ((bar_size >> 8) & 0xFF) as u8;
            pci_space[cap_ptr + 14] = ((bar_size >> 16) & 0xFF) as u8;
            pci_space[cap_ptr + 15] = (bar_size >> 24) as u8;

            cap.extra.iter().enumerate().for_each(|(i, extra_byte)| {
                pci_space[cap_ptr + 16 + i] = *extra_byte;
            });


            let bar_offset = 0x10 + 4 * (cap.bar as usize);
            let mmio = if cap.use_mmio {
                0
            } else {
                1
            };
            pci_space[bar_offset] = ((cap.port & 0xFE) | mmio) as u8;
            pci_space[bar_offset + 1] = ((cap.port >> 8) & 0xFF) as u8;
            pci_space[bar_offset + 2] = ((cap.port >> 16) & 0xFF) as u8;
            pci_space[bar_offset + 3] = ((cap.port >> 24) & 0xFF) as u8;
            let mut port = cap.port as u32 + cap.offset;
            for field in cap.struct_.iter() {
                let read = field.read;
                let write = field.write;
                self.addr_rw.insert(port, AddrRW{
                    read,
                    write
                });
                if cap.use_mmio {
                    assert!(false, 
                        "VirtIO device <{}> mmio capability not implemented.",
                        self.name
                    );
                } else {
                    match field.bytes {
                        4 => {
                            self.store.io_mut().map(|io| {
                                io.register_read(
                                    port, 
                                    Dev::Emulator(self.store.clone()), 
                                    |dev, addr| {
                                        dev.virtio().and_then(|vr| {
                                            Some(vr.shim_read8_on_16(addr))
                                        }).unwrap_or(0)
                                    }, IO::empty_read16, 
                                    |dev, addr| {
                                        dev.virtio().and_then(|vr| {
                                            Some(vr.read32(addr))
                                        }).unwrap_or(0)
                                    }
                                );
                                io.register_write(
                                    port, 
                                    Dev::Emulator(self.store.clone()),
                                    IO::empty_write8, 
                                    IO::empty_write16, 
                                    |dev, port, val| {
                                        dev.virtio_mut().map(|vr| {
                                            vr.write32(port, val as i32)
                                        });
                                    }
                                );
                            });
                        },
                        2 => {
                            self.store.io_mut().map(|io| {
                                io.register_read(
                                    port, 
                                    Dev::Emulator(self.store.clone()), 
                                    |dev, addr| {
                                        dev.virtio().and_then(|vr| {
                                            Some(vr.shim_read8_on_16(addr))
                                        }).unwrap_or(0)
                                    }, 
                                    |dev, addr| {
                                        dev.virtio().and_then(|vr| {
                                            Some(vr.read32(addr) as u16)
                                        }).unwrap_or(0)
                                    },
                                    IO::empty_read32
                                );
                                io.register_write(
                                    port, 
                                    Dev::Emulator(self.store.clone()),
                                    IO::empty_write8,  
                                    |dev, port, val| {
                                        dev.virtio_mut().map(|vr| {
                                            vr.write32(port, val as i32)
                                        });
                                    },
                                    IO::empty_write32,
                                );
                            });
                        },
                        1 => {
                            self.store.io_mut().map(|io| {
                                io.register_read(
                                    port, 
                                    Dev::Emulator(self.store.clone()), 
                                    |dev, addr| {
                                        dev.virtio().and_then(|vr| {
                                            Some(vr.read32(addr) as u8)
                                        }).unwrap_or(0)
                                    }, 
                                    IO::empty_read16,
                                    IO::empty_read32
                                );
                                io.register_write(
                                    port, 
                                    Dev::Emulator(self.store.clone()),
                                    |dev, port, val| {
                                        dev.virtio_mut().map(|vr| {
                                            vr.write32(port, val as i32)
                                        });
                                    },
                                    IO::empty_write16, 
                                    IO::empty_write32,
                                );
                            });
                        }
                        _ => {
                            assert!(false,
                                "VirtIO device <{}> invalid capability field width of {} bytes",
                                self.name,
                                field.bytes
                            );
                        },
                    }
                }
                port += field.bytes as u32;
            }
        }

        let cap_len = (VIRTIO_PCI_CAP_LENGTH + 4) as usize;
        let cap_next = cap_next as usize;
        assert!(
            cap_next + cap_len <= 256,
            "VirtIO device<{}> can't fit all capabilities into 256byte configspace",
            self.name
        );
        pci_space[cap_next] = VIRTIO_PCI_CAP_VENDOR;
        pci_space[cap_next + 1] = 0; // cap next (null terminator)
        pci_space[cap_next + 2] = cap_len as u8;
        pci_space[cap_next + 3] = VIRTIO_PCI_CAP_PCI_CFG; // cap type
        pci_space[cap_next + 4] = 0; // bar (written by device)
        pci_space[cap_next + 5] = 0; // Padding.
        pci_space[cap_next + 6] = 0; // Padding.
        pci_space[cap_next + 7] = 0; // Padding.
    
        // Remaining fields are configured by driver when needed.
    
        // offset
        pci_space[cap_next + 8] = 0;
        pci_space[cap_next + 9] = 0;
        pci_space[cap_next + 10] = 0;
        pci_space[cap_next + 11] = 0;
    
        // bar size
        pci_space[cap_next + 12] = 0;
        pci_space[cap_next + 13] = 0;
        pci_space[cap_next + 14] = 0;
        pci_space[cap_next + 15] = 0;
    
        // cfg_data
        pci_space[cap_next + 16] = 0;
        pci_space[cap_next + 17] = 0;
        pci_space[cap_next + 18] = 0;
        pci_space[cap_next + 19] = 0;

        self.pci_bars = Some(pci_bars)
    }

    fn shim_read8_on_16(&self, addr: u32) -> u8 {
        self.addr_rw.get(&addr).map_or(0, |rw| 
            ((rw.read)(self.store.clone()) & 0xFF) as u8
        )
    }

    fn read32(&self, addr: u32) -> u32 {
        self.addr_rw.get(&addr).map_or(0, |rw| 
            (rw.read)(self.store.clone()) as u32
        )
    }

    fn write32(&mut self, addr: u32, val: i32) {
        self.addr_rw.get_mut(&addr).map(|rw| 
            (rw.write)(self.store.clone(), val)
        );
    }

    pub fn init(&mut self) {
        let mut capabilities = Vec::new();
        capabilities.push(self.create_common_capability());
        capabilities.push(self.create_notification_capability());
        capabilities.push(self.create_isr_capability());
        let device_specific = self.options.device_specific.take();
        device_specific.map(|options| {
            capabilities.push(self.create_device_specific_capability(options));
        });
        self.init_capabilities(capabilities);
        let pci_space = self.pci_space.take().unwrap();
        let pci_bars = self.pci_bars.take()
            .map(|v| 
                v.into_iter()
                    .map(|b| Some(b))
                    .collect::<Vec<_>>()
            ).unwrap();
        let pci_dev = GenericPCIDevice::new(
            self.pci_id,
            pci_space,
            pci_bars,
            &self.name,
        );
        self.store.pci_mut().map(|pci| {
            pci.register_device(pci_dev);
        });
        self.reset();
    }

    fn needs_reset(&mut self) {
        dbg_log!(LOG::VIRTIO, 
            "Device<{}> experienced error - requires reset",
            self.name
        );
        self.device_status |= VIRTIO_STATUS_DEVICE_NEEDS_RESET as i32;

        if self.device_status & (VIRTIO_STATUS_DRIVER_OK as i32) > 0 {
            self.notify_config_changes();
        }
    }
}



struct VirtQueueBufferChain {
    store: StoreT,
    head_idx: usize,

}