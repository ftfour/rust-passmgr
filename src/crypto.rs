use anyhow::{anyhow, Result};
use argon2::{Argon2, Params};
use rand::rngs::OsRng;
use rand::RngCore;

use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

use crate::model::Vault;
use serde_json;


pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 12;

pub fn generate_salt() -> [u8; SALT_LEN]{
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    salt
}

pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let params = Params::new(15000, 2, 1, None).map_err(|e| anyhow!("invalid argon2 params: {e}"))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id,
                                argon2::Version::V0x13,
                                params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("argon2 derive failed: {e}"))?;
    Ok(key)
}

pub fn encrypt_vault(vault: &Vault, password: &str, salt: &[u8]) -> Result<Vec<u8>> {
    let key = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow!("invalid key for AES-GCM: {e}"))?;
    let pt = serde_json::to_vec(vault)?;
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ct = cipher.encrypt(nonce, pt.as_ref())
        .map_err(|e| anyhow!("encrtypt failed: {e}"))?;
    let mut out = Vec::with_capacity(NONCE_LEN + ct.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ct);
    Ok(out)
}

pub fn decrypt_vault(blob: &[u8], password: &str, salt: &[u8]) -> Result<Vault> {
    if blob.len() < NONCE_LEN {
        return Err(anyhow!("blob too short"));
    }
    let (nonce_bytes, ct) = blob.split_at(NONCE_LEN);

    let key = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow!("invalid key for AES_GCM: {e}"))?;
    
    let pt = cipher.decrypt(Nonce::from_slice(nonce_bytes), ct)
        .map_err(|e| anyhow!("decryption failde (bad password or corrupted file): {e}"))?;
    
    let vault: Vault = serde_json::from_slice(&pt)?;
    Ok(vault)
}