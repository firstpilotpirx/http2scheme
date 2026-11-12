use std::sync::Arc;

use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

use super::{MockReplayLink, ReplayLink};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockReplayLink::new();
    mock.expect_execute().returning(|_| OpenOutcomeEnum::Opened);
    let port: Arc<dyn ReplayLink> = Arc::new(mock);
    assert_eq!(port.execute("idea://x").await.label(), "opened");
}
