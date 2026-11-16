//! Port: the skill file of a Claude Code folder.

use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::core::application::exceptions::install_skill_error::InstallSkillError;

/// Reads and writes `skills/http2scheme/SKILL.md` of a Claude Code folder.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait SkillRepository: Send + Sync {
    /// The text of the skill file (`None` when there is none) and, when the folder's `skills` is a
    /// link to another folder, the real file the skill lands in.
    async fn find(
        &self,
        claude_dir: &Path,
    ) -> Result<(Option<String>, Option<PathBuf>), InstallSkillError>;
    /// Writes the skill file, creating the folders it needs; other files stay.
    async fn save(&self, claude_dir: &Path, text: &str) -> Result<(), InstallSkillError>;
}

#[cfg(test)]
#[path = "skill_repository_test.rs"]
mod skill_repository_test;
