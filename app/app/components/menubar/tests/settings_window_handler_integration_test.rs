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
use menubar::adapters::primary::statusmenu::appkit::settings_window_handler::{
    SettingsWindowHandler, WINDOW_TITLE,
};
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

/// A menu whose server turns on and off and whose login item turns on and off, unless the
/// login item is stuck in one state.
#[derive(Default)]
struct FakeMenu {
    stuck_login: Option<LoginItemStateEnum>,
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
        let login = if let Some(stuck) = &self.stuck_login {
            *stuck
        } else if turned(MenuCommandEnum::ToggleLoginItem) {
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
    assert_eq!(window.title(), WINDOW_TITLE);
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

fn settings_window(login: LoginItemStateEnum) -> (Arc<FakeMenu>, SettingsWindowHandler) {
    let mtm = AppKitConfig::main_thread().unwrap();
    let menu = Arc::new(FakeMenu {
        stuck_login: Some(login),
        ..FakeMenu::default()
    });
    let history = HistoryTabHandler::install(mtm, Arc::new(EmptyHistory));
    let window = SettingsWindowHandler::install(mtm, menu.clone(), history);
    window.show_general();
    (menu, window)
}

fn a_copy_outside_applications_cannot_turn_login_on() {
    let (menu, window) = settings_window(LoginItemStateEnum::Unavailable);
    assert!(!window.login_enabled());
    assert_eq!(
        window.lines()[1],
        "Only the app installed in Applications can start at login."
    );
    window.turn_login();
    assert!(
        !menu
            .commands
            .lock()
            .unwrap()
            .contains(&MenuCommandEnum::ToggleLoginItem)
    );
    assert!(!window.switches()[1]);
}

fn a_login_item_waiting_for_approval_says_where_to_approve() {
    let (_menu, window) = settings_window(LoginItemStateEnum::NeedsApproval);
    assert!(window.login_enabled());
    assert_eq!(
        window.lines()[1],
        "Approve the app in System Settings › Login Items."
    );
}

/// A menu whose server reports starting for its first answers, then running; or that always fails.
struct StartingMenu {
    starting: Mutex<usize>,
    broken: bool,
}

#[async_trait]
impl OperateMenu for StartingMenu {
    async fn execute(
        &self,
        _command: MenuCommandEnum,
    ) -> Result<MenuViewValueObject, MenuActionError> {
        if self.broken {
            return Err(MenuActionError::LoginItem("broken".to_owned()));
        }
        let mut starting = self.starting.lock().unwrap();
        let server = if *starting > 0 {
            *starting -= 1;
            ServerStateEnum::Starting
        } else {
            ServerStateEnum::Running("http://127.0.0.1:7777".to_owned())
        };
        Ok(MenuViewValueObject::new(
            "0.1.0",
            &server,
            LoginItemStateEnum::Off,
        ))
    }
}

fn starting_window(menu: StartingMenu) -> SettingsWindowHandler {
    let mtm = AppKitConfig::main_thread().unwrap();
    let history = HistoryTabHandler::install(mtm, Arc::new(EmptyHistory));
    SettingsWindowHandler::install(mtm, Arc::new(menu), history)
}

fn turning_serve_links_on_waits_until_the_server_runs() {
    let window = starting_window(StartingMenu {
        starting: Mutex::new(0),
        broken: false,
    });
    window.show_general();
    window.turn_server();
    assert_eq!(window.lines()[0], "Running on http://127.0.0.1:7777");
    let window = starting_window(StartingMenu {
        starting: Mutex::new(3),
        broken: false,
    });
    window.turn_server();
    assert_eq!(window.lines()[0], "Running on http://127.0.0.1:7777");
    assert!(window.switches()[0]);
}

fn a_failing_menu_leaves_the_switches_as_they_were() {
    let window = starting_window(StartingMenu {
        starting: Mutex::new(0),
        broken: true,
    });
    window.show_general();
    assert_eq!(window.switches(), [false, false]);
    window.turn_server();
    assert_eq!(window.lines(), [String::new(), String::new()]);
}

fn main() {
    settings_shows_the_switches_and_their_lines();
    a_copy_outside_applications_cannot_turn_login_on();
    a_login_item_waiting_for_approval_says_where_to_approve();
    turning_serve_links_on_waits_until_the_server_runs();
    a_failing_menu_leaves_the_switches_as_they_were();
    println!("test result: ok. 5 passed; 0 failed");
}
