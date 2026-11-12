//! Where the app's folders are.

use std::path::{Path, PathBuf};

use super::app_folder_enum::AppFolderEnum;

/// The settings folder and the folder of the request log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppFoldersValueObject {
    settings: PathBuf,
    logs: PathBuf,
}

impl AppFoldersValueObject {
    /// The folders at these paths.
    #[must_use]
    pub const fn new(settings: PathBuf, logs: PathBuf) -> Self {
        Self { settings, logs }
    }

    /// The path of `folder`.
    #[must_use]
    pub fn path(&self, folder: AppFolderEnum) -> &Path {
        match folder {
            AppFolderEnum::Settings => &self.settings,
            AppFolderEnum::Logs => &self.logs,
        }
    }
}

#[cfg(test)]
#[path = "app_folders_value_object_test.rs"]
mod app_folders_value_object_test;
