use std::path::{Path, PathBuf};
use std::sync::Arc;

use mockall::predicate::eq;

use super::InstallSkillUseCase;
use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::application::repositories::skill_repository::MockSkillRepository;
use crate::core::application::use_cases::install_skill::InstallSkill;
use crate::core::application::use_cases::skill_fixtures_test::{current, show};
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[tokio::test]
async fn writes_the_current_text_and_returns_the_place() {
    let real = PathBuf::from("/kit/skills/http2scheme/SKILL.md");
    let mut repository = MockSkillRepository::new();
    repository
        .expect_save()
        .with(eq(PathBuf::from("/c")), eq(current()))
        .times(1)
        .returning(|_, _| Ok(()));
    let found = real.clone();
    repository
        .expect_find()
        .returning(move |_| Ok((Some(current()), Some(found.clone()))));
    let use_case = InstallSkillUseCase::new(show(), Arc::new(repository));
    let place = use_case.execute(Path::new("/c")).await.unwrap();
    assert_eq!(place.state(), SkillStateEnum::Installed);
    assert_eq!(place.file(), Path::new("/c/skills/http2scheme/SKILL.md"));
    assert_eq!(place.real_file(), Some(real.as_path()));
}

#[tokio::test]
async fn a_write_error_is_returned() {
    let mut repository = MockSkillRepository::new();
    repository.expect_save().returning(|_, _| {
        Err(InstallSkillError::Io {
            path: "/c/skills".to_owned(),
            reason: "permission denied".to_owned(),
        })
    });
    let use_case = InstallSkillUseCase::new(show(), Arc::new(repository));
    let error = use_case.execute(Path::new("/c")).await.unwrap_err();
    assert_eq!(error.to_string(), "/c/skills: permission denied");
}
