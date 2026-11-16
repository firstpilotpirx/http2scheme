//! The skill text, its state and the install, for the menu bar app's Agents tab (design D14).

use std::path::Path;
use std::sync::Arc;

use crate::core::application::use_cases::inspect_skill::InspectSkill;
use crate::core::application::use_cases::install_skill::InstallSkill;
use crate::core::application::use_cases::show_skill::ShowSkill;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;

/// The Agents tab's calls into the component; reasons come back as text to show.
#[derive(Clone)]
pub struct AgentSkillHandler {
    show: Arc<dyn ShowSkill>,
    inspect: Arc<dyn InspectSkill>,
    install: Arc<dyn InstallSkill>,
}

impl AgentSkillHandler {
    /// A handler over the three use cases.
    #[must_use]
    pub fn new(
        show: Arc<dyn ShowSkill>,
        inspect: Arc<dyn InspectSkill>,
        install: Arc<dyn InstallSkill>,
    ) -> Self {
        Self {
            show,
            inspect,
            install,
        }
    }

    /// The skill text, the same the skill command prints.
    pub async fn text(&self) -> String {
        self.show.execute().await.text().to_owned()
    }

    /// The skill's place and state in `claude_dir`.
    ///
    /// # Errors
    /// The folder cannot be read; the reason.
    pub async fn inspect(&self, claude_dir: &Path) -> Result<SkillPlaceValueObject, String> {
        self.inspect
            .execute(claude_dir)
            .await
            .map_err(|error| error.to_string())
    }

    /// Writes the skill into `claude_dir`.
    ///
    /// # Errors
    /// The file cannot be written; the reason.
    pub async fn install(&self, claude_dir: &Path) -> Result<SkillPlaceValueObject, String> {
        self.install
            .execute(claude_dir)
            .await
            .map_err(|error| error.to_string())
    }
}

#[cfg(test)]
#[path = "agent_skill_handler_test.rs"]
mod agent_skill_handler_test;
