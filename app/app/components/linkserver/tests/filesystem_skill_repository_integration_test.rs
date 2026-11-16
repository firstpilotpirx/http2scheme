//! `FilesystemSkillRepository` against real temporary folders.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::fs;
use std::os::unix::fs::{PermissionsExt as _, symlink};

use linkserver::adapters::secondary::agentskill::filesystem::filesystem_skill_repository::FilesystemSkillRepository;
use linkserver::core::application::repositories::skill_repository::SkillRepository;
use tempfile::TempDir;

#[tokio::test]
async fn an_empty_folder_holds_no_skill() {
    let claude = TempDir::new().unwrap();
    let found = FilesystemSkillRepository.find(claude.path()).await.unwrap();
    assert_eq!(found, (None, None));
}

#[tokio::test]
async fn save_creates_the_file_and_find_reads_it_back() {
    let claude = TempDir::new().unwrap();
    FilesystemSkillRepository
        .save(claude.path(), "skill text")
        .await
        .unwrap();
    let file = claude.path().join("skills/http2scheme/SKILL.md");
    assert_eq!(fs::read_to_string(file).unwrap(), "skill text");
    let found = FilesystemSkillRepository.find(claude.path()).await.unwrap();
    assert_eq!(found, (Some("skill text".to_owned()), None));
}

#[tokio::test]
async fn save_replaces_the_skill_and_leaves_a_sibling_skill() {
    let claude = TempDir::new().unwrap();
    let sibling = claude.path().join("skills/other/SKILL.md");
    fs::create_dir_all(sibling.parent().unwrap()).unwrap();
    fs::write(&sibling, "other").unwrap();
    FilesystemSkillRepository
        .save(claude.path(), "old")
        .await
        .unwrap();
    FilesystemSkillRepository
        .save(claude.path(), "new")
        .await
        .unwrap();
    let found = FilesystemSkillRepository.find(claude.path()).await.unwrap();
    assert_eq!(found.0.as_deref(), Some("new"));
    assert_eq!(fs::read_to_string(sibling).unwrap(), "other");
}

#[tokio::test]
async fn a_linked_skills_folder_reports_its_real_file() {
    let claude = TempDir::new().unwrap();
    let elsewhere = TempDir::new().unwrap();
    symlink(elsewhere.path(), claude.path().join("skills")).unwrap();
    FilesystemSkillRepository
        .save(claude.path(), "text")
        .await
        .unwrap();
    let real = fs::canonicalize(elsewhere.path())
        .unwrap()
        .join("http2scheme/SKILL.md");
    assert_eq!(fs::read_to_string(&real).unwrap(), "text");
    let found = FilesystemSkillRepository.find(claude.path()).await.unwrap();
    assert_eq!(found, (Some("text".to_owned()), Some(real)));
}

#[tokio::test]
async fn a_folder_it_cannot_write_names_the_folder() {
    let claude = TempDir::new().unwrap();
    fs::set_permissions(claude.path(), fs::Permissions::from_mode(0o500)).unwrap();
    let error = FilesystemSkillRepository
        .save(claude.path(), "text")
        .await
        .unwrap_err();
    fs::set_permissions(claude.path(), fs::Permissions::from_mode(0o700)).unwrap();
    assert!(error.to_string().contains("skills"), "{error}");
}
