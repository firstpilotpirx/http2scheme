use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::InspectSkillUseCase;
use crate::core::application::exceptions::install_skill_error::InstallSkillError;
use crate::core::application::repositories::skill_repository::MockSkillRepository;
use crate::core::application::use_cases::inspect_skill::InspectSkill;
use crate::core::application::use_cases::skill_fixtures_test::{current, show};
use crate::core::domain::skill_state_enum::SkillStateEnum;

async fn inspect(found: Option<String>) -> SkillStateEnum {
    let mut repository = MockSkillRepository::new();
    repository
        .expect_find()
        .returning(move |_| Ok((found.clone(), None)));
    let use_case = InspectSkillUseCase::new(show(), Arc::new(repository));
    use_case.execute(Path::new("/c")).await.unwrap().state()
}

#[tokio::test]
async fn an_absent_file_is_not_installed() {
    assert_eq!(inspect(None).await, SkillStateEnum::NotInstalled);
}

#[tokio::test]
async fn an_equal_file_is_installed() {
    assert_eq!(inspect(Some(current())).await, SkillStateEnum::Installed);
}

#[tokio::test]
async fn a_different_file_is_outdated() {
    assert_eq!(inspect(Some("older".to_owned())).await, SkillStateEnum::Outdated);
}

#[tokio::test]
async fn keeps_the_real_file_and_the_folder() {
    let real = PathBuf::from("/kit/skills/http2scheme/SKILL.md");
    let mut repository = MockSkillRepository::new();
    let found = real.clone();
    repository
        .expect_find()
        .withf(|dir| dir == Path::new("/work/.claude"))
        .returning(move |_| Ok((None, Some(found.clone()))));
    let use_case = InspectSkillUseCase::new(show(), Arc::new(repository));
    let place = use_case.execute(Path::new("/work/.claude")).await.unwrap();
    assert_eq!(place.file(), Path::new("/work/.claude/skills/http2scheme/SKILL.md"));
    assert_eq!(place.real_file(), Some(real.as_path()));
}

#[tokio::test]
async fn a_read_error_is_returned() {
    let mut repository = MockSkillRepository::new();
    repository.expect_find().returning(|_| {
        Err(InstallSkillError::Io {
            path: "/c".to_owned(),
            reason: "denied".to_owned(),
        })
    });
    let use_case = InspectSkillUseCase::new(show(), Arc::new(repository));
    assert!(use_case.execute(Path::new("/c")).await.is_err());
}
