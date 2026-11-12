//! Port: the request log, read back.

use async_trait::async_trait;

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::domain::link_record_value_object::LinkRecordValueObject;

/// Reads the latest requests of the request log.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait LinkHistoryRepository: Send + Sync {
    /// At most `limit` requests, newest first; none when the log does not exist.
    ///
    /// # Errors
    /// The log exists but cannot be read.
    async fn find_latest(
        &self,
        limit: usize,
    ) -> Result<Vec<LinkRecordValueObject>, MenuActionError>;
}

#[cfg(test)]
#[path = "link_history_repository_test.rs"]
mod link_history_repository_test;
