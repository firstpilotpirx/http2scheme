use super::PrepareLinkError;
use crate::core::application::exceptions::settings_error::SettingsError;
use crate::core::domain::link_rejected_error::LinkRejectedError;

#[test]
fn wraps_settings_and_link_errors() {
    let settings = PrepareLinkError::from(SettingsError::Io {
        path: "/k".to_owned(),
        reason: "gone".to_owned(),
    });
    assert_eq!(settings.to_string(), "/k: gone");
    let refused = PrepareLinkError::from(LinkRejectedError::SchemeNotAllowed {
        scheme: "slack".to_owned(),
    });
    assert_eq!(refused.to_string(), "scheme not allowed: slack");
}
