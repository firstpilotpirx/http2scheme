use super::OpenOutcomeEnum;
use crate::core::domain::link_rejected_error::LinkRejectedError;

#[test]
fn labels_name_each_result() {
    assert_eq!(OpenOutcomeEnum::Opened.label(), "opened");
    assert_eq!(OpenOutcomeEnum::WrongKey.label(), "wrong key");
    assert_eq!(OpenOutcomeEnum::Rejected(LinkRejectedError::Empty).label(), "malformed link");
    let refused = LinkRejectedError::SchemeNotAllowed { scheme: "slack".to_owned() };
    assert_eq!(OpenOutcomeEnum::Rejected(refused).label(), "scheme not allowed");
    assert_eq!(OpenOutcomeEnum::OpenFailed.label(), "open failed");
}
