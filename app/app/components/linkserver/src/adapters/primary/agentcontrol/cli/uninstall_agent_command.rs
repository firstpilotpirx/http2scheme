//! `http2scheme uninstall`.

use std::io::Write;
use std::sync::Arc;

use super::terminal_style_config::TerminalStyleConfig;
use crate::core::application::use_cases::uninstall_agent::UninstallAgent;

/// Unloads and removes the agent; the settings, the key and the logs stay.
pub struct UninstallAgentCommand {
    use_case: Arc<dyn UninstallAgent>,
    style: TerminalStyleConfig,
}

impl UninstallAgentCommand {
    /// A command over the use case.
    #[must_use]
    pub fn new(use_case: Arc<dyn UninstallAgent>, style: TerminalStyleConfig) -> Self {
        Self { use_case, style }
    }

    /// Runs the command; the exit status is 0 when no agent is left.
    pub async fn run(&self, out: &mut (dyn Write + Send), err: &mut (dyn Write + Send)) -> u8 {
        match self.use_case.execute().await {
            Ok(true) => {
                let _ = writeln!(
                    out,
                    "{}",
                    self.style
                        .done("agent stopped and removed; settings, key and logs are kept")
                );
                0
            }
            Ok(false) => {
                let _ = writeln!(out, "{}", self.style.ok("nothing was installed"));
                0
            }
            Err(error) => {
                let _ = writeln!(err, "{}", self.style.fail(&error.to_string()));
                1
            }
        }
    }
}
