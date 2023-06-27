use crate::{StoreT, 
    virtio::{
        VirtIO, 
        VirtIOptions, 
        VirtIONotificationCapabilityOptions, VirtQueue, VirtQueueBufferChain
    },
    ContextTrait
};

const ConfigSpaceName: [u8; 6] = [0x68, 0x6F, 0x73, 0x74, 0x39, 0x70];
const Virtio9pVersion: &str = "9P2000.L";
const BLOCKSIZE: u32 = 8192;
const MSIZE: u32 = 8192;


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

impl Virtio9p {
    fn new(store: StoreT) ->  Virtio9p {
        Virtio9p {
            store,
            configspace_tagname: ConfigSpaceName,
            configspace_taglen: ConfigSpaceName.len() as _,
            version: Virtio9pVersion,
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

    fn init(&mut self) {
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
                        }
                    });
                });
            }],
        };
        let options = VirtIOptions {
            name: "virtio-9p".to_string(),
            pci_id: 0x06 << 3,
            device_id: 0x1049,
            subsystem_device_id: 9,
            notification,
            common: todo!(),
            isr_status: todo!(),
            device_specific: todo!(),
        };
        let virtio = VirtIO::new(self.store.clone(), options);
    }

    fn receive_request(&mut self, mut bufchain: VirtQueueBufferChain) {
        let mut buf = vec![0u8; bufchain.length_readable as usize];
        bufchain.get_next_blob(&mut buf);
        

    }

}