//! The request log file of the server, read back for the Link History window.

use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use async_trait::async_trait;

use super::log_line_to_record_mapper::to_record;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::link_history_repository::LinkHistoryRepository;
use crate::core::domain::link_record_value_object::LinkRecordValueObject;

const REQUEST_LOG: &str = "requests.log";

/// Reads `<logs folder>/requests.log`, the file the server appends to.
pub struct FilesystemLinkHistoryRepository {
    file: PathBuf,
}

impl FilesystemLinkHistoryRepository {
    /// The request log in `log_dir`.
    #[must_use]
    pub fn new(log_dir: &Path) -> Self {
        Self {
            file: log_dir.join(REQUEST_LOG),
        }
    }
}

#[async_trait]
impl LinkHistoryRepository for FilesystemLinkHistoryRepository {
    async fn latest(&self, limit: usize) -> Result<Vec<LinkRecordValueObject>, MenuActionError> {
        let text = match std::fs::read(&self.file) {
            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => {
                return Err(MenuActionError::History {
                    path: self.file.display().to_string(),
                    reason: error.to_string(),
                });
            }
        };
        Ok(text
            .lines()
            .rev()
            .filter_map(to_record)
            .take(limit)
            .collect())
    }
}

#[cfg(test)]
#[path = "filesystem_link_history_repository_test.rs"]
mod filesystem_link_history_repository_test;
