//! Steps of the `menu-bar-app` capability for the Settings window and its switches.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::rc::Rc;

use cucumber::{given, then, when};
use menubar::adapters::primary::statusmenu::appkit::settings_window_handler::SettingsWindowHandler;
use menubar::menubar_config::MenubarConfig;

use super::menu_bar_app::{config, show};
use crate::MenubarWorld;

fn read(world: &mut MenubarWorld, window: &SettingsWindowHandler) {
    world.tab = window.selected_tab();
    world.switches = window.switches();
    world.lines = window.lines();
}

fn through_menu(world: &mut MenubarWorld, title: &str) {
    let mtm = MenubarConfig::main_thread().unwrap();
    let config = config(world);
    let menu = config.menu();
    let window = Rc::new(config.settings_window(mtm, menu.clone(), config.history_tab(mtm)));
    let (general, history) = (Rc::clone(&window), Rc::clone(&window));
    let status = config.status_menu(
        mtm,
        menu,
        Box::new(move || general.show_general()),
        Box::new(move || history.show_history()),
    );
    status.choose(title);
    read(world, &window);
}

fn turn(world: &mut MenubarWorld, switch: &str) {
    let mtm = MenubarConfig::main_thread().unwrap();
    let config = config(world);
    let window = config.settings_window(mtm, config.menu(), config.history_tab(mtm));
    window.show_general();
    if switch == "Serve links" {
        window.turn_server();
    } else {
        window.turn_login();
    }
    read(world, &window);
    show(world, None);
}

#[given(regex = r"^the app runs and its server listens on port (\d+)$")]
fn runs_on(world: &mut MenubarWorld, port: u16) {
    config(world).start_server();
    assert!(world.server.answers(), "the fake server listens on {port}");
}

#[when(regex = r"^the user chooses (Settings…|Link History…)$")]
fn chooses_window(world: &mut MenubarWorld, title: String) {
    through_menu(world, &title);
}

#[when(
    regex = r"^the user turns the (Serve links|Start at login) switch (?:on|off) in the Settings window$"
)]
fn turns(world: &mut MenubarWorld, switch: String) {
    turn(world, &switch);
}

#[then(regex = r"^the Settings window shows its (General|History) tab$")]
fn shows_tab(world: &mut MenubarWorld, tab: String) {
    assert_eq!(world.tab, tab);
}

#[then(
    regex = r"^the Serve links switch is on, with the line under it saying the server runs on port (\d+)$"
)]
fn serve_switch_on(world: &mut MenubarWorld, port: u16) {
    assert!(world.switches[0], "{:?}", world.switches);
    assert_eq!(
        world.lines[0],
        format!("Running on http://127.0.0.1:{port}")
    );
}

#[then(regex = r#"^the line under the switch reads "(.+)"$"#)]
fn line_reads(world: &mut MenubarWorld, line: String) {
    assert_eq!(world.lines[0], line);
    assert!(!world.switches[0], "{:?}", world.switches);
}
