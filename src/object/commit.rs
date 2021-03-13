use std::fmt;

use serde::{Deserialize, Serialize};

use crate::Sha1Hash;

#[derive(Debug, Serialize, Deserialize)]

pub struct Commit {
    sha1: Sha1Hash,
    parents: Vec<Sha1Hash>,

    author_info: String,
    committer_info: String,
    user_comment: String,
}

impl Commit {
    pub fn new(
        sha1: Sha1Hash,
        parents: Vec<Sha1Hash>,
        author_info: String,
        committer_info: String,
        user_comment: String,
    ) -> Self {
        Self {
            sha1,
            parents,
            author_info,
            committer_info,
            user_comment,
        }
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "tree {}", self.sha1)?;
        for parent in &self.parents {
            writeln!(f, "parent {}", parent)?;
        }
        writeln!(f, "author {}", self.author_info)?;
        writeln!(f, "committer {}", self.committer_info)?;
        write!(f, "\n{}", self.user_comment)
    }
}
