//! Port: showing a folder to the user.

use async_trait::async_trait;
use std::path::Path;

use crate::core::application::exceptions::menu_action_error::MenuActionError;

/// Shows a folder in the Finder, creating it first when it does not exist.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FolderOpenerService: Send + Sync {
    /// Creates `folder` when missing, then shows it.
    ///
    /// # Errors
    /// The folder cannot be created or shown.
    async fn open(&self, folder: &Path) -> Result<(), MenuActionError>;
}

#[cfg(test)]
#[path = "folder_opener_service_test.rs"]
mod folder_opener_service_test;
