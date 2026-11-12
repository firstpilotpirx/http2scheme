//! The system programs the component runs, handed in by the app.

use std::path::PathBuf;

/// The paths of the macOS programs the adapters run; the component holds no machine literal.
#[derive(Debug, Clone)]
pub struct LinkserverPrograms {
    /// The macOS link and app opener.
    pub open: PathBuf,
    /// The launchd control program.
    pub launchctl: PathBuf,
    /// The `AppleScript` runner.
    pub osascript: PathBuf,
    /// The Launch Services app list tool.
    pub lsappinfo: PathBuf,
}

#[cfg(test)]
#[path = "linkserver_programs_test.rs"]
mod linkserver_programs_test;
