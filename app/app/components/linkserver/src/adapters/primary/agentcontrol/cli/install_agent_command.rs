//! `http2scheme install`.

use std::io::Write;
use std::sync::Arc;

use super::terminal_style_config::TerminalStyleConfig;
use crate::core::application::use_cases::install_agent::InstallAgent;

/// Installs and loads the agent and says where it runs.
pub struct InstallAgentCommand {
    use_case: Arc<dyn InstallAgent>,
    definition_file: String,
    program: String,
    address: String,
    style: TerminalStyleConfig,
}

impl InstallAgentCommand {
    /// A command that reports `definition_file`, `program` and the server `address`.
    #[must_use]
    pub fn new(
        use_case: Arc<dyn InstallAgent>,
        definition_file: String,
        program: String,
        address: String,
        style: TerminalStyleConfig,
    ) -> Self {
        Self {
            use_case,
            definition_file,
            program,
            address,
            style,
        }
    }

    /// Runs the command; the exit status is 0 when the agent runs.
    pub async fn run(&self, out: &mut (dyn Write + Send), err: &mut (dyn Write + Send)) -> u8 {
        if let Err(error) = self.use_case.execute().await {
            let _ = writeln!(err, "{}", self.style.fail(&error.to_string()));
            return 1;
        }
        let style = self.style;
        let _ = writeln!(
            out,
            "{}  {}",
            style.ok("agent definition written"),
            style.path(&self.definition_file)
        );
        let _ = writeln!(
            out,
            "{} {}",
            style.ok("agent loaded, runs"),
            style.path(&format!("{} serve", self.program))
        );
        let _ = writeln!(
            out,
            "{}",
            style.done(&format!("installed and running on {}", self.address))
        );
        0
    }
}

#[cfg(test)]
#[path = "install_agent_command_test.rs"]
mod install_agent_command_test;
