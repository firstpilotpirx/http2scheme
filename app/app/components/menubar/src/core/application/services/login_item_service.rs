//! Port: the app's entry in the macOS login items.

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use async_trait::async_trait;

/// Reads and changes whether macOS starts the app at login.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait LoginItemService: Send + Sync {
    /// The login item's state now.
    async fn status(&self) -> LoginItemStateEnum;
    /// Adds the app to the login items.
    ///
    /// # Errors
    /// macOS refused.
    async fn register(&self) -> Result<(), MenuActionError>;
    /// Removes the app from the login items.
    ///
    /// # Errors
    /// macOS refused.
    async fn unregister(&self) -> Result<(), MenuActionError>;
    /// Opens the Login Items pane of System Settings.
    ///
    /// # Errors
    /// The pane cannot be opened.
    async fn open_system_settings(&self) -> Result<(), MenuActionError>;
}

#[cfg(test)]
#[path = "login_item_service_test.rs"]
mod login_item_service_test;
