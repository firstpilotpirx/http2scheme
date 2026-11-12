use std::time::Duration;

use super::HttpServerHealthClient;
use crate::core::application::services::server_health_service::ServerHealthService;
use crate::core::domain::port_value_object::PortValueObject;

#[tokio::test]
async fn an_address_that_is_no_url_does_not_answer() {
    let client = HttpServerHealthClient::new("no host".to_owned(), Duration::from_millis(200));
    assert!(!client.is_answering(PortValueObject::default()).await);
}
