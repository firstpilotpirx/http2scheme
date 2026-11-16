//! `FilesystemSkillFolderRepository` against a real temporary folder.
#![allow(
    clippy::unwrap_used,
    reason = "test helpers fail loudly, like the tests"
)]

use std::path::PathBuf;

use menubar::adapters::secondary::skillfolder::filesystem::filesystem_skill_folder_repository::FilesystemSkillFolderRepository;
use menubar::core::application::repositories::skill_folder_repository::SkillFolderRepository;

#[tokio::test]
async fn no_record_means_no_folder() {
    let home = tempfile::tempdir().unwrap();
    let repository = FilesystemSkillFolderRepository::new(&home.path().join("settings"));
    assert_eq!(repository.find().await.unwrap(), None);
}

#[tokio::test]
async fn a_saved_folder_reads_back_even_with_spaces() {
    let home = tempfile::tempdir().unwrap();
    let settings = home.path().join("settings");
    let repository = FilesystemSkillFolderRepository::new(&settings);
    let folder = PathBuf::from("/Users/me/Work Profile/.claude");
    repository.save(&folder).await.unwrap();
    assert_eq!(repository.find().await.unwrap(), Some(folder));
    let text = std::fs::read_to_string(settings.join("agent-skill-folder")).unwrap();
    assert_eq!(text, "/Users/me/Work Profile/.claude\n");
}

#[tokio::test]
async fn a_second_save_replaces_the_first() {
    let home = tempfile::tempdir().unwrap();
    let repository = FilesystemSkillFolderRepository::new(home.path());
    repository.save(&PathBuf::from("/a")).await.unwrap();
    repository.save(&PathBuf::from("/b")).await.unwrap();
    assert_eq!(repository.find().await.unwrap(), Some(PathBuf::from("/b")));
}
