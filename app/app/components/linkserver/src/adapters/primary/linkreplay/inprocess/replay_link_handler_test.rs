use std::sync::Arc;

use super::ReplayLinkHandler;
use crate::core::application::use_cases::replay_link::MockReplayLink;
use crate::core::domain::link_rejected_error::LinkRejectedError;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

#[tokio::test]
async fn answers_the_result_word_of_the_use_case() {
    let mut use_case = MockReplayLink::new();
    use_case
        .expect_execute()
        .withf(|link| link == "slack://x")
        .returning(|_| {
            OpenOutcomeEnum::Rejected(LinkRejectedError::SchemeNotAllowed {
                scheme: "slack".to_owned(),
            })
        });
    let handler = ReplayLinkHandler::new(Arc::new(use_case));
    assert_eq!(handler.replay("slack://x").await, "scheme not allowed");
}
