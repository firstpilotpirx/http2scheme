//! Steps of the `menu-bar-app` capability, driven through the component's root on the main thread.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::io::Write as _;
use std::path::PathBuf;
use std::sync::Arc;

use cucumber::{given, then, when};
use menubar::core::domain::login_item_state_enum::LoginItemStateEnum;
use menubar::menubar_config::MenubarConfig;
use menubar::menubar_ports::MenubarPorts;
use menubar::menubar_setup::MenubarSetup;

use crate::MenubarWorld;
use crate::fakes::FakeLoginItem;

const APPLICATIONS: &str = "/Applications/http2scheme.app/Contents/MacOS/http2scheme";
const DOWNLOADS: &str = "/Users/someone/Downloads/http2scheme.app/Contents/MacOS/http2scheme";
const SERVE: usize = 2;
const LOGIN: usize = 5;

pub(super) fn config(world: &MenubarWorld) -> MenubarConfig {
    let home = world.home.path();
    let setup = MenubarSetup {
        version: "0.1.0".to_owned(),
        settings_dir: home.join("settings"),
        log_dir: home.join("logs"),
        program: PathBuf::from(&world.program),
        open_program: world.open.program.clone(),
    };
    let mut ports = MenubarPorts::macos(&setup, &world.output, world.replayer.replayer());
    ports.login_item = world.login_item.clone();
    MenubarConfig::new(setup, ports)
}

fn login_item(world: &mut MenubarWorld, state: LoginItemStateEnum) {
    world.login_item = Arc::new(FakeLoginItem::new(state));
}

fn show(world: &mut MenubarWorld, choice: Option<&str>) {
    let mtm = MenubarConfig::main_thread().unwrap();
    let menu = config(world).status_menu(mtm, Box::new(|| {}));
    if let Some(title) = choice {
        menu.choose(title);
    }
    world.items = menu.items();
}

#[given(regex = r"^the server listens on port (\d+)$")]
fn listens(world: &mut MenubarWorld, port: u16) {
    let banner = format!("[..] http2scheme 0.1.0 listening on http://127.0.0.1:{port}\n");
    world.output.out().write_all(banner.as_bytes()).unwrap();
}

#[given(regex = r"^another program holds the configured port (\d+)$")]
fn port_held(world: &mut MenubarWorld, port: u16) {
    let failure = format!("[fail] port {port} is taken\n");
    world.output.err().write_all(failure.as_bytes()).unwrap();
}

#[given(regex = r"^the app is (not )?in the login items$")]
fn in_login_items(world: &mut MenubarWorld, not: String) {
    let state = if not.is_empty() {
        LoginItemStateEnum::On
    } else {
        LoginItemStateEnum::Off
    };
    login_item(world, state);
}

#[given(regex = r"^macOS waits for the user's approval of the app's login item$")]
fn waits_for_approval(world: &mut MenubarWorld) {
    login_item(world, LoginItemStateEnum::NeedsApproval);
}

#[given(regex = r"^the program does not run from an app bundle$")]
fn not_a_bundle(world: &mut MenubarWorld) {
    login_item(world, LoginItemStateEnum::Unavailable);
}

#[given(
    regex = r"^the app runs from (the Applications folder|a folder other than Applications) for the first time$"
)]
fn first_time(world: &mut MenubarWorld, folder: String) {
    let program = if folder.starts_with("the") {
        APPLICATIONS
    } else {
        DOWNLOADS
    };
    program.clone_into(&mut world.program);
}

#[given(regex = r"^the app added itself to the login items before and the user removed it$")]
async fn removed_before(world: &mut MenubarWorld) {
    config(world).offer_login_item().await.unwrap();
    assert_eq!(world.login_item.state(), LoginItemStateEnum::On);
    login_item(world, LoginItemStateEnum::Off);
}

#[given(regex = r"^the app runs(?: and no request was logged yet)?$")]
fn app_runs(world: &mut MenubarWorld) {
    assert!(!world.home.path().join("logs").exists());
}

#[when(regex = r"^the user opens the menu$")]
fn opens_menu(world: &mut MenubarWorld) {
    show(world, None);
}

