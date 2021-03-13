use std::io::{stdin, Read};

use tracing::debug;

use crate::{Config, GitObject, Result, Sha1Hash};

pub fn commit_tree(
    tree_hash: Sha1Hash,
    parents: Vec<Sha1Hash>,
    config: &Config,
) -> Result<Sha1Hash> {
    let user_comment = {
        let mut buf = String::new();
        let _ = stdin().read_to_string(&mut buf)?;
        buf
    };

    let commit_obj = GitObject::new_commit(tree_hash, parents, user_comment, &config);
    let sha1 = GitObject::from(commit_obj).write_sha1_file(&config.obj_db_path)?;
    debug!("sha1 = {}", sha1);

    Ok(sha1)
}
