//! The background agent that runs the server.

use std::path::{Path, PathBuf};

/// Label, program and log folder of the agent; it always runs `serve`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDefinitionValueObject {
    label: String,
    program: PathBuf,
    log_dir: PathBuf,
}

impl AgentDefinitionValueObject {
    /// An agent running `<program> serve`, logging into `log_dir`.
    #[must_use]
    pub const fn new(label: String, program: PathBuf, log_dir: PathBuf) -> Self {
        Self {
            label,
            program,
            log_dir,
        }
    }

    /// The agent label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The program the agent runs.
    #[must_use]
    pub fn program(&self) -> &Path {
        &self.program
    }

    /// The program's arguments.
    #[must_use]
    pub const fn arguments(&self) -> [&'static str; 1] {
        ["serve"]
    }

    /// Where the agent's standard output goes.
    #[must_use]
    pub fn stdout_log(&self) -> PathBuf {
        self.log_dir.join("agent.out.log")
    }

    /// Where the agent's standard error goes.
    #[must_use]
    pub fn stderr_log(&self) -> PathBuf {
        self.log_dir.join("agent.err.log")
    }
}

#[cfg(test)]
#[path = "agent_definition_value_object_test.rs"]
mod agent_definition_value_object_test;
