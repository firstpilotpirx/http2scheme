use std::sync::Arc;

use super::{LinkHistoryRepository, MockLinkHistoryRepository};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockLinkHistoryRepository::new();
    mock.expect_find_latest()
        .withf(|limit| *limit == 100)
        .returning(|_| Ok(Vec::new()));
    let port: Arc<dyn LinkHistoryRepository> = Arc::new(mock);
    assert_eq!(port.find_latest(100).await, Ok(Vec::new()));
}
