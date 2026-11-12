use std::path::PathBuf;

use super::LsAppInfoAppOrderGateway;
use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::application::services::app_order_service::AppOrderService;

fn gateway() -> LsAppInfoAppOrderGateway {
    LsAppInfoAppOrderGateway::new(
        PathBuf::from("/nonexistent/lsappinfo"),
        PathBuf::from("/nonexistent/open"),
    )
}

#[tokio::test]
async fn a_missing_program_is_a_reorder_failure() {
    let error = gateway().bring_back_now().await.unwrap_err();
    assert!(
        matches!(error, OpenLinkError::ReorderFailed { .. }),
        "{error}"
    );
}

#[tokio::test]
async fn the_port_answers_ok_when_the_reorder_fails() {
    assert!(gateway().bring_back_left_app().await.is_ok());
}
