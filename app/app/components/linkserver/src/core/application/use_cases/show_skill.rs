//! Input port: the skill text for this program.

use async_trait::async_trait;

use crate::core::domain::agent_skill_value_object::AgentSkillValueObject;

/// The skill text naming this program.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ShowSkill: Send + Sync {
    /// Runs the use case.
    async fn execute(&self) -> AgentSkillValueObject;
}

#[cfg(test)]
#[path = "show_skill_test.rs"]
mod show_skill_test;
