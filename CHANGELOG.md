# 🧩 Changelog — rust-passmgr

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/).

---

## [0.1.1] — 2025-11-11
### ✨ Added
- New `update` command — allows self-updating the app from GitHub Releases.
- Integrated [`self_update`](https://crates.io/crates/self_update) backend.
- Added automatic download progress indicator.
- Improved `commands/mod.rs` to include new subcommand.

### 🧰 Changed
- `cli.rs` now imports and routes `handle_update`.
- Documentation updated for all command handlers.

---

## [0.1.0] — 2025-11-10
### 🚀 Initial release
- AES-256-GCM encryption for password storage.
- Argon2id key derivation for master password.
- JSON vault structure (`FileFormat`, `Vault`, `Entry`).
- CLI commands: `init`, `add`, `get`, `list`, `remove`.
- Basic documentation and examples.
