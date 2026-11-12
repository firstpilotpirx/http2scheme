use std::sync::Arc;

use super::PrepareSettingsUseCase;
use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::application::repositories::settings_repository::MockSettingsRepository;
use crate::core::application::services::secret_generator_service::MockSecretGeneratorService;
use crate::core::application::use_cases::prepare_settings::PrepareSettings;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::port_value_object::PortValueObject;
use crate::core::domain::scheme_allowlist_policy::SchemeAllowlistPolicy;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

fn io_error() -> SettingsError {
    SettingsError::Io {
        path: "/s".to_owned(),
        reason: "denied".to_owned(),
    }
}

#[tokio::test]
async fn creates_default_settings_and_a_generated_key_when_missing() {
    let mut settings = MockSettingsRepository::new();
    settings
        .expect_read_settings()
        .times(1)
        .returning(|| Ok(None));
    settings
        .expect_create_settings()
        .withf(|s| *s == ServerSettingsValueObject::defaults())
        .times(1)
        .returning(|_| Ok(()));
    settings.expect_read_key().times(1).returning(|| Ok(None));
    settings
        .expect_create_key()
        .withf(|k| k.to_hex() == "05".repeat(32))
        .times(1)
        .returning(|_| Ok(()));
    let mut generator = MockSecretGeneratorService::new();
    generator
        .expect_generate_key()
        .times(1)
        .returning(|| Ok(AccessKeyValueObject::from_bytes([5; 32])));
    let use_case = PrepareSettingsUseCase::new(Arc::new(settings), Arc::new(generator));
    let (prepared, key) = use_case.execute().await.unwrap();
    assert_eq!(prepared, ServerSettingsValueObject::defaults());
    assert_eq!(key.to_hex(), "05".repeat(32));
}

#[tokio::test]
async fn keeps_existing_settings_and_key() {
    let existing = ServerSettingsValueObject::new(
        PortValueObject::new(9000).unwrap(),
        SchemeAllowlistPolicy::defaults(),
        false,
    );
    let returned = existing.clone();
    let mut settings = MockSettingsRepository::new();
    settings
        .expect_read_settings()
        .returning(move || Ok(Some(returned.clone())));
    settings.expect_create_settings().never();
    settings
        .expect_read_key()
        .returning(|| Ok(Some(AccessKeyValueObject::from_bytes([9; 32]))));
    settings.expect_create_key().never();
    let mut generator = MockSecretGeneratorService::new();
    generator.expect_generate_key().never();
    let use_case = PrepareSettingsUseCase::new(Arc::new(settings), Arc::new(generator));
    let (prepared, key) = use_case.execute().await.unwrap();
    assert_eq!(prepared, existing);
    assert_eq!(key.to_hex(), "09".repeat(32));
}

#[tokio::test]
async fn passes_repository_errors_on() {
    let mut settings = MockSettingsRepository::new();
    settings
        .expect_read_settings()
        .returning(|| Err(io_error()));
    let use_case = PrepareSettingsUseCase::new(
        Arc::new(settings),
        Arc::new(MockSecretGeneratorService::new()),
    );
    assert_eq!(use_case.execute().await.unwrap_err(), io_error());
}

#[tokio::test]
async fn passes_generator_errors_on() {
    let mut settings = MockSettingsRepository::new();
    settings
        .expect_read_settings()
        .returning(|| Ok(Some(ServerSettingsValueObject::defaults())));
    settings.expect_read_key().returning(|| Ok(None));
    let mut generator = MockSecretGeneratorService::new();
    generator
        .expect_generate_key()
        .returning(|| Err(io_error()));
    let use_case = PrepareSettingsUseCase::new(Arc::new(settings), Arc::new(generator));
    assert_eq!(use_case.execute().await.unwrap_err(), io_error());
}
