//! `InProcessLinkReplayGateway` over the app's replayer hook.
#![allow(
    clippy::unwrap_used,
    reason = "test helpers fail loudly, like the tests"
)]

use std::sync::{Arc, Mutex};

use menubar::adapters::secondary::linkreplay::inprocess::in_process_link_replay_gateway::InProcessLinkReplayGateway;
use menubar::core::application::services::link_replay_service::LinkReplayService;

#[tokio::test]
async fn the_link_goes_to_the_hook_and_its_result_comes_back() {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&seen);
    let gateway = InProcessLinkReplayGateway::new(Arc::new(move |link: &str| {
        sink.lock().unwrap().push(link.to_owned());
        "scheme not allowed".to_owned()
    }));
    assert_eq!(gateway.replay("slack://x").await, "scheme not allowed");
    assert_eq!(*seen.lock().unwrap(), ["slack://x"]);
}
