//! `http2scheme skill install [--claude-dir <folder>]`.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use crate::core::application::use_cases::install_skill::InstallSkill;

/// Writes the skill into a Claude Code folder and prints where it landed.
pub struct InstallSkillCommand {
    use_case: Arc<dyn InstallSkill>,
    default_dir: PathBuf,
    style: TerminalStyleConfig,
}

impl InstallSkillCommand {
    /// A command installing into `default_dir` unless the run names a folder.
    #[must_use]
    pub fn new(
        use_case: Arc<dyn InstallSkill>,
        default_dir: PathBuf,
        style: TerminalStyleConfig,
    ) -> Self {
        Self {
            use_case,
            default_dir,
            style,
        }
    }

    /// Runs the command for `claude_dir`, else the default folder; 0 when the file was written.
    pub async fn run(
        &self,
        claude_dir: Option<&Path>,
        out: &mut (dyn Write + Send),
        err: &mut (dyn Write + Send),
    ) -> u8 {
        let dir = claude_dir.unwrap_or(&self.default_dir);
        match self.use_case.execute(dir).await {
            Ok(place) => {
                let _ = writeln!(
                    out,
                    "{} {}",
                    self.style.done("skill installed:"),
                    self.style.path(&place.file().display().to_string())
                );
                if let Some(real) = place.real_file() {
                    let _ = writeln!(
                        out,
                        "{}{}{}",
                        self.style.detail("  lands in "),
                        self.style.path(&real.display().to_string()),
                        self.style.detail(" (skills is a link)")
                    );
                }
                0
            }
            Err(error) => {
                let _ = writeln!(err, "{}", self.style.fail(&error.to_string()));
                1
            }
        }
    }
}

#[cfg(test)]
#[path = "install_skill_command_test.rs"]
mod install_skill_command_test;
