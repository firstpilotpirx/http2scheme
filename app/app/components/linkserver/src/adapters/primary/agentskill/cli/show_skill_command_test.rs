use std::sync::Arc;

use super::ShowSkillCommand;
use crate::core::application::use_cases::show_skill::MockShowSkill;
use crate::core::domain::agent_skill_value_object::AgentSkillValueObject;

#[tokio::test]
async fn prints_the_skill_text_unchanged() {
    let mut use_case = MockShowSkill::new();
    use_case
        .expect_execute()
        .returning(|| AgentSkillValueObject::for_program("/p"));
    let command = ShowSkillCommand::new(Arc::new(use_case));
    let (mut out, mut err) = (Vec::new(), Vec::new());
    assert_eq!(command.run(&mut out, &mut err).await, 0);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        AgentSkillValueObject::for_program("/p").text()
    );
    assert!(err.is_empty());
}
