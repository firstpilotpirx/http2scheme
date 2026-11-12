//! The server's health address over HTTP.

use std::time::Duration;

use async_trait::async_trait;
use ureq::Agent;

use crate::core::application::services::server_health_service::ServerHealthService;
use crate::core::domain::port_value_object::PortValueObject;

/// Asks `GET http://<host>:<port>/health` and waits at most `timeout`.
#[derive(Debug, Clone)]
pub struct HttpServerHealthClient {
    host: String,
    timeout: Duration,
}

impl HttpServerHealthClient {
    /// A client for the server on `host`.
    #[must_use]
    pub const fn new(host: String, timeout: Duration) -> Self {
        Self { host, timeout }
    }

    fn ask(url: &str, timeout: Duration) -> bool {
        let agent: Agent = Agent::config_builder()
            .timeout_global(Some(timeout))
            .build()
            .into();
        agent
            .get(url)
            .call()
            .is_ok_and(|response| response.status().is_success())
    }
}

#[async_trait]
impl ServerHealthService for HttpServerHealthClient {
    async fn is_answering(&self, port: PortValueObject) -> bool {
        let url = format!("http://{}:{}/health", self.host, port.value());
        let timeout = self.timeout;
        tokio::task::spawn_blocking(move || Self::ask(&url, timeout))
            .await
            .unwrap_or(false)
    }
}

#[cfg(test)]
#[path = "http_server_health_client_test.rs"]
mod http_server_health_client_test;
