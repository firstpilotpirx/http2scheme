//! Port: the settings file and the key file.

use async_trait::async_trait;

use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

/// Reads and creates the settings and the key; never overwrites them.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait SettingsRepository: Send + Sync {
    /// The settings, or `None` when the file does not exist.
    async fn read_settings(&self) -> Result<Option<ServerSettingsValueObject>, SettingsError>;
    /// Creates the settings file; an existing file is kept.
    async fn create_settings(&self, settings: &ServerSettingsValueObject) -> Result<(), SettingsError>;
    /// The key, or `None` when the file does not exist.
    async fn read_key(&self) -> Result<Option<AccessKeyValueObject>, SettingsError>;
    /// Creates the key file readable only by its owner; an existing file is kept.
    async fn create_key(&self, key: &AccessKeyValueObject) -> Result<(), SettingsError>;
}
