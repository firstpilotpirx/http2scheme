use std::path::Path;
use std::sync::Arc;

use super::{InspectSkill, MockInspectSkill};
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockInspectSkill::new();
    mock.expect_execute()
        .returning(|dir| Ok(SkillPlaceValueObject::new(dir, None, SkillStateEnum::NotInstalled)));
    let port: Arc<dyn InspectSkill> = Arc::new(mock);
    let place = port.execute(Path::new("/c")).await.unwrap();
    assert_eq!(place.state(), SkillStateEnum::NotInstalled);
}
