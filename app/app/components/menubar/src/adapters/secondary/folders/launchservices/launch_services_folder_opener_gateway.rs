//! Shows a folder in the Finder through `/usr/bin/open`, without a shell.

use std::path::{Path, PathBuf};
use std::process::Command;

use async_trait::async_trait;

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::services::folder_opener_service::FolderOpenerService;

/// Creates the folder when missing, then runs `<program> <folder>`.
pub struct LaunchServicesFolderOpenerGateway {
    program: PathBuf,
}

impl LaunchServicesFolderOpenerGateway {
    /// A gateway running `program` (`/usr/bin/open` in the app).
    #[must_use]
    pub const fn new(program: PathBuf) -> Self {
        Self { program }
    }
}

fn failure(folder: &Path, reason: &impl ToString) -> MenuActionError {
    MenuActionError::Folder {
        path: folder.display().to_string(),
        reason: reason.to_string(),
    }
}

#[async_trait]
impl FolderOpenerService for LaunchServicesFolderOpenerGateway {
    async fn open(&self, folder: &Path) -> Result<(), MenuActionError> {
        std::fs::create_dir_all(folder).map_err(|error| failure(folder, &error))?;
        let status = Command::new(&self.program)
            .arg(folder)
            .status()
            .map_err(|error| failure(folder, &error))?;
        if status.success() {
            Ok(())
        } else {
            Err(failure(folder, &status))
        }
    }
}

#[cfg(test)]
#[path = "launch_services_folder_opener_gateway_test.rs"]
mod launch_services_folder_opener_gateway_test;
