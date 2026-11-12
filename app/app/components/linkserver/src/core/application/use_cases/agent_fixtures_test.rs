//! Shared builders for the agent use case tests.

use std::path::PathBuf;

use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;

pub const LABEL: &str = "io.example.agent";

pub fn definition() -> AgentDefinitionValueObject {
    AgentDefinitionValueObject::new(
        LABEL.to_owned(),
        PathBuf::from("/bin/tool"),
        PathBuf::from("/logs"),
    )
}
