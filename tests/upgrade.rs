use assert_cmd::prelude::*;

use std::env::set_current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;

mod test_utilities;
use test_utilities::{create_toml_config, ensure_correct_dir, ConfigToml, ConfigV03X};

/// Upgrades a v0.1.x config to v0.2.x.
#[test]
fn upgrade_config_v_0_1x_to_v_0_2x() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    create_toml_config(false);

    set_current_dir("platformer_modules")?;

    let config_str = read_to_string("project.toml")?;
    let config_toml: ConfigToml = toml::from_str(&config_str)?;

    // 1. Assert the original configuration is correct.
    assert_eq!(config_toml.godot_project_name, "platformer");
    assert_eq!(config_toml.modules, vec!["Player", "MainScene"]);

    // 2. Assert that the upgrade command was successful.
    let mut cmd_upgrade = Command::new("cargo");
    cmd_upgrade
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("upgrade")
        .arg("0.2.x");
    cmd_upgrade.assert().success();

    // 3. Assert that the old configuration file doesn't exist anymore.
    let old_config_path = Path::new("project.toml");
    assert_eq!(old_config_path.exists(), false);

    // 4. Assert that the new configuration file exists.
    let new_config_path = Path::new("godot-rust-cli.toml");
    assert_eq!(new_config_path.exists(), true);

    let new_config_str = read_to_string(new_config_path)?;
    let new_config_toml: ConfigToml = toml::from_str(&new_config_str)?;

    // 5. Assert that the new configuration is correct.
    assert_eq!(new_config_toml.godot_project_name, "platformer");
    assert_eq!(new_config_toml.modules, vec!["Player", "MainScene"]);

    set_current_dir("../")?;

    remove_dir_all("platformer_modules")?;

    Ok(())
}

/// Upgrades a v0.1.x config to v0.3.x.
#[test]
fn upgrade_config_v_0_1x_to_v_0_3x() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    create_toml_config(false);

    set_current_dir("platformer_modules")?;

    let config_str = read_to_string("project.toml")?;
    let config_toml: ConfigToml = toml::from_str(&config_str)?;

    // 5. Assert that the configuration is correct.
    assert_eq!(config_toml.godot_project_name, "platformer");
    assert_eq!(config_toml.modules, vec!["Player", "MainScene"]);

    // 2. Assert that the upgrade command was successful.
    let mut cmd_upgrade = Command::new("cargo");
    cmd_upgrade
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("upgrade")
        .arg("0.3.x");
    cmd_upgrade.assert().success();

    // 3. Assert that the old configuration file doesn't exist anymore.
    let old_config_path = Path::new("project.toml");
    assert_eq!(old_config_path.exists(), false);

    // 4. Assert that the new configuration file exists.
    let new_config_path = Path::new("godot-rust-cli.json");
    assert_eq!(new_config_path.exists(), true);

    let new_config_str = read_to_string(new_config_path)?;
    let new_config_json: ConfigV03X = serde_json::from_str(&new_config_str)?;

    // 1. Assert the original configuration is correct.
    assert_eq!(
        new_config_json.name,
        serde_json::json!("platformer_modules")
    );
    assert_eq!(
        new_config_json.godot_project_name,
        serde_json::json!("platformer")
    );
    assert_eq!(new_config_json.is_plugin, serde_json::json!(false));
    assert_eq!(new_config_json.modules[0], "Player");
    assert_eq!(new_config_json.modules[1], "MainScene");

    set_current_dir("../")?;

    remove_dir_all("platformer_modules")?;

    Ok(())
}

/// Upgrades a v0.2.x config to v0.3.x.
#[test]
fn upgrade_config_v_0_2x_to_v_0_3x() -> Result<(), Box<dyn Error>> {
    ensure_correct_dir();

    create_toml_config(true);

    set_current_dir("platformer_modules")?;

    let config_str = read_to_string("godot-rust-cli.toml")?;
    let config_toml: ConfigToml = toml::from_str(&config_str)?;

    // 5. Assert that the configuration is correct.
    assert_eq!(config_toml.godot_project_name, "platformer");
    assert_eq!(config_toml.modules, vec!["Player", "MainScene"]);

    // 2. Assert that the upgrade command was successful.
    let mut cmd_upgrade = Command::new("cargo");
    cmd_upgrade
        .arg("run")
        .arg("--manifest-path=../../Cargo.toml")
        .arg("upgrade")
        .arg("0.3.x");
    cmd_upgrade.assert().success();

    // 3. Assert that the old configuration file doesn't exist anymore.
    let old_config_path = Path::new("project.toml");
    assert_eq!(old_config_path.exists(), false);

    // 4. Assert that the new configuration file exists.
    let new_config_path = Path::new("godot-rust-cli.json");
    assert_eq!(new_config_path.exists(), true);

    let new_config_str = read_to_string(new_config_path)?;
    let new_config_json: ConfigV03X = serde_json::from_str(&new_config_str)?;

    // 1. Assert the original configuration is correct.
    assert_eq!(
        new_config_json.name,
        serde_json::json!("platformer_modules")
    );
    assert_eq!(
        new_config_json.godot_project_name,
        serde_json::json!("platformer")
    );
    assert_eq!(new_config_json.is_plugin, serde_json::json!(false));
    assert_eq!(new_config_json.modules[0], "Player");
    assert_eq!(new_config_json.modules[1], "MainScene");

    set_current_dir("../")?;

    remove_dir_all("platformer_modules")?;

    Ok(())
}
