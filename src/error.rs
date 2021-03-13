use std::{io, result};

use crate::Sha1Hash;

pub type Result<T> = result::Result<T, GitError>;

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("{0}")]
    IOError(#[from] io::Error),
    #[error("{0}")]
    FromHexError(#[from] hex::FromHexError),
    #[error("{0}")]
    SerdeError(#[from] bincode::Error),

    #[error("{0} is not a valid object type")]
    ParseObjectError(String),
    #[error("Not a sha1 hash")]
    SizeNotMatch,
    #[error("{0} is not a tree object")]
    NotTreeError(Sha1Hash),
    #[error("{0} is a tree object")]
    IsTreeError(Sha1Hash),
}
