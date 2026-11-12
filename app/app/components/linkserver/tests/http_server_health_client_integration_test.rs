//! `HttpServerHealthClient` against `wiremock` and a closed port.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::net::TcpListener;
use std::time::Duration;

use linkserver::adapters::secondary::serverhealth::http::http_server_health_client::HttpServerHealthClient;
use linkserver::core::application::services::server_health_service::ServerHealthService;
use linkserver::core::domain::port_value_object::PortValueObject;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn port_of(server: &MockServer) -> PortValueObject {
    PortValueObject::new(server.address().port()).unwrap()
}

fn client(timeout: Duration) -> HttpServerHealthClient {
    HttpServerHealthClient::new("127.0.0.1".to_owned(), timeout)
}

#[tokio::test]
async fn a_200_health_answer_is_answering() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;
    assert!(
        client(Duration::from_secs(1))
            .is_answering(port_of(&server))
            .await
    );
}

#[tokio::test]
async fn an_error_status_is_not_answering() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;
    assert!(
        !client(Duration::from_secs(1))
            .is_answering(port_of(&server))
            .await
    );
}

#[tokio::test]
async fn a_slow_answer_times_out() {
    let server = MockServer::start().await;
    let slow = ResponseTemplate::new(200).set_delay(Duration::from_secs(2));
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(slow)
        .mount(&server)
        .await;
    assert!(
        !client(Duration::from_millis(200))
            .is_answering(port_of(&server))
            .await
    );
}

#[tokio::test]
async fn a_closed_port_is_not_answering() {
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    assert!(
        !client(Duration::from_secs(1))
            .is_answering(PortValueObject::new(port).unwrap())
            .await
    );
}
