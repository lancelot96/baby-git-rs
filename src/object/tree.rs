use std::{collections::BTreeMap, fmt, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{cache::CacheEntry, Result, Sha1Hash};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tree {
    pub entries: BTreeMap<PathBuf, TreeEntry>,
}

impl Tree {
    pub fn insert(&mut self, blob_path: PathBuf, cache_entry: CacheEntry) -> Result<()> {
        let entry = TreeEntry::new(cache_entry.st_mode, cache_entry.name, cache_entry.sha1);
        let _ = self.entries.insert(blob_path, entry);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeEntry {
    st_mode: u32,
    name: PathBuf,
    sha1: Sha1Hash,
}

impl TreeEntry {
    pub fn new(st_mode: u32, name: PathBuf, sha1: Sha1Hash) -> Self {
        Self {
            st_mode,
            name,
            sha1,
        }
    }
}

impl fmt::Display for TreeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:o} {:?} ({})", self.st_mode, self.name, self.sha1)
    }
}
