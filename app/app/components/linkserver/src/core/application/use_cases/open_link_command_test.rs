use super::OpenLinkCommand;

#[test]
fn keeps_key_and_link() {
    let command = OpenLinkCommand::new(Some("k".to_owned()), Some("idea://x".to_owned()));
    assert_eq!(command.key(), Some("k"));
    assert_eq!(command.link(), Some("idea://x"));
}

#[test]
fn debug_hides_the_key() {
    let command = OpenLinkCommand::new(Some("secret".to_owned()), None);
    let printed = format!("{command:?}");
    assert!(!printed.contains("secret"), "{printed}");
}
