//! Whether a Claude Code folder holds the current skill.

use super::agent_skill_value_object::AgentSkillValueObject;

/// The state of the skill file in one Claude Code folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillStateEnum {
    /// No skill file.
    NotInstalled,
    /// The file holds exactly the current text.
    Installed,
    /// The file holds another text: an older version or a hand edit.
    Outdated,
}

impl SkillStateEnum {
    /// The state of a file holding `found` (`None` when there is no file) against `skill`.
    #[must_use]
    pub fn of(found: Option<&str>, skill: &AgentSkillValueObject) -> Self {
        match found {
            None => Self::NotInstalled,
            Some(text) if text == skill.text() => Self::Installed,
            Some(_) => Self::Outdated,
        }
    }
}

#[cfg(test)]
#[path = "skill_state_enum_test.rs"]
mod skill_state_enum_test;
