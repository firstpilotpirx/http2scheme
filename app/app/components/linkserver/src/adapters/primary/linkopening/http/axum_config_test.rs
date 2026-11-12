use super::AxumConfig;
use crate::core::domain::port_value_object::PortValueObject;

#[test]
fn serves_each_host_with_the_port() {
    let config = AxumConfig::new(
        "127.0.0.1".to_owned(),
        vec!["127.0.0.1".to_owned(), "LocalHost".to_owned()],
        PortValueObject::new(7777).unwrap(),
    );
    assert_eq!(config.authorities(), ["127.0.0.1:7777", "localhost:7777"]);
}
