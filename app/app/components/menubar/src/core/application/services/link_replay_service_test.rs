use std::sync::Arc;

use super::{LinkReplayService, MockLinkReplayService};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockLinkReplayService::new();
    mock.expect_replay()
        .withf(|link| link == "idea://x")
        .return_const("opened".to_owned());
    let port: Arc<dyn LinkReplayService> = Arc::new(mock);
    assert_eq!(port.replay("idea://x").await, "opened");
}
