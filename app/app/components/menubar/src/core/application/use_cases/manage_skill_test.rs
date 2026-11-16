use std::path::PathBuf;
use std::sync::Arc;

use super::{ManageSkill, MockManageSkill};
use crate::core::domain::agents_view_value_object::AgentsViewValueObject;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockManageSkill::new();
    mock.expect_show().returning(|folder| {
        AgentsViewValueObject::new(folder.unwrap_or_default(), PathBuf::from("/h"), None, None)
    });
    mock.expect_skill_text().returning(|| "skill".to_owned());
    let port: Arc<dyn ManageSkill> = Arc::new(mock);
    let view = port.show(Some(PathBuf::from("/c"))).await;
    assert_eq!(view.folder_line(), "/c");
    assert_eq!(port.skill_text().await, "skill");
}
