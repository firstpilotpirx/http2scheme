//! The skill's place and state in a Claude Code folder for the Agents tab (design D14).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::core::application::use_cases::inspect_skill::InspectSkill;

/// The skill file, the real file behind a linked `skills` folder, and the state's name.
pub type SkillAnswer = (PathBuf, Option<PathBuf>, &'static str);

/// The Agents tab's call; the place comes back as plain values, the reason as text to show.
#[derive(Clone)]
pub struct InspectSkillHandler {
    use_case: Arc<dyn InspectSkill>,
}

impl InspectSkillHandler {
    /// A handler over the inspect use case.
    #[must_use]
    pub fn new(use_case: Arc<dyn InspectSkill>) -> Self {
        Self { use_case }
    }

    /// The skill's place and state in `claude_dir`.
    ///
    /// # Errors
    /// The folder cannot be read; the reason.
    pub async fn inspect(&self, claude_dir: &Path) -> Result<SkillAnswer, String> {
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
#[path = "inspect_skill_handler_test.rs"]
mod inspect_skill_handler_test;
