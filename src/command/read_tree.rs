use tracing::error;

use crate::{Config, GitError, GitObject, Result, Sha1Hash, TreeEntry};

pub fn read_tree(tree_hash: Sha1Hash, config: &Config) -> Result<Vec<TreeEntry>> {
    let obj = GitObject::read_sha1_file(&config.obj_db_path, &tree_hash)?;
    let tree_obj = match obj {
        GitObject::Tree(tree_obj) => tree_obj,
        GitObject::Blob(_) | GitObject::Commit(_) => {
            error!("{} is not a tree object", tree_hash);
            return Err(GitError::NotTreeError(tree_hash));
        }
    };

    let entries = tree_obj
        .entries
        .into_iter()
        .map(|x| x.1)
        .collect::<Vec<_>>();

    Ok(entries)
}
