//! What the user asks of the History tab.

/// One request of the History tab.
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
