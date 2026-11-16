//! The refresh at launch over the remembered folder and the server side's skill.

use std::sync::Arc;

use async_trait::async_trait;

use super::refresh_skill::RefreshSkill;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::skill_folder_repository::SkillFolderRepository;
use crate::core::application::services::agent_skill_service::AgentSkillService;
use crate::core::domain::skill_state_enum::SkillStateEnum;

/// Installs the skill again only where it was installed and only when it is outdated.
pub struct RefreshSkillUseCase {
    folders: Arc<dyn SkillFolderRepository>,
    skill: Arc<dyn AgentSkillService>,
}

impl RefreshSkillUseCase {
    /// A use case over the remembered folder and the skill service.
    #[must_use]
    pub fn new(folders: Arc<dyn SkillFolderRepository>, skill: Arc<dyn AgentSkillService>) -> Self {
        Self { folders, skill }
    }
}

#[async_trait]
impl RefreshSkill for RefreshSkillUseCase {
    async fn execute(&self) -> Result<(), MenuActionError> {
        let Some(folder) = self.folders.find().await? else {
            return Ok(());
        };
        let place = self
            .skill
            .inspect(&folder)
            .await
            .map_err(MenuActionError::Skill)?;
        if place.state() == SkillStateEnum::Outdated {
            self.skill
                .install(&folder)
                .await
                .map_err(MenuActionError::Skill)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "refresh_skill_use_case_test.rs"]
mod refresh_skill_use_case_test;
