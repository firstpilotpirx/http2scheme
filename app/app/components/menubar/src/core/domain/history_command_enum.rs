//! What the user asks of the Link History window.

/// One request of the Link History window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryCommandEnum {
    /// The window is shown: list the latest requests.
    Show,
    /// ▶ was chosen on a row: open its link again.
    Replay(String),
}

#[cfg(test)]
#[path = "history_command_enum_test.rs"]
mod history_command_enum_test;
