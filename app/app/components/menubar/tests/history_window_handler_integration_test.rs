//! `HistoryWindowHandler` on the main thread over a fake history use case (harness off: `AppKit`
//! needs the main thread).
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, UNIX_EPOCH};

use async_trait::async_trait;
use menubar::adapters::primary::statusmenu::appkit::app_kit_config::AppKitConfig;
use menubar::adapters::primary::statusmenu::appkit::history_window_handler::HistoryWindowHandler;
use menubar::core::application::exceptions::menu_action_error::MenuActionError;
use menubar::core::application::use_cases::browse_history::BrowseHistory;
use menubar::core::domain::history_command_enum::HistoryCommandEnum;
use menubar::core::domain::history_view_value_object::{
    HistoryViewValueObject, NOTHING_OPENED_NOTE,
};
use menubar::core::domain::link_record_value_object::LinkRecordValueObject;

#[derive(Default)]
struct FakeHistory {
    links: Mutex<Vec<String>>,
    commands: Mutex<Vec<HistoryCommandEnum>>,
}

#[async_trait]
impl BrowseHistory for FakeHistory {
    async fn execute(
        &self,
        command: HistoryCommandEnum,
    ) -> Result<HistoryViewValueObject, MenuActionError> {
        if let HistoryCommandEnum::Replay(link) = &command {
            self.links.lock().unwrap().insert(0, link.clone());
        }
        self.commands.lock().unwrap().push(command);
        let at = UNIX_EPOCH + Duration::from_secs(1_763_000_000);
        let rows = self
            .links
            .lock()
            .unwrap()
            .iter()
            .map(|link| LinkRecordValueObject::new(at, "opened".to_owned(), link.clone()))
            .collect();
        Ok(HistoryViewValueObject::new(rows))
    }
}

fn an_empty_log_shows_the_note() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let window = HistoryWindowHandler::install(mtm, Arc::new(FakeHistory::default()));
    assert!(!window.is_visible());
    window.show();
    assert!(window.is_visible());
    assert_eq!(window.rows(), Vec::<[String; 3]>::new());
    assert_eq!(window.note().as_deref(), Some(NOTHING_OPENED_NOTE));
}

fn the_play_button_replays_its_row() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let fake = Arc::new(FakeHistory::default());
    fake.links
        .lock()
        .unwrap()
        .extend(["goland://b".to_owned(), "idea://a".to_owned()]);
    let window = HistoryWindowHandler::install(mtm, fake.clone());
    window.show();
    let links: Vec<String> = window.rows().iter().map(|row| row[2].clone()).collect();
    assert_eq!(links, ["goland://b", "idea://a"]);
    assert_eq!(window.rows()[0][1], "opened");
    assert_eq!(window.rows()[0][0].len(), "2025-11-13 02:13:20".len());
    window.replay(1);
    assert_eq!(window.rows()[0][2], "idea://a");
    assert_eq!(window.rows().len(), 3);
    assert_eq!(
        *fake.commands.lock().unwrap().last().unwrap(),
        HistoryCommandEnum::Replay("idea://a".to_owned())
    );
    assert!(window.note().is_none());
}

fn main() {
    an_empty_log_shows_the_note();
    the_play_button_replays_its_row();
    println!("test result: ok. 2 passed; 0 failed");
}
