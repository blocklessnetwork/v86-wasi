#![allow(unused, dead_code)]
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::time::{self, Duration};

use crate::log::LOG;
use crate::virtio::VirtQueueBufferChain;
use crate::virtio9p::{P9_LOCK_BLOCKED, P9_LOCK_SUCCESS};
use crate::{BufferHodler, MarVal, Marshall, Qid, StoreT, UTF8};

const STATUS_INVALID: i8 = -0x1;
const STATUS_OK: i8 = 0x0;
const STATUS_ON_STORAGE: i8 = 0x2;
const STATUS_UNLINKED: i8 = 0x4;
const STATUS_FORWARDING: i8 = 0x5;


const P9_LOCK_TYPE_RDLCK: u8 = 0;
const P9_LOCK_TYPE_WRLCK: u8 = 1;
const P9_LOCK_TYPE_UNLCK: u8 = 2;

pub const S_IRWXUGO: u32 = 0x1FF;
pub const S_IFMT: u32 = 0xF000;
pub const S_IFSOCK: u32 = 0xC000;
pub const S_IFLNK: u32 = 0xA000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFBLK: u32 = 0x6000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFCHR: u32 = 0x2000;

const LOCK_TYPE_RDLCK: u8 = 0;
const LOCK_TYPE_WRLCK: u8 = 1;
const LOCK_TYPE_UNLCK: u8 = 2;

pub const EPERM: i8 = 1;
const ENOTEMPTY: i8 = 39; /* Directory not empty */


#[derive(Clone)]
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
        return region.proc_id == self.proc_id
            && region.client_id == self.client_id
            && region.type_ == self.type_;
    }

    fn may_merge_after(&self, region: &FSLockRegion) -> bool {
        return self.is_alike(region) && region.start + region.length == self.start;
    }
}

type ForeignIdType = i64;
type MountIdType = i64;

#[derive(Clone)]
struct Inode {
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) fid: u64,
    pub(crate) size: u64,
    pub(crate) ctime: u64,
    pub(crate) status: i8,
    pub(crate) atime: u64,
    pub(crate) mtime: u64,
    pub(crate) major: u32,
    pub(crate) minor: u32,
    pub(crate) symlink: String,
    pub(crate) mode: u32,
    pub(crate) qid: Qid,
    pub(crate) caps: Vec<u8>,
    pub(crate) nlinks: u32,
    pub(crate) sha256sum: String,
    pub(crate) mount_id: MountIdType,
    pub(crate) locks: Vec<Rc<FSLockRegion>>,
    pub(crate) foreign_id: ForeignIdType,
    pub(crate) direntries: HashMap<String, i64>,
}

pub struct FsNode {
    name: String,
    size: u64,
    mtime: u64,
    mode: u32,
    uid: u32,
    gid: u32,
    target: String,
    sha256sum: String,
    children: Vec<FsNode>,
}

impl Inode {
    fn new(qidnumber: u32) -> Self {
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
            qid: Qid {
                type_: 0,
                version: 0,
                path: qidnumber,
            },
            nlinks: 0,
            mount_id: -1,
            foreign_id: -1,
            caps: Vec::new(),
            locks: Vec::new(),
            sha256sum: String::new(),
        }
    }
}

struct FSMountInfo {
    backtrack: HashMap<ForeignIdType, usize>,
    fs: FS,
}

struct Event {
    id: u32,
    on_event: Box<dyn FnOnce()>,
}

pub struct FS {
    store: StoreT,
    used_size: u32,
    total_size: u32,
    events: Option<Vec<Event>>,
    inodes: Vec<Rc<Inode>>,
    mounts: Vec<FSMountInfo>,
    qidcounter: Qidcounter,
    inodedata: HashMap<i64, Vec<u8>>,
}

struct Qidcounter {
    last_qidnumber: u32,
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
    #[inline(always)]
    fn is_forwarder(inode: &Inode) -> bool {
        return (*inode).status == STATUS_FORWARDING; 
    }

    fn follow_fs(&self, inode: &Inode) -> &FS {
        let mount = self.mounts.get(inode.mount_id as usize);
        assert!(
            mount.is_some(),
            "Filesystem follow_fs: inode<id={}> should point to valid mounted FS",
            inode.fid
        );
        let mount = mount.unwrap();
        assert!(
            Self::is_forwarder(inode),
            "Filesystem follow_fs: inode should be a forwarding inode"
        );
        &mount.fs
    }

    pub fn add_event(
        &mut self, 
        id: u32, 
        on_event: impl FnOnce() + 'static, 
    ) {
        let mut inode = self.inodes[id as usize].clone();
        if inode.status == STATUS_OK || inode.status == STATUS_ON_STORAGE {
            on_event();
        } else if Self::is_forwarder(&inode) {
            self.follow_fs_mut(id as _).add_event((*inode).foreign_id as _, on_event);
        } else {
            self.events
                .as_mut()
                .map(|events| events.push(Event{id, on_event: Box::new(on_event)}));
        }
    }

