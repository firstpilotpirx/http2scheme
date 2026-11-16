//! The Agents tab over the remembered folder and the server side's skill.

use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;

use super::manage_skill::ManageSkill;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::skill_folder_repository::SkillFolderRepository;
use crate::core::application::services::agent_skill_service::AgentSkillService;
use crate::core::domain::agents_view_value_object::AgentsViewValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

/// Starts from the folder last installed to, else `default_dir`; writes `home` as `~`.
pub struct ManageSkillUseCase {
    default_dir: PathBuf,
    home: PathBuf,
    folders: Arc<dyn SkillFolderRepository>,
    skill: Arc<dyn AgentSkillService>,
}

impl ManageSkillUseCase {
    /// A use case over the remembered folder and the skill service.
    #[must_use]
    pub fn new(
        default_dir: PathBuf,
        home: PathBuf,
        folders: Arc<dyn SkillFolderRepository>,
        skill: Arc<dyn AgentSkillService>,
    ) -> Self {
        Self {
            default_dir,
            home,
            folders,
            skill,
        }
    }

    async fn view(&self, folder: PathBuf, error: Option<String>) -> AgentsViewValueObject {
        match self.skill.inspect(&folder).await {
            Ok(place) => AgentsViewValueObject::new(folder, self.home.clone(), Some(place), error),
            Err(reason) => {
                AgentsViewValueObject::new(folder, self.home.clone(), None, error.or(Some(reason)))
            }
        }
    }
}

#[async_trait]
impl ManageSkill for ManageSkillUseCase {
    async fn show(&self, folder: Option<PathBuf>) -> AgentsViewValueObject {
        let folder = match folder {
            Some(folder) => folder,
            None => match self.folders.find().await {
                Ok(Some(remembered)) => remembered,
                Ok(None) => self.default_dir.clone(),
                Err(error) => {
                    log::warn!("{error}");
                    self.default_dir.clone()
                }
            },
        };
        self.view(folder, None).await
    }

    async fn install(&self, folder: PathBuf) -> AgentsViewValueObject {
        match self.skill.install(&folder).await {
            Ok(place) => {
                let error = self.folders.save(&folder).await.err().map(|e| e.to_string());
                AgentsViewValueObject::new(folder, self.home.clone(), Some(place), error)
            }
            Err(reason) => self.view(folder, Some(reason)).await,
        }
    }

    async fn skill_text(&self) -> String {
        self.skill.text().await
    }

    async fn refresh(&self) -> Result<(), MenuActionError> {
        let Some(folder) = self.folders.find().await? else {
            return Ok(());
        };
        let place = self.skill.inspect(&folder).await.map_err(MenuActionError::Skill)?;
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
#[path = "manage_skill_use_case_test.rs"]
mod manage_skill_use_case_test;
