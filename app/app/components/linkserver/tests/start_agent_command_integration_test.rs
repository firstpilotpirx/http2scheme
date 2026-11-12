//! `StartAgentCommand` through a binary that mounts it over a fake use case.
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
fn says_the_agent_started() {
    let (code, out, _) = output(&["start", "ok"]);
    assert_eq!((code, out.as_str()), (0, "[ok] agent started\n"));
}

#[test]
fn without_install_gives_the_hint() {
    let (code, out, err) = output(&["start", "not-installed"]);
    assert_eq!((code, out.as_str()), (1, ""));
    assert_eq!(
        err,
        "[fail] the agent is not installed; run: http2scheme install\n"
    );
}
