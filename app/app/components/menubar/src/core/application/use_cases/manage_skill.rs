//! Input port: the requests of the Agents tab and the refresh at launch.

use std::path::PathBuf;

use async_trait::async_trait;

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::domain::agents_view_value_object::AgentsViewValueObject;

/// Shows, installs and copies the skill, and keeps the installed copy current.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ManageSkill: Send + Sync {
    /// The view of `folder`, or with `None` of the remembered folder, else the default one.
    async fn show(&self, folder: Option<PathBuf>) -> AgentsViewValueObject;
    /// Installs the skill into `folder`, remembers it, and answers the new view.
    async fn install(&self, folder: PathBuf) -> AgentsViewValueObject;
    /// The skill text for the clipboard.
    async fn skill_text(&self) -> String;
    /// Rewrites the skill in the remembered folder when its text differs (design D8).
    ///
    /// # Errors
    /// The record, the skill file or the write failed.
    async fn refresh(&self) -> Result<(), MenuActionError>;
}

#[cfg(test)]
#[path = "manage_skill_test.rs"]
mod manage_skill_test;
