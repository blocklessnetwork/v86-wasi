#![allow(dead_code)]
use crate::{StoreT, ContextTrait, log::LOG, int_log2};
use std::collections::HashSet;

const DEBUG: bool = false;

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
const VIRTIO_PCI_VENDOR_ID: u16 = 0x1AF4;
// Size (bytes) of the virtq_avail struct per queue size.
const VIRTQ_AVAIL_ENTRYSIZE: u32 = 2;
// Size (bytes) of the virtq_desc struct per queue size.
const VIRTQ_USED_ENTRYSIZE: u32 = 8;

const VIRTIO_F_VERSION_1: u8 = 32;

type NotificationCapabilityFunc = fn(StoreT, usize);

pub struct VirtIONotificationCapabilityOptions {
    initial_port: u16,
    single_handler: bool,
    handles: Vec<NotificationCapabilityFunc>,
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

struct VirtIOCapabilityInfoStruct {
    bytes: u8,
    name: String,
    read: fn(StoreT) -> i32,
    write: fn(StoreT, i32),
}

pub(crate) struct VirtIOCapabilityInfo {
    type_: u8,
    bar: u8,
    port: u16,
    use_mmio: bool,
    offset: usize,
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
}

pub struct VirtIOptions {
    name: String,
    pci_id: u16,
    device_id: u16,
    subsystem_device_id: u16,
    common: VirtIOCommonCapabilityOptions,
    notification: VirtIONotificationCapabilityOptions,
    on_driver_ok: fn(StoreT)
}

pub struct VirtIO {
    store: StoreT,
    pci_id: u16,
    device_id: u16,
    device_feature_select: i32,
    driver_feature_select: i32,
    device_feature: [u32; 4],
    driver_feature: [u32; 4],
    isr_status: i32,
    features_ok: bool,
    device_status: i32,
    config_has_changed: bool,
    config_generation: i32,
    queue_select: i32,
    name: String,
    pci_space: Vec<u8>,
    queues: Vec<VirtQueue>,
    options: VirtIOptions,
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
                    let nth = options.notification.handles.iter().nth(effective_offset);
                    assert!(nth.is_some(), 
                        "VirtIO device<{name}> every queue's notifier must exist");
                    effective_offset
                })
                .collect::<HashSet<_>>();
            options.notification.handles.iter().enumerate().for_each(|(i, h)| {
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
            pci_space,
            isr_status: 0,
            features_ok: true,
            device_status: 0,
            config_has_changed: false,
            config_generation: 0,
            options,
        }
    }

    fn create_common_capability(&self) -> Vec<VirtIOCapabilityInfo> {
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
                        (vr.options.on_driver_ok)(store.clone());
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
                    vr.queue_selected()
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
                store.virtio().and_then(|vr| {
                    vr.queue_selected()
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
                    vr.queue_selected()
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
        rs
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

    pub fn init(&mut self) {
        

    }
}