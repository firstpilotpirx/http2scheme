use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::InProcessAgentSkillGateway;
use crate::core::application::services::agent_skill_service::AgentSkillService;
use crate::core::domain::skill_place_value_object::SkillPlaceValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

#[tokio::test]
async fn answers_what_the_hooks_answer() {
    let gateway = InProcessAgentSkillGateway::new(
        Arc::new(|| "skill".to_owned()),
        Arc::new(|dir| Ok(SkillPlaceValueObject::new(dir.join("f"), None, SkillStateEnum::Outdated))),
        Arc::new(|dir| Err(format!("{}: denied", dir.display()))),
    );
    assert_eq!(gateway.text().await, "skill");
    let place = gateway.inspect(Path::new("/c")).await.unwrap();
    assert_eq!(place.file(), PathBuf::from("/c/f"));
    assert_eq!(gateway.install(Path::new("/c")).await.unwrap_err(), "/c: denied");
}
