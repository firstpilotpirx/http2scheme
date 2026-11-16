use super::key_equivalent;
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[test]
fn return_presses_only_update() {
    assert_eq!(key_equivalent(Some(SkillStateEnum::Outdated)), "\r");
    assert_eq!(key_equivalent(Some(SkillStateEnum::Installed)), "");
    assert_eq!(key_equivalent(Some(SkillStateEnum::NotInstalled)), "");
    assert_eq!(key_equivalent(None), "");
}
