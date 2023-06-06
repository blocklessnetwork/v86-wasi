use std::collections::HashMap;

use crate::StoreT;

const STATUS_INVALID: i8 = -0x1;
const STATUS_OK: i8 = 0x0;
const STATUS_ON_STORAGE: i8 = 0x2;
const STATUS_UNLINKED: i8 = 0x4;
const STATUS_FORWARDING: i8 = 0x5;

const S_IRWXUGO: u16 = 0x1FF;
const S_IFMT: u16 = 0xF000;
const S_IFSOCK: u16 = 0xC000;
const S_IFLNK: u16 = 0xA000;
const S_IFREG: u16 = 0x8000;
const S_IFBLK: u16 = 0x6000;
const S_IFDIR: u16 = 0x4000;
const S_IFCHR: u16 = 0x2000;

const LOCK_TYPE_RDLCK: u8 = 0;
const LOCK_TYPE_WRLCK: u8 = 1;
const LOCK_TYPE_UNLCK: u8 = 2;

struct FSLockRegion {
    type_: u8,
    start: u32,
    length: u32,
    proc_id: i64,
    client_id: String,
}

impl FSLockRegion {

    fn new() -> Self {
        Self {
            type_: LOCK_TYPE_UNLCK,
            start: 0,
            length: u32::MAX,
            proc_id: -1,
            client_id: String::new(),
        }
    }

    fn conflicts_with(&self, region: &FSLockRegion) -> bool {
        if self.proc_id == region.proc_id && self.client_id == region.client_id {
            return false;
        }
        if self.type_ == LOCK_TYPE_UNLCK || region.type_ == LOCK_TYPE_UNLCK {
            return false;
        }
        if self.type_ != LOCK_TYPE_WRLCK && region.type_ != LOCK_TYPE_WRLCK {
            return false;
        }
        if self.start + self.length <= region.start {
            return false;
        }
        if region.start + region.length <= self.start {
            return false;
        } 
        return true;
    }

    fn is_alike(&self, region: &FSLockRegion) -> bool {
        return region.proc_id == self.proc_id &&
            region.client_id == self.client_id &&
            region.type_ == self.type_;
    }

    fn may_merge_after(&self, region: &FSLockRegion) -> bool {
        return self.is_alike(region) && region.start + region.length == self.start;
    }

}

struct QID {
    type_: u8, 
    version: u8, 
    path: i32, 
}

type ForeignIdType = i64;
type MountIdType = i64;

struct Inode {
    uid: u32,
    gid: u32,
    fid: u32,
    size: u32,
    ctime: u32,
    status: i8,
    atime: u32,
    mtime: u32,
    major: u32,
    minor: u32,
    symlink: String,
    mode: u16,
    qid: QID,
    caps: Vec<u8>,
    nlinks: u32,
    sha256sum: String,
    mount_id: MountIdType,
    foreign_id: ForeignIdType,
    direntries: HashMap<String, i64>,
}

impl Inode {
    fn new(qidnumber: i32) -> Self {
        Self {
            direntries: HashMap::new(),
            status: 0,
            size: 0x0,
            uid: 0x0,
            gid: 0x0,
            fid: 0,
            ctime: 0,
            atime: 0,
            mtime: 0,
            major: 0x0,
            minor: 0x0,
            symlink: String::new(),
            mode: 0x01ED,
            qid: QID { 
                type_: 0, 
                version: 0 ,
                path: qidnumber 
            },
            caps: Vec::new(),
            nlinks: 0,
            sha256sum: String::new(),
            mount_id: -1,
            foreign_id: -1,
        }
    }
}

struct FSMountInfo {
    backtrack: HashMap<ForeignIdType, usize>,
    fs: FS,
}

struct FS {
    store: StoreT,
    inodes: Vec<Inode>,
    mounts: Vec<FSMountInfo>,
}


impl FS {
    fn is_forwarder(&self, inode: &Inode) -> bool {
        return inode.status == STATUS_FORWARDING;
    }

    fn follow_fs(&self, inode: &Inode) -> &FS {
        let mount = self.mounts.get(inode.mount_id as usize);
        assert!(mount.is_some(), 
            "Filesystem follow_fs: inode<id={}> should point to valid mounted FS",
            inode.fid
        );
        let mount = mount.unwrap();
        assert!(self.is_forwarder(inode),
            "Filesystem follow_fs: inode should be a forwarding inode");

        &mount.fs
    }

    fn GetInode(&self, idx: usize) -> &Inode {
        assert!(idx >= 0 && idx < self.inodes.len(), "Filesystem GetInode: out of range idx:{idx}");

        let inode = self.inodes.get(idx).unwrap();
        if self.is_forwarder(inode) {
            return self.follow_fs(inode).GetInode(inode.foreign_id as usize);
        }

        return inode;
    }

    fn is_directory(&self, idx: usize) -> bool {
        let inode = self.inodes.get(idx).unwrap();
        if self.is_forwarder(inode) {
            return self.follow_fs(inode).is_directory(inode.foreign_id as usize);
        }
        return (inode.mode & S_IFMT) == S_IFDIR;
    }

    fn is_empty(&self, idx: usize) -> bool {
        let inode = self.inodes.get(idx).unwrap();
        if self.is_forwarder(inode) {
            return self.follow_fs(inode).is_directory(inode.foreign_id as usize);
        }
        for name in inode.direntries.keys() {
            if name != "." && name != ".." {
                return false;
            }
        }
        return true;
    }

    fn get_children(&self, idx: usize) -> Vec<String> {
        assert!(self.is_directory(idx), "Filesystem: cannot get children of non-directory inode");
        let inode = self.inodes.get(idx).unwrap();
        if self.is_forwarder(inode) {
            return self.follow_fs(inode).get_children(inode.foreign_id as usize);
        }
        inode.direntries.keys().filter_map(|name| {
            if name != "." && name != ".." {
                Some(name.clone())
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }
}
