//! `TerminalStyleConfig` as a real process sees its stream, flags and environment.
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

fn colored(command: &mut Command) -> bool {
    String::from_utf8(command.output().unwrap().stdout)
        .unwrap()
        .contains('\u{1b}')
}

#[test]
fn a_pipe_gets_no_color() {
    assert!(!colored(&mut fixture(&["status", "healthy"])));
}

#[test]
fn force_color_turns_it_on() {
    assert!(colored(
        fixture(&["status", "healthy"]).env("FORCE_COLOR", "1")
    ));
}

#[test]
fn no_color_and_the_flag_win_over_force_color() {
    assert!(!colored(
        fixture(&["status", "healthy"])
            .env("FORCE_COLOR", "1")
            .env("NO_COLOR", "1")
    ));
    assert!(!colored(
        fixture(&["status", "healthy", "--no-color"]).env("FORCE_COLOR", "1")
    ));
}

#[test]
fn a_dumb_terminal_stays_plain_unless_forced() {
    assert!(!colored(
        fixture(&["status", "healthy"]).env("TERM", "dumb")
    ));
}
