use std::sync::Arc;

use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

use super::{MockSchemeLinkOpenerService, SchemeLinkOpenerService};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockSchemeLinkOpenerService::new();
    mock.expect_open()
        .withf(|link| link.as_str() == "idea://x")
        .times(1)
        .returning(|_| Ok(()));
    let port: Arc<dyn SchemeLinkOpenerService> = Arc::new(mock);
    assert!(
        port.open(&SchemeLinkValueObject::parse("idea://x").unwrap())
            .await
            .is_ok()
    );
}
