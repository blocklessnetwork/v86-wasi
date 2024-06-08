#![allow(dead_code)]
use crate::{filesystem::{EPERM, FS, S_IFDIR}, virtio::{
        VirtIO, VirtIOCapabilityInfoStruct, VirtIOCommonCapabilityOptions, 
        VirtIODeviceSpecificCapabilityOptions, VirtIOISRCapabilityOptions, 
        VirtIONotificationCapabilityOptions, VirtIOptions, VirtQueue, 
        VirtQueueBufferChain, VirtQueueOptions
    }, BufferHodler, ContextTrait, MarVal, Marshall, State, StoreT, LOG,
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

pub const P9_LOCK_SUCCESS: u8 = 0;
pub const P9_LOCK_BLOCKED: u8 = 1;
pub const P9_LOCK_ERROR: u8 = 2;
pub const P9_LOCK_GRACE: u8 = 3;

const FID_INODE: u8 = 1;

struct Fid {
    inodeid: u32,
    type_: u8,
    uid: u32,
    dbg_name: String,
}

pub struct Virtio9p {
    store: StoreT,
    fs: FS,
    configspace_tagname: [u8; 6],
    configspace_taglen: u8,
    version: &'static str,
    fids: Vec<Fid>,
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
    pub fn new(store: StoreT, fs: FS) ->  Virtio9p {
        Virtio9p {
            fs,
            store,
            fids: Vec::new(),
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
        let mut buffer = vec![0u8; bufchain.length_readable as usize];
        bufchain.get_next_blob(&mut buffer);
        let mut state = State { offset:0 };
        let header = Marshall::unmarshall(&["w", "b", "h"], &buffer, &mut state);
        let mut size = header[0].as_u32().unwrap();
        let id = header[1].as_u8().unwrap();
        let tag = header[1].as_u16().unwrap();
        match id {
            8 => { //statfs
                size = self.fs.get_total_size();
                let space = self.fs.get_space();
                let mut req = vec![MarVal::U32(0x01021997), MarVal::U32(self.blocksize)];
                let req2 = ((space as f64)/(self.blocksize as f64)).floor() as u32;
                req.push(MarVal::U32(req2));
                let req3 = req2 - ((size as f64)/ (self.blocksize as f64).floor()) as u32;
                req.push(MarVal::U32(req3));
                req.push(MarVal::U32(req3));
                let node_count = self.fs.count_used_inodes() as u32;
                req.push(MarVal::U32(node_count));
                req.push(MarVal::U32(node_count));
                req.push(MarVal::U32(0));
                req.push(MarVal::U32(256));
                size = Marshall::marshall(&["w", "w", "d", "d", "d", "d", "d", "d", "w"], &req, BufferHodler::new(&mut self.reply_buffer, 0));
                self.build_reply(id, tag, size);
                self.send_reply(bufchain)
            }
            112|12 => {
                let req = Marshall::unmarshall(&["w", "w"], &buffer, &mut state);
                let fid = req[0].as_u32().unwrap() as usize;
                let mode = req[1].as_u32().unwrap();
                dbg_log!(LOG::P9, "[open] fid={fid}, mode={mode}");
                let idx = self.fids[fid].inodeid;
                let inodeid = idx;
                let qid = self.fs.get_inode(idx as _).map(|inode| inode.qid).unwrap();
                dbg_log!(LOG::P9, "file open {}", self.fids[fid].dbg_name);
                let ret = self.fs.open_inode(idx, mode);
                let store = self.store.clone();
                self.fs.add_event(self.fids[fid].inodeid, move ||  {
                    store.virtio9p_mut().map(|v9p| {
                        dbg_log!(LOG::P9, "file opened {} tag: {tag}", v9p.fids[fid].dbg_name);
                        let req = &[MarVal::QID(qid), MarVal::U32(v9p.msize - 24)];
                        Marshall::marshall(&["Q", "w"], req, BufferHodler::new(&mut v9p.reply_buffer, 7));
                        v9p.build_reply(id, tag, 13+4);
                        v9p.send_reply(bufchain);
                    });
                });
            }
            70 => { // link
                let req = Marshall::unmarshall(&["w", "w", "s"], &buffer, &mut state);
                let dfid = req[0].as_u32().unwrap();
                let fid = req[1].as_u32().unwrap();
                let name = req[2].as_str().unwrap();
                dbg_log!(LOG::P9, "[link] dfid= {dfid} , name={name}");
                let Fid {inodeid, ..} = self.fids[dfid as usize];
                let ret = self.fs.link(inodeid as _, inodeid as _, &name);
                if ret < 0 {
                    let mut error_message;
                    if ret == -EPERM {
                        error_message = "Operation not permitted".to_string();
                    } else {
                        error_message = format!("Unknown error: {}", -ret);
                    }
                    self.send_error(tag, -ret as _);
                    self.send_reply(bufchain);
                }
            }
            16 => { // symlink
                let req = Marshall::unmarshall(&["w", "s", "s", "w"], &buffer, &mut state);
                let fid = req[0].as_u32().unwrap();
                let name = req[1].as_str().unwrap();
                let symgt = req[2].as_str().unwrap();
                let gid = req[3].as_u32().unwrap();
                let Fid {inodeid, ..} = self.fids[fid as usize];
                dbg_log!(LOG::P9, "[symlink] fid={fid}, name={name}, symgt={symgt}, gid={gid}");
                let idx = self.fs.create_symlink(&name, inodeid as _, symgt);
                self.fs.inode_mut(idx as _, |inode| {
                    inode.uid = self.fids[fid as usize].uid;
                    inode.gid = gid;
                });
                Marshall::marshall(&["Q"], &[MarVal::U32(gid)], BufferHodler::new(&mut self.reply_buffer, 7));
                self.build_reply(id, tag, 13);
                self.send_reply(bufchain);
            }
            18 => { // mknod
                let req = Marshall::unmarshall(&["w", "s", "w", "w", "w", "w"], &buffer, &mut state);
                let fid = req[0].as_u32().unwrap();
                let name = req[1].as_str().unwrap();
                let mode = req[2].as_u32().unwrap();
                let major = req[3].as_u32().unwrap();
                let minor = req[4].as_u32().unwrap();
                let gid = req[5].as_u32().unwrap();
                dbg_log!(LOG::P9, "[symlink] fid={fid}, name={name}, major={major}, minor={minor}");
                let idx = self.fs.create_node(&name, self.fids[fid as usize].inodeid as _, major, minor);
                self.fs.inode_mut(idx as _, |inode| {
                    inode.mode = mode as _;
                    inode.uid = self.fids[fid as usize].uid;
                    inode.gid = gid;
                });
                Marshall::marshall(&["Q"], &[MarVal::U32(gid)], BufferHodler::new(&mut self.reply_buffer, 7));
                self.build_reply(id, tag, 13);
                self.send_reply(bufchain);
            }
            22 => {
                let req = Marshall::unmarshall(&["w"], &buffer, &mut state);
                let fid = req[0].as_u32().unwrap();
                let Fid{
                    inodeid,
                    ref dbg_name,
                    ..
                }  = self.fids[fid as usize];
                let symlink = self.fs.get_inode(inodeid as _).map(|inode| {
                    &inode.symlink
                }).unwrap();
                dbg_log!(LOG::P9, "[readlink] fid={fid}, name={dbg_name}, target={symlink}");
                size = Marshall::marshall(&["s"], &[MarVal::String(symlink.clone())], BufferHodler::new(&mut self.reply_buffer, 7));
                self.build_reply(id, tag, size);
                self.send_reply(bufchain);
            }
            72 => {// tmkdir 
                let req = Marshall::unmarshall(&["w", "s", "w", "w"], &buffer, &mut state);
                let fid = req[0].as_u32().unwrap();
                let name = req[1].as_str().unwrap();
                let mode = req[2].as_u32().unwrap();
                let gid = req[3].as_u32().unwrap();
                dbg_log!(LOG::P9, "[mkdir] fid={fid}, name={name}, mode={mode}, gid={gid}");
                let idx = self.fs.create_directory(name, self.fids[fid as usize].inodeid as _);
                let mut qid = None;
                self.fs.inode_mut(idx as _, |inode| {
                    inode.mode = mode |S_IFDIR;
                    inode.uid = self.fids[fid as usize].uid;
                    inode.gid = gid;
                    qid = Some(inode.qid);
                });
                Marshall::marshall(&["Q"], &[MarVal::QID(qid.unwrap())], BufferHodler::new(&mut self.reply_buffer, 7));
                self.build_reply(id, tag, 13);
                self.send_reply(bufchain);
            }
            14 => {// tlcreate
                let req = Marshall::unmarshall(&["w", "s", "w", "w", "w"], &buffer, &mut state);
                let fid = req[0].as_u32().unwrap();
                let name = req[1].as_str().unwrap();
                let flags = req[2].as_u32().unwrap();
                let mode = req[3].as_u32().unwrap();
                let gid = req[4].as_u32().unwrap();
                //self.store.bus_mut().map(|bus| bus.send("9p-create", data));
                dbg_log!(LOG::P9, "[create] fid={fid}, name={name}, flags={flags}, mode={mode}, gid={gid}");
                let idx = self.fs.create_file(&name, self.fids[fid as usize].inodeid as _);
                let uid = {
                    let fid = &mut self.fids[fid as usize];
                    fid.inodeid = idx as _;
                    fid.type_ = FID_INODE;
                    fid.dbg_name = name.to_string();
                    fid.uid
                };
                let mut qid = None;
                self.fs.inode_mut(idx as _, |inode| {
                    inode.uid = uid;
                    inode.gid = gid;
                    inode.mode = mode;
                    qid = Some(inode.qid);
                });

                Marshall::marshall(
                    &["Q", "w"],
                    &[MarVal::QID(qid.unwrap()), MarVal::U32(self.msize - 24)], 
                    BufferHodler::new(&mut self.reply_buffer, 7)
                );
                self.build_reply(id, tag, 13+4);
                self.send_reply(bufchain);
            } 
            52 => {// lock
                let req = Marshall::unmarshall(&["w", "b", "w", "d", "d", "w", "s"], &buffer, &mut state);
                let fid = req[0].as_u32().unwrap();
                let type_ = req[1].as_u8().unwrap();
                let flags = req[2].as_u32().unwrap();
                let start = req[3].as_u32().unwrap();
                let mut lock_length = req[4].as_u32().unwrap();
                if lock_length == 0 {
                    lock_length = u32::MAX; //Infinity
                }
                let proc_id = req[5].as_u32().unwrap();
                let client_id = req[6].as_str().unwrap();
                let lock_request = self.fs.describe_lock(type_, start, lock_length, proc_id, client_id);
                dbg_log!(LOG::P9, "[lock] fid={fid}, type={type_}, start={start}, length={lock_length}, proc_id={proc_id}");


            }
            _ => {
                panic!("Error in Virtio9p: Unknown id {id} received");
            }
        }
    }

    fn build_reply(&mut self,id: u8, tag: u16, payloadsize: u32) {
        assert!(payloadsize >= 0, "9P: Negative payload size");
        Marshall::marshall(
            &["w", "b", "h"], 
            &[MarVal::U32(payloadsize+7), MarVal::U8(id+1), MarVal::U16(tag)], BufferHodler::new(&mut self.reply_buffer, 0)
        );
        if payloadsize+7 >= self.reply_buffer.len() as _ {
            dbg_log!(LOG::P9, "Error in 9p: payloadsize exceeds maximum length");
        }
        self.reply_buffer_size = payloadsize + 7;
    }

    fn send_error(&mut self, tag: u16, errorcode: u32) {
        let size = Marshall::marshall(&["w"], &[MarVal::U32(errorcode)], BufferHodler::new(&mut self.reply_buffer, 7));
        self.build_reply(6, tag, size);
    }

    fn send_reply(&mut self, mut bufchain: VirtQueueBufferChain) {
        assert!(self.reply_buffer_size >= 0, "9P: Negative replybuffersize");
        let mut vec = Vec::new();
        vec.copy_from_slice(&self.reply_buffer[0..self.reply_buffer_size as usize]);
        bufchain.set_next_blob(vec);
        self.store.virtio_mut().and_then(|v| {
            v.queue_nth_mut(0)
        }).map(|q| q.push_reply(bufchain));
    }
    
}
