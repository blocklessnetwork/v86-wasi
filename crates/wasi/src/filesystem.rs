use std::collections::{HashMap, VecDeque};
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

const EPERM: i8 = 1;
const ENOTEMPTY: i8 = 39;  /* Directory not empty */

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
    used_size: usize,
    total_size: usize,
    inodes: Vec<Inode>,
    mounts: Vec<FSMountInfo>,
    qidcounter: Qidcounter,
}

struct Qidcounter {
    last_qidnumber: u64,
}

struct PathInfo {
    id: i64,
    parentid: i64,
    name: String,
    forward_path: Option<String>,
}

struct RecursiveInfo {
    parentid: i64,
    name: String,
}

impl FS {
    fn is_forwarder(inode: &Inode) -> bool {
        return inode.status == STATUS_FORWARDING;
    }

    fn follow_fs(&self, inode: &Inode) -> &FS {
        let mount = self.mounts.get(inode.mount_id as usize);
        assert!(mount.is_some(), 
            "Filesystem follow_fs: inode<id={}> should point to valid mounted FS",
            inode.fid
        );
        let mount = mount.unwrap();
        assert!(Self::is_forwarder(inode),
            "Filesystem follow_fs: inode should be a forwarding inode");

        &mount.fs
    }

    fn follow_fs_mut(&mut self, idx: usize) -> &mut FS {
        let inode = self.inodes.get(idx).unwrap();
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

    fn get_inode(&self, idx: usize) -> &Inode {
        assert!(idx >= 0 && idx < self.inodes.len(), "Filesystem GetInode: out of range idx:{idx}");

        let inode = self.inodes.get(idx).unwrap();
        if Self::is_forwarder(inode) {
            return self.follow_fs(inode).get_inode(inode.foreign_id as usize);
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
            let foreign_id = inode.foreign_id;
            let mount_id = inode.mount_id;
            let foreign_dirid = self.follow_fs_mut(idx).get_parent(foreign_id as usize);
            assert!(foreign_dirid != -1, "Filesystem: should not have invalid parent ids");
            self.get_forwarder(mount_id, foreign_dirid)  as i64
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

    fn set_forwarder(&mut self, idx: usize, mount_id: MountIdType, foreign_id: ForeignIdType) {
        let inode = self.inodes.get_mut(idx).unwrap();

        assert!(inode.nlinks == 0,
            "Filesystem: attempted to convert an inode into forwarder before unlinking the inode");
        if Self::is_forwarder(inode) {
            self.mounts[inode.mount_id as usize].backtrack.remove(&inode.foreign_id);
        }

        inode.status = STATUS_FORWARDING;
        inode.mount_id = mount_id;
        inode.foreign_id = foreign_id;

        self.mounts.get_mut(mount_id as usize).map(|m: &mut FSMountInfo| {
            m.backtrack.insert(foreign_id, idx);
        });
    }

    fn search(&mut self, parentid: i64, name: &str) -> i64 {
        let parent_inode = self.inodes.get(parentid as usize).unwrap();
        if Self::is_forwarder(parent_inode) {
            let foreign_parentid = parent_inode.foreign_id;
            let p_mount_id = parent_inode.mount_id;
            let foreign_id = self.follow_fs_mut(parentid as usize).search(foreign_parentid, name);
            if foreign_id == -1 {
                return -1;
            }
            return self.get_forwarder(p_mount_id, foreign_id) as _;
        }
    
        let childid = parent_inode.direntries.get(name);
        childid.map(|i| *i).unwrap_or(-1)
    }

    fn count_used_inodes(&self) -> usize {
        let mut count = self.inodes.len();
        self.mounts.iter().for_each(|m| {
            count += m.fs.count_used_inodes();
            count -=  m.backtrack.len();
        });
        return count;
    }

    fn count_free_inodes(&self) -> usize {
        let mut count = 1024 * 1024;
        self.mounts.iter().for_each(|m| {
            count += m.fs.count_free_inodes();
        });
        return count;
    }


    fn get_total_size(&self) -> usize {
        let mut count = self.used_size;
        self.mounts.iter().for_each(|m| {
            count += m.fs.get_total_size();
        });
        return count;
    }

    fn get_space(&self) -> usize {
        let mut count = self.total_size;
        self.mounts.iter().for_each(|m| {
            count += m.fs.get_space();
        });
        return count;
    }
    

    fn search_path(&mut self, path: &str) -> PathInfo {
        let path = path.replace("//", "/");
        let mut walk: VecDeque<&str> = path.split("/").collect();
        if walk.len() > 0 && walk[walk.len() - 1].len() == 0 {
            walk.pop_back();
        }
        if walk.len() > 0 && walk[0].len() == 0 {
            walk.pop_front();
        }
        let n = walk.len();
        let mut parentid = -1;
        let mut id = 0;
        let mut forward_path = None;
        for i in 0..n {
            parentid = id;
            id = self.search(parentid, walk[i]);
            if forward_path.is_none() && Self::is_forwarder(&self.inodes[parentid as usize]) {
                let mut s = String::new();
                //join the walk[i..] to string
                let walk_iter = walk.iter().skip(i);
                let n = walk_iter.len();
                for (i, item) in walk_iter.enumerate() {
                    s.push_str(item);
                    if i < n - 1 {
                        s.push('/');
                    }
                }
                forward_path = Some(format!("/{}", s));
            }
            if id == -1 {
                if i < n-1 { 
                     // one name of the path cannot be found
                    return PathInfo{
                        id: -1, 
                        parentid: -1, 
                        name: walk[i].to_string(), 
                        forward_path 
                    };
                }
                // the last element in the path does not exist, but the parent
                return PathInfo {
                    id: -1, 
                    parentid: parentid, 
                    name: walk[i].to_string(), 
                    forward_path
                }; 
            }
        }
        return PathInfo{
            id: id, 
            parentid: parentid, 
            name: walk[walk.len() - 1].to_string(), 
            forward_path
        };
    }

    fn get_recursive_list(&mut self, dirid: i64, list: &mut Vec<RecursiveInfo>) {
        let inode = &self.inodes[dirid as usize];
        if Self::is_forwarder(inode) { 
            let foreign_fs = self.follow_fs(inode);
            let foreign_dirid = inode.foreign_id;
            let mount_id = inode.mount_id;

            let foreign_start = list.len();
            foreign_fs.get_recursive_list(foreign_dirid, list);
            for i in foreign_start..list.len() {
                list[i].parentid = self.get_forwarder(mount_id, list[i].parentid) as _;
            }
            return;
        }
        inode.direntries.iter().for_each(|(name, id)| {
            if name != "." && name != ".." {
                list.push(RecursiveInfo { parentid: dirid, name: name.to_string() });
                if self.is_directory(*id as usize) {
                    self.get_recursive_list(*id, list);
                }
            }
        });
    }

    fn unlink_from_dir(&mut self, parentid: i64, name: &str) {
        let idx = self.search(parentid, name);
        let inode: &mut Inode = &mut self.inodes[idx as _];
        let parent_inode: &mut Inode = &mut self.inodes[parentid as _];
    
        assert!(!Self::is_forwarder(parent_inode), "Filesystem: Can't unlink from forwarders");
        assert!(self.is_directory(parentid as _), "Filesystem: Can't unlink from non-directories");
    
        let exists = parent_inode.direntries.remove(name);
        if exists.is_none()  {
            assert!(false, "Filesystem: Can't unlink non-existent file: {name}");
            return;
        }
    
        inode.nlinks -= 1;
    
        if self.is_directory(idx as _) {
            assert!(
                inode.direntries.get("..").map(|d| *d == parentid).unwrap_or(false),
                "Filesystem: Found directory with bad parent id"
            );
    
            inode.direntries.remove("..");
            parent_inode.nlinks -= 1;
        }
    
        assert!(inode.nlinks >= 0,
            "Filesystem: Found negative nlinks value of {}",
            inode.nlinks);
    }

    fn unlink(&mut self, parentid: i64, name: &str) -> i8 {
        if name == "." || name == ".." {
            // Also guarantees that root cannot be deleted.
            return -EPERM;
        }
        let idx = self.search(parentid, name);
        let inode = &self.inodes[idx as _];
        let parent_inode = &self.inodes[parentid as _];
        //message.Debug("Unlink " + inode.name);
    
        // forward if necessary
        if Self::is_forwarder(parent_inode) {
            assert!(Self::is_forwarder(inode), 
                "Children of forwarders should be forwarders"
            );
    
            let foreign_parentid = parent_inode.foreign_id;
            return self.follow_fs_mut(parentid as _).unlink(foreign_parentid, name);
    
            // Keep the forwarder dangling - file is still accessible.
        }
    
        if self.is_directory(idx as _) && !self.is_empty(idx as _) {
            return -ENOTEMPTY;
        }
    
        self.unlink_from_dir(parentid, name);
    
        if inode.nlinks == 0 {
            // don't delete the content. The file is still accessible
            inode.status = STATUS_UNLINKED;
            // self.notify_listeners(idx, "delet");
        }
        return 0;
    }


    fn recursive_delete(&mut self, path: &str) {
        let mut to_delete = Vec::new();
        let ids = self.search_path(path);
        if ids.id == -1 {
            return;
        } 
    
        self.get_recursive_list(ids.id, &mut to_delete);

        to_delete.iter().rev().for_each(|r| {
            let ret = self.unlink(r.parentid, &r.name);
            assert!(ret == 0, 
                "Filesystem RecursiveDelete failed at parent={} name='{}' with error code: {}",
                r.parentid,
                r.name,
                -ret,
            );
        });
    }
    
}

