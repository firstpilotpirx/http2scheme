//! What the user asks of the menu.

/// One request of the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuCommandEnum {
    /// The menu opens: show the current state.
    Show,
    /// Serve Links was chosen.
    ToggleServer,
    /// Start at Login was chosen.
    ToggleLoginItem,
}

#[cfg(test)]
#[path = "menu_command_enum_test.rs"]
mod menu_command_enum_test;
