use super::OpenLinkError;

#[test]
fn messages_name_the_failed_step() {
    assert_eq!(
        OpenLinkError::OpenFailed {
            reason: "exit 1".to_owned()
        }
        .to_string(),
        "the link opener failed: exit 1"
    );
    assert_eq!(
        OpenLinkError::TabCloseFailed {
            reason: "denied".to_owned()
        }
        .to_string(),
        "closing the browser tab failed: denied"
    );
    assert_eq!(
        OpenLinkError::ReorderFailed {
            reason: "no browser in front".to_owned()
        }
        .to_string(),
        "bringing back the left app failed: no browser in front"
    );
    assert_eq!(
        OpenLinkError::RecordFailed {
            reason: "disk full".to_owned()
        }
        .to_string(),
        "writing the request log failed: disk full"
    );
}
