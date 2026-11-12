//! `PrepareLinkCommand` through a binary that mounts it over a fake use case.
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
fn prints_one_percent_encoded_open_link() {
    let (code, out, err) = output(&["link", "ok", "idea://open?file=/a%20b/Ф.java&line=42"]);
    assert_eq!((code, err.as_str()), (0, ""));
    let key = "01".repeat(32);
    assert_eq!(
        out,
        format!(
            "http://127.0.0.1:7777/open?key={key}&url=idea%3A%2F%2Fopen%3Ffile%3D%2Fa%2520b%2F%D0%A4.java%26line%3D42\n"
        )
    );
}

#[test]
fn a_scheme_not_allowed_prints_the_reason_and_the_hint() {
    let (code, out, err) = output(&["link", "not-allowed", "slack://channel?id=C1"]);
    assert_eq!((code, out.as_str()), (1, ""));
    assert_eq!(
        err,
        "[fail] scheme not allowed: slack\n  add \"slack\" to allowed_schemes in /home/settings.toml\n"
    );
}

#[test]
fn a_malformed_link_prints_the_reason() {
    let (code, out, err) = output(&["link", "ok", "no-scheme"]);
    assert_eq!((code, out.as_str()), (1, ""));
    assert_eq!(err, "[fail] the link has no valid scheme\n");
}

#[test]
fn a_settings_failure_prints_the_reason() {
    let (code, _, err) = output(&["link", "settings", "idea://x"]);
    assert_eq!((code, err.as_str()), (1, "[fail] /home/key: denied\n"));
}
