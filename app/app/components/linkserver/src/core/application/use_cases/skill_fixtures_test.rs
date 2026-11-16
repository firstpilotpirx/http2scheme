//! Shared doubles of the skill use case tests.

use std::sync::Arc;

use crate::core::application::use_cases::show_skill_use_case::ShowSkillUseCase;
use crate::core::domain::agent_skill_value_object::AgentSkillValueObject;

/// The program the tests' skill names.
pub const PROGRAM: &str = "/opt/http2scheme";

/// The skill use case for [`PROGRAM`].
pub fn show() -> Arc<ShowSkillUseCase> {
    Arc::new(ShowSkillUseCase::new(PROGRAM.to_owned()))
}

/// The text of the current skill.
pub fn current() -> String {
    AgentSkillValueObject::for_program(PROGRAM).text().to_owned()
}
