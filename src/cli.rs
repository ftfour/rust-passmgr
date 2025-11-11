use anyhow::Result;
use clap::{Parser, Subcommand, CommandFactory};
use std::path::PathBuf;

use crate::commands::{handle_init, handle_add, handle_list, handle_get, handle_remove};

/// 🔐 Minimal password manager written in Rust.
///
/// Provides a simple offline CLI for creating, viewing,
/// and managing an encrypted JSON-based password vault.
#[derive(Parser)]
#[command(
    name = "rust-passmgr",
    author = "Ersan Egorov <github.com/koilf>",
    version,
    about = "A simple offline password manager",
    long_about = r#"A minimal offline password manager written in Rust.

Subcommands:
  init      Create a new vault (vault.json)
  add       Add a new entry (key, login, password, note)
  list      Show all saved keys
  get       Display a specific entry
  remove    Delete an entry from the vault
  help      Show help information

Examples:
  rust-passmgr init
  rust-passmgr add --file vault.json example.com user123
  rust-passmgr list
  rust-passmgr get example.com
  rust-passmgr remove example.com
"#,
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands for the CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new encrypted vault file.
    Init {
        /// Path to the vault file (default: vault.json)
        #[arg(short, long, default_value = "vault.json")]
        file: PathBuf,
    },
    /// Adds a new entry to the vault.
    Add {
        /// Path to the vault file (default: vault.json)
        #[arg(short, long, default_value = "vault.json")]
        file: PathBuf,
        /// Unique key name for the entry.
        key: String,
        /// Login or username for the entry.
        login: String,
        /// Optional password; will be prompted if omitted.
        #[arg(short, long)]
        password: Option<String>,
        /// Optional notes or description.
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// Lists all keys currently stored in the vault.
    List {
        #[arg(short, long, default_value = "vault.json")]
        file: PathBuf,
    },
    /// Displays a specific entry by key.
    Get {
        #[arg(short, long, default_value = "vault.json")]
        file: PathBuf,
        /// The key name of the entry to display.
        key: String,
    },
    /// Removes an entry from the vault by key.
    Remove {
        #[arg(short, long, default_value = "vault.json")]
        file: PathBuf,
        /// The key name of the entry to delete.
        key: String,
    },
    /// Displays help for the entire program or a specific subcommand.
    Help {
        /// Optional: name of the subcommand to show help for.
        #[arg(value_name = "COMMAND")]
        command: Option<String>,
    },
}

/// Entry point for the CLI.
///
/// Parses arguments, matches the selected subcommand,
/// and invokes the corresponding handler from [`crate::commands`].
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { file }) => handle_init(file)?,
        Some(Commands::Add { file, key, login, password, notes }) => {
            handle_add(file, key, login, password, notes)?
        }
        Some(Commands::List { file }) => handle_list(file)?,
        Some(Commands::Get { file, key }) => handle_get(file, key)?,
        Some(Commands::Remove { file, key }) => handle_remove(file, key)?,
        Some(Commands::Help { command }) => show_help(command)?,
        None => {
            Cli::command().print_help()?;
        }
    }

    Ok(())
}

/// Prints help for the main CLI or a specific subcommand.
///
/// # Arguments
/// * `command` — optional subcommand name to display help for.
///
/// # Behavior
/// If the provided command does not exist, prints an error message
/// and then displays the global help list.
fn show_help(command: Option<String>) -> Result<()> {
    let mut cmd = Cli::command();
    match command {
        Some(name) => {
            if let Some(sub) = cmd.find_subcommand_mut(&name) {
                sub.print_help()?;
            } else {
                eprintln!("❌ Unknown command: {name}\n");
                cmd.print_help()?;
            }
        }
        None => {
            cmd.print_help()?;
        }
    }
    Ok(())
}
