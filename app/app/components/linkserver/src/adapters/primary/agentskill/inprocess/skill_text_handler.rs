//! The skill text for the menu bar app's Copy skill text (design D14).

use std::sync::Arc;

use crate::core::application::use_cases::show_skill::ShowSkill;

/// Answers the skill text, the same the skill command prints.
#[derive(Clone)]
pub struct SkillTextHandler {
    show: Arc<dyn ShowSkill>,
}

impl SkillTextHandler {
    /// A handler over the show use case.
    #[must_use]
    pub fn new(show: Arc<dyn ShowSkill>) -> Self {
        Self { show }
    }

    /// The skill text.
    pub async fn text(&self) -> String {
        self.show.execute().await.text().to_owned()
    }
}

#[cfg(test)]
#[path = "skill_text_handler_test.rs"]
mod skill_text_handler_test;
