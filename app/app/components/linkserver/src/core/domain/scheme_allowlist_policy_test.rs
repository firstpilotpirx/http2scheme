use super::SchemeAllowlistPolicy;
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;
use crate::core::domain::scheme_name_value_object::SchemeNameValueObject;

fn policy(names: &[&str]) -> SchemeAllowlistPolicy {
    SchemeAllowlistPolicy::new(
        names
            .iter()
            .map(|n| SchemeNameValueObject::parse(n).unwrap())
            .collect(),
    )
}

fn link(text: &str) -> SchemeLinkValueObject {
    SchemeLinkValueObject::parse(text).unwrap()
}

#[test]
fn check_permits_a_listed_scheme_in_any_case() {
    assert_eq!(policy(&["idea"]).check(&link("IDEA://open")), Ok(()));
}

#[test]
fn check_refuses_an_unlisted_scheme_naming_it() {
    assert_eq!(
        policy(&["idea"]).check(&link("slack://channel")),
        Err(LinkRejectedError::SchemeNotAllowed {
            scheme: "slack".to_owned()
        })
    );
}

#[test]
fn check_never_permits_web_or_file_links() {
    let all = policy(&["http", "https", "file"]);
    for text in ["http://x", "https://x", "file:///etc/passwd"] {
        assert!(all.check(&link(text)).is_err(), "{text}");
    }
}

#[test]
fn defaults_list_idea_and_goland() {
    let names: Vec<String> = SchemeAllowlistPolicy::defaults()
        .schemes()
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(names, ["idea", "goland"]);
}
