//! `http2scheme start`.

use std::io::Write;
use std::sync::Arc;

use super::terminal_style_config::TerminalStyleConfig;
use crate::core::application::use_cases::start_agent::StartAgent;

/// Loads the installed agent.
pub struct StartAgentCommand {
    use_case: Arc<dyn StartAgent>,
    style: TerminalStyleConfig,
}

impl StartAgentCommand {
    /// A command over the use case.
    #[must_use]
    pub fn new(use_case: Arc<dyn StartAgent>, style: TerminalStyleConfig) -> Self {
        Self { use_case, style }
    }

    /// Runs the command; the exit status is 0 on success.
    pub async fn run(&self, out: &mut (dyn Write + Send), err: &mut (dyn Write + Send)) -> u8 {
        match self.use_case.execute().await {
            Ok(()) => {
                let _ = writeln!(out, "{}", self.style.done("agent started"));
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
#[path = "start_agent_command_test.rs"]
mod start_agent_command_test;
