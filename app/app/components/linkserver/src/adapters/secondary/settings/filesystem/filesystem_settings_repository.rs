//! Settings and key files in one folder.

use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write as _};
use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use config::{Config, ConfigError, File, FileFormat};

use super::settings_record::{FIELDS, SettingsRecord};
use super::settings_record_to_domain_mapper::{to_domain, to_text};
use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::application::repositories::settings_repository::SettingsRepository;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;
use crate::core::domain::settings_invalid_error::SettingsInvalidError;

const SETTINGS_FILE: &str = "settings.toml";
const KEY_FILE: &str = "key";
const OTHERS_MASK: u32 = 0o077;
const OWNER_ONLY: u32 = 0o600;

/// Reads and creates `settings.toml` and `key` in one folder.
#[derive(Debug, Clone)]
pub struct FilesystemSettingsRepository {
    dir: PathBuf,
}

impl FilesystemSettingsRepository {
    /// A repository over `dir`; the folder is created on the first write.
    #[must_use]
    pub const fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    fn settings_path(&self) -> PathBuf {
        self.dir.join(SETTINGS_FILE)
    }

    fn key_path(&self) -> PathBuf {
        self.dir.join(KEY_FILE)
    }
}

#[async_trait]
impl SettingsRepository for FilesystemSettingsRepository {
    async fn read_settings(&self) -> Result<Option<ServerSettingsValueObject>, SettingsError> {
        let path = self.settings_path();
        let Some(text) = read_optional(&path)? else {
            return Ok(None);
        };
        let file = path.display().to_string();
        let record = parse_record(&text, &file)?;
        Ok(Some(to_domain(&record, &file)?))
    }

    async fn create_settings(
        &self,
        settings: &ServerSettingsValueObject,
    ) -> Result<(), SettingsError> {
        create_new(&self.settings_path(), &to_text(settings), None)
    }

    async fn read_key(&self) -> Result<Option<AccessKeyValueObject>, SettingsError> {
        let path = self.key_path();
        let Some(text) = read_optional(&path)? else {
            return Ok(None);
        };
        let file = path.display().to_string();
        let mode = fs::metadata(&path)
            .map_err(|error| io_error(&path, &error))?
            .permissions()
            .mode();
        if mode & OTHERS_MASK != 0 {
            return Err(SettingsInvalidError::new(
                &file,
                "key",
                "must be readable only by its owner (chmod 600)",
            )
            .into());
        }
        AccessKeyValueObject::parse_hex(text.trim())
            .map(Some)
            .ok_or_else(|| {
                SettingsInvalidError::new(&file, "key", "must hold 64 hexadecimal characters")
                    .into()
            })
    }

    async fn create_key(&self, key: &AccessKeyValueObject) -> Result<(), SettingsError> {
        create_new(
            &self.key_path(),
            &format!("{}\n", key.to_hex()),
            Some(OWNER_ONLY),
        )
    }
}

fn parse_record(text: &str, file: &str) -> Result<SettingsRecord, SettingsInvalidError> {
    let config = Config::builder()
        .add_source(File::from_str(text, FileFormat::Toml))
        .build()
        .map_err(|error| SettingsInvalidError::new(file, "settings", &error.to_string()))?;
    let table: HashMap<String, config::Value> = config
        .clone()
        .try_deserialize()
        .map_err(|error| invalid(file, &error))?;
    if let Some(unknown) = table.keys().find(|key| !FIELDS.contains(&key.as_str())) {
        return Err(SettingsInvalidError::new(
            file,
            unknown,
            "is not a known setting",
        ));
    }
    config
        .try_deserialize()
        .map_err(|error| invalid(file, &error))
}

fn invalid(file: &str, error: &ConfigError) -> SettingsInvalidError {
    let field = match error {
        ConfigError::Type { key: Some(key), .. } => key.clone(),
        other => backticked(&other.to_string()).unwrap_or_else(|| "settings".to_owned()),
    };
    SettingsInvalidError::new(file, &field, &format!("is wrong: {error}"))
}

fn backticked(message: &str) -> Option<String> {
    let start = message.find('`')? + 1;
    let length = message[start..].find('`')?;
    Some(message[start..start + length].to_owned())
}

fn read_optional(path: &Path) -> Result<Option<String>, SettingsError> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(io_error(path, &error)),
    }
}

fn create_new(path: &Path, text: &str, mode: Option<u32>) -> Result<(), SettingsError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| io_error(parent, &error))?;
    }
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    if let Some(mode) = mode {
        options.mode(mode);
    }
    match options.open(path) {
        Ok(mut file) => file
            .write_all(text.as_bytes())
            .map_err(|error| io_error(path, &error)),
        Err(error) if error.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(io_error(path, &error)),
    }
}

fn io_error(path: &Path, error: &std::io::Error) -> SettingsError {
    SettingsError::Io {
        path: path.display().to_string(),
        reason: error.to_string(),
    }
}

#[cfg(test)]
#[path = "filesystem_settings_repository_test.rs"]
mod filesystem_settings_repository_test;
