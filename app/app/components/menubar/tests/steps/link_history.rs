//! Steps of the `menu-bar-app` capability for the History tab and its ▶ buttons.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use cucumber::{given, then, when};
use menubar::menubar_config::MenubarConfig;

use super::menu_bar_app::config;
use crate::MenubarWorld;

const INTELLIJ: &str = "idea://open?file=/a/B.java&line=3";
const GOLAND: &str = "goland://open?file=/a/b.go&line=7";
const SLACK: &str = "slack://channel?id=C1";

fn open_history(world: &mut MenubarWorld, replay: Option<usize>) {
    let mtm = MenubarConfig::main_thread().unwrap();
    let config = config(world);
    let window = config.settings_window(mtm, config.menu(), config.history_tab(mtm));
    window.show_history();
    if let Some(index) = replay {
        window.history().replay(index);
    }
    world.rows = window.history().rows();
    world.note = window.history().note();
}

fn link_of(name: &str) -> &'static str {
    match name {
        "IntelliJ" => INTELLIJ,
        "GoLand" => GOLAND,
        "Slack" => SLACK,
        other => panic!("no {other} link"),
    }
}

#[given(
    regex = r"^the request log holds an opened IntelliJ link and, after it, a GoLand link with the wrong key$"
)]
fn log_two(world: &mut MenubarWorld) {
    world
        .replayer
        .append("2026-11-12T10:00:00Z", "opened", INTELLIJ);
    world
        .replayer
        .append("2026-11-12T10:01:00Z", "wrong key", GOLAND);
}

#[given(regex = r"^the request log holds (\d+) requests$")]
fn log_many(world: &mut MenubarWorld, count: usize) {
    for index in 0..count {
        world.replayer.append(
            "2026-11-12T10:00:00Z",
            "opened",
            &format!("idea://open?line={index}"),
        );
    }
}

#[given(regex = r"^the request log holds two requests and a damaged line between them$")]
fn log_damaged(world: &mut MenubarWorld) {
    world
        .replayer
        .append("2026-11-12T10:00:00Z", "opened", INTELLIJ);
    world.replayer.append_damaged();
    world
        .replayer
        .append("2026-11-12T10:01:00Z", "opened", GOLAND);
}

#[given(regex = r"^the request log does not exist$")]
fn no_log(world: &mut MenubarWorld) {
    assert!(!world.home.path().join("logs/requests.log").exists());
}

#[given(
    regex = r#"^the History tab lists an? (?:opened )?(IntelliJ|Slack) link(?: with the result "(.+)")?$"#
)]
fn window_lists(world: &mut MenubarWorld, name: String, result: String) {
    let result = if result.is_empty() {
        "opened".to_owned()
    } else {
        result
    };
    world
        .replayer
        .append("2026-11-12T10:00:00Z", &result, link_of(&name));
    open_history(world, None);
    assert_eq!(world.rows[0][2], link_of(&name));
}

#[when(regex = r"^the user opens the History tab$")]
fn opens_history(world: &mut MenubarWorld) {
    open_history(world, None);
}

#[when(regex = r"^the user chooses ▶ on its row$")]
fn replays(world: &mut MenubarWorld) {
    open_history(world, Some(0));
}

#[then(
    regex = r#"^the (first|second) row shows the (IntelliJ|GoLand|Slack) link with the result "(.+)"$"#
)]
fn row_shows(world: &mut MenubarWorld, place: String, name: String, result: String) {
    let row = &world.rows[usize::from(place == "second")];
    assert_eq!(row[2], link_of(&name), "{:?}", world.rows);
    assert_eq!(row[1], result);
}

#[then(regex = r#"^the first row shows that (IntelliJ|Slack) link with the result "(.+)"$"#)]
fn first_row_that(world: &mut MenubarWorld, name: String, result: String) {
    row_shows(world, "first".to_owned(), name, result);
    assert_eq!(world.rows.len(), 2, "the new request is a row of its own");
}

#[then(regex = r"^it lists (\d+) rows, the latest \d+$")]
fn lists_latest(world: &mut MenubarWorld, count: usize) {
    assert_eq!(world.rows.len(), count);
    assert_eq!(world.rows[0][2], "idea://open?line=104");
    assert_eq!(world.rows[count - 1][2], "idea://open?line=5");
}

#[then(regex = r"^it lists the two requests$")]
fn lists_two(world: &mut MenubarWorld) {
    let links: Vec<&str> = world.rows.iter().map(|row| row[2].as_str()).collect();
    assert_eq!(links, [GOLAND, INTELLIJ]);
}

#[then(regex = r"^the tab says no link has been opened yet$")]
fn says_nothing_yet(world: &mut MenubarWorld) {
    assert_eq!(world.rows, Vec::<[String; 3]>::new());
    assert_eq!(world.note.as_deref(), Some("No link has been opened yet."));
}

#[then(regex = r"^the Mac opens that IntelliJ link$")]
fn opens_intellij(world: &mut MenubarWorld) {
    assert_eq!(world.replayer.opened(), [INTELLIJ]);
}

#[then(regex = r"^the Mac opens nothing$")]
fn opens_nothing(world: &mut MenubarWorld) {
    assert_eq!(world.replayer.opened(), Vec::<String>::new());
}
