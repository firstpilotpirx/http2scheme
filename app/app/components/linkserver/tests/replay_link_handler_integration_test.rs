//! `ReplayLinkHandler` through the component's root with the macOS adapters, a temp home and a
//! recording `open`.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use std::path::Path;

use linkserver::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use linkserver::linkserver_config::LinkserverConfig;
use linkserver::linkserver_ports::LinkserverPorts;
use linkserver::linkserver_programs::LinkserverPrograms;
use linkserver::linkserver_setup::LinkserverSetup;
use support::RecordingProgram;

fn root(home: &Path, open: &RecordingProgram) -> LinkserverConfig {
    let setup = LinkserverSetup {
        settings_dir: home.join("settings"),
        log_dir: home.join("logs"),
        agents_dir: home.join("agents"),
        program: home.join("http2scheme"),
        label: "test.http2scheme".to_owned(),
        host: "127.0.0.1".to_owned(),
        hosts: vec!["127.0.0.1".to_owned()],
        version: "0.0.0".to_owned(),
        uid: 0,
        programs: LinkserverPrograms {
            open: open.program.clone(),
            launchctl: home.join("launchctl"),
            osascript: home.join("osascript"),
            lsappinfo: home.join("lsappinfo"),
        },
    };
    let ports = LinkserverPorts::macos(&setup);
    LinkserverConfig::new(setup, ports, TerminalStyleConfig::new(false))
}

fn last_log_line(home: &Path) -> String {
    let log = std::fs::read_to_string(home.join("logs/requests.log")).unwrap();
    log.lines().last().unwrap().to_owned()
}

#[tokio::test]
async fn an_allowed_link_opens_and_is_logged() {
    let home = tempfile::tempdir().unwrap();
    let open = RecordingProgram::new(home.path(), "open", 0);
    let result = root(home.path(), &open)
        .replay_handler()
        .replay("idea://open?file=/a/B.java&line=3")
        .await;
    assert_eq!(result, "opened");
    assert_eq!(open.calls(), [["idea://open?file=/a/B.java&line=3"]]);
    assert!(last_log_line(home.path()).ends_with("\topened\tidea://open?file=/a/B.java&line=3"));
}

#[tokio::test]
async fn a_slack_link_is_refused_and_logged() {
    let home = tempfile::tempdir().unwrap();
    let open = RecordingProgram::new(home.path(), "open", 0);
    let result = root(home.path(), &open)
        .replay_handler()
        .replay("slack://channel?id=C1")
        .await;
    assert_eq!(result, "scheme not allowed");
    assert_eq!(open.calls(), Vec::<Vec<String>>::new());
    assert!(last_log_line(home.path()).ends_with("\tscheme not allowed\tslack://channel?id=C1"));
}
