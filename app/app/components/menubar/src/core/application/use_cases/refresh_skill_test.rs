use std::sync::Arc;

use super::{MockRefreshSkill, RefreshSkill};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockRefreshSkill::new();
    mock.expect_execute().times(1).returning(|| Ok(()));
    let port: Arc<dyn RefreshSkill> = Arc::new(mock);
    port.execute().await.unwrap();
}