    pub fn handle_event(&mut self, id: u32) {
        unsafe {
            let inode = self.inodes[id as usize].clone();
            if Self::is_forwarder(&inode) {
                self.follow_fs_mut(id as _).handle_event(inode.foreign_id as _);
            }
            let events = self.events.take().unwrap();
            let (newevents, mut removed):(Vec<_>, Vec<_>) = events.into_iter().partition(|e| e.id != id);
            removed.into_iter().for_each(|e| (e.on_event)());
            self.events = Some(newevents);
        }
    }

    fn follow_fs_mut(&mut self, idx: usize) -> &mut FS {
        let inode = self.inodes[idx].clone();
        let mount = self.mounts.get_mut(inode.mount_id as usize);
        assert!(
            mount.is_some(),
            "Filesystem follow_fs: inode<id={}> should point to valid mounted FS",
            inode.fid
        );
        let mount = mount.unwrap();
        assert!(
            Self::is_forwarder(&inode),
            "Filesystem follow_fs: inode should be a forwarding inode"
        );

        &mut mount.fs
    }

    pub fn get_inode(&self, idx: usize) -> Option<&Inode> {
        assert!(
            idx >= 0 && idx < self.inodes.len(),
            "Filesystem GetInode: out of range idx:{idx}"
        );

        self.inodes.get(idx).and_then(|inode| {
            if Self::is_forwarder(inode) {
                self.follow_fs(inode).get_inode(inode.foreign_id as usize)
            } else {
                Some(inode)
            }
        })
    }

    pub fn inode_mut<F>(&mut self, idx: usize, mut f: F)
    where
        F: FnOnce(&mut Inode),
    {
        assert!(
            idx >= 0 && idx < self.inodes.len(),
            "Filesystem GetInode: out of range idx:{idx}"
        );
        let (foreign_id, is_forwarder, inode) = self
            .inodes
            .get_mut(idx)
            .map(|inode| {
                let foreign_id = inode.foreign_id;
                let is_forwarder = Self::is_forwarder(inode);
                (foreign_id, is_forwarder, inode)
            })
            .unwrap();
        if is_forwarder {
            self.follow_fs_mut(idx).inode_mut(foreign_id as usize, f);
        } else {
            f(Rc::make_mut(inode))
        }
    }

    fn is_directory(&self, idx: usize) -> bool {
        let inode = self.inodes[idx].clone();
        if Self::is_forwarder(&inode) {
            return self
                .follow_fs(&inode)
                .is_directory(inode.foreign_id as usize);
        }
        return (inode.mode & S_IFMT) == S_IFDIR;
    }

    fn is_empty(&self, idx: usize) -> bool {
        let inode = self.inodes[idx].clone();
        if Self::is_forwarder(&inode) {
            return self
                .follow_fs(&inode)
                .is_directory(inode.foreign_id as usize);
        }
        for name in inode.direntries.keys() {
            if name != "." && name != ".." {
                return false;
            }
        }
        return true;
    }

