//! Scheme links through `open`.

use std::path::PathBuf;

use async_trait::async_trait;
use tokio::process::Command;

use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::application::services::scheme_link_opener_service::SchemeLinkOpenerService;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

/// Runs `<program> <link>`; the link is one argument and never passes a shell.
#[derive(Debug, Clone)]
pub struct LaunchServicesSchemeLinkOpenerGateway {
    program: PathBuf,
}

impl LaunchServicesSchemeLinkOpenerGateway {
    /// An opener that runs `program`.
    #[must_use]
    pub const fn new(program: PathBuf) -> Self {
        Self { program }
    }
}

#[async_trait]
impl SchemeLinkOpenerService for LaunchServicesSchemeLinkOpenerGateway {
    async fn open(&self, link: &SchemeLinkValueObject) -> Result<(), OpenLinkError> {
        let output = Command::new(&self.program)
            .arg(link.as_str())
            .output()
            .await
            .map_err(|error| OpenLinkError::OpenFailed {
                reason: format!("{}: {error}", self.program.display()),
            })?;
        if output.status.success() {
            return Ok(());
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(OpenLinkError::OpenFailed {
            reason: format!("{}: {}", output.status, stderr.trim()),
        })
    }
}
