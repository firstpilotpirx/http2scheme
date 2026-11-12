//! What the menu shows at one moment.

use super::login_item_state_enum::LoginItemStateEnum;
use super::server_state_enum::ServerStateEnum;

/// The menu's words and the Start at Login check mark.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuViewValueObject {
    title: String,
    status_line: String,
    serving: bool,
    login: LoginItemStateEnum,
}

impl MenuViewValueObject {
    /// The view of the app `version` with this server and login item.
    #[must_use]
    pub fn new(version: &str, server: &ServerStateEnum, login: LoginItemStateEnum) -> Self {
        Self {
            title: format!("http2scheme {version}"),
            status_line: server.status_line(),
            serving: server.is_serving(),
            login,
        }
    }

    /// The app name with its version.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The server's status line.
    #[must_use]
    pub fn status_line(&self) -> &str {
        &self.status_line
    }

    /// Whether Serve Links shows a check mark.
    #[must_use]
    pub const fn serve_checked(&self) -> bool {
        self.serving
    }

    /// The words of Start at Login.
    #[must_use]
    pub const fn login_title(&self) -> &'static str {
        self.login.title()
    }

    /// Whether Start at Login shows a check mark.
    #[must_use]
    pub const fn login_checked(&self) -> bool {
        self.login.is_checked()
    }

    /// Whether Start at Login can be chosen.
    #[must_use]
    pub const fn login_enabled(&self) -> bool {
        self.login.is_available()
    }
}

#[cfg(test)]
#[path = "menu_view_value_object_test.rs"]
mod menu_view_value_object_test;
