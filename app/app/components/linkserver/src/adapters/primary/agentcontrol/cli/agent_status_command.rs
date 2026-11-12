//! `http2scheme status`.

use std::io::Write;
use std::sync::Arc;

use super::terminal_style_config::TerminalStyleConfig;
use crate::core::application::use_cases::check_agent_status::CheckAgentStatus;
use crate::core::domain::agent_status_value_object::AgentStatusValueObject;

/// Prints whether the agent is installed and loaded and the server answers.
pub struct AgentStatusCommand {
    use_case: Arc<dyn CheckAgentStatus>,
    address: String,
    error_log: String,
    style: TerminalStyleConfig,
}

impl AgentStatusCommand {
    /// A command that names the server `address` and, when unhealthy, the agent's `error_log`.
    #[must_use]
    pub fn new(
        use_case: Arc<dyn CheckAgentStatus>,
        address: String,
        error_log: String,
        style: TerminalStyleConfig,
    ) -> Self {
        Self {
            use_case,
            address,
            error_log,
            style,
        }
    }

    /// Runs the command; the exit status is 0 only when all three facts hold.
    pub async fn run(&self, out: &mut (dyn Write + Send), err: &mut (dyn Write + Send)) -> u8 {
        let status = match self.use_case.execute().await {
            Ok(status) => status,
            Err(error) => {
                let _ = writeln!(err, "{}", self.style.fail(&error.to_string()));
                return 1;
            }
        };
        for line in self.lines(status) {
            let _ = writeln!(out, "{line}");
        }
        if status.healthy() {
            return 0;
        }
        let _ = writeln!(
            out,
            "{}{}",
            self.style.detail("  see "),
            self.style.path(&self.error_log)
        );
        1
    }

    fn lines(&self, status: AgentStatusValueObject) -> [String; 3] {
        let line = |good: bool, yes: &str, no: &str| {
            format!("{} {}", self.style.mark(good), if good { yes } else { no })
        };
        let on = format!("on {}", self.style.path(&self.address));
        [
            line(status.installed(), "installed", "not installed"),
            line(status.loaded(), "loaded", "not loaded"),
            line(
                status.answering(),
                &format!("answering {on}"),
                &format!("not answering {on}"),
            ),
        ]
    }
}

#[cfg(test)]
#[path = "agent_status_command_test.rs"]
mod agent_status_command_test;
