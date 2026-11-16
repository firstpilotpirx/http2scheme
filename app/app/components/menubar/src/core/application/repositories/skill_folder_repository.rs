//! Port: the Claude Code folder the skill was last installed to.

use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::core::application::exceptions::menu_action_error::MenuActionError;

/// Remembers the folder the Agents tab last installed the skill to (design D7).
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait SkillFolderRepository: Send + Sync {
    /// The remembered folder, or `None` when the skill was never installed from the app.
    ///
    /// # Errors
    /// The record cannot be read.
    async fn find(&self) -> Result<Option<PathBuf>, MenuActionError>;
    /// Remembers `folder`.
    ///
    /// # Errors
    /// The record cannot be written.
    async fn save(&self, folder: &Path) -> Result<(), MenuActionError>;
}

#[cfg(test)]
#[path = "skill_folder_repository_test.rs"]
mod skill_folder_repository_test;
