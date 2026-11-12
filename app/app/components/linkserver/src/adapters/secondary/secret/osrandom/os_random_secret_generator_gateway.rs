//! Keys from the OS random source.

use async_trait::async_trait;

use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::application::services::secret_generator_service::SecretGeneratorService;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;

/// Makes keys from 32 bytes of the OS random source.
#[derive(Debug, Clone, Copy, Default)]
pub struct OsRandomSecretGeneratorGateway;

#[async_trait]
impl SecretGeneratorService for OsRandomSecretGeneratorGateway {
    async fn generate_key(&self) -> Result<AccessKeyValueObject, SettingsError> {
        let mut bytes = [0_u8; 32];
        getrandom::fill(&mut bytes).map_err(|error| SettingsError::Io {
            path: "OS random source".to_owned(),
            reason: error.to_string(),
        })?;
        Ok(AccessKeyValueObject::from_bytes(bytes))
    }
}

#[cfg(test)]
#[path = "os_random_secret_generator_gateway_test.rs"]
mod os_random_secret_generator_gateway_test;
