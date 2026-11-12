//! The app's entry in the macOS login items.

/// Whether macOS starts the app at login.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginItemStateEnum {
    /// Listed and enabled.
    On,
    /// Not listed, or listed and switched off.
    Off,
    /// Listed, waiting for the user's approval in System Settings.
    NeedsApproval,
    /// The program does not run from an app bundle.
    Unavailable,
}

impl LoginItemStateEnum {
    /// The words of the Start at Login item.
    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::NeedsApproval => "Start at Login (approve in System Settings)",
            _ => "Start at Login",
        }
    }

    /// The line under the Start at login switch of the Settings window.
    #[must_use]
    pub const fn note(self) -> &'static str {
        match self {
            Self::On | Self::Off => "The app starts when you log in to the Mac.",
            Self::NeedsApproval => "Approve the app in System Settings › Login Items.",
            Self::Unavailable => "Only the app installed in Applications can start at login.",
        }
    }

    /// Whether Start at Login shows a check mark.
    #[must_use]
    pub const fn is_checked(self) -> bool {
        matches!(self, Self::On)
    }

    /// Whether Start at Login can be chosen.
    #[must_use]
    pub const fn is_available(self) -> bool {
        !matches!(self, Self::Unavailable)
    }
}

#[cfg(test)]
#[path = "login_item_state_enum_test.rs"]
mod login_item_state_enum_test;
