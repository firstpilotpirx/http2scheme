//! Errors controlling the background agent.

/// A failed agent command.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AgentControlError {
    /// The agent definition does not exist.
    #[error("the agent is not installed; run: http2scheme install")]
    NotInstalled,
    /// The agent definition cannot be read, written or deleted.
    #[error("agent file: {reason}")]
    Io {
        /// What went wrong.
        reason: String,
    },
    /// The agent supervisor refused a command.
    #[error("launchctl: {reason}")]
    Supervisor {
        /// What went wrong.
        reason: String,
    },
}

#[cfg(test)]
#[path = "agent_control_error_test.rs"]
mod agent_control_error_test;
