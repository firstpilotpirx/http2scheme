//! `AppleScriptBrowserTabCloserGateway` with a recording program, and the real `AppleScript` compiler.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

mod support;

use std::time::Duration;

use linkserver::adapters::secondary::browsertabs::applescript::apple_script_browser_tab_closer_gateway::AppleScriptBrowserTabCloserGateway;
use linkserver::adapters::secondary::browsertabs::applescript::port_to_apple_script_mapper::to_script;
use linkserver::core::application::services::browser_tab_closer_service::BrowserTabCloserService;
use linkserver::core::domain::port_value_object::PortValueObject;
use support::RecordingProgram;
use tempfile::TempDir;

fn hosts() -> Vec<String> {
    vec!["127.0.0.1".to_owned(), "localhost".to_owned()]
}

fn port() -> PortValueObject {
    PortValueObject::new(7801).unwrap()
}

#[tokio::test]
async fn runs_one_script_naming_the_browsers_and_both_prefixes() {
    let dir = TempDir::new().unwrap();
    let osascript = RecordingProgram::new(dir.path(), "osascript", 0);
    let gateway =
        AppleScriptBrowserTabCloserGateway::new(osascript.program.clone(), hosts(), Duration::ZERO);
    gateway.close_now(port()).await.unwrap();
    let calls = osascript.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0][0], "-e");
    let script = &calls[0][1];
    for browser in [
        "Safari",
        "Google Chrome",
        "Arc",
        "Brave Browser",
        "Microsoft Edge",
    ] {
        assert!(
            script.contains(&format!("application \\\"{browser}\\\"")),
            "{browser} missing in {script}"
        );
    }
    assert!(script.contains("is running"), "{script}");
    assert!(script.contains("http://127.0.0.1:7801/open?"), "{script}");
    assert!(script.contains("http://localhost:7801/open?"), "{script}");
}

#[tokio::test]
async fn a_failing_program_is_an_error() {
    let dir = TempDir::new().unwrap();
    let osascript = RecordingProgram::new(dir.path(), "osascript", 1);
    let gateway =
        AppleScriptBrowserTabCloserGateway::new(osascript.program.clone(), hosts(), Duration::ZERO);
    assert!(gateway.close_now(port()).await.is_err());
}

#[tokio::test]
async fn close_open_tabs_returns_at_once_and_runs_after_the_delay() {
    let dir = TempDir::new().unwrap();
    let osascript = RecordingProgram::new(dir.path(), "osascript", 0);
    let gateway = AppleScriptBrowserTabCloserGateway::new(
        osascript.program.clone(),
        hosts(),
        Duration::from_millis(50),
    );
    gateway.close_open_tabs(port()).await.unwrap();
    assert_eq!(osascript.calls().len(), 0);
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while osascript.calls().is_empty() && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert_eq!(osascript.calls().len(), 1);
}

#[test]
#[ignore = "runs the real /usr/bin/osacompile on macOS"]
fn the_real_apple_script_compiler_accepts_the_script() {
    let dir = TempDir::new().unwrap();
    let script = to_script(&hosts(), port());
    let status = std::process::Command::new("/usr/bin/osacompile")
        .arg("-e")
        .arg(script)
        .arg("-o")
        .arg(dir.path().join("close.scpt"))
        .status()
        .unwrap();
    assert!(status.success());
}
