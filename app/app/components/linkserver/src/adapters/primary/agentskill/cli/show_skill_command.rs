//! `http2scheme skill`.

use std::io::Write;
use std::sync::Arc;

use crate::core::application::use_cases::show_skill::ShowSkill;

/// Prints the skill text as it is, so it can be saved as `SKILL.md`.
pub struct ShowSkillCommand {
    use_case: Arc<dyn ShowSkill>,
}

impl ShowSkillCommand {
    /// A command over the use case.
    #[must_use]
    pub fn new(use_case: Arc<dyn ShowSkill>) -> Self {
        Self { use_case }
    }

    /// Runs the command; the exit status is 0 when the text was printed.
    pub async fn run(&self, out: &mut (dyn Write + Send), err: &mut (dyn Write + Send)) -> u8 {
        let skill = self.use_case.execute().await;
        match out.write_all(skill.text().as_bytes()) {
            Ok(()) => 0,
            Err(error) => {
                let _ = writeln!(err, "{error}");
                1
            }
        }
    }
}

#[cfg(test)]
#[path = "show_skill_command_test.rs"]
mod show_skill_command_test;
