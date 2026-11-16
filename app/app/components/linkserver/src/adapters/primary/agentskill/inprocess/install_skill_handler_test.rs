use std::path::Path;
use std::sync::Arc;

use super::InstallSkillHandler;
use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::application::use_cases::install_skill::MockInstallSkill;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[tokio::test]
async fn answers_the_place() {
    let mut use_case = MockInstallSkill::new();
    use_case.expect_execute().returning(|dir| {
        Ok(SkillPlaceValueObject::new(
            dir,
            None,
            SkillStateEnum::Outdated,
        ))
    });
    let handler = InstallSkillHandler::new(Arc::new(use_case));
    let (file, real_file, state) = handler.install(Path::new("/c")).await.unwrap();
    assert_eq!(file, Path::new("/c/skills/http2scheme/SKILL.md"));
    assert_eq!(real_file, None);
    assert_eq!(state, "outdated");
}

#[tokio::test]
async fn answers_the_reason_as_text() {
    let mut use_case = MockInstallSkill::new();
    use_case.expect_execute().returning(|_| {
        Err(InstallSkillError::Io {
            path: "/c/skills".to_owned(),
            reason: "denied".to_owned(),
        })
    });
    let handler = InstallSkillHandler::new(Arc::new(use_case));
    assert_eq!(
        handler.install(Path::new("/c")).await.unwrap_err(),
        "/c/skills: denied"
    );
}
