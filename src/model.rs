use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Represents a single password entry in the vault.
///
/// Each entry contains a login, password, and optional notes.
/// The key for each entry is stored separately in the [`Vault`] map.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    /// Account login or username.
    pub login: String,
    /// Account password (stored in plaintext inside the decrypted vault).
    pub password: String,
    /// Optional notes or description for the entry.
    pub notes: Option<String>,
}

/// Represents the entire password vault.
///
/// The vault is stored as a map of key names to [`Entry`] objects.
/// It is serialized and encrypted when saved to disk.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Vault {
    /// A map of entry keys (e.g. "example.com") to their corresponding entries.
    pub entries: BTreeMap<String, Entry>,
}

/// Represents the file storage format of the encrypted vault.
///
/// This struct is serialized to JSON and written to disk.
/// It contains a version number, the salt, and the encrypted blob.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileFormat {
    /// Format version (used for backward compatibility in future releases).
    pub version: u8,
    /// Base64-encoded salt used for key derivation.
    pub salt: String,
    /// Base64-encoded AES-GCM ciphertext of the vault data.
    pub blob: String,
}
