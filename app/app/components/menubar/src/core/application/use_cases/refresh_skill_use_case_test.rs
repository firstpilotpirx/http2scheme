use std::path::{Path, PathBuf};
use std::sync::Arc;

use mockall::predicate::eq;

use super::RefreshSkillUseCase;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::skill_folder_repository::MockSkillFolderRepository;
use crate::core::application::services::agent_skill_service::MockAgentSkillService;
use crate::core::application::use_cases::refresh_skill::RefreshSkill;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

fn place(dir: &Path, state: SkillStateEnum) -> SkillPlaceValueObject {
    SkillPlaceValueObject::new(dir.join("skills/http2scheme/SKILL.md"), None, state)
}

fn inspects(state: SkillStateEnum) -> MockAgentSkillService {
    let mut skill = MockAgentSkillService::new();
    skill
        .expect_inspect()
        .returning(move |dir| Ok(place(dir, state)));
    skill
}

fn remembers(folder: Option<&'static str>) -> MockSkillFolderRepository {
    let mut folders = MockSkillFolderRepository::new();
    folders
        .expect_find()
        .returning(move || Ok(folder.map(PathBuf::from)));
    folders
}

fn use_case(
    folders: MockSkillFolderRepository,
    skill: MockAgentSkillService,
) -> RefreshSkillUseCase {
    RefreshSkillUseCase::new(Arc::new(folders), Arc::new(skill))
}

#[tokio::test]
async fn refresh_installs_only_an_outdated_skill() {
    let mut skill = inspects(SkillStateEnum::Outdated);
    skill
        .expect_install()
        .with(eq(PathBuf::from("/w/.claude")))
        .times(1)
        .returning(|dir| Ok(place(dir, SkillStateEnum::Installed)));
    use_case(remembers(Some("/w/.claude")), skill)
        .execute()
        .await
        .unwrap();
}

#[tokio::test]
async fn refresh_leaves_a_current_or_removed_skill() {
    for state in [SkillStateEnum::Installed, SkillStateEnum::NotInstalled] {
        let mut skill = inspects(state);
        skill.expect_install().never();
        use_case(remembers(Some("/w/.claude")), skill)
            .execute()
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn refresh_does_nothing_without_a_remembered_folder() {
    let mut skill = MockAgentSkillService::new();
    skill.expect_inspect().never();
    use_case(remembers(None), skill).execute().await.unwrap();
}

#[tokio::test]
async fn refresh_reports_a_failed_write() {
    let mut skill = inspects(SkillStateEnum::Outdated);
    skill
        .expect_install()
        .returning(|_| Err("denied".to_owned()));
    let error = use_case(remembers(Some("/w")), skill)
        .execute()
        .await
        .unwrap_err();
    assert_eq!(error, MenuActionError::Skill("denied".to_owned()));
}
