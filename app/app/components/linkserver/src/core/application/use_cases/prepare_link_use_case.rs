//! Builds the open link of a scheme link.

use std::sync::Arc;

use async_trait::async_trait;

use super::prepare_link::PrepareLink;
use super::prepare_settings::PrepareSettings;
use crate::core::application::exceptions::prepare_link_error::PrepareLinkError;
use crate::core::domain::prepared_link_value_object::PreparedLinkValueObject;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

/// Checks the link against the settings and returns port, key and link.
pub struct PrepareLinkUseCase {
    settings: Arc<dyn PrepareSettings>,
}

impl PrepareLinkUseCase {
    /// A use case reading the settings through `settings`.
    #[must_use]
    pub fn new(settings: Arc<dyn PrepareSettings>) -> Self {
        Self { settings }
    }
}

#[async_trait]
impl PrepareLink for PrepareLinkUseCase {
    async fn execute(&self, link: &str) -> Result<PreparedLinkValueObject, PrepareLinkError> {
        let (settings, key) = self.settings.execute().await?;
        let link = SchemeLinkValueObject::parse(link)?;
        settings.allowlist().check(&link)?;
        Ok(PreparedLinkValueObject::new(settings.port(), key, link))
    }
}

#[cfg(test)]
#[path = "prepare_link_use_case_test.rs"]
mod prepare_link_use_case_test;
