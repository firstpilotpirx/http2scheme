//! The remembered Claude Code folder: one line, the path, in the settings folder (design D7).

use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::skill_folder_repository::SkillFolderRepository;

const RECORD: &str = "agent-skill-folder";

/// Keeps the folder as `<settings folder>/agent-skill-folder`.
pub struct FilesystemSkillFolderRepository {
    file: PathBuf,
}

impl FilesystemSkillFolderRepository {
    /// The record in `settings_dir`.
    #[must_use]
    pub fn new(settings_dir: &Path) -> Self {
        Self {
            file: settings_dir.join(RECORD),
        }
    }

    fn error(&self, error: &std::io::Error) -> MenuActionError {
        MenuActionError::SkillFolder {
            path: self.file.display().to_string(),
            reason: error.to_string(),
        }
    }
}

#[async_trait]
impl SkillFolderRepository for FilesystemSkillFolderRepository {
    async fn find(&self) -> Result<Option<PathBuf>, MenuActionError> {
        match std::fs::read_to_string(&self.file) {
            Ok(text) => {
                let line = text.trim_end_matches('\n');
                Ok((!line.is_empty()).then(|| PathBuf::from(line)))
            }
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
            Err(error) => Err(self.error(&error)),
        }
    }

    async fn save(&self, folder: &Path) -> Result<(), MenuActionError> {
        if let Some(dir) = self.file.parent() {
            std::fs::create_dir_all(dir).map_err(|error| self.error(&error))?;
        }
        std::fs::write(&self.file, format!("{}\n", folder.display()))
            .map_err(|error| self.error(&error))
    }
}
