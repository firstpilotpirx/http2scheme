//! The skill through the hooks the app hands in (design D14).

use std::path::Path;

use async_trait::async_trait;

use crate::core::application::services::agent_skill_service::AgentSkillService;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::menubar_hooks::{SkillAction, SkillText};

/// Calls the app's skill hooks, which run the server side's use cases to their end.
pub struct InProcessAgentSkillGateway {
    text: SkillText,
    inspect: SkillAction,
    install: SkillAction,
}

impl InProcessAgentSkillGateway {
    /// A gateway over the three hooks.
    #[must_use]
    pub fn new(text: SkillText, inspect: SkillAction, install: SkillAction) -> Self {
        Self {
            text,
            inspect,
            install,
        }
    }
}

#[async_trait]
impl AgentSkillService for InProcessAgentSkillGateway {
    async fn text(&self) -> String {
        (self.text)()
    }

    async fn inspect(&self, folder: &Path) -> Result<SkillPlaceValueObject, String> {
        (self.inspect)(folder)
    }

    async fn install(&self, folder: &Path) -> Result<SkillPlaceValueObject, String> {
        (self.install)(folder)
    }
}

#[cfg(test)]
#[path = "in_process_agent_skill_gateway_test.rs"]
mod in_process_agent_skill_gateway_test;
