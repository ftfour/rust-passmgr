use std::path::PathBuf;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rpassword;
use crate::{
    crypto::decrypt_vault,
    storage::load_fileformat,
};

/// Handles the `list` subcommand.
///
/// Decrypts the vault and prints the list of stored entry keys.
///
/// # Behavior
/// - If the vault file does not exist, prints an error message and exits gracefully.
/// - Prompts the user for the master password.
/// - Decrypts the vault and lists all stored entries.
/// - If there are no entries, prints `(empty)`.
///
/// # Errors
/// Returns an error if reading, decoding, or decryption fails.
pub fn handle_list(file: PathBuf) -> Result<()> {
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

    // Display the list of saved entries
    if vault.entries.is_empty() {
        println!("(empty)");
    } else {
        println!("📋 List of saved entries:");
        for key in vault.entries.keys() {
            println!("• {}", key);
        }
    }

    Ok(())
}
