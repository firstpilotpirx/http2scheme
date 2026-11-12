//! `SmAppServiceLoginItemGateway` outside an app bundle: the system's login items stay untouched.
#![allow(
    clippy::unwrap_used,
    reason = "test helpers fail loudly, like the tests"
)]

use std::path::Path;

use menubar::adapters::secondary::loginitem::smappservice::sm_app_service_login_item_gateway::SmAppServiceLoginItemGateway;
use menubar::core::application::services::login_item_service::LoginItemService;
use menubar::core::domain::login_item_state_enum::LoginItemStateEnum;

fn outside() -> SmAppServiceLoginItemGateway {
    SmAppServiceLoginItemGateway::new(Path::new("/usr/local/bin/http2scheme"))
}

#[tokio::test]
async fn a_program_outside_a_bundle_has_no_login_item() {
    assert_eq!(outside().status().await, LoginItemStateEnum::Unavailable);
}

#[tokio::test]
async fn every_action_outside_a_bundle_is_refused() {
    let gateway = outside();
    for result in [
        gateway.register().await,
        gateway.unregister().await,
        gateway.open_system_settings().await,
    ] {
        let error = result.unwrap_err();
        assert!(error.to_string().contains("installed app"), "{error}");
    }
}

#[test]
fn only_a_program_inside_an_app_bundle_counts() {
    assert!(SmAppServiceLoginItemGateway::is_bundle_program(Path::new(
        "/Applications/http2scheme.app/Contents/MacOS/http2scheme"
    )));
    assert!(!SmAppServiceLoginItemGateway::is_bundle_program(Path::new(
        "/usr/local/bin/http2scheme"
    )));
}
