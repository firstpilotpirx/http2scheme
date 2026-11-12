use std::path::PathBuf;

use super::LaunchServicesSchemeLinkOpenerGateway;
use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::application::services::scheme_link_opener_service::SchemeLinkOpenerService;
use crate::core::domain::scheme_link_value_object::SchemeLinkValueObject;

#[tokio::test]
async fn a_missing_program_is_an_open_failure() {
    let gateway = LaunchServicesSchemeLinkOpenerGateway::new(PathBuf::from("/nonexistent/open"));
    let link = SchemeLinkValueObject::parse("idea://open?file=/a").unwrap();
    let error = gateway.open(&link).await.unwrap_err();
    assert!(matches!(error, OpenLinkError::OpenFailed { .. }), "{error}");
}
