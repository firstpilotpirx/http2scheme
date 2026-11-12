//! Port: a source of secret keys.

use async_trait::async_trait;

use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;

/// Makes a new random key.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait SecretGeneratorService: Send + Sync {
    /// A key of 256 random bits.
    async fn generate_key(&self) -> Result<AccessKeyValueObject, SettingsError>;
}

#[cfg(test)]
#[path = "secret_generator_service_test.rs"]
mod secret_generator_service_test;
