use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Blob {
    data: Vec<u8>,
}

impl Blob {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl AsRef<[u8]> for Blob {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}
