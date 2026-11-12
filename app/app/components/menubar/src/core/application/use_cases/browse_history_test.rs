use std::sync::Arc;

use super::{BrowseHistory, MockBrowseHistory};
use crate::core::domain::history_command_enum::HistoryCommandEnum;
use crate::core::domain::history_view_value_object::HistoryViewValueObject;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockBrowseHistory::new();
    mock.expect_execute()
        .returning(|_| Ok(HistoryViewValueObject::new(Vec::new())));
    let port: Arc<dyn BrowseHistory> = Arc::new(mock);
    let view = port.execute(HistoryCommandEnum::Show).await.unwrap();
    assert_eq!(view.rows(), []);
}
