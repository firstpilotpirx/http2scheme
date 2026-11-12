//! Brings back the app left for the browser through `lsappinfo` and `open -b`.

use std::path::PathBuf;

use async_trait::async_trait;
use tokio::process::Command;

use super::process_list_to_left_app_mapper::{bundle_id, left_app};
use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::application::services::app_order_service::AppOrderService;

/// Reads the front-to-back app list with `lsappinfo` and activates the app behind the browser
/// with `open -b <bundle id>`, waiting for it.
#[derive(Debug, Clone)]
pub struct LsAppInfoAppOrderGateway {
    lsappinfo: PathBuf,
    open: PathBuf,
}

impl LsAppInfoAppOrderGateway {
    /// A gateway over the `lsappinfo` and `open` programs.
    #[must_use]
    pub const fn new(lsappinfo: PathBuf, open: PathBuf) -> Self {
        Self { lsappinfo, open }
    }

    /// Brings the app back now and says why it did not.
    ///
    /// # Errors
    /// A program fails, no browser is in front, no app is behind it, or it has no bundle id.
    pub async fn bring_back_now(&self) -> Result<(), OpenLinkError> {
        let list = run(&self.lsappinfo, &["visibleProcessList"]).await?;
        let asn =
            left_app(&list).ok_or_else(|| failed("no browser in front with an app behind it"))?;
        let info = run(&self.lsappinfo, &["info", "-only", "bundleid", &asn]).await?;
        let id = bundle_id(&info).ok_or_else(|| failed(&format!("{asn} has no bundle id")))?;
        run(&self.open, &["-b", &id]).await.map(|_| ())
    }
}

#[async_trait]
impl AppOrderService for LsAppInfoAppOrderGateway {
    async fn bring_back_left_app(&self) -> Result<(), OpenLinkError> {
        if let Err(error) = self.bring_back_now().await {
            log::info!("{error}");
        }
        Ok(())
    }
}

fn failed(reason: &str) -> OpenLinkError {
    OpenLinkError::ReorderFailed {
        reason: reason.to_owned(),
    }
}

/// Runs `program` with `args` and answers its output.
async fn run(program: &PathBuf, args: &[&str]) -> Result<String, OpenLinkError> {
    let output = Command::new(program)
        .args(args)
        .output()
        .await
        .map_err(|error| failed(&format!("{}: {error}", program.display())))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(failed(&format!(
            "{} {}: {}",
            program.display(),
            output.status,
            stderr.trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
#[path = "ls_app_info_app_order_gateway_test.rs"]
mod ls_app_info_app_order_gateway_test;
