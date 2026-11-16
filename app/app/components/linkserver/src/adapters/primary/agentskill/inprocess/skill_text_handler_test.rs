use std::sync::Arc;

use super::SkillTextHandler;
use crate::core::application::use_cases::show_skill::MockShowSkill;
use crate::core::domain::agent_skill_value_object::AgentSkillValueObject;

#[tokio::test]
async fn answers_the_printed_text() {
    let mut show = MockShowSkill::new();
    show.expect_execute()
        .returning(|| AgentSkillValueObject::for_program("/p"));
    let handler = SkillTextHandler::new(Arc::new(show));
    assert_eq!(
        handler.text().await,
        AgentSkillValueObject::for_program("/p").text()
    );
}
