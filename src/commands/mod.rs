//! Command handlers for the `rust-passmgr` CLI.
//!
//! This module provides the implementations for each subcommand:
//! - [`handle_init`] — create a new encrypted vault file.
//! - [`handle_add`] — add a new entry to the vault.
//! - [`handle_get`] — retrieve and display a specific entry.
//! - [`handle_list`] — list all saved entries.
//! - [`handle_remove`] — delete an entry by key.
//!
//! Each function uses [`anyhow::Result`] for error propagation
//! and relies on cryptographic utilities from [`crate::crypto`].
//!
//! All handlers are designed to be user-interactive (prompting for master password)
//! and do not modify behavior when errors occur — they fail gracefully.

mod add;
mod get;
mod init;
mod list;
mod remove;
mod update;

pub use update::handle_update;
pub use add::handle_add;
pub use get::handle_get;
pub use init::handle_init;
pub use list::handle_list;
pub use remove::handle_remove;
