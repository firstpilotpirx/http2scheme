use std::sync::Arc;

use super::OpenLinkPorts;
use crate::core::application::repositories::open_attempt_repository::MockOpenAttemptRepository;
use crate::core::application::services::browser_tab_closer_service::MockBrowserTabCloserService;
use crate::core::application::services::scheme_link_opener_service::MockSchemeLinkOpenerService;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

#[tokio::test]
async fn the_ports_reach_their_adapters() {
    let mut opener = MockSchemeLinkOpenerService::new();
    opener.expect_open().times(1).returning(|_| Ok(()));
    let ports = OpenLinkPorts {
        opener: Arc::new(opener),
        closer: Arc::new(MockBrowserTabCloserService::new()),
        attempts: Arc::new(MockOpenAttemptRepository::new()),
    };
    let link = SchemeLinkValueObject::parse("idea://x").unwrap();
    assert!(ports.opener.open(&link).await.is_ok());
}
