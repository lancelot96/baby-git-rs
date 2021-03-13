use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::{Cache, Config, GitObject, Result};

pub fn show_diff(config: &Config) -> Result<()> {
    let cache = Cache::read_cache(&config.cache_path)?;
    for (_, entry) in cache.entries {
        let src_path = &entry.name;
        print!("{:?}: ", &src_path);

        let curr_stat = src_path.metadata()?;
        if entry.match_stat(curr_stat) == 0 {
            println!("ok");
            continue;
        }

        println!("{}", &entry.sha1);

        let obj = GitObject::read_sha1_file(&config.obj_db_path, &entry.sha1)?;
        let blob_obj = match obj {
            GitObject::Blob(blob) => blob,
            GitObject::Tree(_) | GitObject::Commit(_) => {
                unreachable!("expect a blob object");
            }
        };

        let mut diff = Command::new("diff")
            .arg("--strip-trailing-cr")
            .arg("-u")
            .arg("-")
            .arg(src_path)
            .stdin(Stdio::piped())
            .spawn()?;

        diff.stdin
            .take()
            .expect("Child process stdin has not been captured!")
            .write_all(blob_obj.as_ref())?;
        let _ = diff.wait()?;
    }

    Ok(())
}
