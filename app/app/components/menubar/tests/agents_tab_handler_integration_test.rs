//! `AgentsTabHandler` on the main thread over a fake Agents use case (harness off: `AppKit`
//! needs the main thread).
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests"
)]

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use menubar::adapters::primary::statusmenu::appkit::agents_tab_handler::AgentsTabHandler;
use menubar::adapters::primary::statusmenu::appkit::app_kit_config::AppKitConfig;
use menubar::core::application::use_cases::manage_skill::ManageSkill;
use menubar::core::domain::agents_command_enum::AgentsCommandEnum;
use menubar::core::domain::agents_view_value_object::AgentsViewValueObject;
use menubar::menubar_hooks::skill_place;

/// Answers every folder with `state`, an install with `installed` or a failure, and records the
/// commands.
struct FakeAgents {
    state: &'static str,
    failing: bool,
    commands: Mutex<Vec<AgentsCommandEnum>>,
}

impl FakeAgents {
    fn new(state: &'static str, failing: bool) -> Arc<Self> {
        Arc::new(Self {
            state,
            failing,
            commands: Mutex::new(Vec::new()),
        })
    }
}

#[async_trait]
impl ManageSkill for FakeAgents {
    async fn execute(&self, command: AgentsCommandEnum) -> AgentsViewValueObject {
        self.commands.lock().unwrap().push(command.clone());
        let home = PathBuf::from("/Users/someone");
        let (folder, state, error, copied) = match command {
            AgentsCommandEnum::Show(folder) => (folder, self.state, None, false),
            AgentsCommandEnum::Install(folder) if self.failing => {
                (Some(folder), self.state, Some("denied".to_owned()), false)
            }
            AgentsCommandEnum::Install(folder) => (Some(folder), "installed", None, false),
            AgentsCommandEnum::Copy(folder) => (Some(folder), self.state, None, true),
        };
        let folder = folder.unwrap_or_else(|| home.join(".claude"));
        let place = skill_place((folder.join("skills/http2scheme/SKILL.md"), None, state)).unwrap();
        let view = AgentsViewValueObject::new(folder, home, Some(place), error);
        if copied {
            view.copied("the skill".to_owned())
        } else {
            view
        }
    }
}

fn an_older_skill_offers_update() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let tab = AgentsTabHandler::install(mtm, FakeAgents::new("outdated", false));
    tab.show();
    assert_eq!(tab.lines()[0], "~/.claude");
    assert_eq!(tab.names()[1], "Skill · An older version is installed");
    assert_eq!(tab.button(), "Update");
}

fn install_writes_into_the_shown_folder() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let fake = FakeAgents::new("not-installed", false);
    let tab = AgentsTabHandler::install(mtm, fake.clone());
    tab.choose_folder(&PathBuf::from("/w/.claude"));
    tab.install_skill();
    assert_eq!(tab.names()[1], "Skill · Installed");
    assert_eq!(
        fake.commands.lock().unwrap().last(),
        Some(&AgentsCommandEnum::Install(PathBuf::from("/w/.claude")))
    );
}

fn a_failed_install_shows_the_reason() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let tab = AgentsTabHandler::install(mtm, FakeAgents::new("not-installed", true));
    tab.show();
    tab.install_skill();
    assert_eq!(tab.lines()[1], "denied");
}

fn copy_puts_the_text_on_the_clipboard() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let tab = AgentsTabHandler::install(mtm, FakeAgents::new("installed", false));
    tab.show();
    tab.copy();
    assert_eq!(tab.clipboard().as_deref(), Some("the skill"));
    assert_eq!(tab.lines()[2], "Copied to the clipboard.");
}

fn install_before_any_show_uses_the_remembered_folder() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let fake = FakeAgents::new("not-installed", false);
    let tab = AgentsTabHandler::install(mtm, fake.clone());
    tab.install_skill();
    assert_eq!(
        fake.commands.lock().unwrap().last(),
        Some(&AgentsCommandEnum::Install(PathBuf::from(
            "/Users/someone/.claude"
        )))
    );
}

fn the_folder_panel_picks_folders_from_the_shown_one() {
    let mtm = AppKitConfig::main_thread().unwrap();
    let home = tempfile::tempdir().unwrap();
    let tab = AgentsTabHandler::install(mtm, FakeAgents::new("installed", false));
    tab.choose_folder(home.path());
    let (picks, start) = tab.folder_panel();
    assert_eq!(picks, [true, false, true]);
    let start = std::fs::canonicalize(start.unwrap()).unwrap();
    assert_eq!(start, std::fs::canonicalize(home.path()).unwrap());
}

fn main() {
    install_before_any_show_uses_the_remembered_folder();
    the_folder_panel_picks_folders_from_the_shown_one();
    an_older_skill_offers_update();
    install_writes_into_the_shown_folder();
    a_failed_install_shows_the_reason();
    copy_puts_the_text_on_the_clipboard();
    println!("test result: ok. 6 passed; 0 failed");
}
