use super::{bundle_id, left_app};

#[test]
fn a_browser_in_front_answers_the_app_behind_it() {
    let list = "ASN:0x0-0xb40b4-\"Safari\": ASN:0x0-0x9ba9ba-\"Claude\": ASN:0x0-0x52052-\"IntelliJ_IDEA\": ";
    assert_eq!(left_app(list), Some("ASN:0x0-0x9ba9ba".to_owned()));
}

#[test]
fn a_browser_name_with_a_space_is_read() {
    let list = "ASN:0x0-0x1-\"Google_Chrome\": ASN:0x0-0x2-\"Ghostty\":\n";
    assert_eq!(left_app(list), Some("ASN:0x0-0x2".to_owned()));
}

#[test]
fn no_browser_in_front_answers_none() {
    let list = "ASN:0x0-0x9ba9ba-\"Claude\": ASN:0x0-0xb40b4-\"Safari\": ";
    assert_eq!(left_app(list), None);
}

#[test]
fn a_browser_alone_answers_none() {
    assert_eq!(left_app("ASN:0x0-0xb40b4-\"Safari\": "), None);
}

#[test]
fn an_empty_list_answers_none() {
    assert_eq!(left_app(""), None);
    assert_eq!(left_app("garbage"), None);
}

#[test]
fn the_bundle_id_is_read() {
    let info = "[ NULL ]  ASN:0x0-0x9ba9ba: (in front) \n    bundleID=\"com.anthropic.claudefordesktop\"\n    bundle path=[ NULL ] \n";
    assert_eq!(
        bundle_id(info),
        Some("com.anthropic.claudefordesktop".to_owned())
    );
}

#[test]
fn no_bundle_id_answers_none() {
    assert_eq!(bundle_id("    bundleID=[ NULL ] \n"), None);
    assert_eq!(bundle_id("    bundleID=\"\"\n"), None);
}
