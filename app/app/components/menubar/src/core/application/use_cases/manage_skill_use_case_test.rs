use std::path::{Path, PathBuf};
use std::sync::Arc;

use mockall::predicate::eq;

use super::ManageSkillUseCase;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::skill_folder_repository::MockSkillFolderRepository;
use crate::core::application::services::agent_skill_service::MockAgentSkillService;
use crate::core::application::use_cases::manage_skill::ManageSkill;
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
) -> ManageSkillUseCase {
    ManageSkillUseCase::new(
        PathBuf::from("/h/.claude"),
        PathBuf::from("/h"),
        Arc::new(folders),
        Arc::new(skill),
    )
}

#[tokio::test]
async fn without_a_remembered_folder_it_shows_the_default_one() {
    let view = use_case(remembers(None), inspects(SkillStateEnum::NotInstalled))
        .show(None)
        .await;
    assert_eq!(view.folder(), Path::new("/h/.claude"));
    assert_eq!(view.state(), Some(SkillStateEnum::NotInstalled));
}

#[tokio::test]
async fn it_starts_from_the_remembered_folder() {
    let view = use_case(
        remembers(Some("/w/.claude")),
        inspects(SkillStateEnum::Installed),
    )
    .show(None)
    .await;
    assert_eq!(view.folder(), Path::new("/w/.claude"));
}

#[tokio::test]
async fn a_chosen_folder_is_shown_with_its_state() {
    let view = use_case(
        MockSkillFolderRepository::new(),
        inspects(SkillStateEnum::Outdated),
    )
    .show(Some(PathBuf::from("/w/.claude")))
    .await;
    assert_eq!(view.folder(), Path::new("/w/.claude"));
    assert_eq!(view.button(), "Update");
}

#[tokio::test]
async fn install_writes_and_remembers_the_folder() {
    let mut skill = MockAgentSkillService::new();
    skill
        .expect_install()
        .times(1)
        .returning(|dir| Ok(place(dir, SkillStateEnum::Installed)));
    let mut folders = MockSkillFolderRepository::new();
    folders
        .expect_save()
        .with(eq(PathBuf::from("/w/.claude")))
        .times(1)
        .returning(|_| Ok(()));
    let view = use_case(folders, skill)
        .install(PathBuf::from("/w/.claude"))
        .await;
    assert_eq!(view.state(), Some(SkillStateEnum::Installed));
    assert_eq!(view.error(), None);
}

#[tokio::test]
async fn a_failed_install_shows_the_reason_and_remembers_nothing() {
    let mut skill = inspects(SkillStateEnum::NotInstalled);
    skill
        .expect_install()
        .returning(|_| Err("/w/.claude/skills: permission denied".to_owned()));
    let view = use_case(MockSkillFolderRepository::new(), skill)
        .install(PathBuf::from("/w/.claude"))
        .await;
    assert_eq!(view.error(), Some("/w/.claude/skills: permission denied"));
    assert_eq!(view.state(), Some(SkillStateEnum::NotInstalled));
}

#[tokio::test]
async fn copy_returns_the_skill_text() {
    let mut skill = MockAgentSkillService::new();
    skill.expect_text().returning(|| "the skill".to_owned());
    let text = use_case(MockSkillFolderRepository::new(), skill)
        .skill_text()
        .await;
    assert_eq!(text, "the skill");
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
        .refresh()
        .await
        .unwrap();
}

#[tokio::test]
async fn refresh_leaves_a_current_or_removed_skill() {
    for state in [SkillStateEnum::Installed, SkillStateEnum::NotInstalled] {
        let mut skill = inspects(state);
        skill.expect_install().never();
        use_case(remembers(Some("/w/.claude")), skill)
            .refresh()
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn refresh_does_nothing_without_a_remembered_folder() {
    let mut skill = MockAgentSkillService::new();
    skill.expect_inspect().never();
    use_case(remembers(None), skill).refresh().await.unwrap();
}

#[tokio::test]
async fn refresh_reports_a_failed_write() {
    let mut skill = inspects(SkillStateEnum::Outdated);
    skill
        .expect_install()
        .returning(|_| Err("denied".to_owned()));
    let error = use_case(remembers(Some("/w")), skill)
        .refresh()
        .await
        .unwrap_err();
    assert_eq!(error, MenuActionError::Skill("denied".to_owned()));
}
