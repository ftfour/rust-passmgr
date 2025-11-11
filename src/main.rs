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

/// Program entry point.
fn main() -> Result<()> {
    cli::run()
}
