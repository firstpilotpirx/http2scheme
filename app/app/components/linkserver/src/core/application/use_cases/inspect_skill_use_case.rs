//! Reads the skill file of a Claude Code folder and compares it with the current text.

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;

use super::inspect_skill::InspectSkill;
use super::show_skill::ShowSkill;
use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::application::repositories::skill_repository::SkillRepository;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

/// The place and state of the skill: absent, current or outdated (design D5).
pub struct InspectSkillUseCase {
    skill: Arc<dyn ShowSkill>,
    repository: Arc<dyn SkillRepository>,
}

impl InspectSkillUseCase {
    /// A use case comparing `repository`'s file with `skill`'s text.
    #[must_use]
    pub fn new(skill: Arc<dyn ShowSkill>, repository: Arc<dyn SkillRepository>) -> Self {
        Self { skill, repository }
    }
}

#[async_trait]
impl InspectSkill for InspectSkillUseCase {
    async fn execute(&self, claude_dir: &Path) -> Result<SkillPlaceValueObject, InstallSkillError> {
        let skill = self.skill.execute().await;
        let (found, real_file) = self.repository.find(claude_dir).await?;
        let state = SkillStateEnum::of(found.as_deref(), &skill);
        Ok(SkillPlaceValueObject::new(claude_dir, real_file, state))
    }
}

#[cfg(test)]
#[path = "inspect_skill_use_case_test.rs"]
mod inspect_skill_use_case_test;
