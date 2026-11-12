use std::io::Write as _;

use super::ServeOutputServerStateGateway;
use crate::core::application::services::server_state_service::ServerStateService;
use crate::core::domain::server_state_enum::ServerStateEnum;

#[tokio::test]
async fn a_new_gateway_is_starting() {
    assert_eq!(
        ServeOutputServerStateGateway::new().current().await,
        ServerStateEnum::Starting
    );
}

#[tokio::test]
async fn a_line_counts_only_once_it_ends() {
    let gateway = ServeOutputServerStateGateway::new();
    let mut err = gateway.err();
    err.write_all(b"[fail] port 1 is").unwrap();
    assert_eq!(gateway.current().await, ServerStateEnum::Starting);
    err.write_all(b" taken\n").unwrap();
    assert_eq!(
        gateway.current().await,
        ServerStateEnum::Stopped("port 1 is taken".to_owned())
    );
}

#[tokio::test]
async fn the_detail_line_of_the_banner_changes_nothing() {
    let gateway = ServeOutputServerStateGateway::new();
    writeln!(gateway.out(), "  allowed schemes: idea").unwrap();
    assert_eq!(gateway.current().await, ServerStateEnum::Starting);
}

#[tokio::test]
async fn turning_on_hands_a_shutdown_to_the_launcher_and_turning_off_completes_it() {
    use crate::core::application::services::server_switch_service::ServerSwitchService as _;
    use crate::menubar_hooks::ServeShutdown;
    use std::sync::{Arc, Mutex};
    let runs: Arc<Mutex<Vec<ServeShutdown>>> = Arc::default();
    let sink = Arc::clone(&runs);
    let gateway = ServeOutputServerStateGateway::with_launcher(Arc::new(move |shutdown, _, _| {
        sink.lock().unwrap().push(shutdown);
    }));
    gateway.turn_on().await;
    assert_eq!(gateway.current().await, ServerStateEnum::Starting);
    gateway.turn_off().await;
    assert_eq!(gateway.current().await, ServerStateEnum::Off);
    let shutdown = runs.lock().unwrap().pop().unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(1), shutdown)
        .await
        .unwrap();
}

#[tokio::test]
async fn a_line_after_turning_off_keeps_off() {
    use crate::core::application::services::server_switch_service::ServerSwitchService as _;
    let gateway = ServeOutputServerStateGateway::new();
    gateway.turn_off().await;
    gateway.err().write_all(b"[fail] stopped\n").unwrap();
    assert_eq!(gateway.current().await, ServerStateEnum::Off);
}

#[tokio::test]
async fn the_launched_server_prints_its_state_to_the_handed_output() {
    use crate::core::application::services::server_switch_service::ServerSwitchService as _;
    use std::sync::Arc;
    let gateway = ServeOutputServerStateGateway::with_launcher(Arc::new(|_, mut out, _| {
        out.write_all(b"listening on http://127.0.0.1:7777\n")
            .unwrap();
    }));
    gateway.turn_on().await;
    assert_eq!(
        gateway.current().await,
        ServerStateEnum::Running("http://127.0.0.1:7777".to_owned())
    );
}

#[tokio::test]
async fn a_default_gateway_is_starting_and_shows_its_state() {
    let mut gateway = ServeOutputServerStateGateway::default();
    gateway.flush().unwrap();
    assert_eq!(gateway.current().await, ServerStateEnum::Starting);
    assert!(format!("{gateway:?}").contains("Starting"));
}
