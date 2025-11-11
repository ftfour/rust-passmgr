mod crypto;
mod model;
mod storage;

use crypto::{decrypt_vault, derive_key, encrypt_vault, generate_salt};
use model::{Entry, FileFormat, Vault};
use storage::{load_fileformat, save_fileformat};

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use clap::{Parser, Subcommand};
use rpassword;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rust-passmgr")]
#[command(about = "Мини-менеджер паролей на Rust")]
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
    Add {
        #[arg(short, long, default_value = "vault.json")]
        file: PathBuf,
        key: String,
        login: String,

        #[arg(short, long)]
        password: Option<String>,
        #[arg(short, long)]
        notes: Option<String>,
    },
    List {
        #[arg(short, long, default_value = "vault.json")]
        file: PathBuf,
    },
    Get {
        #[arg(short, long, default_value = "vault.json")]
        file: PathBuf,
        key: String,
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

        Commands::Add {
            file,
            key,
            login,
            password,
            notes,
        } => {
            if !file.exists() {
                println!("Файл {:?} не найден. Сначало запустите 'init'.", file);
                return Ok(());
            }

            let ff = load_fileformat(&file)?.expect("Ошибка при чтении файла");
            let salt = general_purpose::STANDARD.decode(&ff.salt)?;
            let blob = general_purpose::STANDARD.decode(&ff.blob)?;

            let master = rpassword::prompt_password("Мастер-пароль: ")?;
            let mut vault = decrypt_vault(&blob, &master, &salt)?;

            let pass = match password {
                Some(p) => p,
                None => rpassword::prompt_password("Пароль для новой записи: ")?,
            };
            let notes = match notes {
                Some(n) => Some(n),
                None => {
                    println!("Добавить заметку? (оставь пусто, если не нужно):");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    let trimmed = input.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.to_string())
                    }
                }
            };

            vault.entries.insert(
                key.clone(),
                Entry {
                    login,
                    password: pass,
                    notes,
                },
            );

            let new_blob = encrypt_vault(&vault, &master, &salt)?;
            let new_ff = FileFormat {
                version: ff.version,
                salt: ff.salt,
                blob: general_purpose::STANDARD.encode(&new_blob),
            };

            save_fileformat(&file, &new_ff)?;
            println!("Добавлена запись: {}", key)
        }

        Commands::List { file } => {
            if !file.exists() {
                println!("Файл {:?} не найден. Сначало запустите 'init'", file);
                return Ok(());
            }
            let ff = load_fileformat(&file)?.expect("Ошибка при чтении файла");
            let salt = general_purpose::STANDARD.decode(&ff.salt)?;
            let blob = general_purpose::STANDARD.decode(&ff.blob)?;
            let master = rpassword::prompt_password("Мастер пароль: ")?;
            let vault = decrypt_vault(&blob, &master, &salt)?;

            if vault.entries.is_empty() {
                println!("(пусто)");
            } else {
                println!("Список сохраненных записей:");
                for key in vault.entries.keys() {
                    println!("• {}", key);
                }
            }
        }
        Commands::Get { file, key } => {
            if !file.exists() {
                println!("Файл {:?} не найден. Сначала запусти `init`.", file);
                return Ok(());
            }

            let ff = load_fileformat(&file)?.expect("Ошибка при чтении файла");
            let salt = general_purpose::STANDARD.decode(&ff.salt)?;
            let blob = general_purpose::STANDARD.decode(&ff.blob)?;
            let master = rpassword::prompt_password("Мастер-пароль: ")?;
            let vault = decrypt_vault(&blob, &master, &salt)?;

            match vault.entries.get(&key) {
                Some(entry) => {
                    println!("Запись: {}", key);
                    println!("Логин: {}", entry.login);
                    println!("Пароль: {}", entry.password);
                    if let Some(notes) = &entry.notes {
                        println!("Заметка: {}", notes);
                    }
                }
                None => {
                    println!(" Запись '{}' не найдена.", key);
                }
            }
        }
    }
    Ok(())
}
