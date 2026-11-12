//! Lists the latest requests and opens a listed link again.

use std::sync::Arc;

use async_trait::async_trait;

use super::browse_history::BrowseHistory;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::repositories::link_history_repository::LinkHistoryRepository;
use crate::core::application::services::link_replay_service::LinkReplayService;
use crate::core::domain::history_command_enum::HistoryCommandEnum;
use crate::core::domain::history_view_value_object::HistoryViewValueObject;

/// How many requests the Link History window lists.
pub const HISTORY_LIMIT: usize = 100;

/// The window's use case over the request log and the replay.
pub struct BrowseHistoryUseCase {
    history: Arc<dyn LinkHistoryRepository>,
    replay: Arc<dyn LinkReplayService>,
}

impl BrowseHistoryUseCase {
    /// A use case over the request log and the replay.
    #[must_use]
    pub fn new(
        history: Arc<dyn LinkHistoryRepository>,
        replay: Arc<dyn LinkReplayService>,
    ) -> Self {
        Self { history, replay }
    }
}

#[async_trait]
impl BrowseHistory for BrowseHistoryUseCase {
    async fn execute(
        &self,
        command: HistoryCommandEnum,
    ) -> Result<HistoryViewValueObject, MenuActionError> {
        if let HistoryCommandEnum::Replay(link) = command {
            // WHY: the server logs the attempt with its result; the reread rows show it.
            self.replay.replay(&link).await;
        }
        let rows = self.history.find_latest(HISTORY_LIMIT).await?;
        Ok(HistoryViewValueObject::new(rows))
    }
}

#[cfg(test)]
#[path = "browse_history_use_case_test.rs"]
mod browse_history_use_case_test;
