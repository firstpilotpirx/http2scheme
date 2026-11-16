use std::fs;
use std::path::Path;

use super::{FilesystemSkillRepository, io_error, real_file};
use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::application::repositories::skill_repository::SkillRepository;

#[test]
fn an_io_error_names_the_path_and_the_reason() {
    let error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
    assert_eq!(
        io_error(Path::new("/c/skills"), &error),
        InstallSkillError::Io {
            path: "/c/skills".to_owned(),
            reason: "denied".to_owned(),
        }
    );
}

#[test]
fn a_plain_skills_folder_has_no_real_file() {
    let claude = tempfile::tempdir().unwrap();
    fs::create_dir_all(claude.path().join("skills")).unwrap();
    assert_eq!(real_file(claude.path()).unwrap(), None);
}

#[test]
fn a_missing_skills_folder_has_no_real_file() {
    let claude = tempfile::tempdir().unwrap();
    assert_eq!(real_file(claude.path()).unwrap(), None);
}

#[test]
fn a_broken_skills_link_is_a_reason() {
    let claude = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(claude.path().join("gone"), claude.path().join("skills")).unwrap();
    assert!(real_file(claude.path()).is_err());
}

#[test]
fn a_claude_folder_that_is_a_file_is_a_reason() {
    let home = tempfile::tempdir().unwrap();
    let claude = home.path().join("claude");
    fs::write(&claude, "a file").unwrap();
    assert!(real_file(&claude).is_err());
}

#[tokio::test]
async fn a_save_into_a_file_fails_with_its_path() {
    let home = tempfile::tempdir().unwrap();
    let claude = home.path().join("claude");
    fs::write(&claude, "a file").unwrap();
    let error = FilesystemSkillRepository
        .save(&claude, "text")
        .await
        .unwrap_err();
    let InstallSkillError::Io { path, .. } = error;
    assert!(path.starts_with(&claude.display().to_string()), "{path}");
}
