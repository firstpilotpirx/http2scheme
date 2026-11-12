//! Input port: one request of the History tab.

use async_trait::async_trait;

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::domain::history_command_enum::HistoryCommandEnum;
use crate::core::domain::history_view_value_object::HistoryViewValueObject;

/// Runs one window request and answers what the window shows next.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait BrowseHistory: Send + Sync {
    /// Runs the use case.
    ///
    /// # Errors
    /// The request log cannot be read.
    async fn execute(
        &self,
        command: HistoryCommandEnum,
    ) -> Result<HistoryViewValueObject, MenuActionError>;
}

#[cfg(test)]
#[path = "browse_history_test.rs"]
mod browse_history_test;
