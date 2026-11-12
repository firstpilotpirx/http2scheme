use std::sync::Arc;

use super::{FirstLaunchRepository, MockFirstLaunchRepository};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockFirstLaunchRepository::new();
    mock.expect_exists().returning(|| Ok(false));
    mock.expect_record().returning(|| Ok(()));
    let port: Arc<dyn FirstLaunchRepository> = Arc::new(mock);
    assert_eq!(port.exists().await, Ok(false));
    assert_eq!(port.record().await, Ok(()));
}
