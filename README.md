# 🔐 rust-passmgr

A simple **offline password manager** written in [Rust](https://www.rust-lang.org/).  
It stores your passwords in a **locally encrypted JSON vault** using **AES-256-GCM** and **Argon2id**.

---

## ✨ Features

- 🧠 Secure key derivation via **Argon2id**
- 🔒 AES-256-GCM encryption/decryption
- 💾 JSON-based storage (human-readable after decryption)
- 🧰 Minimal command-line interface
- 🪶 Fully offline — no external servers or APIs
- ⚙️ Cross-platform (Windows, Linux, macOS)

---

## 🚀 Installation

### Build from source
```bash
git clone https://github.com/koilf/rust-passmgr.git
cd rust-passmgr
cargo build --release
```

The binary will be in:
```bash
target/release/rust-passmgr
```
(Optional) Add to PATH

Move it somewhere in your PATH:
```bash
mv target/release/rust-passmgr ~/.local/bin/
```
# Usage
## Create a new vault
```bash
rust-passmgr init
```
Prompts for a master password and creates an encrypted vault.json.
## Add a new entry
```bash
rust-passmgr add example.com user123
```
## List saved entries
```bash
rust-passmgr list
```
## View a specific entry
```bash
rust-passmgr get example.com
```
## Remove an entry
```bash
rust-passmgr remove example.com
```
# 🔧 Example session
```bash
$ rust-passmgr init
Enter master password:
Confirm password:
✅ Vault created: vault.json

$ rust-passmgr add example.com user123
Master password:
Password for new entry:
✅ Entry added: example.com

$ rust-passmgr list
📋 List of saved entries:
• example.com
```
# 🧠 Technical details
| Component      | Description                               |
| -------------- | ----------------------------------------- |
| Encryption     | AES-256-GCM (authenticated encryption)    |
| KDF            | Argon2id with random 128-bit salt         |
| File format    | JSON (`version`, `salt`, `blob`)          |
| Randomness     | `rand::rngs::OsRng`                       |
| CLI            | [clap](https://crates.io/crates/clap)     |
| Error handling | [anyhow](https://crates.io/crates/anyhow) |
# 🧩 Roadmap
🔄 Self-update via GitHub Releases (rust-passmgr update)
🪟 GUI frontend using egui or Tauri
📱 Mobile version (Rust + Flutter FFI)
🔑 Password generator feature
# 🧑‍💻 Author
Ersan Egorov
📎 github.com/ftfour
# 📜 License

MIT License © 2025 Ersan Egorov
You are free to use, modify, and distribute this software with attribution.