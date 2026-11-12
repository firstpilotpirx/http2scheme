//! Where the server the app runs is.

/// The state of the app's server, in the words of the `serve` command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerStateEnum {
    /// Started, not listening yet.
    Starting,
    /// Listening on this address.
    Running(String),
    /// Failed to start or stopped, with the reason `serve` printed.
    Stopped(String),
}

impl ServerStateEnum {
    /// The menu's status line.
    #[must_use]
    pub fn status_line(&self) -> String {
        match self {
            Self::Starting => "Starting…".to_owned(),
            Self::Running(address) => format!("Running on {address}"),
            Self::Stopped(reason) => format!("Stopped: {reason}"),
        }
    }
}

#[cfg(test)]
#[path = "server_state_enum_test.rs"]
mod server_state_enum_test;
