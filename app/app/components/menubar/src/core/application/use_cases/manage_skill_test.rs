use std::path::PathBuf;
use std::sync::Arc;

use super::{ManageSkill, MockManageSkill};
use crate::core::domain::agents_command_enum::AgentsCommandEnum;
use crate::core::domain::agents_view_value_object::AgentsViewValueObject;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockManageSkill::new();
    mock.expect_execute().returning(|_| {
        AgentsViewValueObject::new(PathBuf::from("/c"), PathBuf::from("/h"), None, None)
    });
    let port: Arc<dyn ManageSkill> = Arc::new(mock);
    let view = port.execute(AgentsCommandEnum::Show(None)).await;
    assert_eq!(view.folder_line(), "/c");
}
