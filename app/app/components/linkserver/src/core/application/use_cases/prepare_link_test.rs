use std::sync::Arc;

use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

use super::{MockPrepareLink, PrepareLink};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockPrepareLink::new();
    mock.expect_execute()
        .returning(|link| Err(SchemeLinkValueObject::parse(link).unwrap_err().into()));
    let port: Arc<dyn PrepareLink> = Arc::new(mock);
    assert!(port.execute("no scheme").await.is_err());
}
