#![allow(dead_code)]
use crate::{StoreT, 
    virtio::{
        VirtIO, 
        VirtIOptions, 
        VirtIONotificationCapabilityOptions, 
        VirtQueue, 
        VirtQueueBufferChain, 
        VirtIOCommonCapabilityOptions, 
        VirtQueueOptions, 
        VirtIOISRCapabilityOptions, 
        VirtIODeviceSpecificCapabilityOptions, 
        VirtIOCapabilityInfoStruct
    },
    ContextTrait
};

const CONFIGSPACENAME: [u8; 6] = [0x68, 0x6F, 0x73, 0x74, 0x39, 0x70];
const VIRTIO9PVERSION: &str = "9P2000.L";
const BLOCKSIZE: u32 = 8192;
const MSIZE: u32 = 8192;

const VIRTIO_9P_F_MOUNT_TAG: u8 = 0;
const VIRTIO_9P_MAX_TAGLEN: u8 = 254;

const VIRTIO_F_RING_INDIRECT_DESC: u8 = 28;
const VIRTIO_F_RING_EVENT_IDX: u8 = 29;
const VIRTIO_F_VERSION_1: u8 = 32;

pub struct Virtio9p {
    store: StoreT,
    configspace_tagname: [u8; 6],
    configspace_taglen: u8,
    version: &'static str,
    blocksize: u32,
    msize: u32,
    virtio: Option<VirtIO>,
    reply_buffer: Vec<u8>,
    reply_buffer_size: u32,
}

macro_rules! struct_push {
    ($struct:ident, $i:literal) => {
        $struct.push(VirtIOCapabilityInfoStruct {
            bytes: 1,
            name: format!("mount tag name {}", $i),
            read: |store: StoreT| -> i32 {
                store.virtio9p().map_or(0, |v9p| {
                    if $i < v9p.configspace_tagname.len() {
                        v9p.configspace_tagname[$i] as _
                    } else {
                        0
                    }
                })
            },
            write: |_, _| {},
        });
    };
}

impl Virtio9p {
    pub fn new(store: StoreT) ->  Virtio9p {
        Virtio9p {
            store,
            configspace_tagname: CONFIGSPACENAME,
            configspace_taglen: CONFIGSPACENAME.len() as _,
            version: VIRTIO9PVERSION,
            blocksize: BLOCKSIZE,
            msize: BLOCKSIZE,
            virtio: None,
            reply_buffer: Vec::new(),
            reply_buffer_size: 0,
        }
    }

    #[inline]
    pub fn virtio(&self) -> Option<&VirtIO> {
        self.virtio.as_ref()
    }

    #[inline]
    pub fn virtio_mut(&mut self) -> Option<&mut VirtIO> {
        self.virtio.as_mut()
    }

    pub fn init(&mut self) {
        let notification = VirtIONotificationCapabilityOptions {
            initial_port: 0xA900,
            single_handler: false,
            handlers: vec![|store: StoreT, queue_id: i32| {
                if queue_id != 0 {
                    assert!(false, "Virtio9P Notified for non-existent queue: {queue_id} (expected queue_id of 0)");
                    return;
                }
                store.virtio_mut().map(|v| {
                    v.queue_nth_mut(0).map(|virtqueue: &mut VirtQueue| {
                        while virtqueue.has_request() {
                            let bufchain = virtqueue.pop_request();
                            store.virtio9p_mut().map(|v9p| {
                                v9p.receive_request(bufchain);
                            });
                            virtqueue.notify_me_after(0);
                            // Don't flush replies here: async replies are not completed yet.
                        }
                    });
                });
            }],
        };
        let common = VirtIOCommonCapabilityOptions {
            initial_port: 0xA800,
            queues: vec![VirtQueueOptions{ 
                size_supported: 32, 
                notify_offset: 0 
            }],
            features: vec![
                VIRTIO_9P_F_MOUNT_TAG,
                VIRTIO_F_VERSION_1,
                VIRTIO_F_RING_EVENT_IDX,
                VIRTIO_F_RING_INDIRECT_DESC,
            ],
            on_driver_ok: |_| {},
        };
        let mut device_specific_struct = vec![VirtIOCapabilityInfoStruct { 
            bytes: 2, 
            name: "mount tag length".to_string(), 
            read: |store: StoreT| {
                store
                    .virtio9p()
                    .map_or(0, |v9p| v9p.configspace_taglen as _)
            }, 
            write: |_store, _v| {}, 
        }];
        struct_push!(device_specific_struct, 0);
        struct_push!(device_specific_struct, 1);
        struct_push!(device_specific_struct, 2);
        struct_push!(device_specific_struct, 3);
        struct_push!(device_specific_struct, 4);
        struct_push!(device_specific_struct, 5);
        (6..254)
            .into_iter()
            .for_each(|i| {
                device_specific_struct.push(
                    VirtIOCapabilityInfoStruct {
                        bytes: 1,
                        name: format!("mount tag name {i}"),
                        read: |_store| -> i32 {
                            0
                        },
                        write: |_, _| {},
                    }
                );
            });
        let device_specific = VirtIODeviceSpecificCapabilityOptions {
            initial_port: 0xA600,
            struct_: device_specific_struct,
        };
        let options = VirtIOptions {
            name: "virtio-9p".to_string(),
            pci_id: 0x06 << 3,
            device_id: 0x1049,
            subsystem_device_id: 9,
            notification,
            common,
            isr_status: VirtIOISRCapabilityOptions {
                initial_port: 0xA700,
            },
            device_specific: Some(device_specific),
        };
        let virtio = VirtIO::new(self.store.clone(), options);
        self.virtio = Some(virtio);
        self.virtio.as_mut().map(|virtio| {
            virtio.init();
        });
    }

    fn receive_request(&mut self, mut bufchain: VirtQueueBufferChain) {
        let mut buf = vec![0u8; bufchain.length_readable as usize];
        bufchain.get_next_blob(&mut buf);
        
    }

}