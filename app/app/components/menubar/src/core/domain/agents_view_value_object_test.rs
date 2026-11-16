use std::path::PathBuf;

use super::AgentsViewValueObject;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

fn view(state: SkillStateEnum, real: Option<&str>, error: Option<&str>) -> AgentsViewValueObject {
    let place = SkillPlaceValueObject::new(
        PathBuf::from("/Users/me/.claude/skills/http2scheme/SKILL.md"),
        real.map(PathBuf::from),
        state,
    );
    AgentsViewValueObject::new(
        PathBuf::from("/Users/me/.claude"),
        PathBuf::from("/Users/me"),
        Some(place),
        error.map(str::to_owned),
    )
}

#[test]
fn the_folder_shows_home_as_a_tilde() {
    let shown = view(SkillStateEnum::NotInstalled, None, None);
    assert_eq!(shown.folder_line(), "~/.claude");
    let other = AgentsViewValueObject::new(
        PathBuf::from("/Volumes/work/.claude"),
        PathBuf::from("/Users/me"),
        None,
        None,
    );
    assert_eq!(other.folder_line(), "/Volumes/work/.claude");
}

#[test]
fn the_state_line_names_the_state() {
    assert_eq!(view(SkillStateEnum::NotInstalled, None, None).skill_title(), "Skill · Not installed");
    assert_eq!(view(SkillStateEnum::Installed, None, None).skill_title(), "Skill · Installed");
    assert_eq!(
        view(SkillStateEnum::Outdated, None, None).skill_title(),
        "Skill · An older version is installed"
    );
}

#[test]
fn an_older_skill_offers_update() {
    assert_eq!(view(SkillStateEnum::Outdated, None, None).button(), "Update");
    assert_eq!(view(SkillStateEnum::NotInstalled, None, None).button(), "Install");
}

#[test]
fn the_line_under_the_skill_is_the_file() {
    let shown = view(SkillStateEnum::Installed, None, None);
    assert_eq!(shown.skill_line(), "~/.claude/skills/http2scheme/SKILL.md");
}

#[test]
fn a_linked_folder_names_the_real_place() {
    let shown = view(
        SkillStateEnum::Installed,
        Some("/Users/me/kit/skills/http2scheme/SKILL.md"),
        None,
    );
    assert_eq!(
        shown.skill_line(),
        "Lands in ~/kit/skills/http2scheme/SKILL.md (skills is a link)"
    );
}

#[test]
fn a_failure_replaces_the_line() {
    let shown = view(SkillStateEnum::NotInstalled, None, Some("/x: permission denied"));
    assert_eq!(shown.skill_line(), "/x: permission denied");
    assert_eq!(shown.error(), Some("/x: permission denied"));
}

#[test]
fn an_unread_folder_has_no_state() {
    let shown = AgentsViewValueObject::new(
        PathBuf::from("/c"),
        PathBuf::from("/h"),
        None,
        Some("/c: denied".to_owned()),
    );
    assert_eq!(shown.skill_title(), "Skill");
    assert_eq!(shown.button(), "Install");
}

#[test]
fn a_copy_changes_the_note() {
    let shown = view(SkillStateEnum::Installed, None, None);
    assert_eq!(shown.copy_line(), "Paste it into any agent's skill folder.");
    let copied = shown.copied();
    assert!(copied.is_copied());
    assert_eq!(copied.copy_line(), "Copied to the clipboard.");
}
