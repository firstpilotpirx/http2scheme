//! The skill file of the shown Claude Code folder, as the server side reports it.

use std::path::{Path, PathBuf};

use super::skill_state_enum::SkillStateEnum;

/// The skill file, the real file when the skills folder is a link, and the state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillPlaceValueObject {
    file: PathBuf,
    real_file: Option<PathBuf>,
    state: SkillStateEnum,
}

impl SkillPlaceValueObject {
    /// A place reported for one folder.
    #[must_use]
    pub const fn new(file: PathBuf, real_file: Option<PathBuf>, state: SkillStateEnum) -> Self {
        Self {
            file,
            real_file,
            state,
        }
    }

    /// The skill file under the Claude Code folder.
    #[must_use]
    pub fn file(&self) -> &Path {
        &self.file
    }

    /// The file the write lands in when the skills folder is a link.
    #[must_use]
    pub fn real_file(&self) -> Option<&Path> {
        self.real_file.as_deref()
    }

    /// Whether the folder holds the app's skill.
    #[must_use]
    pub const fn state(&self) -> SkillStateEnum {
        self.state
    }
}

#[cfg(test)]
#[path = "skill_place_value_object_test.rs"]
mod skill_place_value_object_test;
