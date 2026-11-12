//! Browser tabs closed through `osascript`.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tokio::process::Command;

use super::port_to_apple_script_mapper::to_script;
use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::application::services::browser_tab_closer_service::BrowserTabCloserService;
use crate::core::domain::port_value_object::PortValueObject;

/// Closes the server's open-address tabs with `<program> -e <script>`, after a delay.
#[derive(Debug, Clone)]
pub struct AppleScriptBrowserTabCloserGateway {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    program: PathBuf,
    hosts: Vec<String>,
    delay: Duration,
}

impl AppleScriptBrowserTabCloserGateway {
    /// A closer that runs `program` for tabs on any of `hosts`, `delay` after it is asked.
    #[must_use]
    pub fn new(program: PathBuf, hosts: Vec<String>, delay: Duration) -> Self {
        Self {
            inner: Arc::new(Inner {
                program,
                hosts,
                delay,
            }),
        }
    }

    /// Runs the script now and waits for it.
    ///
    /// # Errors
    /// The program cannot start or exits with an error.
    pub async fn close_now(&self, port: PortValueObject) -> Result<(), OpenLinkError> {
        let script = to_script(&self.inner.hosts, port);
        let output = Command::new(&self.inner.program)
            .arg("-e")
            .arg(script)
            .output()
            .await
            .map_err(|error| OpenLinkError::TabCloseFailed {
                reason: format!("{}: {error}", self.inner.program.display()),
            })?;
        if output.status.success() {
            return Ok(());
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(OpenLinkError::TabCloseFailed {
            reason: format!("{}: {}", output.status, stderr.trim()),
        })
    }
}

#[async_trait]
impl BrowserTabCloserService for AppleScriptBrowserTabCloserGateway {
    async fn close_open_tabs(&self, port: PortValueObject) -> Result<(), OpenLinkError> {
        let closer = self.clone();
        tokio::spawn(async move {
            tokio::time::sleep(closer.inner.delay).await;
            if let Err(error) = closer.close_now(port).await {
                log::warn!("{error}");
            }
        });
        Ok(())
    }
}
