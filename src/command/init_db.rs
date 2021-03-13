use std::fs::{create_dir, File};

use crate::{Cache, Config, Result};

pub fn init_db(config: &Config) -> Result<()> {
    create_dir(&config.repo_path)?;

    create_dir(&config.obj_db_path)?;
    for i in 0..256 {
        let dir = config.obj_db_path.join(format!("{:02x}", i));
        create_dir(dir)?;
    }

    let mut cache = File::create(&config.cache_path)?;
    bincode::serialize_into(&mut cache, &Cache::default())?;

    Ok(())
}
