//! Steps of the `link-sharing` capability, driven through the link command over a temporary home.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use cucumber::{given, then, when};
use percent_encoding::percent_decode_str;

use super::home::{
    config, free_port, key, reset_output, serve_in_background, settings_dir, write_settings,
};
use crate::LinkserverWorld;

const INTELLIJ_LINE_42: &str = "idea://open?file=/Users/me/src/TokenController.java&line=42";
const INTELLIJ_SPACES: &str =
    "idea://open?file=/Users/me/My%20Project/Контроллер%20токенов.java&line=3";
const SLACK: &str = "slack://channel?id=C123";

fn parameter(link: &str, name: &str) -> String {
    let query = link.split_once('?').unwrap().1;
    let value = query
        .split('&')
        .find_map(|pair| pair.strip_prefix(&format!("{name}=")))
        .unwrap();
    percent_decode_str(value)
        .decode_utf8()
        .unwrap()
        .into_owned()
}

fn printed(world: &LinkserverWorld) -> String {
    let text = world.printed.out.text();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 1, "one line expected: {text}");
    lines[0].to_owned()
}

#[given(regex = r"^the settings hold port 7777 and the IntelliJ scheme on the allowed list$")]
fn settings_with_intellij(world: &mut LinkserverWorld) {
    write_settings(world, 7777, &["idea"]);
}

#[given(regex = r"^the settings hold a free port and the IntelliJ scheme on the allowed list$")]
fn settings_on_a_free_port(world: &mut LinkserverWorld) {
    write_settings(world, free_port(), &["idea"]);
}

#[given(regex = r"^no settings exist yet$")]
fn no_settings(world: &mut LinkserverWorld) {
    assert!(!settings_dir(world).exists());
}

#[when(
    regex = r"^the user asks for the open link of (an IntelliJ link to a file at line 42|an IntelliJ link whose file path holds spaces written as %20 and non-Latin letters|a Slack link)$"
)]
async fn ask_for_link(world: &mut LinkserverWorld, which: String) {
    let link = match which.as_str() {
        "an IntelliJ link to a file at line 42" => INTELLIJ_LINE_42,
        "a Slack link" => SLACK,
        _ => INTELLIJ_SPACES,
    };
    world.link = Some(link.to_owned());
    reset_output(world);
    let command = config(world).link_command();
    let (mut out, mut err) = (world.printed.out.clone(), world.printed.err.clone());
    world.printed.code = Some(command.run(link, &mut out, &mut err).await);
}

#[then(regex = r"^the command prints one open link to the loopback address on port 7777$")]
fn prints_open_link(world: &mut LinkserverWorld) {
    assert!(printed(world).starts_with("http://127.0.0.1:7777/open?"));
    assert_eq!(world.printed.code, Some(0));
}

#[then(regex = r"^following that link opens exactly that IntelliJ link$")]
async fn link_carries_scheme_link(world: &mut LinkserverWorld) {
    let link = printed(world);
    let path = &link["http://127.0.0.1:7777".len()..];
    let port = free_port();
    write_settings(world, port, &["idea"]);
    serve_in_background(world, port).await;
    let response = reqwest::get(format!("http://127.0.0.1:{port}{path}"))
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(world.fakes.opener.links(), [world.link.clone().unwrap()]);
}

#[then(regex = r"^the printed link holds only printable ASCII characters$")]
fn only_printable_ascii(world: &mut LinkserverWorld) {
    let link = printed(world);
    assert!(link.chars().all(|c| c.is_ascii_graphic()), "{link}");
}

#[then(regex = r"^the settings and a new key are created$")]
fn settings_created(world: &mut LinkserverWorld) {
    assert!(settings_dir(world).join("settings.toml").exists());
    assert_eq!(key(world).len(), 64);
}

#[then(regex = r"^the printed link carries that key$")]
fn carries_key(world: &mut LinkserverWorld) {
    assert_eq!(parameter(&printed(world), "key"), key(world));
}

#[then(regex = r"^the command prints no link$")]
fn prints_no_link(world: &mut LinkserverWorld) {
    assert_eq!(world.printed.out.text(), "");
}

#[then(regex = r"^it says the Slack scheme is not allowed and how to allow it$")]
fn says_not_allowed(world: &mut LinkserverWorld) {
    let text = world.printed.err.text();
    assert!(text.contains("scheme not allowed: slack"), "{text}");
    assert!(
        text.contains("add \"slack\" to allowed_schemes in"),
        "{text}"
    );
}

#[then(regex = r"^it ends with a failure status$")]
fn failure_status(world: &mut LinkserverWorld) {
    assert_ne!(world.printed.code, Some(0));
    assert!(world.printed.code.is_some());
}

#[then(regex = r"^it ends with a success status$")]
fn success_status(world: &mut LinkserverWorld) {
    assert_eq!(world.printed.code, Some(0));
}

#[when(regex = r"^the browser follows the printed link through the server$")]
async fn follow_printed(world: &mut LinkserverWorld) {
    let link = printed(world);
    let port: u16 = link
        .strip_prefix("http://127.0.0.1:")
        .and_then(|rest| rest.split('/').next())
        .unwrap()
        .parse()
        .unwrap();
    serve_in_background(world, port).await;
    let response = reqwest::get(&link).await.unwrap();
    world.answer.status = response.status().as_u16();
    world.answer.body = response.text().await.unwrap();
    assert_eq!(world.answer.status, 200, "{}", world.answer.body);
}
