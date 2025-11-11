mod model;
mod storage;
mod crypto;


use model::{Vault, Entry, FileFormat};
use storage::{load_fileformat, save_fileformat};
use crypto::{derive_key, generate_salt, decrypt_vault, encrypt_vault};

use clap::{Parser, Subcommand};
use base64::{engine::general_purpose, Engine as _};
use rpassword;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Parser)]
#[command(name="rust-passmgr")]
#[command(about="Мини-менеджер паролей на Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long, default_value = "vault.json")]
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { file } => {
            if file.exists() {
                println!("Файл {:?} существует, не перезаписываю.", file);
                return Ok(());
            }
            
            let pass1 = rpassword::prompt_password("Введите мастер-пароль: ")?;
            let pass2 = rpassword::prompt_password("Повторите пароль: ")?;
            if pass1 != pass2 {
                println!("Пароли не совпадают");
                return Ok(());
            }

            let salt = generate_salt();

            let vault = Vault::default();

            let blob = encrypt_vault(&vault, &pass1, &salt)?;

            let ff = FileFormat {
                version: 1,
                salt: general_purpose::STANDARD.encode(&salt),
                blob: general_purpose::STANDARD.encode(&blob),
            };

            save_fileformat(&file, &ff)?;
            println!("Хранилище создано {:?}", file);
        }
    }

    Ok(())
}