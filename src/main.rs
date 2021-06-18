/// Godot Rust CLI Upgrader - A tool used to upgrade your Godot Rust CLI project
/// to be compatible with the latest version of Godot Rust CLI.
///
/// Since Godot Rust CLI undergoes breaking changes pre-1.0.0 and will naturally
/// undergo breaking changes between major versions, this tool helps you upgrade
/// your project so that you don't have to start over.
use structopt::StructOpt;

mod upgrade;

#[path = "./utils/config.rs"]
mod config_utils;
#[path = "./utils/log.rs"]
mod log_utils;

/// Describes the commands available to the user.
#[derive(Debug, StructOpt)]
#[structopt(
    about = "Provides an easy way to upgrade your Godot Rust CLI project to be compatible with the latest version of Godot Rust CLI"
)]
enum GodotRustCliUpgrader {
    /// Upgrades the provided library to be compatible with the latest version
    /// of Godot Rust CLI. This command has to be run from within the library
    /// to upgrade.
    ///
    /// Available versions to upgrade to: "0.2.x", "0.3.x".
    ///
    /// Usage: godot-rust-cli-upgrader --version 0.3.x
    Upgrade {
        /// The version to upgrade the library to.
        #[structopt()]
        version: String,
    },
}

fn main() {
    match GodotRustCliUpgrader::from_args() {
        GodotRustCliUpgrader::Upgrade { version } => upgrade::upgrade(&version),
    }
}
