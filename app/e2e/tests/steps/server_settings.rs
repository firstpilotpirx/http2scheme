//! Steps of the `server-settings` capability on the built binary.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::fs;
use std::os::unix::fs::PermissionsExt as _;

use cucumber::{given, then, when};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

use crate::E2eWorld;
use crate::support::{Home, answers, free_port, get};

const KEY: &str = "0707070707070707070707070707070707070707070707070707070707070707";
const SLACK: &str = "slack://channel?id=C123";

fn settings_port(home: &Home) -> Option<u16> {
    let text = fs::read_to_string(home.settings_dir().join("settings.toml")).ok()?;
    text.lines()
        .find_map(|line| line.strip_prefix("port = "))?
        .parse()
        .ok()
}

/// Starts `serve` and waits until it answers or ends; it keeps running when it answers.
pub async fn server_starts_in(world: &mut E2eWorld) {
    let mut child = world.home.spawn_serve();
    let mut answering = false;
    for _ in 0..500 {
        if let Some(port) = settings_port(&world.home)
            && answers(port).await
        {
            answering = true;
            break;
        }
        if child.try_wait().unwrap().is_some() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    if answering {
        world.printed.code = None;
        world.server = Some(child);
        return;
    }
    let output = child.wait_with_output().await.unwrap();
    world.printed.code = output.status.code();
    world.printed.out = String::from_utf8(output.stdout).unwrap();
    world.printed.err = String::from_utf8(output.stderr).unwrap();
}

#[given(regex = r"^two homes with no settings$")]
fn two_homes(world: &mut E2eWorld) {
    assert!(!world.home.settings_dir().exists());
}

#[given(regex = r"^the settings hold port 9000 and a key exists$")]
fn settings_and_key(world: &mut E2eWorld) {
    world.home.write_settings(9000, &["idea", "goland"], true);
    world.home.write_key(KEY, 0o600);
}

#[given(regex = r"^the user added the Slack scheme to the allowed list in the settings$")]
fn slack_added(world: &mut E2eWorld) {
    let port = free_port();
    world
        .home
        .write_settings(port, &["idea", "goland", "slack"], true);
    world.port = Some(port);
}

#[given(regex = r"^the settings hold port 80$")]
fn port_80(world: &mut E2eWorld) {
    world.home.write_settings(80, &["idea"], true);
}

#[given(regex = r"^the key file is readable by other users$")]
fn key_readable(world: &mut E2eWorld) {
    world.home.write_settings(free_port(), &["idea"], true);
    world.home.write_key(KEY, 0o644);
}

#[when(regex = r"^the server starts$")]
async fn server_starts(world: &mut E2eWorld) {
    server_starts_in(world).await;
}

#[when(regex = r"^the server starts once in each$")]
async fn starts_in_each(world: &mut E2eWorld) {
    for _ in 0..2 {
        world.server = None;
        world.home = Home::new();
        server_starts_in(world).await;
        world.server = None;
        let key = world.home.key();
        world.keys.push(key);
    }
}

#[when(regex = r"^the server starts and the browser follows a keyed open link to a Slack link$")]
async fn follow_slack(world: &mut E2eWorld) {
    server_starts_in(world).await;
    let port = world.port.unwrap();
    let address = format!("127.0.0.1:{port}");
    let path = format!(
        "/open?key={}&url={}",
        world.home.key(),
        utf8_percent_encode(SLACK, NON_ALPHANUMERIC)
    );
    (world.answer.status, world.answer.body) = get(&address, &address, &path).await.unwrap();
}

#[then(
    regex = r"^the settings hold port 7777, the allowed schemes IntelliJ and GoLand, and tab closing on$"
)]
fn default_settings(world: &mut E2eWorld) {
    let text = world.home.settings_text();
    assert!(text.contains("port = 7777"), "{text}");
    assert!(
        text.contains(r#"allowed_schemes = ["idea", "goland"]"#),
        "{text}"
    );
    assert!(text.contains("close_tab = true"), "{text}");
}

#[then(regex = r"^a key of 64 hexadecimal characters exists that only the user can read$")]
fn private_key(world: &mut E2eWorld) {
    let key = world.home.key();
    assert_eq!(key.len(), 64);
    assert!(key.chars().all(|letter| letter.is_ascii_hexdigit()));
    let mode = fs::metadata(world.home.settings_dir().join("key"))
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(mode & 0o077, 0);
}

#[then(regex = r"^the two keys differ$")]
fn keys_differ(world: &mut E2eWorld) {
    assert_eq!(world.keys.len(), 2);
    assert_ne!(world.keys[0], world.keys[1]);
}

#[then(regex = r"^the settings still hold port 9000$")]
fn still_9000(world: &mut E2eWorld) {
    assert!(world.home.settings_text().contains("port = 9000"));
}

#[then(regex = r"^the key is unchanged$")]
fn key_unchanged(world: &mut E2eWorld) {
    assert_eq!(world.home.key(), KEY);
}

#[then(regex = r"^the Mac opens that Slack link$")]
fn opens_slack(world: &mut E2eWorld) {
    assert_eq!(world.answer.status, 200, "{}", world.answer.body);
    assert_eq!(world.home.calls("open"), [vec![SLACK.to_owned()]]);
}

#[then(regex = r"^it does not start$")]
fn does_not_start(world: &mut E2eWorld) {
    assert_eq!(world.printed.code, Some(1), "{}", world.printed.err);
    assert!(!world.printed.out.contains("listening"));
}

#[then(regex = r"^it says the port in the settings file must be between 1024 and 65535$")]
fn says_port_range(world: &mut E2eWorld) {
    let text = &world.printed.err;
    assert!(
        text.contains("settings.toml") && text.contains("port"),
        "{text}"
    );
    assert!(text.contains("must be between 1024 and 65535"), "{text}");
}

#[then(regex = r"^it says the key file must be readable only by its owner$")]
fn says_key_private(world: &mut E2eWorld) {
    let text = &world.printed.err;
    assert!(
        text.contains("must be readable only by its owner"),
        "{text}"
    );
}
