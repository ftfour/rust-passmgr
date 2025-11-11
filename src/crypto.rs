//! Cryptography utilities for rust-passmgr.
//!
//! Provides functions for generating salts, deriving encryption keys,
//! and encrypting/decrypting [`Vault`] data using AES-256-GCM with Argon2id.

use anyhow::{anyhow, Result};
use argon2::{Argon2, Params};
use rand::{rngs::OsRng, RngCore};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use serde_json;

use crate::model::Vault;

/// Length of the salt in bytes (128 bits).
pub const SALT_LEN: usize = 16;

/// Length of the AES-GCM nonce in bytes (96 bits).
pub const NONCE_LEN: usize = 12;

/// Length of the derived encryption key in bytes (256 bits).
pub const KEY_LEN: usize = 32;

/// Generates a cryptographically secure random salt.
///
/// Salts are used with Argon2id to ensure unique key derivation
/// even for identical passwords.
pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Derives a 256-bit key from a password and salt using Argon2id.
///
/// # Arguments
/// * `password` — user password to derive the key from.
/// * `salt` — random salt of [`SALT_LEN`] bytes.
///
/// # Errors
/// Returns an error if the parameters or hashing process fail.
///
/// # Security
/// Argon2id protects against GPU and side-channel attacks.
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; KEY_LEN]> {
    if password.is_empty() {
        return Err(anyhow!("password cannot be empty"));
    }

    const ARGON_TIME_COST: u32 = 15000;
    const ARGON_MEMORY_COST: u32 = 2;
    const ARGON_PARALLELISM: u32 = 1;

    let params = Params::new(ARGON_TIME_COST, ARGON_MEMORY_COST, ARGON_PARALLELISM, None)
        .map_err(|e| anyhow!("invalid argon2 params: {e}"))?;

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params,
    );

    let mut key = [0u8; KEY_LEN];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("argon2 derive failed: {e}"))?;
    Ok(key)
}

/// Encrypts a [`Vault`] structure into a binary blob using AES-256-GCM.
///
/// The output format is:
/// ```text
/// [ nonce (12 bytes) | ciphertext... ]
/// ```
///
/// # Arguments
/// * `vault` — reference to the vault structure.
/// * `password` — password to derive the key from.
/// * `salt` — random salt used for key derivation.
///
/// # Returns
/// A vector containing the nonce and ciphertext.
pub fn encrypt_vault(vault: &Vault, password: &str, salt: &[u8]) -> Result<Vec<u8>> {
    let key = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow!("invalid key for AES-GCM: {e}"))?;

    let pt = serde_json::to_vec(vault)?;
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ct = cipher
        .encrypt(nonce, pt.as_ref())
        .map_err(|e| anyhow!("encrypt failed: {e}"))?;

    let mut out = Vec::with_capacity(NONCE_LEN + ct.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ct);
    Ok(out)
}

/// Decrypts a binary blob back into a [`Vault`] structure.
///
/// The expected input format is:
/// ```text
/// [ nonce (12 bytes) | ciphertext... ]
/// ```
///
/// # Errors
/// Returns an error if the password is incorrect,
/// the file is corrupted, or the JSON cannot be deserialized.
pub fn decrypt_vault(blob: &[u8], password: &str, salt: &[u8]) -> Result<Vault> {
    if blob.len() < NONCE_LEN {
        return Err(anyhow!("blob too short"));
    }

    let (nonce_bytes, ct) = blob.split_at(NONCE_LEN);
    let key = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow!("invalid key for AES-GCM: {e}"))?;

    let pt = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ct)
        .map_err(|e| anyhow!("decryption failed (bad password or corrupted file): {e}"))?;

    let vault: Vault = serde_json::from_slice(&pt)?;
    Ok(vault)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Vault;

    /// Tests that encrypting and decrypting produces identical data.
    #[test]
    fn encrypt_decrypt_cycle() {
        let vault = Vault::default();
        let salt = generate_salt();
        let password = "secret";
        let enc = encrypt_vault(&vault, password, &salt).unwrap();
        let dec = decrypt_vault(&enc, password, &salt).unwrap();
        assert_eq!(vault, dec);
    }
}
