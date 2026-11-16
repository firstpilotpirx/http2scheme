//! Renders the skill for the running program.

use async_trait::async_trait;

use super::show_skill::ShowSkill;
use crate::core::domain::agent_skill_value_object::AgentSkillValueObject;

/// The skill naming `program`, the full path of the running program.
pub struct ShowSkillUseCase {
    program: String,
}

impl ShowSkillUseCase {
    /// A use case naming `program` in the skill.
    #[must_use]
    pub const fn new(program: String) -> Self {
        Self { program }
    }
}

#[async_trait]
impl ShowSkill for ShowSkillUseCase {
    async fn execute(&self) -> AgentSkillValueObject {
        AgentSkillValueObject::for_program(&self.program)
    }
}

#[cfg(test)]
#[path = "show_skill_use_case_test.rs"]
mod show_skill_use_case_test;
