use std::path::Path;
use std::sync::Arc;

use super::{InstallSkill, MockInstallSkill};
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockInstallSkill::new();
    mock.expect_execute()
        .times(1)
        .returning(|dir| Ok(SkillPlaceValueObject::new(dir, None, SkillStateEnum::Installed)));
    let port: Arc<dyn InstallSkill> = Arc::new(mock);
    assert!(port.execute(Path::new("/c")).await.is_ok());
}
