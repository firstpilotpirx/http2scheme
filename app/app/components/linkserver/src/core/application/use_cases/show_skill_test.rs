use std::sync::Arc;

use super::{MockShowSkill, ShowSkill};
use crate::core::domain::agent_skill_value_object::AgentSkillValueObject;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockShowSkill::new();
    mock.expect_execute()
        .times(1)
        .returning(|| AgentSkillValueObject::for_program("/p"));
    let port: Arc<dyn ShowSkill> = Arc::new(mock);
    assert!(port.execute().await.text().contains("'/p' link"));
}
