use std::path::PathBuf;

use super::MenubarConfig;
use crate::menubar_setup::MenubarSetup;

fn config(program: &str, settings: PathBuf) -> MenubarConfig {
    let setup = MenubarSetup {
        version: "0.1.0".to_owned(),
        settings_dir: settings,
        log_dir: PathBuf::from("logs"),
        program: PathBuf::from(program),
        open_program: PathBuf::from("open"),
    };
    MenubarConfig::macos(setup)
}

#[tokio::test]
async fn a_program_outside_applications_is_not_offered() {
    let home = tempfile::tempdir().unwrap();
    let settings = home.path().join("settings");
    config("/usr/local/bin/http2scheme", settings.clone())
        .offer_login_item()
        .await
        .unwrap();
    assert!(!settings.exists());
}

#[test]
fn a_test_thread_has_no_main_thread_marker() {
    assert!(MenubarConfig::main_thread().is_none());
}

#[test]
fn only_a_program_inside_an_app_bundle_is_a_bundle_program() {
    assert!(MenubarConfig::is_bundle_program(&PathBuf::from(
        "/Applications/http2scheme.app/Contents/MacOS/http2scheme"
    )));
    assert!(!MenubarConfig::is_bundle_program(&PathBuf::from(
        "/usr/local/bin/http2scheme"
    )));
}

#[tokio::test]
async fn the_macos_menu_reads_the_server_output() {
    use std::io::Write as _;
    let config = config("/usr/local/bin/http2scheme", PathBuf::from("settings"));
    config
        .server_output()
        .out()
        .write_all(b"listening on http://127.0.0.1:7777\n")
        .unwrap();
    let state = config.ports.server.current().await;
    assert_eq!(state.status_line(), "Running on http://127.0.0.1:7777");
}
