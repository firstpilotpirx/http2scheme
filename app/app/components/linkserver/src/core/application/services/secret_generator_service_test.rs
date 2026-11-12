use std::sync::Arc;

use crate::core::domain::access_key_value_object::AccessKeyValueObject;

use super::{MockSecretGeneratorService, SecretGeneratorService};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockSecretGeneratorService::new();
    mock.expect_generate_key()
        .returning(|| Ok(AccessKeyValueObject::from_bytes([1; 32])));
    let port: Arc<dyn SecretGeneratorService> = Arc::new(mock);
    assert_eq!(port.generate_key().await.unwrap().to_hex(), "01".repeat(32));
}
