//! `ServeOutputServerStateGateway` fed with what the serve command prints.
#![allow(
    clippy::unwrap_used,
    reason = "test helpers fail loudly, like the tests"
)]

use std::io::Write as _;

use menubar::adapters::secondary::serverstate::serveoutput::serve_output_server_state_gateway::ServeOutputServerStateGateway;
use menubar::core::application::services::server_state_service::ServerStateService;
use menubar::core::domain::server_state_enum::ServerStateEnum;

#[tokio::test]
async fn the_banner_on_the_output_means_running() {
    let gateway = ServeOutputServerStateGateway::new();
    let mut out = gateway.out();
    out.write_all(
        b"[..] http2scheme 0.1.0 listening on http://127.0.0.1:7777\n  allowed schemes: idea\n",
    )
    .unwrap();
    assert_eq!(
        gateway.current().await,
        ServerStateEnum::Running("http://127.0.0.1:7777".to_owned())
    );
}

#[tokio::test]
async fn a_failure_on_the_error_stream_means_stopped_with_its_reason() {
    let gateway = ServeOutputServerStateGateway::new();
    let mut err = gateway.err();
    err.write_all(b"[fail] port 7777 is ").unwrap();
    assert_eq!(gateway.current().await, ServerStateEnum::Starting);
    err.write_all(b"taken\n").unwrap();
    assert_eq!(
        gateway.current().await,
        ServerStateEnum::Stopped("port 7777 is taken".to_owned())
    );
}
