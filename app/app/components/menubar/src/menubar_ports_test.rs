use std::path::PathBuf;

use super::MenubarPorts;
use crate::adapters::secondary::serverstate::serveoutput::serve_output_server_state_gateway::ServeOutputServerStateGateway;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use crate::core::domain::server_state_enum::ServerStateEnum;
use crate::menubar_setup::MenubarSetup;

fn setup() -> MenubarSetup {
    MenubarSetup {
        version: "0.1.0".to_owned(),
        settings_dir: PathBuf::from("settings"),
        log_dir: PathBuf::from("logs"),
        program: PathBuf::from("/usr/local/bin/http2scheme"),
        open_program: PathBuf::from("open"),
    }
}

#[tokio::test]
async fn the_macos_ports_read_the_server_output_and_no_bundle() {
    let replay = std::sync::Arc::new(|link: &str| format!("opened {link}"));
    let ports = MenubarPorts::macos(&setup(), &ServeOutputServerStateGateway::new(), replay);
    assert_eq!(ports.server.current().await, ServerStateEnum::Starting);
    assert_eq!(
        ports.login_item.status().await,
        LoginItemStateEnum::Unavailable
    );
    assert_eq!(ports.history.latest(100).await, Ok(Vec::new()));
    assert_eq!(ports.replay.replay("idea://x").await, "opened idea://x");
}
