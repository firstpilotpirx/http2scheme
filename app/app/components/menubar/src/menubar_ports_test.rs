use std::path::PathBuf;
use std::sync::Arc;

use super::MenubarPorts;
use crate::adapters::secondary::serverstate::serveoutput::serve_output_server_state_gateway::ServeOutputServerStateGateway;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use crate::core::domain::server_state_enum::ServerStateEnum;
use crate::menubar_hooks::MenubarHooks;
use crate::menubar_setup::MenubarSetup;

fn setup() -> MenubarSetup {
    MenubarSetup {
        version: "0.1.0".to_owned(),
        settings_dir: PathBuf::from("settings"),
        log_dir: PathBuf::from("logs"),
        claude_dir: PathBuf::from("/h/.claude"),
        program: PathBuf::from("/usr/local/bin/http2scheme"),
    }
}

#[tokio::test]
async fn the_macos_ports_read_the_server_output_and_no_bundle() {
    let hooks = MenubarHooks {
        serve: Arc::new(|_, _, _| {}),
        replay: Arc::new(|link: &str| format!("opened {link}")),
        skill_text: Arc::new(|| "skill".to_owned()),
        inspect_skill: Arc::new(|_| Err("not read".to_owned())),
        install_skill: Arc::new(|_| Err("not written".to_owned())),
    };
    let ports = MenubarPorts::macos(&setup(), &ServeOutputServerStateGateway::new(), hooks);
    assert_eq!(ports.server.current().await, ServerStateEnum::Starting);
    assert_eq!(
        ports.login_item.status().await,
        LoginItemStateEnum::Unavailable
    );
    assert_eq!(ports.history.find_latest(100).await, Ok(Vec::new()));
    assert_eq!(ports.replay.replay("idea://x").await, "opened idea://x");
    assert_eq!(ports.skill.text().await, "skill");
    assert_eq!(ports.skill_folder.find().await, Ok(None));
}
