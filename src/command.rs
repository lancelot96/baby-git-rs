mod cat_file;
mod commit_tree;
mod init_db;
mod read_tree;
mod show_diff;
mod update_cache;
mod write_tree;

use std::{fs::File, path::PathBuf};

use rand::{
    distributions::Alphanumeric,
    {thread_rng, Rng},
};
use tracing::debug;

use crate::Result;

pub use cat_file::cat_file;
pub use commit_tree::commit_tree;
pub use init_db::init_db;
pub use read_tree::read_tree;
pub use show_diff::show_diff;
pub use update_cache::update_cache;
pub use write_tree::write_tree;

const TEMPLATE: &str = "temp_git_file_";

fn tempfile() -> Result<(PathBuf, File)> {
    let mut rng = thread_rng();
    let suffix = (0..6)
        .map(|_| rng.sample(Alphanumeric))
        .map(char::from)
        .collect::<String>();

    let filename = PathBuf::from(format!("{}{}", TEMPLATE, suffix));
    debug!("temp file {:?}", &filename);

    let file = File::create(&filename)?;
    Ok((filename, file))
}
