use super::SchemeNameValueObject;
use crate::core::domain::link_rejected_error::LinkRejectedError;

#[test]
fn parse_accepts_rfc3986_scheme_and_lowers_it() {
    let name = SchemeNameValueObject::parse("Git+SSH.v2-x").unwrap();
    assert_eq!(name.as_str(), "git+ssh.v2-x");
}

#[test]
fn parse_rejects_empty_name() {
    assert_eq!(SchemeNameValueObject::parse(""), Err(LinkRejectedError::NoScheme));
}

#[test]
fn parse_rejects_name_starting_with_digit() {
    assert_eq!(SchemeNameValueObject::parse("1idea"), Err(LinkRejectedError::NoScheme));
}

#[test]
fn parse_rejects_name_with_forbidden_character() {
    assert_eq!(SchemeNameValueObject::parse("id_ea"), Err(LinkRejectedError::NoScheme));
}

#[test]
fn display_prints_the_lower_case_name() {
    let name = SchemeNameValueObject::parse("IDEA").unwrap();
    assert_eq!(name.to_string(), "idea");
}
