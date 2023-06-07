use std::collections::HashMap;
use std::time::{self, Duration};

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
    path: u64, 
}

type ForeignIdType = i64;
type MountIdType = i64;

struct Inode {
    uid: u32,
    gid: u32,
    fid: u64,
    size: u32,
    ctime: u64,
    status: i8,
    atime: u64,
    mtime: u64,
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
    fn new(qidnumber: u64) -> Self {
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
    qidcounter: Qidcounter,
}

struct Qidcounter {
    last_qidnumber: u64,
}


impl FS {
    fn is_forwarder(inode: &Inode) -> bool {
        return inode.status == STATUS_FORWARDING;
    }

    fn follow_fs(&mut self, inode: &Inode) -> &mut FS {
        let mount = self.mounts.get_mut(inode.mount_id as usize);
        assert!(mount.is_some(), 
            "Filesystem follow_fs: inode<id={}> should point to valid mounted FS",
            inode.fid
        );
        let mount = mount.unwrap();
        assert!(Self::is_forwarder(inode),
            "Filesystem follow_fs: inode should be a forwarding inode");

        &mut mount.fs
    }

    fn GetInode(&mut self, idx: usize) -> &Inode {
        assert!(idx >= 0 && idx < self.inodes.len(), "Filesystem GetInode: out of range idx:{idx}");

        let inode = self.inodes.get(idx).unwrap();
        if Self::is_forwarder(inode) {
            return self.follow_fs(inode).GetInode(inode.foreign_id as usize);
        }

        return inode;
    }

    fn is_directory(&self, idx: usize) -> bool {
        let inode = self.inodes.get(idx).unwrap();
        if Self::is_forwarder(inode) {
            return self.follow_fs(inode).is_directory(inode.foreign_id as usize);
        }
        return (inode.mode & S_IFMT) == S_IFDIR;
    }

    fn is_empty(&self, idx: usize) -> bool {
        let inode = self.inodes.get(idx).unwrap();
        if Self::is_forwarder(inode) {
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
        if Self::is_forwarder(inode) {
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

    fn should_be_linked(inode: &Inode) -> bool {
        // Note: Non-root forwarder inode could still have a non-forwarder parent, so don't use
        // parent inode to check.
        return !Self::is_forwarder(inode) || inode.foreign_id == 0;
    }

    fn get_parent(&mut self, idx: usize) -> i64 {
        assert!(self.is_directory(idx), "Filesystem: cannot get parent of non-directory inode");

        let inode = self.inodes.get(idx).unwrap();
        if Self::should_be_linked(inode) {
            *inode.direntries.get("..").unwrap()
        } else {
            let foreign_dirid = self.follow_fs(inode).get_parent(inode.foreign_id as usize);
            assert!(foreign_dirid != -1, "Filesystem: should not have invalid parent ids");
            self.get_forwarder(inode.mount_id, foreign_dirid)  as i64
        }
    }

    fn create_inode(&mut self) -> Inode {
        //console.log("CreateInode", Error().stack);
        let dur: Duration = time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .unwrap();
        let now = dur.as_millis() as u64;
        self.qidcounter.last_qidnumber += 1;
        let mut inode = Inode::new(self.qidcounter.last_qidnumber);
        inode.mtime = now;
        inode.ctime = now;
        inode.atime = now;
        return inode;
    }

    fn create_forwarder(&mut self, mount_id: MountIdType, foreign_id: ForeignIdType) -> u64 {
        let mut inode = self.create_inode();

        let idx = self.inodes.len();
        
        inode.fid = idx as u64;
        self.inodes.push(inode);

        self.set_forwarder(idx, mount_id, foreign_id);
        return idx as u64;
    }

    fn get_forwarder(&mut self, mount_id: MountIdType, foreign_id: ForeignIdType) -> u64 {
        let mount = self.mounts.get(mount_id as usize);
        assert!(foreign_id >= 0, "Filesystem get_forwarder: invalid foreign_id: {foreign_id}");
        assert!(mount.is_some(), "Filesystem get_forwarder: invalid mount number: {mount_id}");
        let mount = mount.unwrap();
        let result = mount.backtrack.get(&foreign_id);

        if result.is_none() {
            // Create if not already exists.
            return self.create_forwarder(mount_id, foreign_id);
        }
        *result.unwrap() as u64
    }

    fn set_forwarder(&self, idx: usize, mount_id: MountIdType, foreign_id: ForeignIdType) {
        let inode = self.inodes.get(idx).unwrap();

        assert!(inode.nlinks == 0,
            "Filesystem: attempted to convert an inode into forwarder before unlinking the inode");
        todo!()
        // if self.is_forwarder(inode) {
        //     self.mounts[inode.mount_id as usize].backtrack.delete(inode.foreign_id);
        // }

        // inode.status = STATUS_FORWARDING;
        // inode.mount_id = mount_id;
        // inode.foreign_id = foreign_id;

        // self.mounts[mount_id].backtrack.set(foreign_id, idx);
    }
}
