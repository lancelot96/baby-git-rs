use std::{
    convert::{TryFrom, TryInto},
    fmt,
    path::PathBuf,
    str::FromStr,
};

use hex::{decode, encode};
use serde::{Deserialize, Serialize};

use crate::GitError;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Sha1Hash([u8; 20]);

impl Sha1Hash {
    pub fn sha1_file_name(&self) -> PathBuf {
        let hex_str = encode(self);
        PathBuf::from(&hex_str[..2]).join(&hex_str[2..])
    }
}

impl fmt::Display for Sha1Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex_str = encode(self);
        f.write_str(&hex_str)
    }
}

impl AsRef<[u8]> for Sha1Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl FromStr for Sha1Hash {
    type Err = GitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes_vec = decode(s)?;
        (&*bytes_vec).try_into()
    }
}

impl TryFrom<&[u8]> for Sha1Hash {
    type Error = GitError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let bytes = value.try_into().map_err(|_| GitError::SizeNotMatch)?;
        Ok(Self(bytes))
    }
}
