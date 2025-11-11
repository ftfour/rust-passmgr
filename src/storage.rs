use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use anyhow::Result;
use crate::model::FileFormat;

pub fn load_fileformat(path: &PathBuf) -> Result<Option<FileFormat>> {
    if !path.exists() {
        return Ok(None);
    }

    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    let ff: FileFormat = serde_json::from_str(&s)?;
    Ok(Some(ff))
}

pub fn save_fileformat(path: &PathBuf, ff: &FileFormat) -> Result<()> {
    let serialized = serde_json::to_string_pretty(ff)?;
    let mut f = File::create(path)?;
    f.write_all(serialized.as_bytes())?;
    f.sync_all()?;
    Ok(())
}