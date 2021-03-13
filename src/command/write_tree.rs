use tracing::debug;

use crate::{Cache, Config, GitObject, Result, Sha1Hash};

pub fn write_tree(config: &Config) -> Result<Sha1Hash> {
    let cache = Cache::read_cache(&config.cache_path)?;
    let mut tree_obj = GitObject::new_tree();

    for (blob_path, entry) in cache.entries {
        tree_obj.insert(blob_path, entry)?;
    }

    let sha1 = GitObject::from(tree_obj).write_sha1_file(&config.obj_db_path)?;
    debug!("sha1 = {}", sha1);

    Ok(sha1)
}
