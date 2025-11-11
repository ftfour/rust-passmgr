use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use anyhow::{Result, Context};
use crate::model::FileFormat;

/// Loads a [`FileFormat`] structure from a JSON file.
///
/// Returns `Ok(None)` if the file does not exist.
///
/// # Arguments
/// * `path` — Path to the vault file (usually `vault.json`).
///
/// # Returns
/// - `Ok(Some(FileFormat))` if the file was successfully read and parsed.
/// - `Ok(None)` if the file does not exist.
/// - `Err` if reading or parsing fails.
///
/// # Example
/// ```
/// let path = PathBuf::from("vault.json");
/// if let Some(ff) = load_fileformat(&path)? {
///     println!("Loaded version: {}", ff.version);
/// }
/// ```
pub fn load_fileformat(path: &PathBuf) -> Result<Option<FileFormat>> {
    if !path.exists() {
        return Ok(None);
    }

    let mut s = String::new();
    File::open(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?
        .read_to_string(&mut s)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let ff: FileFormat = serde_json::from_str(&s)
        .with_context(|| format!("Failed to parse JSON in {}", path.display()))?;

    Ok(Some(ff))
}

/// Saves a [`FileFormat`] structure to a JSON file.
///
/// The file is written in a human-readable format using
/// [`serde_json::to_string_pretty()`]. The function also calls
/// [`File::sync_all()`] to ensure all data is flushed to disk.
///
/// # Arguments
/// * `path` — Path to the file to write.
/// * `ff` — Reference to the [`FileFormat`] structure to save.
///
/// # Errors
/// Returns an error if serialization, file creation, or writing fails.
///
/// # Example
/// ```
/// let ff = FileFormat {
///     version: 1,
///     salt: "abcd".to_string(),
///     blob: "1234".to_string(),
/// };
/// save_fileformat(&PathBuf::from("vault.json"), &ff)?;
/// ```
pub fn save_fileformat(path: &PathBuf, ff: &FileFormat) -> Result<()> {
    let serialized = serde_json::to_string_pretty(ff)
        .with_context(|| "Failed to serialize FileFormat to JSON")?;

    let mut f = File::create(path)
        .with_context(|| format!("Failed to create file: {}", path.display()))?;

    f.write_all(serialized.as_bytes())
        .with_context(|| format!("Failed to write data to file: {}", path.display()))?;

    f.sync_all()
        .with_context(|| format!("Failed to sync file to disk: {}", path.display()))?;

    Ok(())
}
