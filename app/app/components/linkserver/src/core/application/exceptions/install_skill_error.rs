//! Errors reading or writing the skill file.

/// The skill file cannot be read or written.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InstallSkillError {
    /// A file or folder cannot be read or written.
    #[error("{path}: {reason}")]
    Io {
        /// The file or folder.
        path: String,
        /// What went wrong.
        reason: String,
    },
}

#[cfg(test)]
#[path = "install_skill_error_test.rs"]
mod install_skill_error_test;
