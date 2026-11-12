//! launchd through `launchctl`.

use std::path::PathBuf;
use std::process::Output;

use async_trait::async_trait;
use tokio::process::Command;

use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::services::agent_supervisor_service::AgentSupervisorService;
use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;

/// Loads, unloads and inspects agents in the user's `gui/<uid>` domain.
#[derive(Debug, Clone)]
pub struct LaunchdAgentSupervisorGateway {
    program: PathBuf,
    uid: u32,
    agents_dir: PathBuf,
}

impl LaunchdAgentSupervisorGateway {
    /// A supervisor that runs `program` for user `uid`; definitions live in `agents_dir`.
    #[must_use]
    pub const fn new(program: PathBuf, uid: u32, agents_dir: PathBuf) -> Self {
        Self {
            program,
            uid,
            agents_dir,
        }
    }

    fn domain(&self) -> String {
        format!("gui/{}", self.uid)
    }

    fn service(&self, label: &str) -> String {
        format!("gui/{}/{label}", self.uid)
    }

    async fn run(&self, arguments: &[String]) -> Result<Output, AgentControlError> {
        Command::new(&self.program)
            .args(arguments)
            .output()
            .await
            .map_err(|error| AgentControlError::Supervisor {
                reason: format!("{}: {error}", self.program.display()),
            })
    }

    async fn run_checked(&self, arguments: &[String]) -> Result<(), AgentControlError> {
        let output = self.run(arguments).await?;
        if output.status.success() {
            return Ok(());
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(AgentControlError::Supervisor {
            reason: format!("{} {}: {}", arguments[0], output.status, stderr.trim()),
        })
    }
}

#[async_trait]
impl AgentSupervisorService for LaunchdAgentSupervisorGateway {
    async fn load(&self, definition: &AgentDefinitionValueObject) -> Result<(), AgentControlError> {
        let plist = self
            .agents_dir
            .join(format!("{}.plist", definition.label()));
        self.run_checked(&[
            "bootstrap".to_owned(),
            self.domain(),
            plist.display().to_string(),
        ])
        .await
    }

    async fn unload(&self, label: &str) -> Result<(), AgentControlError> {
        self.run_checked(&["bootout".to_owned(), self.service(label)])
            .await
    }

    async fn is_loaded(&self, label: &str) -> Result<bool, AgentControlError> {
        let output = self.run(&["print".to_owned(), self.service(label)]).await?;
        Ok(output.status.success())
    }
}
