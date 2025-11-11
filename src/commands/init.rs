use std::path::PathBuf;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rpassword;
use crate::{
    crypto::{encrypt_vault, generate_salt},
    model::{FileFormat, Vault},
    storage::save_fileformat,
};

/// Handles the `init` subcommand.
///
/// Creates a new empty encrypted vault file and saves it to disk.
///
/// # Behavior
/// - If the file already exists, prints a warning and exits without overwriting.
/// - Prompts the user twice to confirm the master password.
/// - Generates a random salt and creates an empty vault.
/// - Encrypts and saves the vault as a JSON file containing the salt and ciphertext.
///
/// # Errors
/// Returns an error if encryption or file operations fail.
pub fn handle_init(file: PathBuf) -> Result<()> {
    // Prevent overwriting existing vault file
    if file.exists() {
        println!("⚠️  File {:?} already exists. Not overwriting.", file);
        return Ok(());
    }

    // Prompt user for master password twice
    let pass1 = rpassword::prompt_password("Enter master password: ")?;
    let pass2 = rpassword::prompt_password("Confirm password: ")?;
    if pass1 != pass2 {
        println!("Passwords do not match.");
        return Ok(());
    }

    // Generate salt and create an empty vault
    let salt = generate_salt();
    let vault = Vault::default();

    // Encrypt empty vault using master password
    let blob = encrypt_vault(&vault, &pass1, &salt)?;

    // Encode salt and ciphertext to base64 for storage
    let ff = FileFormat {
        version: 1,
        salt: general_purpose::STANDARD.encode(&salt),
        blob: general_purpose::STANDARD.encode(&blob),
    };

    // Save to disk
    save_fileformat(&file, &ff)?;
    println!("✅ Vault created: {:?}", file);
    Ok(())
}
