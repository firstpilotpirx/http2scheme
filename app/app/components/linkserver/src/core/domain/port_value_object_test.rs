use super::PortValueObject;

#[test]
fn new_accepts_the_range_ends() {
    assert_eq!(PortValueObject::new(1024).unwrap().value(), 1024);
    assert_eq!(PortValueObject::new(65535).unwrap().value(), 65535);
}

#[test]
fn new_rejects_a_privileged_port() {
    assert!(PortValueObject::new(1023).is_none());
    assert!(PortValueObject::new(80).is_none());
}

#[test]
fn default_is_7777() {
    assert_eq!(PortValueObject::default().value(), 7777);
}
