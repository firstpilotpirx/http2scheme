//! `InspectSkillHandler` through the component's root with the macOS adapters and a temp home.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use std::fs;

use support::{blocked_folder, root_in, skill_file};

#[tokio::test]
async fn an_empty_folder_has_no_skill() {
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join("claude");
    let answer = root_in(home.path())
        .inspect_skill_handler()
        .inspect(&dir)
        .await;
    assert_eq!(answer.unwrap(), (skill_file(&dir), None, "not-installed"));
}

#[tokio::test]
async fn another_text_is_outdated() {
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join("claude");
    fs::create_dir_all(skill_file(&dir).parent().unwrap()).unwrap();
    fs::write(skill_file(&dir), "an older skill").unwrap();
    let answer = root_in(home.path())
        .inspect_skill_handler()
        .inspect(&dir)
        .await;
    assert_eq!(answer.unwrap().2, "outdated");
}

#[tokio::test]
async fn an_unreadable_folder_answers_the_reason() {
    let home = tempfile::tempdir().unwrap();
    let dir = blocked_folder(home.path());
    let reason = root_in(home.path())
        .inspect_skill_handler()
        .inspect(&dir)
        .await
        .unwrap_err();
    assert!(reason.starts_with(&dir.display().to_string()), "{reason}");
}
