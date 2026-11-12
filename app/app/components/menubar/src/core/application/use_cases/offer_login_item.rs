//! Input port: the first launch from Applications starts the app at login.

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use async_trait::async_trait;

/// Adds the app to the login items once, at its first launch from Applications.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait OfferLoginItem: Send + Sync {
    /// Runs the use case.
    ///
    /// # Errors
    /// macOS refused, or the record cannot be read or written.
    async fn execute(&self) -> Result<(), MenuActionError>;
}

#[cfg(test)]
#[path = "offer_login_item_test.rs"]
mod offer_login_item_test;
