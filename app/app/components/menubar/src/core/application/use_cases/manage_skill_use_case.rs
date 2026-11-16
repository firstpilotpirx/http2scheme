//! The Agents tab over the remembered folder and the server side's skill.

use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;

use super::manage_skill::ManageSkill;
use crate::core::application::repositories::skill_folder_repository::SkillFolderRepository;
use crate::core::application::services::agent_skill_service::AgentSkillService;
use crate::core::domain::agents_command_enum::AgentsCommandEnum;
use crate::core::domain::agents_view_value_object::AgentsViewValueObject;

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

    async fn show(&self, folder: Option<PathBuf>) -> AgentsViewValueObject {
        match folder {
            Some(folder) => self.view(folder, None).await,
            None => self.remembered().await,
        }
    }

    /// The folder last installed to, else the default one with the reason the record is unreadable.
    async fn remembered(&self) -> AgentsViewValueObject {
        match self.folders.find().await {
            Ok(Some(remembered)) => self.view(remembered, None).await,
            Ok(None) => self.view(self.default_dir.clone(), None).await,
            Err(error) => {
                self.view(self.default_dir.clone(), Some(error.to_string()))
                    .await
            }
        }
    }

    async fn install(&self, folder: PathBuf) -> AgentsViewValueObject {
        match self.skill.install(&folder).await {
            Ok(place) => {
                let error = self
                    .folders
                    .save(&folder)
                    .await
                    .err()
                    .map(|e| e.to_string());
                AgentsViewValueObject::new(folder, self.home.clone(), Some(place), error)
            }
            Err(reason) => self.view(folder, Some(reason)).await,
        }
    }

    async fn copy(&self, folder: PathBuf) -> AgentsViewValueObject {
        let text = self.skill.text().await;
        self.view(folder, None).await.copied(text)
    }
}

#[async_trait]
impl ManageSkill for ManageSkillUseCase {
    async fn execute(&self, command: AgentsCommandEnum) -> AgentsViewValueObject {
        match command {
            AgentsCommandEnum::Show(folder) => self.show(folder).await,
            AgentsCommandEnum::Install(folder) => self.install(folder).await,
            AgentsCommandEnum::Copy(folder) => self.copy(folder).await,
        }
    }
}

#[cfg(test)]
#[path = "manage_skill_use_case_test.rs"]
mod manage_skill_use_case_test;
