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

#[tokio::test]
async fn a_launched_server_stops_when_turned_off_and_starts_again_when_turned_on() {
    use menubar::core::application::services::server_switch_service::ServerSwitchService as _;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let running = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&running);
    let gateway =
        ServeOutputServerStateGateway::with_launcher(Arc::new(move |shutdown, mut out, _| {
            let counter = Arc::clone(&counter);
            counter.fetch_add(1, Ordering::SeqCst);
            out.write_all(b"listening on http://127.0.0.1:7777\n")
                .unwrap();
            tokio::spawn(async move {
                shutdown.await;
                counter.fetch_sub(1, Ordering::SeqCst);
            });
        }));
    gateway.turn_on().await;
    assert_eq!(running.load(Ordering::SeqCst), 1);
    gateway.turn_off().await;
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        while running.load(Ordering::SeqCst) != 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(gateway.current().await, ServerStateEnum::Off);
    gateway.turn_on().await;
    assert_eq!(running.load(Ordering::SeqCst), 1);
    assert_eq!(
        gateway.current().await,
        ServerStateEnum::Running("http://127.0.0.1:7777".to_owned())
    );
}
