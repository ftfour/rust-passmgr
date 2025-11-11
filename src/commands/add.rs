use std::path::PathBuf;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rpassword;
use crate::{
    crypto::{decrypt_vault, encrypt_vault},
    model::{Entry, FileFormat},
    storage::{load_fileformat, save_fileformat},
};

/// Handles the `add` subcommand.
///
/// Decrypts the existing vault, prompts the user (if necessary)
/// for password and notes, inserts a new entry, and then re-encrypts and saves
/// the updated vault to disk.
pub fn handle_add(
    file: PathBuf,
    key: String,
    login: String,
    password: Option<String>,
    notes: Option<String>,
) -> Result<()> {
    // Check if the vault file exists before proceeding
    if !file.exists() {
        println!("❌ File {:?} not found. Please run 'init' first.", file);
        return Ok(());
    }

    // Load the existing file format (contains salt and encrypted blob)
    let ff = load_fileformat(&file)?.expect("Error reading file");
    let salt = general_purpose::STANDARD.decode(&ff.salt)?;
    let blob = general_purpose::STANDARD.decode(&ff.blob)?;

    // Ask user for the master password (hidden input)
    let master = rpassword::prompt_password("Master password: ")?;
    let mut vault = decrypt_vault(&blob, &master, &salt)?;

    // Determine the password for the new entry
    let pass = match password {
        Some(p) => p,
        None => rpassword::prompt_password("Password for new entry: ")?,
    };

    // Determine the notes for the new entry
    let notes = match notes {
        Some(n) => Some(n),
        None => {
            println!("Add a note? (press Enter to skip):");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let trimmed = input.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }
    };

    // Insert the new entry into the vault’s entries map
    vault.entries.insert(
        key.clone(),
        Entry {
            login,
            password: pass,
            notes,
        },
    );

    // Re-encrypt the vault and prepare a new FileFormat
    let new_blob = encrypt_vault(&vault, &master, &salt)?;
    let new_ff = FileFormat {
        version: ff.version,
        salt: ff.salt,
        blob: general_purpose::STANDARD.encode(&new_blob),
    };

    // Save the updated file format back to disk
    save_fileformat(&file, &new_ff)?;

    // Notify user
    println!("✅ Entry added: {}", key);
    Ok(())
}
