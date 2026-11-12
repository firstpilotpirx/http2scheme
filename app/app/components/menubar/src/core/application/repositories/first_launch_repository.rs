//! Port: the record that the app once added itself to the login items.

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use async_trait::async_trait;

/// Remembers that the first launch from Applications already happened.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FirstLaunchRepository: Send + Sync {
    /// Whether the record exists.
    ///
    /// # Errors
    /// The record cannot be read.
    async fn exists(&self) -> Result<bool, MenuActionError>;
    /// Writes the record.
    ///
    /// # Errors
    /// The record cannot be written.
    async fn record(&self) -> Result<(), MenuActionError>;
}

#[cfg(test)]
#[path = "first_launch_repository_test.rs"]
mod first_launch_repository_test;
