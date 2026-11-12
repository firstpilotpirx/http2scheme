//! Steps of the `server-settings` capability, driven through the serve command over a temporary home.

use std::fs;
use std::os::unix::fs::PermissionsExt as _;

use cucumber::{given, then, when};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

use super::home::{
    KEY, free_port, key, serve_in_background, serve_once, settings_dir, settings_text, write_key,
    write_settings,
};
use crate::LinkserverWorld;

const SLACK: &str = "slack://channel?id=C123";

#[given(regex = r"^two homes with no settings$")]
fn two_homes(world: &mut LinkserverWorld) {
    assert!(world.home.is_none());
}

#[given(regex = r"^the settings hold port 9000 and a key exists$")]
fn settings_and_key(world: &mut LinkserverWorld) {
    write_settings(world, 9000, &["idea", "goland"]);
    write_key(world, 0o600);
}

#[given(regex = r"^the user added the Slack scheme to the allowed list in the settings$")]
fn slack_added(world: &mut LinkserverWorld) {
    write_settings(world, free_port(), &["idea", "goland", "slack"]);
}

#[given(regex = r"^the settings hold port 80$")]
fn port_80(world: &mut LinkserverWorld) {
    write_settings(world, 80, &["idea"]);
}

#[given(regex = r"^the key file is readable by other users$")]
fn key_readable(world: &mut LinkserverWorld) {
    write_settings(world, free_port(), &["idea"]);
    write_key(world, 0o644);
}

#[when(regex = r"^the server starts$")]
async fn server_starts(world: &mut LinkserverWorld) {
    serve_once(world).await;
}

#[when(regex = r"^the server starts once in each$")]
async fn starts_in_each(world: &mut LinkserverWorld) {
    for _ in 0..2 {
        world.home = None;
        serve_once(world).await;
        let key = key(world);
        world.keys.push(key);
    }
}

#[when(regex = r"^the server starts and the browser follows a keyed open link to a Slack link$")]
async fn follow_slack(world: &mut LinkserverWorld) {
    let port: u16 = settings_text(world)
        .lines()
        .find_map(|line| line.strip_prefix("port = "))
        .unwrap()
        .parse()
        .unwrap();
    serve_in_background(world, port).await;
    let link = format!(
        "http://127.0.0.1:{port}/open?key={}&url={}",
        key(world),
        utf8_percent_encode(SLACK, NON_ALPHANUMERIC)
    );
    let response = reqwest::get(link).await.unwrap();
    world.answer.status = response.status().as_u16();
}

#[then(
    regex = r"^the settings hold port 7777, the allowed schemes IntelliJ and GoLand, and tab closing on$"
)]
fn default_settings(world: &mut LinkserverWorld) {
    let text = settings_text(world);
    assert!(text.contains("port = 7777"), "{text}");
    assert!(
        text.contains(r#"allowed_schemes = ["idea", "goland"]"#),
        "{text}"
    );
    assert!(text.contains("close_tab = true"), "{text}");
}

#[then(
    regex = r"^a key of 64 lower-case hexadecimal characters exists that only the user can read$"
)]
fn private_key(world: &mut LinkserverWorld) {
    let key = key(world);
    assert_eq!(key.len(), 64);
    assert!(key.bytes().all(|b| b"0123456789abcdef".contains(&b)));
    let mode = fs::metadata(settings_dir(world).join("key"))
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(mode & 0o077, 0);
}

#[then(regex = r"^the two keys differ$")]
fn keys_differ(world: &mut LinkserverWorld) {
    assert_eq!(world.keys.len(), 2);
    assert_ne!(world.keys[0], world.keys[1]);
}

#[then(regex = r"^the settings still hold port 9000$")]
fn still_9000(world: &mut LinkserverWorld) {
    assert!(settings_text(world).contains("port = 9000"));
}

#[then(regex = r"^the key is unchanged$")]
fn key_unchanged(world: &mut LinkserverWorld) {
    assert_eq!(key(world), KEY);
}

#[then(regex = r"^the Mac opens that Slack link$")]
fn opens_slack(world: &mut LinkserverWorld) {
    assert_eq!(world.answer.status, 200);
    assert_eq!(world.fakes.opener.links(), [SLACK.to_owned()]);
}

#[then(regex = r"^it does not start$")]
fn does_not_start(world: &mut LinkserverWorld) {
    assert_eq!(world.printed.code, Some(1));
    assert!(!world.printed.out.text().contains("listening"));
}

#[then(regex = r"^it says the port in the settings file must be between 1024 and 65535$")]
fn says_port_range(world: &mut LinkserverWorld) {
    let text = world.printed.err.text();
    assert!(text.contains("settings.toml"), "{text}");
    assert!(text.contains("port"), "{text}");
    assert!(text.contains("must be between 1024 and 65535"), "{text}");
}

#[then(regex = r"^it says the key file must be readable only by its owner$")]
fn says_key_private(world: &mut LinkserverWorld) {
    let text = world.printed.err.text();
    assert!(text.contains("key"), "{text}");
    assert!(
        text.contains("must be readable only by its owner"),
        "{text}"
    );
}
