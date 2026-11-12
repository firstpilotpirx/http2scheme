use std::path::PathBuf;

use super::LinkserverPorts;
use crate::linkserver_programs::LinkserverPrograms;
use crate::linkserver_setup::LinkserverSetup;

#[tokio::test]
async fn the_macos_settings_read_the_handed_folder() {
    let folder = tempfile::tempdir().unwrap();
    let setup = LinkserverSetup {
        settings_dir: folder.path().join("settings"),
        log_dir: folder.path().join("logs"),
        agents_dir: folder.path().join("agents"),
        program: PathBuf::from("/bin/tool"),
        label: "io.example.agent".to_owned(),
        host: "127.0.0.1".to_owned(),
        hosts: vec!["127.0.0.1".to_owned()],
        version: "0.1.0".to_owned(),
        uid: 501,
        programs: LinkserverPrograms {
            open: PathBuf::from("/nonexistent/open"),
            launchctl: PathBuf::from("/nonexistent/launchctl"),
            osascript: PathBuf::from("/nonexistent/osascript"),
            lsappinfo: PathBuf::from("/nonexistent/lsappinfo"),
        },
    };
    let ports = LinkserverPorts::macos(&setup);
    assert!(ports.settings.read_settings().await.unwrap().is_none());
    assert!(!ports.agents.exists("io.example.agent").await.unwrap());
}
