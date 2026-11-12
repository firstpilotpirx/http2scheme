//! The folders the menu opens.

/// A folder of the app the user can open from the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppFolderEnum {
    /// The folder of the settings file and the key.
    Settings,
    /// The folder of the request log.
    Logs,
}

impl AppFolderEnum {
    /// The words of its menu item.
    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::Settings => "Open Settings Folder",
            Self::Logs => "Open Logs Folder",
        }
    }
}

#[cfg(test)]
#[path = "app_folder_enum_test.rs"]
mod app_folder_enum_test;
