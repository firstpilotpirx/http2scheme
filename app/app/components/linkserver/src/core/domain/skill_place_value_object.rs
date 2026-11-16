//! Where the skill file of one Claude Code folder is, and its state.

use std::path::{Path, PathBuf};

use super::skill_state_enum::SkillStateEnum;

/// The skill file inside a Claude Code folder.
pub const SKILL_FILE: &str = "skills/http2scheme/SKILL.md";

/// The skill file of one Claude Code folder, the real file when `skills` is a link, and its state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillPlaceValueObject {
    file: PathBuf,
    real_file: Option<PathBuf>,
    state: SkillStateEnum,
}

impl SkillPlaceValueObject {
    /// The skill file of `claude_dir`.
    #[must_use]
    pub fn file_in(claude_dir: &Path) -> PathBuf {
        claude_dir.join(SKILL_FILE)
    }

    /// The place of the skill in `claude_dir`; `real_file` is set when the skills folder is a link.
    #[must_use]
    pub fn new(claude_dir: &Path, real_file: Option<PathBuf>, state: SkillStateEnum) -> Self {
        Self {
            file: Self::file_in(claude_dir),
            real_file,
            state,
        }
    }

    /// The skill file under the Claude Code folder.
    #[must_use]
    pub fn file(&self) -> &Path {
        &self.file
    }

    /// The file the write lands in when the skills folder is a link to another folder.
    #[must_use]
    pub fn real_file(&self) -> Option<&Path> {
        self.real_file.as_deref()
    }

    /// Whether the folder holds the current skill.
    #[must_use]
    pub const fn state(&self) -> SkillStateEnum {
        self.state
    }
}

#[cfg(test)]
#[path = "skill_place_value_object_test.rs"]
mod skill_place_value_object_test;
