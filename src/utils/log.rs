/// log - Utilities that deal with logging text to the console and maybe in the
/// future, to a file.
use colored::Colorize;

/// Logs an error message to the console.
///
/// # Arguments
///
/// `message` - The message to log to the console.
pub fn log_error_to_console(message: &str) {
    println!("[error] {}", message.red());
}

/// Logs an informational message to the console.
///
/// # Arguments
///
/// `message` - The message to log to the console.
pub fn log_info_to_console(message: &str) {
    println!("[info] {}", message.cyan());
}

/// Logs a success message to the console.
///
/// # Arguments
///
/// `message` - The message to log to the console.
pub fn log_success_to_console(message: &str) {
    println!("[success] {}", message.green());
}
