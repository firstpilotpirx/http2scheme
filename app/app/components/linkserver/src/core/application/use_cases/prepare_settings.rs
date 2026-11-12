//! Input port: read the settings and the key, creating what is missing.

use async_trait::async_trait;

use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

/// read the settings and the key, creating what is missing.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PrepareSettings: Send + Sync {
    /// Runs the use case.
    async fn execute(&self) -> Result<(ServerSettingsValueObject, AccessKeyValueObject), SettingsError>;
}
