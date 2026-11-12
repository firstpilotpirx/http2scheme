//! The three facts `status` reports.

/// Whether the agent is installed, loaded, and the server answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentStatusValueObject {
    installed: bool,
    loaded: bool,
    answering: bool,
}

impl AgentStatusValueObject {
    /// The three facts.
    #[must_use]
    pub const fn new(installed: bool, loaded: bool, answering: bool) -> Self {
        Self {
            installed,
            loaded,
            answering,
        }
    }

    /// The agent definition exists.
    #[must_use]
    pub const fn installed(self) -> bool {
        self.installed
    }

    /// The agent is loaded.
    #[must_use]
    pub const fn loaded(self) -> bool {
        self.loaded
    }

    /// The server answers its health address.
    #[must_use]
    pub const fn answering(self) -> bool {
        self.answering
    }

    /// All three facts hold.
    #[must_use]
    pub const fn healthy(self) -> bool {
        self.installed && self.loaded && self.answering
    }
}

#[cfg(test)]
#[path = "agent_status_value_object_test.rs"]
mod agent_status_value_object_test;
