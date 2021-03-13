use std::{io::Write, path::PathBuf};

use tracing::error;

use super::tempfile;
use crate::{Config, GitError, GitObject, Result, Sha1Hash};

pub fn cat_file(obj_hash: Sha1Hash, config: &Config) -> Result<(PathBuf, String)> {
    let obj = GitObject::read_sha1_file(&config.obj_db_path, &obj_hash)?;

    let (tmp_path, mut tmp_file) = tempfile()?;
    match &obj {
        GitObject::Blob(blob_obj) => {
            tmp_file.write_all(blob_obj.as_ref())?;
        }
        GitObject::Commit(commit_obj) => {
            tmp_file.write_all(commit_obj.to_string().as_bytes())?;
        }
        GitObject::Tree(_) => {
            error!("{} is a tree object", obj_hash);
            return Err(GitError::IsTreeError(obj_hash));
        }
    }

    Ok((tmp_path, obj.to_string()))
}
