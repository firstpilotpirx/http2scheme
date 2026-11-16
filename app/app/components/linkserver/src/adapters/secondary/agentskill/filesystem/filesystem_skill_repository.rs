//! The skill file of a Claude Code folder on the disk.

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::application::repositories::skill_repository::SkillRepository;
use crate::core::domain::skill_place_value_object::{SKILL_FILE, SkillPlaceValueObject};

const SKILLS_DIR: &str = "skills";

/// Reads and writes `<claude dir>/skills/http2scheme/SKILL.md`; a linked `skills` folder is followed.
#[derive(Debug, Clone, Copy, Default)]
pub struct FilesystemSkillRepository;

fn io_error(path: &Path, error: &std::io::Error) -> InstallSkillError {
    InstallSkillError::Io {
        path: path.display().to_string(),
        reason: error.to_string(),
    }
}

/// The real skill file when `<claude dir>/skills` is a link (design D6).
fn real_file(claude_dir: &Path) -> Result<Option<PathBuf>, InstallSkillError> {
    let skills = claude_dir.join(SKILLS_DIR);
    match fs::symlink_metadata(&skills) {
        Ok(meta) if meta.file_type().is_symlink() => {
            let target = fs::canonicalize(&skills).map_err(|error| io_error(&skills, &error))?;
            let inside = Path::new(SKILL_FILE)
                .strip_prefix(SKILLS_DIR)
                .unwrap_or_else(|_| Path::new(SKILL_FILE));
            Ok(Some(target.join(inside)))
        }
        Ok(_) => Ok(None),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(io_error(&skills, &error)),
    }
}

#[async_trait]
impl SkillRepository for FilesystemSkillRepository {
    async fn find(
        &self,
        claude_dir: &Path,
    ) -> Result<(Option<String>, Option<PathBuf>), InstallSkillError> {
        let file = SkillPlaceValueObject::file_in(claude_dir);
        let text = match fs::read_to_string(&file) {
            Ok(text) => Some(text),
            Err(error) if error.kind() == ErrorKind::NotFound => None,
            Err(error) => return Err(io_error(&file, &error)),
        };
        Ok((text, real_file(claude_dir)?))
    }

    async fn save(&self, claude_dir: &Path, text: &str) -> Result<(), InstallSkillError> {
        let file = SkillPlaceValueObject::file_in(claude_dir);
        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir).map_err(|error| io_error(dir, &error))?;
        }
        fs::write(&file, text).map_err(|error| io_error(&file, &error))
    }
}

#[cfg(test)]
#[path = "filesystem_skill_repository_test.rs"]
mod filesystem_skill_repository_test;
