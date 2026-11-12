//! The pages the browser shows.

use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse as _, Response};

use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

const STYLE: &str = "body{margin:0;padding:48px;font-family:-apple-system,BlinkMacSystemFont,'Helvetica Neue',sans-serif;background:#f4f4f5}\
.card{max-width:640px;background:#fff;border:1px solid #e4e4e7;border-radius:12px;padding:28px}\
h1{margin:0 0 8px;font-size:22px;font-weight:600}.ok{color:#15803d}.fail{color:#b91c1c}\
.link{margin:0 0 8px;font-size:14px;color:#3f3f46;font-family:ui-monospace,Menlo,monospace;word-break:break-all}\
p{margin:0;font-size:13px;color:#52525b}\
@media (prefers-color-scheme:dark){body{background:#09090b}.card{background:#18181b;border-color:#3f3f46}\
.ok{color:#4ade80}.fail{color:#f87171}.link{color:#d4d4d8}p{color:#a1a1aa}}";

/// Builds the page for every answer; names the settings and log files in its hints.
#[derive(Debug, Clone)]
pub struct OpenLinkResponse {
    settings_file: String,
    log_file: String,
}

impl OpenLinkResponse {
    /// Pages that name `settings_file` and `log_file` in their hints.
    #[must_use]
    pub const fn new(settings_file: String, log_file: String) -> Self {
        Self {
            settings_file,
            log_file,
        }
    }

    /// The page for the outcome of an open request that carried `link`.
    #[must_use]
    pub fn for_outcome(&self, outcome: &OpenOutcomeEnum, link: Option<&str>) -> Response {
        match outcome {
            OpenOutcomeEnum::Opened => opened(link.unwrap_or_default()),
            OpenOutcomeEnum::WrongKey => failed(
                StatusCode::FORBIDDEN,
                "Wrong key",
                "Nothing was opened. Get a fresh link with: http2scheme link <scheme link>",
            ),
            OpenOutcomeEnum::Rejected(LinkRejectedError::SchemeNotAllowed { scheme }) => failed(
                StatusCode::BAD_REQUEST,
                &format!("Scheme not allowed: {scheme}"),
                &format!(
                    "Nothing was opened. To allow it, add \"{scheme}\" to allowed_schemes in {} and restart the server.",
                    self.settings_file
                ),
            ),
            OpenOutcomeEnum::Rejected(_) => failed(
                StatusCode::BAD_REQUEST,
                "Malformed link",
                "Nothing was opened. The link has a space or a line break, has no scheme, or is longer than 4096 characters.",
            ),
            OpenOutcomeEnum::OpenFailed => failed(
                StatusCode::BAD_GATEWAY,
                "Could not open the app",
                &format!(
                    "macOS found no app for this link, or the app failed to start. Details are in {}",
                    self.log_file
                ),
            ),
        }
    }

    /// The page for an address the server does not serve.
    #[must_use]
    pub fn not_found() -> Response {
        failed(
            StatusCode::NOT_FOUND,
            "Not found",
            "This server serves only its open links.",
        )
    }

    /// The page for a method other than GET.
    #[must_use]
    pub fn method_not_allowed() -> Response {
        failed(
            StatusCode::METHOD_NOT_ALLOWED,
            "Method not allowed",
            "Open links use GET.",
        )
    }

    /// The page for a request under a host name the server does not serve.
    #[must_use]
    pub fn misdirected() -> Response {
        failed(
            StatusCode::MISDIRECTED_REQUEST,
            "Host not served",
            "Nothing was opened. This server answers only on its loopback address.",
        )
    }

    /// The plain-text health answer.
    #[must_use]
    pub fn health() -> Response {
        (StatusCode::OK, "ok").into_response()
    }
}

fn opened(link: &str) -> Response {
    let body = format!(
        "<h1 class=\"ok\">✓ Opened</h1><p class=\"link\">{}</p><p>This tab closes by itself. If it stays, you can close it.</p>\
<script>setTimeout(() => window.close(), 1000)</script>",
        escape(link)
    );
    page(StatusCode::OK, "Opened", &body)
}

fn failed(status: StatusCode, title: &str, hint: &str) -> Response {
    let body = format!(
        "<h1 class=\"fail\">✗ {}</h1><p>{}</p>",
        escape(title),
        escape(hint)
    );
    page(status, title, &body)
}

fn page(status: StatusCode, title: &str, body: &str) -> Response {
    let html = format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\">\
<meta name=\"color-scheme\" content=\"light dark\"><title>{} · http2scheme</title><style>{STYLE}</style></head>\
<body><div class=\"card\">{body}</div></body></html>",
        escape(title)
    );
    (status, [(CONTENT_TYPE, "text/html; charset=utf-8")], html).into_response()
}

/// The text with the HTML special characters escaped.
#[must_use]
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
#[path = "open_link_response_test.rs"]
mod open_link_response_test;
