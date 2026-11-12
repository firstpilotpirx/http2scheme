//! The request log file.

use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::PathBuf;

use async_trait::async_trait;

use super::event_to_log_line_mapper::to_line;
use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;

/// Appends one line per open attempt to a file.
#[derive(Debug, Clone)]
pub struct FilesystemOpenAttemptRepository {
    file: PathBuf,
}

impl FilesystemOpenAttemptRepository {
    /// A log in `file`; its folder is created on the first write.
    #[must_use]
    pub const fn new(file: PathBuf) -> Self {
        Self { file }
    }

    fn append(&self, line: &str) -> std::io::Result<()> {
        if let Some(parent) = self.file.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file)?;
        file.write_all(format!("{line}\n").as_bytes())
    }
}

#[async_trait]
impl OpenAttemptRepository for FilesystemOpenAttemptRepository {
    async fn record(&self, event: &LinkOpenAttemptedDomainEvent) -> Result<(), OpenLinkError> {
        self.append(&to_line(event))
            .map_err(|error| OpenLinkError::RecordFailed {
                reason: format!("{}: {error}", self.file.display()),
            })
    }
}

#[cfg(test)]
#[path = "filesystem_open_attempt_repository_test.rs"]
mod filesystem_open_attempt_repository_test;
