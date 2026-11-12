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
