use std::path::Path;
use std::sync::Arc;

use super::AgentSkillHandler;
use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::application::use_cases::inspect_skill::MockInspectSkill;
use crate::core::application::use_cases::install_skill::MockInstallSkill;
use crate::core::application::use_cases::show_skill::MockShowSkill;
use crate::core::domain::agent_skill_value_object::AgentSkillValueObject;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[tokio::test]
async fn answers_the_text_the_state_and_the_reason() {
    let mut show = MockShowSkill::new();
    show.expect_execute()
        .returning(|| AgentSkillValueObject::for_program("/p"));
    let mut inspect = MockInspectSkill::new();
    inspect.expect_execute().returning(|dir| {
        Ok(SkillPlaceValueObject::new(
            dir,
            None,
            SkillStateEnum::Outdated,
        ))
    });
    let mut install = MockInstallSkill::new();
    install.expect_execute().returning(|_| {
        Err(InstallSkillError::Io {
            path: "/c/skills".to_owned(),
            reason: "denied".to_owned(),
        })
    });
    let handler = AgentSkillHandler::new(Arc::new(show), Arc::new(inspect), Arc::new(install));
    assert_eq!(
        handler.text().await,
        AgentSkillValueObject::for_program("/p").text()
    );
    let place = handler.inspect(Path::new("/c")).await.unwrap();
    assert_eq!(place.state(), SkillStateEnum::Outdated);
    assert_eq!(
        handler.install(Path::new("/c")).await.unwrap_err(),
        "/c/skills: denied"
    );
}
