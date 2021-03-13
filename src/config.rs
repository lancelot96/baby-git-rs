use std::{env, path::PathBuf};

use chrono::{DateTime, Utc};
use whoami::{hostname, realname, username};

use crate::Result;

const DB_ENVIRONMENT: &str = "SHA1_FILE_DIRECTORY";
const DEFAULT_DB_ENVIRONMENT: &str = "objects";

const COMMITTER_NAME_ENV: &str = "COMMITTER_NAME";
const COMMITTER_EMAIL_ENV: &str = "COMMITTER_EMAIL";

#[derive(Debug)]
pub struct Config {
    pub repo_path: PathBuf,
    pub obj_db_path: PathBuf,
    pub cache_path: PathBuf,

    pub author_name: String,
    pub author_email: String,

    pub committer_name: String,
    pub committer_email: String,

    pub commit_date: DateTime<Utc>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let repo_path = PathBuf::from(".dircache");
        let obj_db_path = match env::var(DB_ENVIRONMENT) {
            Ok(path) => PathBuf::from(path),
            Err(_) => repo_path.join(DEFAULT_DB_ENVIRONMENT),
        };
        let cache_path = repo_path.join("index");

        let author_name = realname();
        let author_email = format!("{}@{}", username(), hostname());

        let committer_name = env::var(COMMITTER_NAME_ENV).unwrap_or_else(|_| author_name.clone());
        let committer_email =
            env::var(COMMITTER_EMAIL_ENV).unwrap_or_else(|_| author_email.clone());

        Ok(Self {
            repo_path,
            obj_db_path,
            cache_path,
            author_name,
            author_email,
            committer_name,
            committer_email,
            commit_date: Utc::now(),
        })
    }

    pub fn author_info(&self) -> String {
        format!(
            "{},,, <{}> {}",
            self.author_name, self.author_email, self.commit_date,
        )
    }

    pub fn committer_info(&self) -> String {
        format!(
            "{},,, <{}> {}",
            self.committer_name, self.committer_email, self.commit_date,
        )
    }
}
