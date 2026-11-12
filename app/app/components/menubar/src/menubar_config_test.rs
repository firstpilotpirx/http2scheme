use std::path::PathBuf;

use super::MenubarConfig;
use crate::menubar_ports::MenubarPorts;
use crate::menubar_setup::MenubarSetup;

fn config(program: &str, settings: PathBuf) -> MenubarConfig {
    let setup = MenubarSetup {
        version: "0.1.0".to_owned(),
        settings_dir: settings,
        log_dir: PathBuf::from("logs"),
        program: PathBuf::from(program),
        open_program: PathBuf::from("open"),
    };
    let ports = MenubarPorts::macos(&setup, &MenubarConfig::server_output());
    MenubarConfig::new(setup, ports)
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
