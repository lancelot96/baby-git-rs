use std::{fs, path::PathBuf};

use tracing::{debug, error};

use crate::{Cache, Config, GitObject, Result};

pub fn update_cache(paths: Vec<PathBuf>, config: &Config) -> Result<()> {
    let mut cache = Cache::read_cache(&config.cache_path)?;
    let lock_path = config.cache_path.with_file_name("index.lock");
    let lock_file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&lock_path)?;

    for src_path in paths {
        let sha1 = match GitObject::new_blob(&src_path) {
            Ok(blob_obj) => GitObject::from(blob_obj).write_sha1_file(&config.obj_db_path)?,
            Err(e) => {
                error!("Ignoring path {:?} cause: {}", &src_path, e);
                continue;
            }
        };
        debug!("src_path = {:?}, sha1 = {}", src_path, sha1,);

        cache.insert(src_path, sha1)?;
    }

    cache.write_cache(&lock_file)?;
    fs::rename(&lock_path, &config.cache_path)?;

    Ok(())
}
