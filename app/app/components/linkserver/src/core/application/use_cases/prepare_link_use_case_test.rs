use std::sync::Arc;

use super::PrepareLinkUseCase;
use crate::core::application::exceptions::prepare_link_error::PrepareLinkError;
use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::application::use_cases::prepare_link::PrepareLink;
use crate::core::application::use_cases::prepare_settings::MockPrepareSettings;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;

fn use_case() -> PrepareLinkUseCase {
    let mut settings = MockPrepareSettings::new();
    settings.expect_execute().returning(|| {
        Ok((
            ServerSettingsValueObject::defaults(),
            AccessKeyValueObject::from_bytes([3; 32]),
        ))
    });
    PrepareLinkUseCase::new(Arc::new(settings))
}

#[tokio::test]
async fn returns_port_key_and_link_for_an_allowed_link() {
    let prepared = use_case().execute("idea://open?file=/a%20b").await;
    let prepared = prepared.unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(prepared.port().value(), 7777);
    assert_eq!(prepared.key().to_hex(), "03".repeat(32));
}

#[tokio::test]
async fn refuses_a_malformed_link() {
    let error = use_case().execute("no-scheme").await.unwrap_err();
    assert_eq!(
        error,
        PrepareLinkError::Rejected(LinkRejectedError::NoScheme)
    );
}

#[tokio::test]
async fn refuses_a_scheme_not_allowed() {
    let error = use_case().execute("slack://x").await.unwrap_err();
    assert_eq!(
        error,
        PrepareLinkError::Rejected(LinkRejectedError::SchemeNotAllowed {
            scheme: "slack".to_owned()
        })
    );
}

#[tokio::test]
async fn passes_settings_errors_on() {
    let mut settings = MockPrepareSettings::new();
    settings.expect_execute().returning(|| {
        Err(SettingsError::Io {
            path: "/k".to_owned(),
            reason: "x".to_owned(),
        })
    });
    let error = PrepareLinkUseCase::new(Arc::new(settings))
        .execute("idea://x")
        .await
        .unwrap_err();
    assert!(matches!(error, PrepareLinkError::Settings(_)));
}
