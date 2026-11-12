//! The first-launch record: an empty marker file in the settings folder.

use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::first_launch_repository::FirstLaunchRepository;

const MARKER: &str = "login-item-offered";

/// Keeps the record as `<settings folder>/login-item-offered`.
pub struct FilesystemFirstLaunchRepository {
    file: PathBuf,
}

impl FilesystemFirstLaunchRepository {
    /// The record in `settings_dir`.
    #[must_use]
    pub fn new(settings_dir: &Path) -> Self {
        Self {
            file: settings_dir.join(MARKER),
        }
    }

    fn error(&self, error: &std::io::Error) -> MenuActionError {
        MenuActionError::FirstLaunch {
            path: self.file.display().to_string(),
            reason: error.to_string(),
        }
    }
}

#[async_trait]
impl FirstLaunchRepository for FilesystemFirstLaunchRepository {
    async fn exists(&self) -> Result<bool, MenuActionError> {
        self.file.try_exists().map_err(|error| self.error(&error))
    }

    async fn record(&self) -> Result<(), MenuActionError> {
        if let Some(folder) = self.file.parent() {
            std::fs::create_dir_all(folder).map_err(|error| self.error(&error))?;
        }
        std::fs::write(&self.file, b"").map_err(|error| self.error(&error))
    }
}

#[cfg(test)]
#[path = "filesystem_first_launch_repository_test.rs"]
mod filesystem_first_launch_repository_test;
