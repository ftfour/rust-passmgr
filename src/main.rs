mod model;
mod storage;
mod crypto;


use model::{Vault, Entry, FileFormat};
use storage::{load_fileformat, save_fileformat};
use std::path::PathBuf;
use anyhow::Result;
use crypto::{derive_key, generate_salt, decrypt_vault, encrypt_vault};
use std::str;

fn main() -> Result<()> {
    let mut v = Vault::default();
    v.entries.insert(
        "example.com".into(),
        Entry {
            login: "user".into(),
            password: "pass123".into(),
            notes: Some("demo".into()),
        }
    );

    let password = "test123";
    let salt = generate_salt();

    let blob = encrypt_vault(&v, password, &salt)?;
    println!("Encrypted blob size: {}", blob.len());

    let v2 = decrypt_vault(&blob, password, &salt)?;
    println!("Roundtrip ok, entries: {}", v2.entries.len());

    Ok(())
}