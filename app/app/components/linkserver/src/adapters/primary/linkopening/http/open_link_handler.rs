//! The HTTP routes of the server.

use std::future::{Ready, ready};
use std::sync::Arc;

use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::response::Response;

use super::open_link_request::OpenLinkRequest;
use super::open_link_response::OpenLinkResponse;
use crate::core::application::use_cases::open_link::OpenLink;

/// Turns open requests into the open-link use case and its outcome into a page.
pub struct OpenLinkHandler {
    open_link: Arc<dyn OpenLink>,
    pages: OpenLinkResponse,
}

impl OpenLinkHandler {
    /// A handler over the use case and the pages.
    #[must_use]
    pub fn new(open_link: Arc<dyn OpenLink>, pages: OpenLinkResponse) -> Self {
        Self { open_link, pages }
    }

    /// `GET /open?key=…&url=…`; an unreadable query counts as a request with no key and no link.
    pub async fn open(
        State(handler): State<Arc<Self>>,
        query: Result<Query<OpenLinkRequest>, QueryRejection>,
    ) -> Response {
        let request = query.map(|Query(request)| request).unwrap_or_default();
        let link = request.link().map(str::to_owned);
        let outcome = handler.open_link.execute(request.into_command()).await;
        handler.pages.for_outcome(&outcome, link.as_deref())
    }

    /// `GET /health`.
    pub fn health() -> Ready<Response> {
        ready(OpenLinkResponse::health())
    }

    /// Any address the server does not serve.
    pub fn not_found() -> Ready<Response> {
        ready(OpenLinkResponse::not_found())
    }

    /// Any method other than GET on a served address.
    pub fn method_not_allowed() -> Ready<Response> {
        ready(OpenLinkResponse::method_not_allowed())
    }
}
