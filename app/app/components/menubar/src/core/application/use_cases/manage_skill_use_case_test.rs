use std::path::{Path, PathBuf};
use std::sync::Arc;

use mockall::predicate::eq;

use super::ManageSkillUseCase;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::skill_folder_repository::MockSkillFolderRepository;
use crate::core::application::services::agent_skill_service::MockAgentSkillService;
use crate::core::application::use_cases::manage_skill::ManageSkill;
use crate::core::domain::agents_command_enum::AgentsCommandEnum;
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
        .execute(AgentsCommandEnum::Show(None))
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
    .execute(AgentsCommandEnum::Show(None))
    .await;
    assert_eq!(view.folder(), Path::new("/w/.claude"));
}

#[tokio::test]
async fn a_chosen_folder_is_shown_with_its_state() {
    let view = use_case(
        MockSkillFolderRepository::new(),
        inspects(SkillStateEnum::Outdated),
    )
    .execute(AgentsCommandEnum::Show(Some(PathBuf::from("/w/.claude"))))
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
        .execute(AgentsCommandEnum::Install(PathBuf::from("/w/.claude")))
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
        .execute(AgentsCommandEnum::Install(PathBuf::from("/w/.claude")))
        .await;
    assert_eq!(view.error(), Some("/w/.claude/skills: permission denied"));
    assert_eq!(view.state(), Some(SkillStateEnum::NotInstalled));
}

#[tokio::test]
async fn copy_shows_the_folder_and_hands_over_the_text() {
    let mut skill = inspects(SkillStateEnum::Installed);
    skill.expect_text().returning(|| "the skill".to_owned());
    let view = use_case(MockSkillFolderRepository::new(), skill)
        .execute(AgentsCommandEnum::Copy(PathBuf::from("/w/.claude")))
        .await;
    assert_eq!(view.folder(), Path::new("/w/.claude"));
    assert_eq!(view.clipboard(), Some("the skill"));
}

#[tokio::test]
async fn an_unreadable_record_shows_the_default_folder_and_why() {
    let mut folders = MockSkillFolderRepository::new();
    folders
        .expect_find()
        .returning(|| Err(MenuActionError::Skill("record unreadable".to_owned())));
    let view = use_case(folders, inspects(SkillStateEnum::NotInstalled))
        .execute(AgentsCommandEnum::Show(None))
        .await;
    assert_eq!(view.folder(), Path::new("/h/.claude"));
    assert!(view.error().is_some());
}

#[tokio::test]
async fn an_unreadable_folder_shows_the_reason() {
    let mut skill = MockAgentSkillService::new();
    skill
        .expect_inspect()
        .returning(|_| Err("/w/.claude: denied".to_owned()));
    let view = use_case(MockSkillFolderRepository::new(), skill)
        .execute(AgentsCommandEnum::Show(Some(PathBuf::from("/w/.claude"))))
        .await;
    assert_eq!(view.state(), None);
    assert_eq!(view.error(), Some("/w/.claude: denied"));
}

#[tokio::test]
async fn a_failed_record_after_install_is_shown() {
    let mut skill = MockAgentSkillService::new();
    skill
        .expect_install()
        .returning(|dir| Ok(place(dir, SkillStateEnum::Installed)));
    let mut folders = MockSkillFolderRepository::new();
    folders
        .expect_save()
        .returning(|_| Err(MenuActionError::Skill("record not written".to_owned())));
    let view = use_case(folders, skill)
        .execute(AgentsCommandEnum::Install(PathBuf::from("/w/.claude")))
        .await;
    assert_eq!(view.state(), Some(SkillStateEnum::Installed));
    assert!(view.error().is_some());
}
