//! Steps of the `link-sharing` capability on the built binary.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use cucumber::{given, then, when};

use crate::E2eWorld;
use crate::support::{free_port, get, wait_until_answering};

const INTELLIJ_LINE_42: &str = "idea://open?file=/Users/me/src/TokenController.java&line=42";
const INTELLIJ_SPACES: &str =
    "idea://open?file=/Users/me/My%20Project/Контроллер%20токенов.java&line=3";
const SLACK: &str = "slack://channel?id=C123";

fn printed(world: &E2eWorld) -> String {
    let lines: Vec<&str> = world.printed.out.lines().collect();
    assert_eq!(lines.len(), 1, "one line expected: {}", world.printed.out);
    lines[0].to_owned()
}

fn key_parameter(link: &str) -> String {
    let query = link.split_once('?').unwrap().1;
    query
        .split('&')
        .find_map(|pair| pair.strip_prefix("key="))
        .unwrap()
        .to_owned()
}

#[given(regex = r"^the settings hold a free port and the IntelliJ scheme on the allowed list$")]
fn settings_on_a_free_port(world: &mut E2eWorld) {
    let port = free_port();
    world.home.write_settings(port, &["idea"], true);
    world.port = Some(port);
}

#[given(regex = r"^no settings exist yet$")]
fn no_settings(world: &mut E2eWorld) {
    assert!(!world.home.settings_dir().exists());
}

#[when(
    regex = r"^the user asks for the open link of (an IntelliJ link to a file at line 42|an IntelliJ link whose file path holds spaces written as %20 and non-Latin letters|a Slack link)$"
)]
async fn ask_for_link(world: &mut E2eWorld, which: String) {
    let link = match which.as_str() {
        "an IntelliJ link to a file at line 42" => INTELLIJ_LINE_42,
        "a Slack link" => SLACK,
        _ => INTELLIJ_SPACES,
    };
    world.link = Some(link.to_owned());
    let (code, out, err) = world.home.run(&["link", link]).await;
    (world.printed.code, world.printed.out, world.printed.err) = (Some(code), out, err);
}

#[then(regex = r"^the command prints one open link to the loopback address on that port$")]
fn prints_open_link(world: &mut E2eWorld) {
    let prefix = format!("http://127.0.0.1:{}/open?", world.port.unwrap());
    assert!(printed(world).starts_with(&prefix), "{}", world.printed.out);
    assert_eq!(world.printed.code, Some(0));
}

#[then(regex = r"^following that link opens exactly that IntelliJ link$")]
async fn follow_printed(world: &mut E2eWorld) {
    let link = printed(world);
    let port = world.port.unwrap();
    let mut server = world.home.spawn_serve();
    assert!(wait_until_answering(&mut server, Some(port)).await);
    world.server = Some(server);
    let address = format!("127.0.0.1:{port}");
    let path = link.strip_prefix(&format!("http://{address}")).unwrap();
    let (status, body) = get(&address, &address, path).await.unwrap();
    assert_eq!(status, 200, "{body}");
    assert_eq!(
        world.home.calls("open"),
        [vec![world.link.clone().unwrap()]]
    );
}

#[then(regex = r"^the printed link holds only printable ASCII characters$")]
fn only_printable_ascii(world: &mut E2eWorld) {
    let link = printed(world);
    assert!(link.chars().all(|c| c.is_ascii_graphic()), "{link}");
}

#[then(regex = r"^the settings and a new key are created$")]
fn settings_created(world: &mut E2eWorld) {
    assert!(world.home.settings_dir().join("settings.toml").exists());
    assert_eq!(world.home.key().len(), 64);
}

#[then(regex = r"^the printed link carries that key$")]
fn carries_key(world: &mut E2eWorld) {
    assert_eq!(key_parameter(&printed(world)), world.home.key());
}

#[then(regex = r"^the command prints no link$")]
fn prints_no_link(world: &mut E2eWorld) {
    assert_eq!(world.printed.out, "");
}

#[then(regex = r"^it says the Slack scheme is not allowed and how to allow it$")]
fn says_not_allowed(world: &mut E2eWorld) {
    assert!(
        world.printed.err.contains("scheme not allowed: slack"),
        "{}",
        world.printed.err
    );
    assert!(
        world
            .printed
            .err
            .contains("add \"slack\" to allowed_schemes in"),
        "{}",
        world.printed.err
    );
}

#[then(regex = r"^it ends with a (success|failure) status$")]
fn exit_status(world: &mut E2eWorld, which: String) {
    let code = world.printed.code.unwrap();
    assert_eq!(code == 0, which == "success", "status {code}");
}
