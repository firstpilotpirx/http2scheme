use axum::http::StatusCode;

use super::{OpenLinkResponse, escape};
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

fn pages() -> OpenLinkResponse {
    OpenLinkResponse::new("/s.toml".to_owned(), "/r.log".to_owned())
}

#[test]
fn escapes_html_characters() {
    assert_eq!(
        escape("<a href=\"x\">&'"),
        "&lt;a href=&quot;x&quot;&gt;&amp;&#39;"
    );
}

#[test]
fn each_outcome_has_its_status() {
    let cases = [
        (OpenOutcomeEnum::Opened, StatusCode::OK),
        (OpenOutcomeEnum::WrongKey, StatusCode::FORBIDDEN),
        (
            OpenOutcomeEnum::Rejected(LinkRejectedError::Empty),
            StatusCode::BAD_REQUEST,
        ),
        (
            OpenOutcomeEnum::Rejected(LinkRejectedError::SchemeNotAllowed {
                scheme: "x".to_owned(),
            }),
            StatusCode::BAD_REQUEST,
        ),
        (OpenOutcomeEnum::OpenFailed, StatusCode::BAD_GATEWAY),
    ];
    for (outcome, status) in cases {
        assert_eq!(
            pages().for_outcome(&outcome, Some("idea://x")).status(),
            status,
            "{outcome:?}"
        );
    }
}

#[test]
fn other_answers_have_their_status() {
    assert_eq!(
        OpenLinkResponse::not_found().status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        OpenLinkResponse::method_not_allowed().status(),
        StatusCode::METHOD_NOT_ALLOWED
    );
    assert_eq!(
        OpenLinkResponse::misdirected().status(),
        StatusCode::MISDIRECTED_REQUEST
    );
    assert_eq!(OpenLinkResponse::health().status(), StatusCode::OK);
}
