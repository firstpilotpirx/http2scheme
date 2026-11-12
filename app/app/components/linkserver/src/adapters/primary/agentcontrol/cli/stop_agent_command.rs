//! `http2scheme stop`.

use std::io::Write;
use std::sync::Arc;

use super::terminal_style_config::TerminalStyleConfig;
use crate::core::application::use_cases::stop_agent::StopAgent;

/// Unloads the agent until the next start or login.
pub struct StopAgentCommand {
    use_case: Arc<dyn StopAgent>,
    style: TerminalStyleConfig,
}

impl StopAgentCommand {
    /// A command over the use case.
    #[must_use]
    pub fn new(use_case: Arc<dyn StopAgent>, style: TerminalStyleConfig) -> Self {
        Self { use_case, style }
    }

    /// Runs the command; the exit status is 0 on success.
    pub async fn run(&self, out: &mut (dyn Write + Send), err: &mut (dyn Write + Send)) -> u8 {
        match self.use_case.execute().await {
            Ok(()) => {
                let _ = writeln!(
                    out,
                    "{}",
                    self.style
                        .done("agent stopped until the next start or login")
                );
                0
            }
            Err(error) => {
                let _ = writeln!(err, "{}", self.style.fail(&error.to_string()));
                1
            }
        }
    }
}
