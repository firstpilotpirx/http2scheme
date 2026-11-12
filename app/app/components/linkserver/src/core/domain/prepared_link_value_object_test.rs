use super::PreparedLinkValueObject;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::port_value_object::PortValueObject;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

#[test]
fn keeps_port_key_and_link() {
    let prepared = PreparedLinkValueObject::new(
        PortValueObject::default(),
        AccessKeyValueObject::from_bytes([1; 32]),
        SchemeLinkValueObject::parse("idea://open").unwrap(),
    );
    assert_eq!(prepared.port().value(), 7777);
    assert_eq!(prepared.key().to_hex(), "01".repeat(32));
    assert_eq!(prepared.link().as_str(), "idea://open");
}
