//! `UninstallAgentCommand` through a binary that mounts it over a fake use case.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use assert_cmd::Command;

fn fixture(arguments: &[&str]) -> Command {
    let mut command = Command::cargo_bin("linkserver-cli-fixture").unwrap();
    command
        .args(arguments)
        .env_remove("FORCE_COLOR")
        .env_remove("NO_COLOR");
    command
}

fn output(arguments: &[&str]) -> (i32, String, String) {
    let output = fixture(arguments).output().unwrap();
    let text = |bytes: Vec<u8>| String::from_utf8(bytes).unwrap();
    (
        output.status.code().unwrap(),
        text(output.stdout),
        text(output.stderr),
    )
}

#[test]
fn says_the_agent_is_removed_and_the_files_kept() {
    let (code, out, _) = output(&["uninstall", "removed"]);
    assert_eq!(
        (code, out.as_str()),
        (
            0,
            "[ok] agent stopped and removed; settings, key and logs are kept\n"
        )
    );
}

#[test]
fn says_nothing_was_installed() {
    let (code, out, _) = output(&["uninstall", "nothing"]);
    assert_eq!((code, out.as_str()), (0, "[ok] nothing was installed\n"));
}

#[test]
fn a_failure_prints_the_reason() {
    let (code, _, err) = output(&["uninstall", "fail"]);
    assert_eq!((code, err.as_str()), (1, "[fail] launchctl: exit 5\n"));
}
