//! `HistoryTabHandler` on the main thread over a fake history use case (harness off: `AppKit`
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
use menubar::adapters::primary::statusmenu::appkit::history_tab_handler::HistoryTabHandler;
use menubar::core::application::exceptions::menu_action_error::MenuActionError;
use menubar::core::application::use_cases::browse_history::BrowseHistory;
use menubar::core::domain::history_command_enum::HistoryCommandEnum;
use menubar::core::domain::history_view_value_object::{
    HistoryViewValueObject, NOTHING_OPENED_NOTE,
};
use menubar::core::domain::link_record_value_object::LinkRecordValueObject;
use objc2_foundation::{NSDate, NSTimeZone};

#[derive(Default)]
struct FakeHistory {
    unreadable: bool,
    links: Mutex<Vec<String>>,
    commands: Mutex<Vec<HistoryCommandEnum>>,
}

#[async_trait]
impl BrowseHistory for FakeHistory {
    async fn execute(
        &self,
        command: HistoryCommandEnum,
    ) -> Result<HistoryViewValueObject, MenuActionError> {
        if self.unreadable {
            return Err(MenuActionError::History {
                path: "/l/requests.log".to_owned(),
                reason: "denied".to_owned(),
            });
        }
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
    let window = HistoryTabHandler::install(mtm, Arc::new(FakeHistory::default()));
    assert_eq!(window.note(), None);
    window.show();
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
    let window = HistoryTabHandler::install(mtm, fake.clone());
    window.show();
    let links: Vec<String> = window.rows().iter().map(|row| row[2].clone()).collect();
    assert_eq!(links, ["goland://b", "idea://a"]);
    assert_eq!(window.rows()[0][1], "opened");
    let at = UNIX_EPOCH + Duration::from_secs(1_763_000_000);
    let date = NSDate::dateWithTimeIntervalSince1970(1_763_000_000.0);
    let offset = NSTimeZone::localTimeZone().secondsFromGMTForDate(&date);
    let record = LinkRecordValueObject::new(at, "opened".to_owned(), "idea://a".to_owned());
    assert_eq!(
        window.rows()[0][0],
        record.local_time(i64::try_from(offset).unwrap())
    );
    window.replay(1);
    assert_eq!(window.rows()[0][2], "idea://a");
    assert_eq!(window.rows().len(), 3);
    assert_eq!(
        *fake.commands.lock().unwrap().last().unwrap(),
        HistoryCommandEnum::Replay("idea://a".to_owned())
    );
    assert!(window.note().is_none());
}

fn showing_again_reads_the_log_again() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let fake = Arc::new(FakeHistory::default());
    let window = HistoryTabHandler::install(mtm, fake.clone());
    window.show();
    assert!(window.note().is_some());
    fake.links.lock().unwrap().push("idea://new".to_owned());
    window.show();
    assert_eq!(window.rows()[0][2], "idea://new");
    let shows = fake
        .commands
        .lock()
        .unwrap()
        .iter()
        .filter(|command| **command == HistoryCommandEnum::Show)
        .count();
    assert_eq!(shows, 2);
}

fn an_unreadable_log_says_why() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let fake = FakeHistory {
        unreadable: true,
        ..FakeHistory::default()
    };
    let window = HistoryTabHandler::install(mtm, Arc::new(fake));
    window.show();
    assert_eq!(window.rows(), Vec::<[String; 3]>::new());
    let note = window.note().unwrap();
    assert!(
        note.starts_with("The link history could not be read: "),
        "{note}"
    );
    assert!(note.contains("denied"), "{note}");
}

fn main() {
    an_unreadable_log_says_why();
    an_empty_log_shows_the_note();
    showing_again_reads_the_log_again();
    the_play_button_replays_its_row();
    println!("test result: ok. 4 passed; 0 failed");
}
