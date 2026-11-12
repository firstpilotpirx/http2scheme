//! What the user asks of the menu.

use super::app_folder_enum::AppFolderEnum;

/// One request of the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuCommandEnum {
    /// The menu opens: show the current state.
    Show,
    /// Start at Login was chosen.
    ToggleLoginItem,
    /// An Open … Folder item was chosen.
    OpenFolder(AppFolderEnum),
}

#[cfg(test)]
#[path = "menu_command_enum_test.rs"]
mod menu_command_enum_test;
