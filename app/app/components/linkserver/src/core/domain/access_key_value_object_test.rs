use super::AccessKeyValueObject;

fn key() -> AccessKeyValueObject {
    AccessKeyValueObject::from_bytes([0xab; 32])
}

#[test]
fn from_bytes_gives_64_lower_case_hex_characters() {
    assert_eq!(key().to_hex(), "ab".repeat(32));
}

#[test]
fn parse_hex_reads_what_to_hex_wrote() {
    let parsed = AccessKeyValueObject::parse_hex(&"ab".repeat(32)).unwrap();
    assert!(parsed.matches(&key().to_hex()));
}

#[test]
fn parse_hex_ignores_a_trailing_line_break() {
    assert!(AccessKeyValueObject::parse_hex(&format!("{}\n", "0f".repeat(32))).is_some());
}

#[test]
fn parse_hex_rejects_wrong_length_and_non_hex() {
    assert!(AccessKeyValueObject::parse_hex(&"ab".repeat(31)).is_none());
    assert!(AccessKeyValueObject::parse_hex(&"zz".repeat(32)).is_none());
    assert!(AccessKeyValueObject::parse_hex(&"AB".repeat(32)).is_none());
}

#[test]
fn matches_only_the_same_key() {
    let other = format!("{}ac", "ab".repeat(31));
    assert!(key().matches(&"ab".repeat(32)));
    assert!(!key().matches(&other));
}

#[test]
fn matches_rejects_empty_and_other_length_candidates() {
    assert!(!key().matches(""));
    assert!(!key().matches(&"ab".repeat(31)));
    assert!(!key().matches(&"ab".repeat(33)));
}

#[test]
fn debug_hides_the_key() {
    assert_eq!(format!("{:?}", key()), "AccessKey(****)");
}