#[when(regex = r"^the user chooses (Start at Login|Open Settings Folder|Open Logs Folder)$")]
fn chooses(world: &mut MenubarWorld, item: String) {
    let title = if item == "Start at Login" {
        world.login_item.state().title().to_owned()
    } else {
        item
    };
    show(world, Some(&title));
}

#[when(regex = r"^the app starts(?: again)?$")]
async fn app_starts(world: &mut MenubarWorld) {
    config(world).offer_login_item().await.unwrap();
}

#[then(regex = r"^the menu shows the app name with its version$")]
fn shows_name(world: &mut MenubarWorld) {
    assert_eq!(world.items[0].0, "http2scheme 0.1.0");
}

#[then(regex = r#"^the status line reads "(.+)"$"#)]
fn status_line(world: &mut MenubarWorld, line: String) {
    assert_eq!(world.items[1].0, line);
}

#[then(regex = r"^the status line says the server runs on the loopback address with port (\d+)$")]
fn status_running(world: &mut MenubarWorld, port: u16) {
    assert_eq!(
        world.items[1].0,
        format!("Running on http://127.0.0.1:{port}")
    );
}

#[then(regex = r"^the app is (not )?in the login items$")]
fn is_in_login_items(world: &mut MenubarWorld, not: String) {
    let expected = if not.is_empty() {
        LoginItemStateEnum::On
    } else {
        LoginItemStateEnum::Off
    };
    assert_eq!(
        world.login_item.state(),
        expected,
        "{:?}",
        world.login_item.actions()
    );
}

#[then(regex = r"^Start at Login shows (a|no) check mark$")]
fn check_mark(world: &mut MenubarWorld, mark: String) {
    assert_eq!(world.items[LOGIN].0, "Start at Login");
    assert_eq!(world.items[LOGIN].2, mark == "a");
}

#[then(regex = r"^the Login Items pane of System Settings opens$")]
fn settings_pane(world: &mut MenubarWorld) {
    assert_eq!(world.login_item.actions(), ["open_system_settings"]);
}

#[then(regex = r"^Start at Login is disabled$")]
fn login_disabled(world: &mut MenubarWorld) {
    assert!(!world.items[LOGIN].1, "{:?}", world.items);
}

#[then(regex = r"^the login items are unchanged$")]
fn unchanged(world: &mut MenubarWorld) {
    assert_eq!(world.login_item.actions(), Vec::<&str>::new());
    assert!(!world.home.path().join("settings").exists());
}

#[then(regex = r"^the Finder opens the (settings|logs) folder$")]
fn finder_opens(world: &mut MenubarWorld, folder: String) {
    let path = world.home.path().join(folder);
    assert_eq!(world.open.calls(), [[path.display().to_string()]]);
}

#[then(regex = r"^the logs folder exists$")]
fn logs_exist(world: &mut MenubarWorld) {
    assert!(world.home.path().join("logs").is_dir());
}

#[then(regex = r"^the Finder opens it$")]
fn finder_opens_it(world: &mut MenubarWorld) {
    finder_opens(world, "logs".to_owned());
}

#[given(regex = r"^the app runs and its server answers$")]
fn server_answers(world: &mut MenubarWorld) {
    config(world).start_server();
    assert!(world.server.answers());
}

#[given(regex = r"^the user turned the server off$")]
fn turned_off(world: &mut MenubarWorld) {
    server_answers(world);
    show(world, Some("Serve Links"));
    assert!(!world.server.answers());
}

#[when(regex = r"^the user chooses Serve Links$")]
fn chooses_serve(world: &mut MenubarWorld) {
    show(world, Some("Serve Links"));
}

#[then(regex = r"^the server (no longer answers|answers again)$")]
fn server_state(world: &mut MenubarWorld, state: String) {
    assert_eq!(world.server.answers(), state == "answers again");
    if state == "answers again" {
        assert_eq!(world.server.starts(), 2);
    }
}

#[then(regex = r"^Serve Links shows (a|no) check mark$")]
fn serve_mark(world: &mut MenubarWorld, mark: String) {
    assert_eq!(world.items[SERVE].0, "Serve Links");
    assert_eq!(world.items[SERVE].2, mark == "a");
}
