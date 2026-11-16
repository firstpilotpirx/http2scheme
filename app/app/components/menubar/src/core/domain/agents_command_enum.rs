//! What the user asks of the Agents tab.

use std::path::PathBuf;

/// One request of the Agents tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentsCommandEnum {
    /// Show a folder; `None` is the remembered folder, else the default one.
    Show(Option<PathBuf>),
    /// Install or Update into the folder and remember it.
    Install(PathBuf),
    /// Copy skill text while the folder is shown.
    Copy(PathBuf),
}

#[cfg(test)]
#[path = "agents_command_enum_test.rs"]
mod agents_command_enum_test;