    fn get_children(&self, idx: usize) -> Vec<String> {
        assert!(
            self.is_directory(idx),
            "Filesystem: cannot get children of non-directory inode"
        );
        let inode = self.inodes[idx].clone();
        unsafe {
            if Self::is_forwarder(&inode) {
                return self
                    .follow_fs(&inode)
                    .get_children(inode.foreign_id as usize);
            }
            inode
                .direntries
                .keys()
                .filter_map(|name| {
                    if name != "." && name != ".." {
                        Some(name.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        }
    }

    fn should_be_linked(inode: &Inode) -> bool {
        // Note: Non-root forwarder inode could still have a non-forwarder parent, so don't use
        // parent inode to check.
        return !Self::is_forwarder(inode) || inode.foreign_id == 0;
    }

    fn get_parent(&mut self, idx: usize) -> i64 {
        assert!(
            self.is_directory(idx),
            "Filesystem: cannot get parent of non-directory inode"
        );

        let inode = self.inodes[idx].clone();
        if Self::should_be_linked(&inode) {
            *inode.direntries.get("..").unwrap()
        } else {
            let foreign_id = inode.foreign_id;
            let mount_id = inode.mount_id;
            let foreign_dirid = self.follow_fs_mut(idx).get_parent(foreign_id as usize);
            assert!(
                foreign_dirid != -1,
                "Filesystem: should not have invalid parent ids"
            );
            self.get_forwarder(mount_id, foreign_dirid) as i64
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
        self.inodes.push(Rc::new(inode));

        self.set_forwarder(idx, mount_id, foreign_id);
        return idx as u64;
    }

    pub fn open_inode(&mut self, id: u32, mode: u32) -> bool {
        let id = id as usize;
        let id = id as usize;
        let inode = self.inodes[id].clone();
        if Self::is_forwarder(&*inode) {
            return self.follow_fs_mut(id).open_inode((*inode).foreign_id as u32, mode);
        }
        if (*inode).mode & S_IFMT == S_IFDIR {
            self.fill_directory(id as _);
        }
        return true;
    }

    fn fill_directory(&mut self, dirid: u32) {
        let dirid = dirid as usize;
        let mut inode: Rc<Inode> = self.inodes[dirid].clone();
        if Self::is_forwarder(&*inode) {
            self.follow_fs_mut(dirid as _).fill_directory((*inode).foreign_id as _)
        }
        let mut size = 0;
        for name in (*inode).direntries.keys() {
            size += 13 + 8 + 1 + 2 + UTF8::utf8_length(name);
        }
        self.inodedata.insert(dirid as i64, vec![0u8; size as usize]);
        Rc::make_mut(&mut inode).size = size as _;
        let mut offset = 0x0u32;
        for (name, id) in (*inode).direntries.iter() {
            let (child_mode, child_qid) = {
                let child = self.get_inode(*id as _).unwrap();
                (child.mode, child.qid)
            };
            let data = self.inodedata.get_mut(&(dirid as i64));
            offset += Marshall::marshall(&["Q", "d", "b", "s"], &[
                MarVal::QID(child_qid), 
                MarVal::U32(offset+13+8+1+2+UTF8::utf8_length(name)),
                MarVal::U8((child_mode>>12) as u8),
                MarVal::String(name.to_string()),
            ], BufferHodler::new(data.unwrap(), offset as _));
        }
    }

    fn get_forwarder(&mut self, mount_id: MountIdType, foreign_id: ForeignIdType) -> u64 {
        let mount = self.mounts.get(mount_id as usize);
        assert!(
            foreign_id >= 0,
            "Filesystem get_forwarder: invalid foreign_id: {foreign_id}"
        );
        assert!(
            mount.is_some(),
            "Filesystem get_forwarder: invalid mount number: {mount_id}"
        );
        let mount = mount.unwrap();
        let result = mount.backtrack.get(&foreign_id);

        if result.is_none() {
            // Create if not already exists.
            return self.create_forwarder(mount_id, foreign_id);
        }
        *result.unwrap() as u64
    }

    fn set_forwarder(&mut self, idx: usize, mount_id: MountIdType, foreign_id: ForeignIdType) {
        let mut inode = self.inodes[idx].clone();
        assert!(
            (*inode).nlinks == 0,
            "Filesystem: attempted to convert an inode into forwarder before unlinking the inode"
        );
        if Self::is_forwarder(&inode) {
            self.mounts[inode.mount_id as usize]
                .backtrack
                .remove(&inode.foreign_id);
        }
        let inode_ = Rc::make_mut(&mut inode);
        inode_.status = STATUS_FORWARDING;
        inode_.mount_id = mount_id;
        inode_.foreign_id = foreign_id;

        self.mounts
            .get_mut(mount_id as usize)
            .map(|m: &mut FSMountInfo| {
                m.backtrack.insert(foreign_id, idx);
            });
    }

    fn search(&mut self, parentid: i64, name: &str) -> i64 {
        let mut parent_inode = self.inodes[parentid as usize].clone();
        if Self::is_forwarder(&parent_inode) {
            let foreign_parentid = parent_inode.foreign_id;
            let p_mount_id = parent_inode.mount_id;
            let foreign_id = self
                .follow_fs_mut(parentid as usize)
                .search(foreign_parentid, name);
            if foreign_id == -1 {
                return -1;
            }
            return self.get_forwarder(p_mount_id, foreign_id) as _;
        }

        let childid = parent_inode.direntries.get(name);
        childid.map(|i| *i).unwrap_or(-1)
    }

    pub fn count_used_inodes(&self) -> usize {
        let mut count = self.inodes.len();
        self.mounts.iter().for_each(|m| {
            count += m.fs.count_used_inodes();
            count -= m.backtrack.len();
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

    pub fn get_total_size(&self) -> u32 {
        let mut count = self.used_size;
        self.mounts.iter().for_each(|m| {
            count += m.fs.get_total_size();
        });
        return count;
    }

    pub fn get_space(&self) -> u32 {
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
                if i < n - 1 {
                    // one name of the path cannot be found
                    return PathInfo {
                        id: -1,
                        parentid: -1,
                        name: walk[i].to_string(),
                        forward_path,
                    };
                }
                // the last element in the path does not exist, but the parent
                return PathInfo {
                    id: -1,
                    parentid: parentid,
                    name: walk[i].to_string(),
                    forward_path,
                };
            }
        }
        return PathInfo {
            id: id,
            parentid: parentid,
            name: walk[walk.len() - 1].to_string(),
            forward_path,
        };
    }

    fn get_recursive_list(&mut self, dirid: i64, list: &mut Vec<RecursiveInfo>) {
        let inode = self.inodes[dirid as usize].clone();
        unsafe {
            if Self::is_forwarder(&inode) {
                let foreign_id = inode.foreign_id;
                let mount_id = inode.mount_id;
                let foreign_fs = self.follow_fs_mut(dirid as usize);

                let foreign_start = list.len();
                foreign_fs.get_recursive_list(foreign_id, list);
                for i in foreign_start..list.len() {
                    list[i].parentid = self.get_forwarder(mount_id, list[i].parentid) as _;
                }
                return;
            }
            let ids = inode
                .direntries
                .iter()
                .filter_map(|(name, id)| {
                    if name != "." && name != ".." {
                        list.push(RecursiveInfo {
                            parentid: dirid,
                            name: name.to_string(),
                        });
                        if self.is_directory(*id as usize) {
                            let id = *id;
                            return Some(id);
                        }
                    }
                    return None;
                })
                .collect::<Vec<_>>();
            ids.into_iter().for_each(|id| {
                self.get_recursive_list(id, list);
            });
        }
    }

    fn unlink_from_dir(&mut self, parentid: i64, name: &str) {
        let idx = self.search(parentid, name);
        let parentid = parentid as usize;
        let idx = idx as usize;
        // let inode: &mut Inode = &mut self.inodes[idx];
        // let parent_inode: &mut Inode = &mut self.inodes[parentid];
        let mut parent_inode = self.inodes[parentid].clone();
        let mut inode = self.inodes[idx].clone();
        assert!(
            !Self::is_forwarder(&*parent_inode),
            "Filesystem: Can't unlink from forwarders"
        );
        let inode_ = Rc::make_mut(&mut inode);
        let parent_inode_ = Rc::make_mut(&mut parent_inode);

        assert!(
            self.is_directory(parentid),
            "Filesystem: Can't unlink from non-directories"
        );

        let exists = parent_inode_.direntries.remove(name);
        if exists.is_none() {
            assert!(false, "Filesystem: Can't unlink non-existent file: {name}");
            return;
        }

        inode_.nlinks -= 1;

        if self.is_directory(idx) {
            assert!(
                inode_
                    .direntries
                    .get("..")
                    .map(|d| *d == parentid as i64)
                    .unwrap_or(false),
                "Filesystem: Found directory with bad parent id"
            );

            inode_.direntries.remove("..");
            parent_inode_.nlinks -= 1;
        }

        assert!(
            inode_.nlinks >= 0,
            "Filesystem: Found negative nlinks value of {}",
            inode_.nlinks
        );
    }

    fn unlink(&mut self, parentid: i64, name: &str) -> i8 {
        if name == "." || name == ".." {
            // Also guarantees that root cannot be deleted.
            return -EPERM;
        }
        let idx = self.search(parentid, name) as usize;
        let parentid = parentid as usize;
        let mut inode = self.inodes[idx].clone();
        let mut parent_inode = self.inodes[parentid].clone();
        //message.Debug("Unlink " + inode.name);

        // forward if necessary
        if Self::is_forwarder(&parent_inode) {
            assert!(
                Self::is_forwarder(&inode),
                "Children of forwarders should be forwarders"
            );

            let foreign_parentid = parent_inode.foreign_id;
            return self.follow_fs_mut(parentid).unlink(foreign_parentid, name);

            // Keep the forwarder dangling - file is still accessible.
        }

        if self.is_directory(idx) && !self.is_empty(idx) {
            return -ENOTEMPTY;
        }

        self.unlink_from_dir(parentid as _, name);

        if (*inode).nlinks == 0 {
            // don't delete the content. The file is still accessible
            Rc::make_mut(&mut inode).status = STATUS_UNLINKED;
            // self.notify_listeners(idx, "delet");
        }
        return 0;
    }

    #[inline(always)]
    fn is_a_root(&self, idx: usize) -> bool {
        self.get_inode(idx).unwrap().fid == 0
    }

    pub fn lock(&mut self, id: i64, request: FSLockRegion, flags: u32) -> u8 {
        let mut inode = self.inodes[id as usize].clone();
        if Self::is_forwarder(&*inode) {
            return self.follow_fs_mut(id as _).lock((*inode).foreign_id, request, flags);
        }
        let request = Rc::new(request.clone());
        
        // (1) Check whether lock is possible before any modification.
        if (*request).type_ != P9_LOCK_TYPE_UNLCK && self.get_lock(id, &(*request)).is_some() {
            return P9_LOCK_BLOCKED;
        }

        // (2) Subtract requested region from locks of the same owner.
        let mut i = 0;
        
        while i < (*inode).locks.len() {
            let region = (*inode).locks[i].clone();
            assert!((*region).length > 0,
                "Filesystem: Found non-positive lock region length: {}", (*region).length);
            assert!((*region).type_ == P9_LOCK_TYPE_RDLCK || (*region).type_ == P9_LOCK_TYPE_WRLCK,
                "Filesystem: Found invalid lock type: {}", (*region).type_);
            assert!((*inode).locks.get(i-1).is_none() || (*(*inode).locks[i-1]).start <= (*region).start,
                "Filesystem: Locks should be sorted by starting offset");

            // Skip to requested region.
            if (*region).start + (*region).length <= (*request).start {
                i += 1;
                continue;
            }
                
            // Check whether we've skipped past the requested region.
            if (*request).start + (*request).length <= (*region).start {
                break;
            } 

            // Skip over locks of different owners.
            if (*region).proc_id != (*request).proc_id || &(*region).client_id != &(*request).client_id {
                assert!(!(*region).conflicts_with(&(*request)),
                    "Filesytem: Found conflicting lock region, despite already checked for conflicts");
                i += 1;
                continue;
            }

            // Pretend region would be split into parts 1 and 2.
            let start1 = region.start;
            let start2 = request.start + request.length;
            let length1 = request.start - start1;
            let length2 = region.start + region.length - start2;

            if(length1 > 0 && length2 > 0 && (*region).type_ == (*request).type_) {
                // Requested region is already locked with the required type.
                // Return early - no need to modify anything.
                return P9_LOCK_SUCCESS;
            }
            
            if length1 > 0 {
                let mut region_cl = region.clone();
                let region_ = Rc::make_mut(&mut region_cl);
                // Shrink from right / first half of the split.
                region_.length = length1;

            }

            if length1 <= 0 && length2 > 0 {
                let mut region_cl = region.clone();
                let region_ = Rc::make_mut(&mut region_cl);
                // Shrink from left.
                region_.start = start2;
                region_.length = length2;
            } else if length2 > 0 {
                // Add second half of the split.
                // Fast-forward to correct location.
                while i < (*inode).locks.len() && (*(*inode).locks[i]).start < start2 {
                    i+=1;
                }
                let lock = self.describe_lock((*region).type_, start2, length2, region.proc_id as _, &region.client_id);
                let inode_ = Rc::make_mut(&mut inode);
                inode_.locks.insert(i, Rc::new(lock));
            } else if length1 <= 0 {
                let inode_ = Rc::make_mut(&mut inode);
                // Requested region completely covers this region. Delete.
                inode_.locks.remove(i);
                i -= 1;
            }
            i += 1;
        }

        // (3) Insert requested lock region as a whole.
        // No point in adding the requested lock region as fragmented bits in the above loop
        // and having to merge them all back into one.
        if request.type_ != P9_LOCK_TYPE_UNLCK {
            let mut new_region = request.clone();
            let mut has_merged = false;
            let mut i = 0;

            
            // Fast-forward to requested position, and try merging with previous region.
            while i < inode.locks.len() {
                if new_region.may_merge_after(&inode.locks[i]) {
                    let mut cl_inode = inode.clone();
                    let cl_inode_ = Rc::get_mut(&mut cl_inode).unwrap();
                    let mut l = cl_inode.locks[i].clone();
                    let l_ = Rc::make_mut(&mut l);
                    l_.length += (*request).length;
                    new_region = (*inode).locks[i].clone();
                    has_merged = true;
                }
                if (*request).start <= (*(*inode).locks[i]).start {
                    break;
                }
                i += 1
            }

            if !has_merged {
                let mut cl_inode = inode.clone();
                let cl_inode_ = Rc::make_mut(&mut cl_inode);
                cl_inode_.locks.insert(i, new_region.clone());
                i += 1
            }
            let mut inode_cl = inode.clone();
            let inode_ = Rc::make_mut(&mut inode_cl);
            let mut new_region_cl = new_region.clone();
            let new_region_ = Rc::make_mut(&mut new_region_cl);
            // Try merging with the subsequent alike region.
            while i < inode.locks.len() {
                if !inode.locks[i].is_alike(&new_region) {
                    i += 1;
                    continue;
                }
                if inode.locks[i].may_merge_after(&new_region) {
                    new_region_.length += (*(*inode).locks[i]).length;
                    inode_.locks.remove(i);
                }
                // No more mergable regions after this.
                break;
            }
        }

        return P9_LOCK_SUCCESS;
    }

    fn get_lock(&self, id: i64, request: &FSLockRegion) -> Option<Rc<FSLockRegion>> {
        let inode = self.inodes[id as usize].clone();
        if Self::is_forwarder(&inode) {
            let foreign_id = inode.foreign_id;
            return self.follow_fs(&inode).get_lock(foreign_id, request);
        }

        for region in inode.locks.iter() {
            if request.conflicts_with(region) {
                return Some(region.clone());
            }
        }
        return None;
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
            assert!(
                ret == 0,
                "Filesystem RecursiveDelete failed at parent={} name='{}' with error code: {}",
                r.parentid,
                r.name,
                -ret,
            );
        });
    }

    pub fn create_node(&mut self, filename: &str, parentid: i64, major: u32, minor: u32) -> u64 {
        let parent_inode = self.inodes[parentid as usize].clone();
        unsafe {
            if Self::is_forwarder(&parent_inode) {
                let foreign_parentid = parent_inode.foreign_id;
                let mount_id = parent_inode.mount_id;
                let foreign_id = self.follow_fs_mut(parentid as _).create_node(
                    filename,
                    foreign_parentid,
                    major,
                    minor,
                );
                return self.create_forwarder(mount_id, foreign_id as _);
            }
        
            let mut x = self.create_inode();
            x.major = major;
            x.minor = minor;
            x.uid = self.inodes[parentid as usize].uid;
            x.gid = self.inodes[parentid as usize].gid;
            x.qid.type_ = (S_IFSOCK >> 8) as u8;
            x.mode = (self.inodes[parentid as usize].mode) & 0x1B6;
            self.push_inode(x, parentid, filename);
        }   
        return (self.inodes.len() - 1) as _;
    }

    fn push_inode(&mut self, mut inode: Inode, parentid: i64, name: &str) {
        if parentid != -1 {
            let fid = self.inodes.len();
            inode.fid = fid as _;
            self.inodes.push(Rc::new(inode));
            self.link_under_dir(parentid, fid as _, name);
            return;
        } else {
            if self.inodes.len() == 0 {
                // if root directory
                inode.direntries.insert(".".to_string(), 0);
                inode.direntries.insert("..".to_string(), 0);
                inode.nlinks = 2;
                self.inodes.push(Rc::new(inode));
                return;
            }
        }
    }

    fn link_under_dir(&mut self, parentid: i64, idx: i64, name: &str) {
        let parentid = parentid as usize;
        let idx = idx as usize;
        let mut inode = self.inodes[idx].clone();
        let mut parent_inode = self.inodes[parentid].clone();
        assert!(
            !Self::is_forwarder(&parent_inode),
            "Filesystem: Shouldn't link under fowarder parents"
        );
        assert!(
            self.is_directory(parentid),
            "Filesystem: Can't link under non-directories"
        );

        assert!(
            Self::should_be_linked(&inode),
            "Filesystem: Can't link across filesystems apart from their root"
        );
        assert!(
            inode.nlinks >= 0,
            "Filesystem: Found negative nlinks value of {}",
            inode.nlinks
        );

        assert!(
            !parent_inode.direntries.contains_key(name),
            "Filesystem: Name '{name}' is already taken"
        );
        let parent_inode_ = Rc::make_mut(&mut parent_inode);
        let inode_ = Rc::make_mut(&mut inode);
        parent_inode_.direntries.insert(name.to_string(), idx as _);
        inode_.nlinks += 1;

        if self.is_directory(idx) {
            assert!(
                !inode_.direntries.contains_key(".."),
                "Filesystem: Cannot link a directory twice"
            );

            if !inode_.direntries.contains_key(".") {
                inode_.nlinks += 1;
            }
            inode_.direntries.insert(".".to_string(), idx as _);

            inode_.direntries.insert("..".to_string(), parentid as _);
            parent_inode_.nlinks += 1;
        }
    }

    fn delete_forwarder<'a>(&'a mut self, inode: &'a Inode) {
        assert!(
            Self::is_forwarder(inode),
            "Filesystem delete_forwarder: expected forwarder"
        );
        self.mounts[inode.mount_id as usize]
            .backtrack
            .remove(&inode.foreign_id);
    }

    pub fn create_symlink(&mut self, filename: &str, parentid: i64, symlink: &str) -> u64 {
        let parentid = parentid as usize;
        let parent_inode = self.inodes[parentid].clone();
        if Self::is_forwarder(&parent_inode) {
            let foreign_parentid = parent_inode.foreign_id;
            let mount_id = parent_inode.mount_id;
            let foreign_id =
                self.follow_fs_mut(parentid)
                    .create_symlink(filename, foreign_parentid, symlink);
            return self.create_forwarder(mount_id, foreign_id as _);
        }
        let mut x = self.create_inode();
        x.uid = self.inodes[parentid].uid;
        x.gid = self.inodes[parentid].gid;
        x.qid.type_ = (S_IFLNK >> 8) as u8;
        x.symlink = symlink.into();
        x.mode = S_IFLNK;
        self.push_inode(x, parentid as i64, filename);
        return (self.inodes.len() - 1) as u64;
    }

    pub fn link(&mut self, parentid: i64, targetid: i64, name: &str) -> i8 {
        if self.is_directory(targetid as _) {
            return -EPERM;
        }
        let parentid = parentid as usize;
        let targetid = targetid as usize;
        // let parent_inode = self.inodes[parentid];
        // let inode = this.inodes[targetid];

        let (parent_foreign_id, parent_is_forwarder, parent_mount_id) = self
            .inodes
            .get(parentid)
            .map(|parent_inode| {
                (
                    parent_inode.foreign_id,
                    Self::is_forwarder(parent_inode),
                    parent_inode.mount_id,
                )
            })
            .unwrap();

        let (inode_foreign_id, inode_is_forwarder, inode_mount_id) = self
            .inodes
            .get(targetid)
            .map(|inode| {
                (inode.foreign_id, Self::is_forwarder(inode), inode.mount_id)
            })
            .unwrap();
        if parent_is_forwarder {
            if !inode_is_forwarder || inode_mount_id != parent_mount_id {
                dbg_log!(
                    LOG::P9,
                    "XXX: Attempted to hardlink a file into a child filesystem - skipped"
                );
                return -EPERM;
            }
            return self
                .follow_fs_mut(parentid)
                .link(parent_foreign_id, inode_foreign_id, name);
        }

        if inode_is_forwarder {
            dbg_log!(
                LOG::P9,
                "XXX: Attempted to hardlink file across filesystems - skipped"
            );
            return -EPERM;
        }

        self.link_under_dir(parentid as _, targetid as _, name);
        return 0;
    }

    fn load_recursive(&mut self, data: &FsNode, parentid: i64) {
        let mut inode = self.create_inode();

        let name = &data.name;
        inode.size = data.size;
        inode.mtime = data.mtime;
        inode.ctime = inode.mtime;
        inode.atime = inode.mtime;
        inode.mode = data.mode;
        inode.uid = data.uid;
        inode.gid = data.gid;

        let ifmt = inode.mode & S_IFMT;

        if ifmt == S_IFDIR {
            self.push_inode(inode, parentid, &name);
            let parentid = self.inodes.len() - 1;
            self.load_dir(parentid as _, &data.children);
        } else if ifmt == S_IFREG {
            inode.status = STATUS_ON_STORAGE;
            inode.sha256sum = data.sha256sum.clone();
            assert!(inode.sha256sum.len() > 0);
            self.push_inode(inode, parentid, name);
        } else if ifmt == S_IFLNK {
            inode.symlink = data.target.clone();
            self.push_inode(inode, parentid, name);
        } else if ifmt == S_IFSOCK {
            // socket: ignore
        } else {
            dbg_log!(LOG::E, "Unexpected ifmt: {ifmt:x} ({name})");
        }
    }

    fn load_dir(&mut self, parentid: i64, children: &Vec<FsNode>) {
        children
            .iter()
            .for_each(|c| self.load_recursive(c, parentid));
    }

    fn divert(&mut self, parentid: i64, filename: &str) -> usize {
        let old_idx = self.search(parentid, filename);
        let mut old_inode = self.inodes[old_idx as usize].clone();
        assert!(
            self.is_directory(old_idx as usize) || old_inode.nlinks <= 1,
            "Filesystem: can't divert hardlinked file '{filename}' with nlinks={}",
            (*old_inode).nlinks
        );

        let old_inode_forward = Self::is_forwarder(&old_inode);
        let old_inode_should_be_linked = Self::should_be_linked(&old_inode);
        let old_inode_mount_id = (*old_inode).foreign_id;
        let mut new_inode = old_inode.clone();

        let idx = self.inodes.len();
        let old_inode_ = Rc::make_mut(&mut old_inode);
        old_inode_.fid = idx as _;
        self.inodes.push(new_inode.clone());

        // Relink references
        if old_inode_forward {
            self.mounts[old_inode_mount_id as usize]
                .backtrack
                .insert(old_inode_mount_id, idx);
        }
        if old_inode_should_be_linked {
            self.unlink_from_dir(parentid, filename);
            self.link_under_dir(parentid, idx as _, filename);
        }

        // Update children
        if self.is_directory(old_idx as _) && !old_inode_forward {
            for (name, child_id) in new_inode.direntries.iter() {
                if name == "." || name == ".."{
                    continue;
                }
                if(self.is_directory(*child_id as _)) {
                    let mut cid_inode = self.inodes[*child_id as usize].clone();
                    Rc::make_mut(&mut cid_inode).direntries.insert("..".to_string(), idx as _);
                }
            }
        }

        // Relocate local data if any.
        self.inodedata
            .remove(&old_idx)
            .map(|d| self.inodedata.insert(idx as i64, d));
        let old_inode_ = Rc::make_mut(&mut old_inode);
        old_inode_.direntries = HashMap::new();
        old_inode_.nlinks = 0;
        return idx;
    }

    pub fn create_directory(&mut self, name: &str, parentid: i64) -> i64 {
        let parent_inode = self.inodes[parentid as usize].clone();
        unsafe {
            if parentid >= 0 && Self::is_forwarder(&parent_inode) {
                let foreign_parentid = parent_inode.foreign_id;
                let foreign_id = self
                    .follow_fs_mut(parentid as usize)
                    .create_directory(name, parent_inode.foreign_id);
                return self.create_forwarder(parent_inode.mount_id, foreign_id as _) as i64;
            }
            let mut x = self.create_inode();
            x.mode = 0x01FF | S_IFDIR;
            if parentid >= 0 {
                self.inodes.get(parentid as usize).map(|parent_node| unsafe {
                    x.uid = (&**parent_node).uid;
                    x.gid = (&**parent_node).gid;
                    x.mode = ((**parent_node).mode & 0x1FF) | S_IFDIR;
                });
            }
            x.qid.type_ = (S_IFDIR >> 8) as u8;
            self.push_inode(x, parentid, name);
            // this.NotifyListeners(this.inodes.length-1, 'newdir');
            return (self.inodes.len() - 1) as i64;
        }
    }
    pub fn describe_lock(&mut self, type_: u8, start: u32, length: u32, proc_id: u32, client_id: &str) -> FSLockRegion {
        assert!(type_ == P9_LOCK_TYPE_RDLCK ||
            type_ == P9_LOCK_TYPE_WRLCK ||
            type_ == P9_LOCK_TYPE_UNLCK,
            "Filesystem: Invalid lock type: {type_}");
        assert!(start >= 0, "Filesystem: Invalid negative lock starting offset:{start}");
        assert!(length > 0, "Filesystem: Invalid non-positive lock length:{length}");
        let mut lock = FSLockRegion::new();
        lock.type_ = type_;
        lock.start = start;
        lock.length = length;
        lock.proc_id = proc_id as _;
        lock.client_id = client_id.to_string();
        return lock;
    }

    pub fn create_file(&mut self, filename: &str, parentid: i64) -> i64 {
        let (parent_forwarder, parent_foreign_id, parent_mount_id) = self
            .inodes
            .get(parentid as usize)
            .map(|parent_inode| {
                let forwarder = Self::is_forwarder(parent_inode);
                let foreign_id = parent_inode.foreign_id;
                let mount_id = parent_inode.mount_id;
                (forwarder, foreign_id, mount_id)
            })
            .unwrap();
        if parent_forwarder {
            let foreign_id = self
                .follow_fs_mut(parentid as usize)
                .create_file(filename, parent_foreign_id);
            return self.create_forwarder(parent_mount_id, foreign_id) as i64;
        }
        let mut x = self.create_inode();
        self.inodes.get(parentid as usize).map(|parent_inode| unsafe {
            x.uid = parent_inode.uid;
            x.gid = parent_inode.gid;
            x.qid.type_ = (S_IFREG >> 8) as u8;
            x.mode = (parent_inode.mode & 0x1B6) | S_IFREG;
        });

        self.push_inode(x, parentid, filename);
        // this.NotifyListeners(this.inodes.length-1, 'newfile');
        return (self.inodes.len() - 1) as i64;
    }

    fn set_data(&mut self, idx: i64, buffer: Vec<u8>) {
        // Current scheme: Save all modified buffers into local inodedata.
        self.inodedata.insert(idx, buffer);
        let mut inode = self.inodes[idx as usize].clone();
        let inode = Rc::make_mut(&mut inode);
        if inode.status == STATUS_ON_STORAGE {
            inode.status = STATUS_OK;
            todo!()
            //this.storage.uncache(this.inodes[idx].sha256sum);
        }
    }

    fn create_binary_file(&mut self, filename: &str, parentid: i64, buffer: Vec<u8>) -> i64 {
        let (parent_forwarder, parent_foreign_id, parent_mount_id) = self
            .inodes
            .get(parentid as usize)
            .map(|parent_inode| unsafe {
                let forwarder = Self::is_forwarder(parent_inode);
                let foreign_id = parent_inode.foreign_id;
                let mount_id = parent_inode.mount_id;
                (forwarder, foreign_id, mount_id)
            })
            .unwrap();
        if parent_forwarder {
            let foreign_id = self.follow_fs_mut(parentid as _).create_binary_file(
                filename,
                parent_foreign_id,
                buffer,
            );
            return self.create_forwarder(parent_mount_id, foreign_id) as i64;
        }
        let id = self.create_file(filename, parentid);

        let buf_len = buffer.len();
        self.set_data(id, buffer);
        let mut inode = self.inodes[id as usize].clone();
        let x = Rc::make_mut(&mut inode);
        x.size = buf_len as u64;
        return id;
    }

    fn prepare_caps(&mut self, id: i64) -> usize {
        match self.get_inode(id as usize).and_then(|inode| {
            if inode.caps.len() > 0 {
                Some(inode.caps.len())
            } else {
                None
            }
        }) {
            Some(n) => return n,
            None => {}
        };

        let mut caps = vec![0u8; 20];
        // format is little endian
        // note: getxattr returns -EINVAL if using revision 1 format.
        // note: getxattr presents revision 3 as revision 2 when revision 3 is not needed.
        // magic_etc (revision=0x02: 20 bytes)
        caps[0] = 0x00;
        caps[1] = 0x00;
        caps[2] = 0x00;
        caps[3] = 0x02;

        // lower
        // permitted (first 32 capabilities)
        caps[4] = 0xFF;
        caps[5] = 0xFF;
        caps[6] = 0xFF;
        caps[7] = 0xFF;
        // inheritable (first 32 capabilities)
        caps[8] = 0xFF;
        caps[9] = 0xFF;
        caps[10] = 0xFF;
        caps[11] = 0xFF;

        // higher
        // permitted (last 6 capabilities)
        caps[12] = 0x3F;
        caps[13] = 0x00;
        caps[14] = 0x00;
        caps[15] = 0x00;
        // inheritable (last 6 capabilities)
        caps[16] = 0x3F;
        caps[17] = 0x00;
        caps[18] = 0x00;
        caps[19] = 0x00;
        let l = caps.len();
        self.inode_mut(id as usize, |inode| {
            inode.caps = caps;
        });
        l
    }
}
