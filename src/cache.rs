use std::{
    collections::BTreeMap,
    fs::{File, Metadata},
    os::unix::prelude::MetadataExt,
    path::{Path, PathBuf},
};

use bincode::{deserialize_from, serialize_into};
use clap::crate_version;
use serde::{Deserialize, Serialize};

use crate::{Result, Sha1Hash};

const CACHE_SIGNATURE: u32 = 0x44495243;
const MTIME_CHANGED: u64 = 0x0001;
const CTIME_CHANGED: u64 = 0x0002;
const OWNER_CHANGED: u64 = 0x0004;
const MODE_CHANGED: u64 = 0x0008;
const INODE_CHANGED: u64 = 0x0010;
const DATA_CHANGED: u64 = 0x0020;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cache {
    header: CacheHeader,
    pub entries: BTreeMap<PathBuf, CacheEntry>,
}

impl Cache {
    pub fn read_cache(cache_path: &Path) -> Result<Self> {
        let file = File::open(cache_path)?;
        let cache = deserialize_from(&file)?;

        Ok(cache)
    }

    pub fn write_cache(&self, file: &File) -> Result<()> {
        serialize_into(file, self)?;
        Ok(())
    }

    pub fn insert(&mut self, src_path: PathBuf, sha1: Sha1Hash) -> Result<()> {
        let entry = CacheEntry::new(src_path.clone(), sha1)?;
        let _ = self.entries.insert(src_path, entry);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CacheHeader {
    signature: u32,
    version: String,
}

impl Default for CacheHeader {
    fn default() -> Self {
        Self {
            signature: CACHE_SIGNATURE,
            version: crate_version!().to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry {
    pub ctime: i64,
    pub ctime_nsec: i64,
    pub mtime: i64,
    pub mtime_nsec: i64,
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_size: u64,
    pub sha1: Sha1Hash,
    pub name: PathBuf,
}

impl CacheEntry {
    fn new(path: PathBuf, sha1: Sha1Hash) -> Result<Self> {
        let meta = path.metadata()?;
        let entry = Self {
            ctime: meta.ctime(),
            ctime_nsec: meta.ctime_nsec(),
            mtime: meta.mtime(),
            mtime_nsec: meta.mtime_nsec(),
            st_dev: meta.dev(),
            st_ino: meta.ino(),
            st_mode: meta.mode(),
            st_uid: meta.uid(),
            st_gid: meta.gid(),
            st_size: meta.size(),
            sha1,
            name: path,
        };
        Ok(entry)
    }

    pub fn match_stat(&self, meta: Metadata) -> u64 {
        let mut changed = 0;

        if self.mtime != meta.mtime() || self.mtime_nsec != meta.mtime_nsec() {
            changed |= MTIME_CHANGED;
        }
        if self.ctime != meta.ctime() || self.ctime_nsec != meta.ctime_nsec() {
            changed |= CTIME_CHANGED;
        }
        if self.st_uid != meta.uid() || self.st_gid != meta.gid() {
            changed |= OWNER_CHANGED;
        }
        if self.st_mode != meta.mode() {
            changed |= MODE_CHANGED;
        }
        if self.st_dev != meta.dev() || self.st_ino != meta.ino() {
            changed |= INODE_CHANGED;
        }
        if self.st_size != meta.size() {
            changed |= DATA_CHANGED;
        }

        changed
    }
}
