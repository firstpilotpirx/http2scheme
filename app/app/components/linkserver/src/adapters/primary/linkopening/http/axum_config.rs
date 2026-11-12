//! The axum router, its guards and the listening socket.

use std::future::Future;
use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::HeaderValue;
use axum::http::header::{CACHE_CONTROL, HOST, REFERRER_POLICY};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::get;
use tokio::net::TcpListener;

use super::open_link_handler::OpenLinkHandler;
use super::open_link_response::OpenLinkResponse;
use crate::core::domain::port_value_object::PortValueObject;

/// Where the server listens and which host names it serves.
#[derive(Debug, Clone)]
pub struct AxumConfig {
    bind_host: String,
    hosts: Vec<String>,
    port: PortValueObject,
}

impl AxumConfig {
    /// A server bound to `bind_host:port` that serves `hosts` with that port.
    #[must_use]
    pub const fn new(bind_host: String, hosts: Vec<String>, port: PortValueObject) -> Self {
        Self {
            bind_host,
            hosts,
            port,
        }
    }

    /// The `Host` header values the server serves, lower case.
    #[must_use]
    pub fn authorities(&self) -> Vec<String> {
        self.hosts
            .iter()
            .map(|host| format!("{}:{}", host.to_ascii_lowercase(), self.port.value()))
            .collect()
    }

    /// The routes, the host guard and the privacy headers.
    pub fn router(&self, handler: OpenLinkHandler) -> Router {
        Router::new()
            .route(
                "/open",
                get(OpenLinkHandler::open).fallback(OpenLinkHandler::method_not_allowed),
            )
            .route(
                "/health",
                get(OpenLinkHandler::health).fallback(OpenLinkHandler::method_not_allowed),
            )
            .fallback(OpenLinkHandler::not_found)
            .with_state(Arc::new(handler))
            .layer(middleware::from_fn_with_state(
                Arc::new(self.authorities()),
                check_host,
            ))
            .layer(middleware::map_response(add_privacy_headers))
    }

    /// Binds the listening socket.
    ///
    /// # Errors
    /// The address is taken or cannot be bound.
    pub async fn listen(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind((self.bind_host.as_str(), self.port.value())).await
    }

    /// Serves `router` on `listener` until `shutdown` completes.
    ///
    /// # Errors
    /// The server fails to accept connections.
    pub async fn serve(
        listener: TcpListener,
        router: Router,
        shutdown: impl Future<Output = ()> + Send + 'static,
    ) -> std::io::Result<()> {
        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown)
            .await
    }
}

async fn check_host(
    State(authorities): State<Arc<Vec<String>>>,
    request: Request,
    next: Next,
) -> Response {
    let served = request
        .headers()
        .get(HOST)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|host| {
            authorities
                .iter()
                .any(|authority| authority.eq_ignore_ascii_case(host))
        });
    if served {
        next.run(request).await
    } else {
        OpenLinkResponse::misdirected()
    }
}

async fn add_privacy_headers(mut response: Response) -> Response {
    let headers = response.headers_mut();
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    headers.insert(REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
    response
}

#[cfg(test)]
#[path = "axum_config_test.rs"]
mod axum_config_test;
