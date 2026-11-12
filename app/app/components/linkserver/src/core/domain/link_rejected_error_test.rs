use super::LinkRejectedError;

#[test]
fn messages_say_why_the_link_was_refused() {
    assert_eq!(LinkRejectedError::Missing.to_string(), "the link is missing");
    assert_eq!(LinkRejectedError::Empty.to_string(), "the link is empty");
    assert_eq!(LinkRejectedError::TooLong.to_string(), "the link is longer than 4096 characters");
    assert_eq!(
        LinkRejectedError::ForbiddenCharacter.to_string(),
        "the link holds a space or a control character"
    );
    assert_eq!(LinkRejectedError::NoScheme.to_string(), "the link has no valid scheme");
    let refused = LinkRejectedError::SchemeNotAllowed { scheme: "slack".to_owned() };
    assert_eq!(refused.to_string(), "scheme not allowed: slack");
}

#[test]
fn only_scheme_not_allowed_is_not_malformed() {
    assert!(LinkRejectedError::TooLong.is_malformed());
    assert!(!LinkRejectedError::SchemeNotAllowed { scheme: "x".to_owned() }.is_malformed());
}
