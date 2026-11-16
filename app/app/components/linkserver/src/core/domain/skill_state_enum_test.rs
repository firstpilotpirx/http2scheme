use super::SkillStateEnum;
use crate::core::domain::agent_skill_value_object::AgentSkillValueObject;

#[test]
fn no_file_is_not_installed() {
    let skill = AgentSkillValueObject::for_program("/p");
    assert_eq!(
        SkillStateEnum::of(None, &skill),
        SkillStateEnum::NotInstalled
    );
}

#[test]
fn the_same_text_is_installed() {
    let skill = AgentSkillValueObject::for_program("/p");
    assert_eq!(
        SkillStateEnum::of(Some(skill.text()), &skill),
        SkillStateEnum::Installed
    );
}

#[test]
fn another_text_is_outdated() {
    let skill = AgentSkillValueObject::for_program("/p");
    let older = AgentSkillValueObject::for_program("/old");
    assert_eq!(
        SkillStateEnum::of(Some(older.text()), &skill),
        SkillStateEnum::Outdated
    );
}
