use crate::log_utils::{log_info_to_console, log_success_to_console};

use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::{read_to_string, remove_file, rename, write};
use std::path::Path;

/// The v0.1.0-v0.1.3 configuration object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigV01 {
    pub godot_project_name: String,
    pub modules: Vec<String>,
}

/// Defines the versions that are used throughout the program to define the
/// current version of the library and versions that can be upgraded to.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Version {
    V01X,
    V02X,
    V03X,
}

/// Returns the version of the config based on either the cli version in the
/// config or other context pre-cli version.
pub fn get_config_version() -> Version {
    let current_dir_path = current_dir()
        .expect("Unable to get current directory while getting the configuration version.");

    if current_dir_path.join("project.toml").exists() {
        // If the library has a configuration file named `project.toml` then
        // we know that it is a v0.1.x configuration.
        return Version::V01X;
    } else if current_dir_path.join("godot-rust-cli.toml").exists() {
        // If the library has a configuration file named `godot-rust-cli.toml`
        // then we know that it is a v0.2.x configuration.
        return Version::V02X;
    } else {
        // For v0.3.x forward, Godot Rust CLI switched to a JSON config.
        // However, in v0.3.x versions, the cli version property doesn't exist
        // exist in the config so we know if it's v0.3.x depending on that.
        let config = read_to_string("godot-rust-cli.json")
            .expect("Unable to read configuration file while getting the configuration version.");
        let config_json: serde_json::Value = serde_json::from_str(&config).expect(
            "Unable to parse configuration file to JSON while getting the configuration version.",
        );

        let cli_version = &config_json["cliVersion"];
        let cli_version_str = cli_version
            .as_str()
            .expect("Unable to parse cli version while getting the configuration version.");
        if cli_version.is_string() {
            return get_version_as_enum_variant(&cli_version_str);
        } else {
            return Version::V03X;
        }
    }
}

/// Returns the version as a Version enum variant.
///
/// `version_to_convert` - The version to convert to an enum variant.
pub fn get_version_as_enum_variant(version_to_convert: &str) -> Version {
    return match version_to_convert {
        "0.2.x" => Version::V02X,
        "0.3.x" => Version::V03X,
        _ => Version::V03X,
    };
}

/// Updates the configuration from v0.1.x to v0.2.x by renaming the
/// configuration file from `project.toml` to `godot-rust-cli.toml`.
pub fn upgrade_config_v0_1_to_v0_2() {
    log_info_to_console("Starting configuration upgrade from v0.1.x to v0.2.x...");

    let original_config_path = Path::new("project.toml");
    let new_config_path = Path::new("godot-rust-cli.toml");
    rename(original_config_path, new_config_path)
        .expect("Unable to rename configuration file while upgrading from v0.1.x to v0.2.x");

    log_success_to_console("Finished upgrading configuration from v0.1.x to v0.2.x");
}

/// Updates the configuration from v0.2.x to 0.3.x by replacing the
/// toml contents and file type with json.
pub fn upgrade_config_v0_2_to_v0_3() {
    log_info_to_console("Starting configuration upgrade from v0.2.x to v0.3.x...");

    let original_config_path = Path::new("godot-rust-cli.toml");
    let new_config_path = Path::new("godot-rust-cli.json");

    let toml_config_as_str = read_to_string(&original_config_path)
        .expect("Unable to read original configuration while upgrading from v0.2.x to v0.3.x.");
    let toml_config: toml::Value = toml::from_str(&toml_config_as_str)
        .expect("Unable to parse original configuration while upgrading from v0.2.x to v0.3.x.");
    let godot_project_name = toml_config["godot_project_name"]
        .as_str()
        .expect("Unable to convert godot project name while reading configuration.");

    let current_dir = std::env::current_dir()
        .expect("Unable to get current directory while upgrading from v0.2.x to v0.3.x.");
    let library_name = current_dir
        .file_stem()
        .expect("Unable to get library name while upgrading from v0.2.x to v0.3.x")
        .to_str()
        .expect("Unable to get library name while upgrading from v0.2.x to v0.3.x");

    let json_config = serde_json::json!({
        "name": library_name,
        "godot_project_name": godot_project_name,
        "is_plugin": false,
        "modules": toml_config["modules"].as_array().expect("Unable to convert modules while reading configuration.")
    });
    let json_config_as_string = serde_json::to_string_pretty(&json_config)
        .expect("Unable to convert new configuration to JSON string");

    write(new_config_path, json_config_as_string)
        .expect("Unable to create new configuration file while upgrading from v0.2.x to v0.3.x.");

    remove_file(original_config_path)
        .expect("Unable to delete old configuration file while upgrading from v0.2.x to v0.3.x.");

    log_success_to_console("Finished upgrading configuration from v0.2.x to v0.3.x");
}
