//! `SkillTextHandler` through the component's root with the macOS adapters and a temp home.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use support::root_in;

#[tokio::test]
async fn answers_the_text_the_skill_command_prints() {
    let home = tempfile::tempdir().unwrap();
    let root = root_in(home.path());
    let (mut out, mut err) = (Vec::new(), Vec::new());
    assert_eq!(root.skill_command().run(&mut out, &mut err).await, 0);
    assert_eq!(
        root.skill_text_handler().text().await,
        String::from_utf8(out).unwrap()
    );
}
