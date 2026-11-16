use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::{AgentSkillService, MockAgentSkillService};
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockAgentSkillService::new();
    mock.expect_text().returning(|| "skill".to_owned());
    mock.expect_inspect()
        .returning(|_| Err("denied".to_owned()));
    mock.expect_install().returning(|dir| {
        Ok(SkillPlaceValueObject::new(
            dir.join("f"),
            None,
            SkillStateEnum::Installed,
        ))
    });
    let port: Arc<dyn AgentSkillService> = Arc::new(mock);
    assert_eq!(port.text().await, "skill");
    assert!(port.inspect(Path::new("/c")).await.is_err());
    let place = port.install(Path::new("/c")).await.unwrap();
    assert_eq!(place.file(), PathBuf::from("/c/f"));
}
