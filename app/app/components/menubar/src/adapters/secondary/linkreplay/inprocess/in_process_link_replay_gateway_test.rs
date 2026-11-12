use std::sync::Arc;

use super::InProcessLinkReplayGateway;
use crate::core::application::services::link_replay_service::LinkReplayService;

#[tokio::test]
async fn answers_what_the_replayer_answers() {
    let gateway = InProcessLinkReplayGateway::new(Arc::new(|link| format!("opened {link}")));
    assert_eq!(gateway.replay("idea://x").await, "opened idea://x");
}
