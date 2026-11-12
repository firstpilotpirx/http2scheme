//! What the app hands the component: its version, folders and program.

use std::path::PathBuf;

/// The names, folders and program the app hands in; the component holds no machine literal.
#[derive(Debug, Clone)]
pub struct MenubarSetup {
    /// The program version.
    pub version: String,
    /// The settings folder, which also holds the first-launch record.
    pub settings_dir: PathBuf,
    /// The folder of the request log.
    pub log_dir: PathBuf,
    /// The running program.
    pub program: PathBuf,
}

#[cfg(test)]
#[path = "menubar_setup_test.rs"]
mod menubar_setup_test;
