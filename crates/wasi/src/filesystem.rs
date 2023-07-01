#![allow(unused, dead_code)]
use std::collections::{HashMap, VecDeque};
use std::time::{self, Duration};

use crate::log::LOG;
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

#[derive(Clone)]
struct QID {
    type_: u8,
    version: u8,
    path: i64,
}

type ForeignIdType = i64;
type MountIdType = i64;

#[derive(Clone)]
struct Inode {
    uid: u32,
    gid: u32,
    fid: u64,
    size: u64,
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
    locks: Vec<FSLockRegion>,
    foreign_id: ForeignIdType,
    direntries: HashMap<String, i64>,
}

pub struct FsNode {
    name: String,
    size: u64,
    mtime: u64,
    mode: u16,
    uid: u32,
    gid: u32,
    target: String,
    sha256sum: String,
    children: Vec<FsNode>,
}

impl Inode {
    fn new(qidnumber: i64) -> Self {
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

struct FS {
    store: StoreT,
    used_size: usize,
    total_size: usize,
    inodes: Vec<Inode>,
    mounts: Vec<FSMountInfo>,
    qidcounter: Qidcounter,
    inodedata: HashMap<i64, Vec<u8>>,
}

struct Qidcounter {
    last_qidnumber: i64,
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

    fn follow_fs_mut(&mut self, idx: usize) -> &mut FS {
        let inode = self.inodes.get(idx).unwrap();
        let mount = self.mounts.get_mut(inode.mount_id as usize);
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

        &mut mount.fs
    }

    fn get_inode(&self, idx: usize) -> Option<&Inode> {
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

    fn inode_mut<F>(&mut self, idx: usize, mut f: F)
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
            f(inode)
        }
    }

    fn is_directory(&self, idx: usize) -> bool {
        let inode = self.inodes.get(idx).unwrap();
        if Self::is_forwarder(inode) {
            return self
                .follow_fs(inode)
                .is_directory(inode.foreign_id as usize);
        }
        return (inode.mode & S_IFMT) == S_IFDIR;
    }

    fn is_empty(&self, idx: usize) -> bool {
        let inode = self.inodes.get(idx).unwrap();
        if Self::is_forwarder(inode) {
            return self
                .follow_fs(inode)
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
        let inode = self.inodes.get(idx).unwrap();
        if Self::is_forwarder(inode) {
            return self
                .follow_fs(inode)
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

        let inode = self.inodes.get(idx).unwrap();
        if Self::should_be_linked(inode) {
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
        self.inodes.push(inode);

        self.set_forwarder(idx, mount_id, foreign_id);
        return idx as u64;
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
        let inode = self.inodes.get_mut(idx).unwrap();

        assert!(
            inode.nlinks == 0,
            "Filesystem: attempted to convert an inode into forwarder before unlinking the inode"
        );
        if Self::is_forwarder(inode) {
            self.mounts[inode.mount_id as usize]
                .backtrack
                .remove(&inode.foreign_id);
        }

        inode.status = STATUS_FORWARDING;
        inode.mount_id = mount_id;
        inode.foreign_id = foreign_id;

        self.mounts
            .get_mut(mount_id as usize)
            .map(|m: &mut FSMountInfo| {
                m.backtrack.insert(foreign_id, idx);
            });
    }

    fn search(&mut self, parentid: i64, name: &str) -> i64 {
        let parent_inode = self.inodes.get(parentid as usize).unwrap();
        if Self::is_forwarder(parent_inode) {
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

    fn count_used_inodes(&self) -> usize {
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
        let inode = &self.inodes[dirid as usize];
        if Self::is_forwarder(inode) {
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

    fn unlink_from_dir(&mut self, parentid: i64, name: &str) {
        let idx = self.search(parentid, name);
        let parentid = parentid as usize;
        let idx = idx as usize;
        // let inode: &mut Inode = &mut self.inodes[idx];
        // let parent_inode: &mut Inode = &mut self.inodes[parentid];
        self.inodes.get(parentid).map(|p| {
            assert!(
                !Self::is_forwarder(p),
                "Filesystem: Can't unlink from forwarders"
            );
        });

        assert!(
            self.is_directory(parentid),
            "Filesystem: Can't unlink from non-directories"
        );

        let exists = self
            .inodes
            .get_mut(parentid)
            .map(|parent_inode| parent_inode.direntries.remove(name))
            .unwrap();
        if exists.is_none() {
            assert!(false, "Filesystem: Can't unlink non-existent file: {name}");
            return;
        }

        self.inodes.get_mut(idx).map(|inode| inode.nlinks -= 1);

        if self.is_directory(idx) {
            self.inodes.get_mut(idx).map(|inode| {
                assert!(
                    inode
                        .direntries
                        .get("..")
                        .map(|d| *d == parentid as i64)
                        .unwrap_or(false),
                    "Filesystem: Found directory with bad parent id"
                );

                inode.direntries.remove("..");
            });

            self.inodes.get_mut(parentid).map(|parent_inode| {
                parent_inode.nlinks -= 1;
            });
        }

        self.inodes.get(idx).map(|inode| {
            assert!(
                inode.nlinks >= 0,
                "Filesystem: Found negative nlinks value of {}",
                inode.nlinks
            );
        });
    }

    fn unlink(&mut self, parentid: i64, name: &str) -> i8 {
        if name == "." || name == ".." {
            // Also guarantees that root cannot be deleted.
            return -EPERM;
        }
        let idx = self.search(parentid, name) as usize;
        let parentid = parentid as usize;
        // let inode = &mut self.inodes[idx];
        //let parent_inode = &self.inodes[parentid];
        //message.Debug("Unlink " + inode.name);

        // forward if necessary
        let (parent_forwarder, parent_foreign_id) = self
            .inodes
            .get(parentid)
            .map(|parent_inode| (Self::is_forwarder(parent_inode), parent_inode.foreign_id))
            .unwrap();

        let (inode_forwarder, inode_nlinks) = self
            .inodes
            .get(idx)
            .map(|inode| (Self::is_forwarder(inode), inode.nlinks))
            .unwrap();
        if parent_forwarder {
            assert!(
                inode_forwarder,
                "Children of forwarders should be forwarders"
            );

            let foreign_parentid = parent_foreign_id;
            return self.follow_fs_mut(parentid).unlink(foreign_parentid, name);

            // Keep the forwarder dangling - file is still accessible.
        }

        if self.is_directory(idx) && !self.is_empty(idx) {
            return -ENOTEMPTY;
        }

        self.unlink_from_dir(parentid as _, name);

        if inode_nlinks == 0 {
            // don't delete the content. The file is still accessible
            self.inodes
                .get_mut(idx)
                .map(|inode| inode.status = STATUS_UNLINKED);
            // self.notify_listeners(idx, "delet");
        }
        return 0;
    }

    #[inline(always)]
    fn is_a_root(&self, idx: usize) -> bool {
        self.get_inode(idx).unwrap().fid == 0
    }

    fn get_lock(&self, id: i64, request: FSLockRegion) -> Option<FSLockRegion> {
        let inode = &self.inodes[id as usize];

        if Self::is_forwarder(inode) {
            let foreign_id = inode.foreign_id;
            return self.follow_fs(inode).get_lock(foreign_id, request);
        }

        for region in inode.locks.iter() {
            if request.conflicts_with(region) {
                return Some((*region).clone());
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

    fn create_node(&mut self, filename: &str, parentid: i64, major: u32, minor: u32) -> u64 {
        let parent_inode = &self.inodes[parentid as usize];
        if Self::is_forwarder(parent_inode) {
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
        x.mode = self.inodes[parentid as usize].mode & 0x1B6;
        self.push_inode(x, parentid, filename);
        return (self.inodes.len() - 1) as _;
    }

    fn push_inode(&mut self, mut inode: Inode, parentid: i64, name: &str) {
        if parentid != -1 {
            let fid = self.inodes.len();
            inode.fid = fid as _;
            self.inodes.push(inode);
            self.link_under_dir(parentid, fid as _, name);
            return;
        } else {
            if self.inodes.len() == 0 {
                // if root directory
                inode.direntries.insert(".".to_string(), 0);
                inode.direntries.insert("..".to_string(), 0);
                inode.nlinks = 2;
                self.inodes.push(inode);
                return;
            }
        }
    }

    fn link_under_dir(&mut self, parentid: i64, idx: i64, name: &str) {
        let parentid = parentid as usize;
        let idx = idx as usize;
        // const inode = this.inodes[idx];
        // const parent_inode = this.inodes[parentid];
        self.inodes.get(parentid).map(|parent_inode| {
            assert!(
                !Self::is_forwarder(parent_inode),
                "Filesystem: Shouldn't link under fowarder parents"
            );
            assert!(
                self.is_directory(parentid),
                "Filesystem: Can't link under non-directories"
            );
        });

        self.inodes.get(idx).map(|inode| {
            assert!(
                Self::should_be_linked(inode),
                "Filesystem: Can't link across filesystems apart from their root"
            );
            assert!(
                inode.nlinks >= 0,
                "Filesystem: Found negative nlinks value of {}",
                inode.nlinks
            );
        });

        self.inodes.get_mut(parentid).map(|parent_inode| {
            assert!(
                !parent_inode.direntries.contains_key(name),
                "Filesystem: Name '{name}' is already taken"
            );
            parent_inode.direntries.insert(name.to_string(), idx as _);
        });
        self.inodes.get_mut(idx).map(|inode| {
            inode.nlinks += 1;
        });

        if self.is_directory(idx) {
            self.inodes.get_mut(idx).map(|inode| {
                assert!(
                    !inode.direntries.contains_key(".."),
                    "Filesystem: Cannot link a directory twice"
                );

                if !inode.direntries.contains_key(".") {
                    inode.nlinks += 1;
                }
                inode.direntries.insert(".".to_string(), idx as _);

                inode.direntries.insert("..".to_string(), parentid as _);
            });
            self.inodes.get_mut(parentid).map(|parent_inode| {
                parent_inode.nlinks += 1;
            });
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

    fn create_symlink(&mut self, filename: &str, parentid: i64, symlink: &str) -> u64 {
        let parentid = parentid as usize;
        let parent_inode = &self.inodes[parentid];
        if Self::is_forwarder(parent_inode) {
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

    fn link(&mut self, parentid: i64, targetid: i64, name: &str) -> i8 {
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
            .map(|inode| (inode.foreign_id, Self::is_forwarder(inode), inode.mount_id))
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
        let old_inode = self
            .inodes
            .get(old_idx as usize)
            .map(|old_inode| old_inode.clone())
            .unwrap();
        assert!(
            self.is_directory(old_idx as usize) || old_inode.nlinks <= 1,
            "Filesystem: can't divert hardlinked file '{filename}' with nlinks={}",
            old_inode.nlinks
        );

        let old_inode_forward = Self::is_forwarder(&old_inode);
        let old_inode_should_be_linked = Self::should_be_linked(&old_inode);
        let old_inode_mount_id = old_inode.foreign_id;
        let mut new_inode = old_inode;

        let idx = self.inodes.len();
        new_inode.fid = idx as _;
        self.inodes.push(new_inode);

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
            let rs = self.inodes.get(idx).map(|new_inode| {
                new_inode
                    .direntries
                    .iter()
                    .filter_map(|(name, child_id)| {
                        let child_id = *child_id as usize;
                        if name == "." || name == ".." {
                            return None;
                        }
                        Some((self.is_directory(child_id), child_id))
                    })
                    .collect::<Vec<_>>()
            });
            rs.map(|rs| {
                rs.iter().for_each(|(is_directory, child_id)| {
                    if *is_directory {
                        self.inodes[*child_id]
                            .direntries
                            .insert("..".to_string(), idx as i64);
                    }
                });
            });
        }

        // Relocate local data if any.
        self.inodedata
            .remove(&old_idx)
            .map(|d| self.inodedata.insert(idx as i64, d));

        self.inodes.get_mut(old_idx as usize).map(|old_inode| {
            // Retire old reference information.
            old_inode.direntries = HashMap::new();
            old_inode.nlinks = 0;
        });
        return idx;
    }

    fn create_directory(&mut self, name: &str, parentid: i64) -> i64 {
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
        if parentid >= 0 && parent_forwarder {
            let foreign_id = self
                .follow_fs_mut(parentid as usize)
                .create_directory(name, parent_foreign_id);
            return self.create_forwarder(parent_mount_id, foreign_id as _) as i64;
        }
        let mut x = self.create_inode();
        x.mode = 0x01FF | S_IFDIR;
        if parentid >= 0 {
            self.inodes.get(parentid as usize).map(|parent_node| {
                x.uid = parent_node.uid;
                x.gid = parent_node.gid;
                x.mode = (parent_node.mode & 0x1FF) | S_IFDIR;
            });
        }
        x.qid.type_ = (S_IFDIR >> 8) as u8;
        self.push_inode(x, parentid, name);
        // this.NotifyListeners(this.inodes.length-1, 'newdir');
        return (self.inodes.len() - 1) as i64;
    }

    fn create_file(&mut self, filename: &str, parentid: i64) -> i64 {
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
        self.inodes.get(parentid as usize).map(|parent_node| {
            x.uid = parent_node.uid;
            x.gid = parent_node.gid;
            x.qid.type_ = (S_IFREG >> 8) as u8;
            x.mode = (parent_node.mode & 0x1B6) | S_IFREG;
        });

        self.push_inode(x, parentid, filename);
        // this.NotifyListeners(this.inodes.length-1, 'newfile');
        return (self.inodes.len() - 1) as i64;
    }

    fn set_data(&mut self, idx: i64, buffer: Vec<u8>) {
        // Current scheme: Save all modified buffers into local inodedata.
        self.inodedata.insert(idx, buffer);
        self.inodes.get_mut(idx as usize).map(|inode| {
            if inode.status == STATUS_ON_STORAGE {
                inode.status = STATUS_OK;
                todo!()
                //this.storage.uncache(this.inodes[idx].sha256sum);
            }
        });
    }

    fn create_binary_file(&mut self, filename: &str, parentid: i64, buffer: Vec<u8>) -> i64 {
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
        self.inodes.get_mut(id as usize).map(|x| {
            x.size = buf_len as u64;
        });
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