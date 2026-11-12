use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;

use super::BrowseHistoryUseCase;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::link_history_repository::MockLinkHistoryRepository;
use crate::core::application::services::link_replay_service::MockLinkReplayService;
use crate::core::application::use_cases::browse_history::BrowseHistory;
use crate::core::domain::history_command_enum::HistoryCommandEnum;
use crate::core::domain::link_record_value_object::LinkRecordValueObject;

fn record(link: &str) -> LinkRecordValueObject {
    LinkRecordValueObject::new(UNIX_EPOCH, "opened".to_owned(), link.to_owned())
}

#[tokio::test]
async fn show_lists_the_latest_hundred_requests() {
    let mut history = MockLinkHistoryRepository::new();
    history
        .expect_latest()
        .withf(|limit| *limit == 100)
        .returning(|_| Ok(vec![record("idea://b"), record("idea://a")]));
    let mut replay = MockLinkReplayService::new();
    replay.expect_replay().never();
    let view = BrowseHistoryUseCase::new(Arc::new(history), Arc::new(replay))
        .execute(HistoryCommandEnum::Show)
        .await
        .unwrap();
    assert_eq!(view.rows(), [record("idea://b"), record("idea://a")]);
}

#[tokio::test]
async fn replay_opens_the_link_then_reads_the_log_again() {
    let order = Arc::new(Mutex::new(Vec::new()));
    let (replayed, read) = (Arc::clone(&order), Arc::clone(&order));
    let mut replay = MockLinkReplayService::new();
    replay
        .expect_replay()
        .withf(|link| link == "idea://a")
        .times(1)
        .returning(move |_| {
            replayed.lock().unwrap().push("replay");
            "opened".to_owned()
        });
    let mut history = MockLinkHistoryRepository::new();
    history.expect_latest().returning(move |_| {
        read.lock().unwrap().push("read");
        Ok(vec![record("idea://a")])
    });
    let view = BrowseHistoryUseCase::new(Arc::new(history), Arc::new(replay))
        .execute(HistoryCommandEnum::Replay("idea://a".to_owned()))
        .await
        .unwrap();
    assert_eq!(view.rows().len(), 1);
    assert_eq!(*order.lock().unwrap(), ["replay", "read"]);
}

#[tokio::test]
async fn an_unreadable_log_is_an_error() {
    let mut history = MockLinkHistoryRepository::new();
    history.expect_latest().returning(|_| {
        Err(MenuActionError::History {
            path: "/l/requests.log".to_owned(),
            reason: "denied".to_owned(),
        })
    });
    let result =
        BrowseHistoryUseCase::new(Arc::new(history), Arc::new(MockLinkReplayService::new()))
            .execute(HistoryCommandEnum::Show)
            .await;
    assert!(matches!(result, Err(MenuActionError::History { .. })));
}
