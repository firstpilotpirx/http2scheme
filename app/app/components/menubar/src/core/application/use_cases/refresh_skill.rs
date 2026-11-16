//! Input port: the refresh of the installed skill at launch (design D8).

use async_trait::async_trait;

use crate::core::application::exceptions::menu_action_error::MenuActionError;

/// Rewrites the skill in the remembered folder when its text differs from the app's text.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait RefreshSkill: Send + Sync {
    /// Runs the use case.
    ///
    /// # Errors
    /// The record, the skill file or the write failed.
    async fn execute(&self) -> Result<(), MenuActionError>;
}

#[cfg(test)]
#[path = "refresh_skill_test.rs"]
mod refresh_skill_test;
