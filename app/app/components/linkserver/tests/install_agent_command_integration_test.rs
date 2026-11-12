//! `InstallAgentCommand` through a binary that mounts it over a fake use case.
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
fn says_where_the_agent_runs() {
    let (code, out, _) = output(&["install", "ok"]);
    assert_eq!(code, 0);
    assert_eq!(
        out,
        "[ok] agent definition written  /home/agent.plist\n[ok] agent loaded, runs /bin/http2scheme serve\n[ok] installed and running on http://127.0.0.1:7777\n"
    );
}

#[test]
fn a_failure_prints_the_reason() {
    let (code, out, err) = output(&["install", "fail"]);
    assert_eq!(
        (code, out.as_str(), err.as_str()),
        (1, "", "[fail] launchctl: exit 5\n")
    );
}
