use super::open_url;
use crate::core::domain::access_key_value_object::AccessKeyValueObject;
use crate::core::domain::port_value_object::PortValueObject;
use crate::core::domain::prepared_link_value_object::PreparedLinkValueObject;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

#[test]
fn encodes_every_reserved_character() {
    let prepared = PreparedLinkValueObject::new(
        PortValueObject::new(7777).unwrap(),
        AccessKeyValueObject::from_bytes([1; 32]),
        SchemeLinkValueObject::parse("idea://open?file=/a%20b/Файл.java&line=4~2").unwrap(),
    );
    let url = open_url("127.0.0.1", &prepared);
    let key = "01".repeat(32);
    assert_eq!(
        url,
        format!(
            "http://127.0.0.1:7777/open?key={key}&url=idea%3A%2F%2Fopen%3Ffile%3D%2Fa%2520b%2F%D0%A4%D0%B0%D0%B9%D0%BB.java%26line%3D4~2"
        )
    );
}
