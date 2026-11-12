use std::net::TcpListener;
use std::time::Duration;

use super::HttpServerHealthClient;
use crate::core::application::services::server_health_service::ServerHealthService;
use crate::core::domain::port_value_object::PortValueObject;

#[tokio::test]
async fn a_closed_port_does_not_answer() {
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let client = HttpServerHealthClient::new("127.0.0.1".to_owned(), Duration::from_millis(200));
    assert!(
        !client
            .is_answering(PortValueObject::new(port).unwrap())
            .await
    );
}
