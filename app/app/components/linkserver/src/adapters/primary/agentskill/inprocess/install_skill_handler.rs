//! Writes the skill into a Claude Code folder for the Agents tab (design D14).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::core::application::use_cases::install_skill::InstallSkill;

/// The skill file, the real file behind a linked `skills` folder, and the state's name.
pub type SkillAnswer = (PathBuf, Option<PathBuf>, &'static str);

/// The Agents tab's call; the place comes back as plain values, the reason as text to show.
#[derive(Clone)]
pub struct InstallSkillHandler {
    use_case: Arc<dyn InstallSkill>,
}

impl InstallSkillHandler {
    /// A handler over the install use case.
    #[must_use]
    pub fn new(use_case: Arc<dyn InstallSkill>) -> Self {
        Self { use_case }
    }

    /// Writes the skill into `claude_dir` and answers its new place and state.
    ///
    /// # Errors
    /// The file cannot be written; the reason.
    pub async fn install(&self, claude_dir: &Path) -> Result<SkillAnswer, String> {
        match self.use_case.execute(claude_dir).await {
            Ok(place) => Ok((
                place.file().to_path_buf(),
                place.real_file().map(Path::to_path_buf),
                place.state().name(),
            )),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(test)]
#[path = "install_skill_handler_test.rs"]
mod install_skill_handler_test;
