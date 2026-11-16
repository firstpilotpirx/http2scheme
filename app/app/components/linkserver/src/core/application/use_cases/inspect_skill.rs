//! Input port: the skill's place and state in a Claude Code folder.

use std::path::Path;

use async_trait::async_trait;

use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;

/// Where the skill of a Claude Code folder lands and whether it is current.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait InspectSkill: Send + Sync {
    /// Runs the use case for `claude_dir`.
    async fn execute(&self, claude_dir: &Path) -> Result<SkillPlaceValueObject, InstallSkillError>;
}

#[cfg(test)]
#[path = "inspect_skill_test.rs"]
mod inspect_skill_test;
