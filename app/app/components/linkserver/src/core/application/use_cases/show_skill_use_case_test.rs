use super::ShowSkillUseCase;
use crate::core::application::use_cases::show_skill::ShowSkill;
use crate::core::domain::agent_skill_value_object::AgentSkillValueObject;

#[tokio::test]
async fn renders_the_skill_for_the_program() {
    let skill = ShowSkillUseCase::new("/opt/http2scheme".to_owned())
        .execute()
        .await;
    assert_eq!(
        skill,
        AgentSkillValueObject::for_program("/opt/http2scheme")
    );
}
