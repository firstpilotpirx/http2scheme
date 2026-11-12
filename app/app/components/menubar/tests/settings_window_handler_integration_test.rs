//! `SettingsWindowHandler` on the main thread over fake menu and history use cases (harness off:
//! `AppKit` needs the main thread).
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;
use menubar::adapters::primary::statusmenu::appkit::app_kit_config::AppKitConfig;
use menubar::adapters::primary::statusmenu::appkit::history_tab_handler::HistoryTabHandler;
use menubar::adapters::primary::statusmenu::appkit::settings_window_handler::SettingsWindowHandler;
use menubar::core::application::exceptions::menu_action_error::MenuActionError;
use menubar::core::application::use_cases::browse_history::BrowseHistory;
use menubar::core::application::use_cases::operate_menu::OperateMenu;
use menubar::core::domain::history_command_enum::HistoryCommandEnum;
use menubar::core::domain::history_view_value_object::{
    HistoryViewValueObject, NOTHING_OPENED_NOTE,
};
use menubar::core::domain::login_item_state_enum::LoginItemStateEnum;
use menubar::core::domain::menu_command_enum::MenuCommandEnum;
use menubar::core::domain::menu_view_value_object::MenuViewValueObject;
use menubar::core::domain::server_state_enum::ServerStateEnum;

/// A history whose request log is empty.
struct EmptyHistory;

#[async_trait]
impl BrowseHistory for EmptyHistory {
    async fn execute(
        &self,
        _command: HistoryCommandEnum,
    ) -> Result<HistoryViewValueObject, MenuActionError> {
        Ok(HistoryViewValueObject::new(Vec::new()))
    }
}

/// A menu whose server turns on and off and whose login item turns on and off.
#[derive(Default)]
struct FakeMenu {
    commands: Mutex<Vec<MenuCommandEnum>>,
}

#[async_trait]
impl OperateMenu for FakeMenu {
    async fn execute(
        &self,
        command: MenuCommandEnum,
    ) -> Result<MenuViewValueObject, MenuActionError> {
        let mut commands = self.commands.lock().unwrap();
        commands.push(command);
        let turned = |turn| commands.iter().filter(|c| **c == turn).count() % 2 == 1;
        let server = if turned(MenuCommandEnum::ToggleServer) {
            ServerStateEnum::Off
        } else {
            ServerStateEnum::Running("http://127.0.0.1:7777".to_owned())
        };
        let login = if turned(MenuCommandEnum::ToggleLoginItem) {
            LoginItemStateEnum::On
        } else {
            LoginItemStateEnum::Off
        };
        Ok(MenuViewValueObject::new("0.1.0", &server, login))
    }
}

fn settings_shows_the_switches_and_their_lines() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let menu = Arc::new(FakeMenu::default());
    let history = HistoryTabHandler::install(mtm, Arc::new(EmptyHistory));
    let window = SettingsWindowHandler::install(mtm, menu.clone(), history);
    window.show_general();
    assert!(window.is_visible());
    assert_eq!(window.selected_tab(), "General");
    assert_eq!(window.switches(), [true, false]);
    assert!(window.login_enabled());
    assert_eq!(
        window.lines(),
        [
            "Running on http://127.0.0.1:7777".to_owned(),
            "The app starts when you log in to the Mac.".to_owned(),
        ]
    );
    window.turn_server();
    window.turn_login();
    assert_eq!(window.switches(), [false, true]);
    assert_eq!(window.lines()[0], "Off");
    let commands = menu.commands.lock().unwrap().clone();
    assert!(
        commands.contains(&MenuCommandEnum::ToggleServer),
        "{commands:?}"
    );
    assert!(
        commands.contains(&MenuCommandEnum::ToggleLoginItem),
        "{commands:?}"
    );
    window.show_history();
    assert_eq!(window.selected_tab(), "History");
    assert_eq!(
        window.history().note().as_deref(),
        Some(NOTHING_OPENED_NOTE)
    );
}

fn main() {
    settings_shows_the_switches_and_their_lines();
    println!("test result: ok. 1 passed; 0 failed");
}
