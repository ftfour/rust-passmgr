//! rust-passmgr
//!
//! A minimal offline password manager built in Rust.
//! Provides a simple CLI for creating, encrypting, and managing password vaults.

mod crypto;
mod model;
mod storage;
mod commands;
mod cli;

use anyhow::Result;

fn check_for_updates() {
    if let Ok(list) = self_update::backends::github::ReleaseList::configure()
        .repo_owner("koilf")
        .repo_name("rust-passmgr")
        .build()
        .and_then(|l| l.fetch())
    {
        if let Some(latest) = list.first() {
            let current = env!("CARGO_PKG_VERSION");
            if latest.version != current {
                println!("⬆️  A new version is available: {}", latest.version);
                println!("Run `rust-passmgr update` to upgrade.");
            }
        }
    }
}

/// Program entry point.
fn main() -> Result<()> {
    check_for_updates();
    cli::run()
}
