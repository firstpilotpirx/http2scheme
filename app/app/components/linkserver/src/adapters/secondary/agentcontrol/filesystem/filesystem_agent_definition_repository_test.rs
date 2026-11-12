use std::path::PathBuf;

use super::FilesystemAgentDefinitionRepository;
use crate::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;

#[tokio::test]
async fn a_missing_folder_holds_no_agent() {
    let repository = FilesystemAgentDefinitionRepository::new(PathBuf::from("/nonexistent/agents"));
    assert!(!repository.exists("io.example.agent").await.unwrap());
}
