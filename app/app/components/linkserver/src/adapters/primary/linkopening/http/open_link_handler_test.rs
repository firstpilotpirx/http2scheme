use axum::http::StatusCode;

use super::OpenLinkHandler;

#[tokio::test]
async fn the_static_answers_have_their_status() {
    assert_eq!(OpenLinkHandler::health().await.status(), StatusCode::OK);
    assert_eq!(
        OpenLinkHandler::not_found().await.status(),
        StatusCode::NOT_FOUND
    );
    let refused = OpenLinkHandler::method_not_allowed().await;
    assert_eq!(refused.status(), StatusCode::METHOD_NOT_ALLOWED);
}
