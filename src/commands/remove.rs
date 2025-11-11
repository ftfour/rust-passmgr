use std::path::PathBuf;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rpassword;
use crate::{
    crypto::{decrypt_vault, encrypt_vault},
    model::FileFormat,
    storage::{load_fileformat, save_fileformat},
};

/// Handles the `remove` subcommand.
///
/// Decrypts the vault, removes an entry by key, and saves the updated vault back to disk.
///
/// # Behavior
/// - If the vault file does not exist, prints an error message and exits gracefully.
/// - Prompts the user for the master password.
/// - Removes the entry with the given key if it exists.
/// - If removal succeeds, re-encrypts and saves the updated vault.
/// - If the entry does not exist, prints a warning.
///
/// # Errors
/// Returns an error if file I/O, base64 decoding, decryption, or encryption fails.
pub fn handle_remove(file: PathBuf, key: String) -> Result<()> {
    // Ensure the vault file exists
    if !file.exists() {
        println!("❌ File {:?} not found. Please run 'init' first.", file);
        return Ok(());
    }

    // Load and decode the file contents
    let ff = load_fileformat(&file)?.expect("Error reading file");
    let salt = general_purpose::STANDARD.decode(&ff.salt)?;
    let blob = general_purpose::STANDARD.decode(&ff.blob)?;

    // Ask for master password (hidden input)
    let master = rpassword::prompt_password("Master password: ")?;
    let mut vault = decrypt_vault(&blob, &master, &salt)?;

    // Attempt to remove the specified entry
    if vault.entries.remove(&key).is_some() {
        // Re-encrypt and save the updated vault
        let new_blob = encrypt_vault(&vault, &master, &salt)?;
        let new_ff = FileFormat {
            version: ff.version,
            salt: ff.salt,
            blob: general_purpose::STANDARD.encode(&new_blob),
        };
        save_fileformat(&file, &new_ff)?;
        println!("🗑️  Removed: {}", key);
    } else {
        println!("⚠️  Entry '{}' not found.", key);
    }

    Ok(())
}
