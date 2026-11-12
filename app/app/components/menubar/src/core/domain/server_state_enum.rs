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
    /// Turned off by the user with Serve Links.
    Off,
}

impl ServerStateEnum {
    /// The menu's status line.
    #[must_use]
    pub fn status_line(&self) -> String {
        match self {
            Self::Starting => "Starting…".to_owned(),
            Self::Running(address) => format!("Running on {address}"),
            Self::Stopped(reason) => format!("Stopped: {reason}"),
            Self::Off => "Off".to_owned(),
        }
    }

    /// Whether Serve Links shows a check mark: the user has not turned the server off.
    #[must_use]
    pub const fn is_serving(&self) -> bool {
        !matches!(self, Self::Off)
    }
}

#[cfg(test)]
#[path = "server_state_enum_test.rs"]
mod server_state_enum_test;
