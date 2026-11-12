use super::OpenLinkRequest;
use crate::core::application::use_cases::open_link_command::OpenLinkCommand;

#[test]
fn becomes_a_command_with_both_parts() {
    let request = OpenLinkRequest {
        key: Some("k".to_owned()),
        url: Some("idea://x".to_owned()),
    };
    assert_eq!(request.link(), Some("idea://x"));
    assert_eq!(
        request.into_command(),
        OpenLinkCommand::new(Some("k".to_owned()), Some("idea://x".to_owned()))
    );
}

#[test]
fn an_empty_request_has_no_parts() {
    assert_eq!(
        OpenLinkRequest::default().into_command(),
        OpenLinkCommand::new(None, None)
    );
}
