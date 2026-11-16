use std::path::{Path, PathBuf};

use super::FilesystemSkillFolderRepository;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::skill_folder_repository::SkillFolderRepository;

#[tokio::test]
async fn an_empty_record_remembers_no_folder() {
    let settings = tempfile::tempdir().unwrap();
    std::fs::write(settings.path().join("agent-skill-folder"), "\n").unwrap();
    let found = FilesystemSkillFolderRepository::new(settings.path())
        .find()
        .await;
    assert_eq!(found.unwrap(), None);
}

#[tokio::test]
async fn the_record_is_one_line_with_the_path() {
    let settings = tempfile::tempdir().unwrap();
    let repository = FilesystemSkillFolderRepository::new(settings.path());
    repository.save(Path::new("/w/.claude")).await.unwrap();
    let text = std::fs::read_to_string(settings.path().join("agent-skill-folder")).unwrap();
    assert_eq!(text, "/w/.claude\n");
    assert_eq!(
        repository.find().await.unwrap(),
        Some(PathBuf::from("/w/.claude"))
    );
}

#[tokio::test]
async fn an_unreadable_record_names_its_file() {
    let settings = tempfile::tempdir().unwrap();
    std::fs::create_dir(settings.path().join("agent-skill-folder")).unwrap();
    let error = FilesystemSkillFolderRepository::new(settings.path())
        .find()
        .await
        .unwrap_err();
    assert!(
        matches!(&error, MenuActionError::SkillFolder { path, .. } if path.ends_with("agent-skill-folder")),
        "{error:?}"
    );
}

#[tokio::test]
async fn a_settings_folder_that_is_a_file_fails_the_save() {
    let home = tempfile::tempdir().unwrap();
    let settings = home.path().join("settings");
    std::fs::write(&settings, "a file").unwrap();
    let error = FilesystemSkillFolderRepository::new(&settings)
        .save(Path::new("/w"))
        .await
        .unwrap_err();
    assert!(
        matches!(error, MenuActionError::SkillFolder { .. }),
        "{error:?}"
    );
}

#[tokio::test]
async fn a_record_that_cannot_be_written_fails_the_save() {
    let settings = tempfile::tempdir().unwrap();
    std::fs::create_dir(settings.path().join("agent-skill-folder")).unwrap();
    let error = FilesystemSkillFolderRepository::new(settings.path())
        .save(Path::new("/w"))
        .await
        .unwrap_err();
    assert!(
        matches!(error, MenuActionError::SkillFolder { .. }),
        "{error:?}"
    );
}
