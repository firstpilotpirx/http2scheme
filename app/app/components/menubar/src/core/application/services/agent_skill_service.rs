//! Port: the server side's skill text, its state in a folder and the install.

use std::path::Path;

use async_trait::async_trait;

use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;

/// The skill as the server side knows it; the reasons are ready to show.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AgentSkillService: Send + Sync {
    /// The skill text, the same the skill command prints.
    async fn text(&self) -> String;
    /// The skill's place and state in `folder`.
    ///
    /// # Errors
    /// The folder cannot be read; the reason.
    async fn inspect(&self, folder: &Path) -> Result<SkillPlaceValueObject, String>;
    /// Writes the skill into `folder`.
    ///
    /// # Errors
    /// The file cannot be written; the reason.
    async fn install(&self, folder: &Path) -> Result<SkillPlaceValueObject, String>;
}

#[cfg(test)]
#[path = "agent_skill_service_test.rs"]
mod agent_skill_service_test;
