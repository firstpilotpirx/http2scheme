//! `ServeCommand` through a binary that mounts it over a fake use case.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::io::{BufRead as _, BufReader};
use std::net::TcpListener;
use std::process::{Command, Stdio};

use assert_cmd::cargo::CommandCargoExt as _;

fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

fn serve(case: &str, port: u16) -> Command {
    let mut command = Command::cargo_bin("linkserver-cli-fixture").unwrap();
    command
        .args(["serve", case, &port.to_string()])
        .env_remove("FORCE_COLOR");
    command
}

#[tokio::test]
async fn serves_on_a_free_port_and_prints_the_address() {
    let port = free_port();
    let mut child = serve("ok", port).stdout(Stdio::piped()).spawn().unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let first = lines.next().unwrap().unwrap();
    let second = lines.next().unwrap().unwrap();
    let health = reqwest::get(format!("http://127.0.0.1:{port}/health"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    child.kill().unwrap();
    child.wait().unwrap();
    assert_eq!(
        first,
        format!("[..] http2scheme 0.1.0 listening on http://127.0.0.1:{port}")
    );
    assert_eq!(
        second,
        "  allowed schemes: idea, goland · log: /home/requests.log"
    );
    assert_eq!(health, "ok");
}

#[test]
fn a_taken_port_fails_with_the_reason() {
    let holder = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = holder.local_addr().unwrap().port();
    let output = serve("ok", port).output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        format!("[fail] port {port} is taken\n")
    );
}

#[test]
fn broken_settings_stop_the_start_with_the_reason() {
    let output = serve("broken", free_port()).output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "[fail] /home/settings.toml: port must be between 1024 and 65535\n"
    );
}
