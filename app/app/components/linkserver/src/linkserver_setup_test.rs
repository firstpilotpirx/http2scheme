use std::path::PathBuf;

use super::LinkserverSetup;
use crate::linkserver_programs::LinkserverPrograms;

fn setup() -> LinkserverSetup {
    LinkserverSetup {
        settings_dir: PathBuf::from("/h/settings"),
        log_dir: PathBuf::from("/h/logs"),
        agents_dir: PathBuf::from("/h/agents"),
        program: PathBuf::from("/bin/tool"),
        label: "io.example.agent".to_owned(),
        host: "127.0.0.1".to_owned(),
        hosts: vec!["127.0.0.1".to_owned()],
        version: "0.1.0".to_owned(),
        uid: 501,
        programs: LinkserverPrograms {
            open: PathBuf::from("/bin/open"),
            launchctl: PathBuf::from("/bin/launchctl"),
            osascript: PathBuf::from("/bin/osascript"),
            lsappinfo: PathBuf::from("/usr/bin/lsappinfo"),
        },
    }
}

#[test]
fn files_live_in_the_handed_folders() {
    let setup = setup();
    assert_eq!(setup.settings_file(), "/h/settings/settings.toml");
    assert_eq!(setup.request_log(), PathBuf::from("/h/logs/requests.log"));
    let definition = setup.definition();
    assert_eq!(definition.label(), "io.example.agent");
    assert_eq!(
        definition.stderr_log(),
        PathBuf::from("/h/logs/agent.err.log")
    );
}
