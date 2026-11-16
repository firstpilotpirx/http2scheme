//! Writes the current skill into a Claude Code folder.

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;

use super::install_skill::InstallSkill;
use super::show_skill::ShowSkill;
use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::application::repositories::skill_repository::SkillRepository;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

/// Replaces the skill file with the current text and returns its place.
pub struct InstallSkillUseCase {
    skill: Arc<dyn ShowSkill>,
    repository: Arc<dyn SkillRepository>,
}

impl InstallSkillUseCase {
    /// A use case writing `skill`'s text through `repository`.
    #[must_use]
    pub fn new(skill: Arc<dyn ShowSkill>, repository: Arc<dyn SkillRepository>) -> Self {
        Self { skill, repository }
    }
}

#[async_trait]
impl InstallSkill for InstallSkillUseCase {
    async fn execute(&self, claude_dir: &Path) -> Result<SkillPlaceValueObject, InstallSkillError> {
        let skill = self.skill.execute().await;
        self.repository.save(claude_dir, skill.text()).await?;
        let (_, real_file) = self.repository.find(claude_dir).await?;
        Ok(SkillPlaceValueObject::new(
            claude_dir,
            real_file,
            SkillStateEnum::Installed,
        ))
    }
}

#[cfg(test)]
#[path = "install_skill_use_case_test.rs"]
mod install_skill_use_case_test;
