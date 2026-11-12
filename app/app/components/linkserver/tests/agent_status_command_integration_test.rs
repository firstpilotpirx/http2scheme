//! `AgentStatusCommand` through a binary that mounts it over a fake use case.
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
fn a_healthy_setup_succeeds() {
    let (code, out, _) = output(&["status", "healthy"]);
    assert_eq!(code, 0);
    assert_eq!(
        out,
        "[ok] installed\n[ok] loaded\n[ok] answering on 127.0.0.1:7777\n"
    );
}

#[test]
fn a_stopped_server_fails_and_points_at_the_log() {
    let (code, out, _) = output(&["status", "stopped"]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "[ok] installed\n[fail] not loaded\n[fail] not answering on 127.0.0.1:7777\n  see /home/agent.err.log\n"
    );
}
