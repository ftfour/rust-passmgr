use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub login: String,
    pub password: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Vault {
    pub entries: BTreeMap<String, Entry>,
}

#[derive(Serialize, Deserialize)]
pub struct FileFormat {
    pub version: u8,
    pub salt: String,
    pub blob: String,
}
