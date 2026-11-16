//! The agent skill text that teaches coding agents to write IDE links.

/// The skill template; `{{program}}` stands for the program's full path.
const TEMPLATE: &str = include_str!("../../../../../../../skills/http2scheme/SKILL.md");

/// The skill text rendered for one program path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSkillValueObject {
    text: String,
}

impl AgentSkillValueObject {
    /// The skill naming `program` for the link command.
    #[must_use]
    pub fn for_program(program: &str) -> Self {
        Self {
            text: TEMPLATE.replace("{{program}}", program),
        }
    }

    /// The whole `SKILL.md` text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
#[path = "agent_skill_value_object_test.rs"]
mod agent_skill_value_object_test;
