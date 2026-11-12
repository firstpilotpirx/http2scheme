use std::path::Path;

use objc2_service_management::SMAppServiceStatus;

use super::SmAppServiceLoginItemGateway;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;

#[test]
fn a_bundle_program_sits_in_contents_macos_of_an_app() {
    let yes = Path::new("/Applications/http2scheme.app/Contents/MacOS/http2scheme");
    assert!(SmAppServiceLoginItemGateway::is_bundle_program(yes));
    for no in [
        "/usr/local/bin/http2scheme",
        "/x/http2scheme/Contents/MacOS/http2scheme",
        "/x/http2scheme.app/MacOS/http2scheme",
        "http2scheme",
    ] {
        assert!(
            !SmAppServiceLoginItemGateway::is_bundle_program(Path::new(no)),
            "{no}"
        );
    }
}

#[test]
fn each_system_status_has_its_state() {
    let state = SmAppServiceLoginItemGateway::state_of;
    assert_eq!(state(SMAppServiceStatus::Enabled), LoginItemStateEnum::On);
    assert_eq!(
        state(SMAppServiceStatus::NotRegistered),
        LoginItemStateEnum::Off
    );
    assert_eq!(state(SMAppServiceStatus::NotFound), LoginItemStateEnum::Off);
    assert_eq!(
        state(SMAppServiceStatus::RequiresApproval),
        LoginItemStateEnum::NeedsApproval
    );
    assert_eq!(
        state(SMAppServiceStatus(99)),
        LoginItemStateEnum::Unavailable
    );
}
