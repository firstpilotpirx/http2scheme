//! Reads the settings and the key, creating what is missing.

use std::sync::Arc;

use async_trait::async_trait;

use super::prepare_settings::PrepareSettings;
use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::application::repositories::settings_repository::SettingsRepository;
use crate::core::application::services::secret_generator_service::SecretGeneratorService;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

/// Creates default settings and a fresh key on first use; keeps existing ones.
pub struct PrepareSettingsUseCase {
    settings: Arc<dyn SettingsRepository>,
    generator: Arc<dyn SecretGeneratorService>,
}

impl PrepareSettingsUseCase {
    /// A use case over the settings repository and the key source.
    #[must_use]
    pub fn new(
        settings: Arc<dyn SettingsRepository>,
        generator: Arc<dyn SecretGeneratorService>,
    ) -> Self {
        Self {
            settings,
            generator,
        }
    }

    async fn settings(&self) -> Result<ServerSettingsValueObject, SettingsError> {
        if let Some(existing) = self.settings.read_settings().await? {
            return Ok(existing);
        }
        let defaults = ServerSettingsValueObject::defaults();
        self.settings.create_settings(&defaults).await?;
        Ok(defaults)
    }

    async fn key(&self) -> Result<AccessKeyValueObject, SettingsError> {
        if let Some(existing) = self.settings.read_key().await? {
            return Ok(existing);
        }
        let key = self.generator.generate_key().await?;
        self.settings.create_key(&key).await?;
        Ok(key)
    }
}

#[async_trait]
impl PrepareSettings for PrepareSettingsUseCase {
    async fn execute(
        &self,
    ) -> Result<(ServerSettingsValueObject, AccessKeyValueObject), SettingsError> {
        let settings = self.settings().await?;
        let key = self.key().await?;
        Ok((settings, key))
    }
}

#[cfg(test)]
#[path = "prepare_settings_use_case_test.rs"]
mod prepare_settings_use_case_test;
