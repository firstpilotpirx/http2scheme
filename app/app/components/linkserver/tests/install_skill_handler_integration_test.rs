//! `InstallSkillHandler` through the component's root with the macOS adapters and a temp home.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use std::fs;
use std::os::unix::fs::symlink;

use support::{blocked_folder, root_in, skill_file};

#[tokio::test]
async fn writes_the_skill_and_answers_installed() {
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join("work");
    let answer = root_in(home.path())
        .install_skill_handler()
        .install(&dir)
        .await;
    assert_eq!(answer.unwrap(), (skill_file(&dir), None, "installed"));
    assert!(
        fs::read_to_string(skill_file(&dir))
            .unwrap()
            .contains("name: http2scheme")
    );
}

#[tokio::test]
async fn a_linked_skills_folder_answers_the_real_file() {
    let home = tempfile::tempdir().unwrap();
    let (dir, elsewhere) = (home.path().join("claude"), home.path().join("elsewhere"));
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&elsewhere).unwrap();
    symlink(&elsewhere, dir.join("skills")).unwrap();
    let (_, real_file, _) = root_in(home.path())
        .install_skill_handler()
        .install(&dir)
        .await
        .unwrap();
    let real = fs::canonicalize(&elsewhere)
        .unwrap()
        .join("http2scheme/SKILL.md");
    assert_eq!(real_file, Some(real));
}

#[tokio::test]
async fn a_folder_that_cannot_hold_the_skill_answers_the_reason() {
    let home = tempfile::tempdir().unwrap();
    let dir = blocked_folder(home.path());
    let reason = root_in(home.path())
        .install_skill_handler()
        .install(&dir)
        .await
        .unwrap_err();
    assert!(reason.starts_with(&dir.display().to_string()), "{reason}");
}
