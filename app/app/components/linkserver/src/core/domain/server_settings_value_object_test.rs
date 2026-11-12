use super::ServerSettingsValueObject;

#[test]
fn defaults_hold_port_7777_two_schemes_and_tab_closing_on() {
    let settings = ServerSettingsValueObject::defaults();
    assert_eq!(settings.port().value(), 7777);
    assert_eq!(settings.allowlist().schemes().len(), 2);
    assert!(settings.close_tab());
}
