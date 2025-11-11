use std::path::PathBuf;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rpassword;
use crate::{
    crypto::decrypt_vault,
    storage::load_fileformat,
};

/// Handles the `get` subcommand.
///
/// Decrypts the vault and displays a specific entry by key.
///
/// # Arguments
/// * `file` — Path to the vault file.
/// * `key` — The unique identifier of the entry to retrieve.
///
/// # Behavior
/// - If the vault file does not exist, prints an error message and exits gracefully.
/// - Prompts the user for the master password.
/// - Decrypts the vault and looks for the requested key.
/// - Prints the entry if found, or a warning if it doesn’t exist.
///
/// # Errors
/// Returns an error if file operations, base64 decoding, or decryption fail.
pub fn handle_get(file: PathBuf, key: String) -> Result<()> {
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
    let vault = decrypt_vault(&blob, &master, &salt)?;

    // Look up the requested entry
    match vault.entries.get(&key) {
        Some(entry) => {
            println!("🔑 Entry: {}", key);
            println!("Login: {}", entry.login);
            println!("Password: {}", entry.password);
            if let Some(notes) = &entry.notes {
                println!("Notes: {}", notes);
            }
        }
        None => {
            println!("⚠️  Entry '{}' not found.", key);
        }
    }

    Ok(())
}
