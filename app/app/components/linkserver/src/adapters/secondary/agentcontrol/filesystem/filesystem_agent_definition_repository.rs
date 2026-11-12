//! `LaunchAgent` property lists in a folder.

use std::fs;
use std::path::PathBuf;

use async_trait::async_trait;

use super::agent_definition_record::{file_name, to_plist};
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;
use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;

/// Keeps one `<label>.plist` per agent in a folder.
#[derive(Debug, Clone)]
pub struct FilesystemAgentDefinitionRepository {
    dir: PathBuf,
}

impl FilesystemAgentDefinitionRepository {
    /// A repository over `dir`; the folder is created on the first write.
    #[must_use]
    pub const fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    fn path(&self, label: &str) -> PathBuf {
        self.dir.join(file_name(label))
    }
}

fn io_error(path: &std::path::Path, error: &std::io::Error) -> AgentControlError {
    AgentControlError::Io {
        reason: format!("{}: {error}", path.display()),
    }
}

#[async_trait]
impl AgentDefinitionRepository for FilesystemAgentDefinitionRepository {
    async fn exists(&self, label: &str) -> Result<bool, AgentControlError> {
        let path = self.path(label);
        path.try_exists().map_err(|error| io_error(&path, &error))
    }

    async fn save(&self, definition: &AgentDefinitionValueObject) -> Result<(), AgentControlError> {
        fs::create_dir_all(&self.dir).map_err(|error| io_error(&self.dir, &error))?;
        let path = self.path(definition.label());
        fs::write(&path, to_plist(definition)).map_err(|error| io_error(&path, &error))
    }

    async fn delete(&self, label: &str) -> Result<(), AgentControlError> {
        let path = self.path(label);
        fs::remove_file(&path).map_err(|error| io_error(&path, &error))
    }
}

#[cfg(test)]
#[path = "filesystem_agent_definition_repository_test.rs"]
mod filesystem_agent_definition_repository_test;
