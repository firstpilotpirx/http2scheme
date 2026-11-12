use super::SchemeLinkValueObject;
use crate::core::domain::link_rejected_error::LinkRejectedError;

#[test]
fn parse_keeps_the_exact_text_and_reads_the_scheme() {
    let text = "IDEA://open?file=/a%20b/\"q\";x.java&line=17";
    let link = SchemeLinkValueObject::parse(text).unwrap();
    assert_eq!(link.as_str(), text);
    assert_eq!(link.scheme().as_str(), "idea");
}

#[test]
fn parse_rejects_empty_text() {
    assert_eq!(SchemeLinkValueObject::parse(""), Err(LinkRejectedError::Empty));
}

#[test]
fn parse_accepts_4096_characters() {
    let text = format!("idea:{}", "a".repeat(4091));
    assert!(SchemeLinkValueObject::parse(&text).is_ok());
}

#[test]
fn parse_rejects_more_than_4096_characters() {
    let text = format!("idea:{}", "a".repeat(4092));
    assert_eq!(SchemeLinkValueObject::parse(&text), Err(LinkRejectedError::TooLong));
}

#[test]
fn parse_rejects_a_line_break() {
    assert_eq!(
        SchemeLinkValueObject::parse("idea://open\n?file=x"),
        Err(LinkRejectedError::ForbiddenCharacter)
    );
}

#[test]
fn parse_rejects_a_raw_space() {
    assert_eq!(
        SchemeLinkValueObject::parse("idea://open?file=a b"),
        Err(LinkRejectedError::ForbiddenCharacter)
    );
}

#[test]
fn parse_rejects_a_delete_character() {
    assert_eq!(
        SchemeLinkValueObject::parse("idea://\u{7f}"),
        Err(LinkRejectedError::ForbiddenCharacter)
    );
}

#[test]
fn parse_rejects_text_without_a_colon() {
    assert_eq!(SchemeLinkValueObject::parse("idea"), Err(LinkRejectedError::NoScheme));
}

#[test]
fn parse_rejects_an_invalid_scheme() {
    assert_eq!(SchemeLinkValueObject::parse("/x:y"), Err(LinkRejectedError::NoScheme));
}
