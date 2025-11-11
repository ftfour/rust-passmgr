use anyhow::Result;
use self_update::backends::github::Update;

/// Handles the `update` subcommand.
///
/// Checks for a newer release of rust-passmgr on GitHub and replaces
/// the current binary if an update is available.
///
/// # Behavior
/// - Compares the current version (from Cargo) with the latest GitHub release.
/// - If newer, downloads and replaces the running executable.
/// - Displays download progress in the terminal.
pub fn handle_update() -> Result<()> {
    println!("🔍 Checking for updates...");

    let status = Update::configure()
        .repo_owner("ftfour")
        .repo_name("rust-passmgr")
        .bin_name("rust-passmgr")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    match status {
        self_update::Status::UpToDate(current) => {
            println!("✅ Already up to date ({current}).");
        }
        self_update::Status::Updated(to) => {
            println!("✅ Updated to {to}.");
        }
    }

    Ok(())
}