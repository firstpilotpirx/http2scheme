//! Input port: write the current skill into a Claude Code folder.

use std::path::Path;

use async_trait::async_trait;

use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;

/// Writes the skill into a Claude Code folder and says where it landed.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait InstallSkill: Send + Sync {
    /// Runs the use case for `claude_dir`.
    async fn execute(&self, claude_dir: &Path) -> Result<SkillPlaceValueObject, InstallSkillError>;
}

#[cfg(test)]
#[path = "install_skill_test.rs"]
mod install_skill_test;
