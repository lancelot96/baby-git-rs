mod blob;
mod commit;
mod tree;

use std::{
    convert::TryInto,
    fmt, fs,
    io::{Read, Write},
    path::Path,
};

use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use snap::{read::FrameDecoder, write::FrameEncoder};

pub use self::{
    blob::Blob,
    commit::Commit,
    tree::{Tree, TreeEntry},
};
use crate::{Config, Result, Sha1Hash};

#[derive(Debug, Serialize, Deserialize)]
pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

impl GitObject {
    pub fn new_blob(blob_path: &Path) -> Result<Blob> {
        let bytes = fs::read(blob_path)?;
        let blob = Blob::new(bytes);

        Ok(blob)
    }

    pub fn new_tree() -> Tree {
        Tree::default()
    }

    pub fn new_commit(
        sha1: Sha1Hash,
        parents: Vec<Sha1Hash>,
        user_comment: String,
        config: &Config,
    ) -> Commit {
        Commit::new(
            sha1,
            parents,
            config.author_info(),
            config.committer_info(),
            user_comment,
        )
    }

    pub fn read_sha1_file(obj_db_path: &Path, sha1: &Sha1Hash) -> Result<Self> {
        let src_path = obj_db_path.join(sha1.sha1_file_name());
        let src = fs::File::open(src_path)?;

        let mut bytes = Vec::new();
        let _ = FrameDecoder::new(src).read_to_end(&mut bytes)?;

        let obj = deserialize(&bytes)?;
        Ok(obj)
    }

    pub fn write_sha1_file(&self, obj_db_path: &Path) -> Result<Sha1Hash> {
        let bytes = serialize(self)?;

        let sha1: Sha1Hash = Sha1::digest(&bytes).as_slice().try_into()?;
        let dest_path = obj_db_path.join(&sha1.sha1_file_name());

        let dest = fs::File::create(dest_path)?;
        FrameEncoder::new(dest).write_all(&bytes)?;

        Ok(sha1)
    }
}

impl fmt::Display for GitObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GitObject::Blob(_) => "blob",
            GitObject::Tree(_) => "tree",
            GitObject::Commit(_) => "commit",
        };

        write!(f, "{}", s)
    }
}

impl From<Blob> for GitObject {
    fn from(blob: Blob) -> Self {
        Self::Blob(blob)
    }
}

impl From<Tree> for GitObject {
    fn from(tree: Tree) -> Self {
        Self::Tree(tree)
    }
}

impl From<Commit> for GitObject {
    fn from(commit: Commit) -> Self {
        Self::Commit(commit)
    }
}
