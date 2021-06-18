use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::config_utils::{
    get_config_version, get_version_as_enum_variant, upgrade_config_v0_1_to_v0_2,
    upgrade_config_v0_2_to_v0_3, Version,
};
use crate::log_utils::{log_error_to_console, log_info_to_console, log_success_to_console};

lazy_static! {
    /// A HashMap that maps versions to the functions that are used to upgrade the
    /// configuration from the previous version to that version.
    pub static ref VERSION_UPGRADE_HANDLER: HashMap<Version, fn() -> ()> = {
        let mut version_upgrade_handler = HashMap::new();
        version_upgrade_handler.insert(Version::V02X, upgrade_config_v0_1_to_v0_2 as fn() -> ());
        version_upgrade_handler.insert(Version::V03X, upgrade_config_v0_2_to_v0_3 as fn() -> ());

        version_upgrade_handler
    };
}

/// Upgrades the library in the current directory to be compatible with the
/// specified version of Godot Rust CLI.
///
/// # Arguments
///
/// `version` - The version to upgrade the library to.
pub fn upgrade(version: &str) {
    let version_as_enum_value = get_version_as_enum_variant(version);
    if !is_version_in_accepted_versions(&version_as_enum_value) {
        log_error_to_console("The provided version cannot be upgraded to. Check the documentation for valid versions that can be upgraded to.");
    }

    log_info_to_console("Retrieving current version...");
    let current_config_version = get_config_version();

    // Get the index of the current version of the configuration and the
    // desired version to upgrade to in the versions vec.
    let versions = get_versions();
    let index_of_current_config_version_in_versions = &versions
        .iter()
        .position(|i| i == &current_config_version)
        .expect("Unable to get index of current configuration version while upgrading");
    let index_of_version_to_upgrade_to_in_versions = &versions
        .iter()
        .position(|i| i == &version_as_enum_value)
        .expect("Unable to get index of the version to upgrade to while upgrading");

    for (pos, v) in versions.iter().enumerate() {
        if pos < *index_of_current_config_version_in_versions || v == &current_config_version {
            // If the version is at or below the index of our current version
            // then we can continue past it.
            continue;
        }

        // Run the function associated with the version to upgrade the
        // configuration to.
        let upgrade_fn_to_run = VERSION_UPGRADE_HANDLER
            .get(&v)
            .expect("Unable to run the upgrade handler for the specified version.");
        upgrade_fn_to_run();

        if pos + 1 > *index_of_version_to_upgrade_to_in_versions {
            // If the version is past the upgrade index then we can return as
            // we are done upgrading.
            break;
        }
    }

    log_success_to_console("Upgrade complete");
}

/// Indicates whether the provided version to upgrade to is on the list of
/// supported versions.
///
/// # Arguments
///
/// `version` - The version to upgrade the library to.
fn is_version_in_accepted_versions(version: &Version) -> bool {
    let accepted_versions = get_versions();
    if accepted_versions.iter().any(|i| &i == &version) {
        return true;
    }
    return false;
}

/// Returns the versions used while upgrading the configuration.
fn get_versions() -> Vec<Version> {
    return vec![Version::V01X, Version::V02X, Version::V03X];
}
