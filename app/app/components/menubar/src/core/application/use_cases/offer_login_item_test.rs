use std::sync::Arc;

use super::{MockOfferLoginItem, OfferLoginItem};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockOfferLoginItem::new();
    mock.expect_execute().times(1).returning(|| Ok(()));
    let port: Arc<dyn OfferLoginItem> = Arc::new(mock);
    assert_eq!(port.execute().await, Ok(()));
}
