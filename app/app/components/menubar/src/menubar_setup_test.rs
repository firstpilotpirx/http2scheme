use std::path::PathBuf;

use super::MenubarSetup;

#[test]
fn a_setup_keeps_what_the_app_hands_in() {
    let setup = MenubarSetup {
        version: "0.1.0".to_owned(),
        settings_dir: PathBuf::from("settings"),
        log_dir: PathBuf::from("logs"),
        program: PathBuf::from("program"),
    };
    assert_eq!(setup.version, "0.1.0");
}
