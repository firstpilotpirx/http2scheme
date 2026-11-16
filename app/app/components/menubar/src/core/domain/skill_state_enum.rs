//! Whether the shown Claude Code folder holds the current skill.

/// The skill state the Agents tab shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillStateEnum {
    /// No skill file.
    NotInstalled,
    /// The file holds the app's text.
    Installed,
    /// The file holds another text.
    Outdated,
}

impl SkillStateEnum {
    /// The words after `Skill · `.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotInstalled => "Not installed",
            Self::Installed => "Installed",
            Self::Outdated => "An older version is installed",
        }
    }

    /// The install button's title.
    #[must_use]
    pub const fn button(self) -> &'static str {
        match self {
            Self::Outdated => "Update",
            Self::NotInstalled | Self::Installed => "Install",
        }
    }
}

#[cfg(test)]
#[path = "skill_state_enum_test.rs"]
mod skill_state_enum_test;
