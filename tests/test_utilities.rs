use serde::{Deserialize, Serialize};
use std::fs::{create_dir, write};
use std::path::Path;

/// The structure of the v0.1.x and v0.2.x configuration file.
#[derive(Serialize, Deserialize)]
pub struct ConfigToml {
    pub godot_project_name: String,
    pub modules: Vec<String>,
}

/// The structure of the v0.3.x configuration file.
#[derive(Serialize, Deserialize)]
pub struct ConfigV03X {
    pub name: String,
    pub godot_project_name: String,
    pub is_plugin: bool,
    pub modules: Vec<String>,
}

/// Some tests need to change directory to run correctly so this function is
/// called after those tests to go back to the tests directory that every
/// function expects to be in.
#[allow(dead_code)]
pub fn ensure_correct_dir() {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir_basename = current_dir.file_stem().unwrap();

    if current_dir_basename != "tests" {
        std::env::set_current_dir("tests").expect("Unable to change to tests directory");
    }
}

/// Creates a directory with a v0.1.x configuration file.
///
/// # Arguments
///
/// `is_v_0_2x` - Indicates whether the configuration file is v0.2.x or not.
#[allow(dead_code)]
pub fn create_toml_config(is_v_0_2x: bool) {
    let config = ConfigToml {
        godot_project_name: "platformer".to_owned(),
        modules: vec!["Player".to_owned(), "MainScene".to_owned()],
    };
    let config_toml = toml::to_string_pretty(&config).unwrap();

    let config_dir_path = Path::new("platformer_modules");
    let config_path = if is_v_0_2x {
        config_dir_path.join("godot-rust-cli.toml")
    } else {
        config_dir_path.join("project.toml")
    };

    create_dir(config_dir_path).unwrap();
    write(config_path, config_toml).unwrap();
}

/// Creates a directory with a v0.3.x configuration file.
#[allow(dead_code)]
pub fn create_v_0_3x_config() {
    let config = ConfigV03X {
        name: "platformer_modules".to_owned(),
        godot_project_name: "platformer".to_owned(),
        is_plugin: false,
        modules: vec!["Player".to_owned(), "MainScene".to_owned()],
    };
    let config_json = serde_json::to_string_pretty(&config).unwrap();

    let config_dir_path = Path::new("platformer_modules");
    let config_path = config_dir_path.join("godot-rust-cli.json");

    create_dir(config_dir_path).unwrap();
    write(config_path, config_json).unwrap();
}
