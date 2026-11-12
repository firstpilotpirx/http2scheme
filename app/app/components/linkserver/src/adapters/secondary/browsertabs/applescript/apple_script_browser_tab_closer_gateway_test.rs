use std::path::PathBuf;
use std::time::Duration;

use super::AppleScriptBrowserTabCloserGateway;
use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::application::services::browser_tab_closer_service::BrowserTabCloserService;
use crate::core::domain::port_value_object::PortValueObject;

fn gateway() -> AppleScriptBrowserTabCloserGateway {
    AppleScriptBrowserTabCloserGateway::new(
        PathBuf::from("/nonexistent/osascript"),
        vec!["127.0.0.1".to_owned()],
        Duration::from_millis(1),
    )
}

#[tokio::test]
async fn a_missing_program_is_a_tab_close_failure() {
    let error = gateway()
        .close_now(PortValueObject::new(7777).unwrap())
        .await
        .unwrap_err();
    assert!(
        matches!(error, OpenLinkError::TabCloseFailed { .. }),
        "{error}"
    );
}

#[tokio::test]
async fn the_background_close_returns_at_once() {
    assert!(
        gateway()
            .close_open_tabs(PortValueObject::new(7777).unwrap())
            .await
            .is_ok()
    );
}
