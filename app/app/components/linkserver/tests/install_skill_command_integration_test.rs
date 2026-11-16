//! `InstallSkillCommand` through the component's root with the macOS adapters and a temp home.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use support::{blocked_folder, root_in, skill_file};

#[tokio::test]
async fn installs_into_the_setup_folder_and_prints_the_file() {
    let home = tempfile::tempdir().unwrap();
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let command = root_in(home.path()).install_skill_command();
    assert_eq!(command.run(None, &mut out, &mut err).await, 0);
    let file = skill_file(&home.path().join("claude"));
    assert!(
        std::fs::read_to_string(&file)
            .unwrap()
            .contains("name: http2scheme")
    );
    assert_eq!(
        String::from_utf8(out).unwrap(),
        format!("[ok] skill installed: {}\n", file.display())
    );
    assert_eq!(err, Vec::<u8>::new());
}

#[tokio::test]
async fn a_folder_that_cannot_hold_the_skill_fails_with_the_reason() {
    let home = tempfile::tempdir().unwrap();
    let dir = blocked_folder(home.path());
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let command = root_in(home.path()).install_skill_command();
    assert_eq!(command.run(Some(&dir), &mut out, &mut err).await, 1);
    assert_eq!(out, Vec::<u8>::new());
    let err = String::from_utf8(err).unwrap();
    assert!(
        err.starts_with(&format!("[fail] {}", dir.display())),
        "{err}"
    );
}
