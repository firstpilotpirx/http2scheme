//! Steps of the `link-opening` capability on the built binary.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::time::Duration;

use cucumber::{given, then, when};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

use crate::E2eWorld;
use crate::support::{assert_loopback_only, free_port, get, wait_until_answering};

const WRONG_KEY: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const INTELLIJ_LINE_17: &str = "idea://open?file=/src/TokenController.java&line=17";
const INTELLIJ: &str = "idea://open?file=/src/Main.java&line=1";
const SPECIAL: &str = "idea://open?file=/My%20Project/\"q\";'x'.java&line=1";

/// Writes the settings and starts the server once; returns its port.
pub async fn start(world: &mut E2eWorld) -> u16 {
    if let Some(port) = world.port
        && world.server.is_some()
    {
        return port;
    }
    let port = free_port();
    let allowed: Vec<&str> = world.allowed.iter().map(String::as_str).collect();
    world.home.write_settings(port, &allowed, world.close_tab);
    let mut child = world.home.spawn_serve();
    assert!(wait_until_answering(&mut child, Some(port)).await);
    world.server = Some(child);
    world.port = Some(port);
    port
}

fn open_path(key: Option<&str>, link: Option<&str>) -> String {
    let mut parameters = Vec::new();
    if let Some(key) = key {
        parameters.push(format!("key={key}"));
    }
    if let Some(link) = link {
        let link = utf8_percent_encode(link, NON_ALPHANUMERIC);
        parameters.push(format!("url={link}"));
    }
    format!("/open?{}", parameters.join("&"))
}

async fn request(world: &mut E2eWorld, path: &str, host: Option<String>) {
    let port = start(world).await;
    let address = format!("127.0.0.1:{port}");
    let host = host.unwrap_or_else(|| address.clone());
    (world.answer.status, world.answer.body) = get(&address, &host, path).await.unwrap();
}

async fn follow(world: &mut E2eWorld, right_key: Option<bool>, link: Option<&str>) {
    start(world).await;
    world.link = link.map(str::to_owned);
    let key = match right_key {
        Some(true) => Some(world.home.key()),
        Some(false) => Some(WRONG_KEY.to_owned()),
        None => None,
    };
    request(world, &open_path(key.as_deref(), link), None).await;
}

#[given(regex = r"^the server runs with (?:only )?the IntelliJ scheme on the allowed list$")]
fn intellij_only(world: &mut E2eWorld) {
    world.allowed = vec!["idea".to_owned()];
}

#[given(regex = r"^the server runs(?: with tab closing on)?$")]
fn server_runs(world: &mut E2eWorld) {
    world.close_tab = true;
}

#[given(regex = r"^the server runs with tab closing on and the browser refuses to close tabs$")]
fn closer_refuses(world: &mut E2eWorld) {
    world.home.make_fail("osascript");
}

#[given(regex = r"^the settings turn tab closing off$")]
fn tab_closing_off(world: &mut E2eWorld) {
    world.close_tab = false;
}

#[given(regex = r"^the settings list https and file as allowed schemes$")]
fn web_and_file_allowed(world: &mut E2eWorld) {
    world.allowed = vec!["idea".to_owned(), "https".to_owned(), "file".to_owned()];
}

#[given(regex = r"^the server runs and the Mac's link opener fails for the IntelliJ link$")]
fn opener_fails(world: &mut E2eWorld) {
    world.home.make_fail("open");
}

#[when(
    regex = r"^the browser follows an open link with the right key and an IntelliJ link to a file at line 17$"
)]
async fn follow_line_17(world: &mut E2eWorld) {
    follow(world, Some(true), Some(INTELLIJ_LINE_17)).await;
}

#[when(
    regex = r"^the browser follows an open link whose IntelliJ link names a file path with spaces written as %20, quotes and a semicolon$"
)]
async fn follow_special(world: &mut E2eWorld) {
    follow(world, Some(true), Some(SPECIAL)).await;
}

#[when(
    regex = r"^the browser follows a keyed open link to (an IntelliJ link|that IntelliJ link|a Slack link|a web page|a local file)$"
)]
async fn follow_keyed(world: &mut E2eWorld, target: String) {
    let link = match target.as_str() {
        "a Slack link" => "slack://channel?id=C123",
        "a web page" => "https://example.com/",
        "a local file" => "file:///etc/hosts",
        _ => INTELLIJ,
    };
    follow(world, Some(true), Some(link)).await;
}

#[when(regex = r"^the browser follows an open link with a wrong key$")]
async fn follow_wrong_key(world: &mut E2eWorld) {
    follow(world, Some(false), Some(INTELLIJ)).await;
}

#[when(regex = r"^the browser follows an open link without a key$")]
async fn follow_without_key(world: &mut E2eWorld) {
    follow(world, None, Some(INTELLIJ)).await;
}

#[when(regex = r"^the browser follows a keyed open link whose scheme link is (.+)$")]
async fn follow_malformed(world: &mut E2eWorld, bad: String) {
    let long = format!("idea://{}", "a".repeat(4100));
    let link = match bad.as_str() {
        "missing" => None,
        "empty" => Some(""),
        "without a scheme" => Some("/src/Main.java"),
        "with a line break inside" => Some("idea://open?file=a\nb"),
        "with a raw space" => Some("idea://open?file=a b"),
        "longer than 4096 characters" => Some(long.as_str()),
        other => panic!("unknown bad link: {other}"),
    };
    follow(world, Some(true), link).await;
}

#[when(
    regex = r"^a keyed open link arrives under a host name other than the loopback address or localhost$"
)]
async fn follow_foreign_host(world: &mut E2eWorld) {
    let port = start(world).await;
    let path = open_path(Some(&world.home.key()), Some(INTELLIJ));
    request(world, &path, Some(format!("evil.example:{port}"))).await;
}

#[when(regex = r"^the browser asks for an address the server does not serve$")]
async fn unknown_address(world: &mut E2eWorld) {
    request(world, "/nope", None).await;
}

#[when(regex = r"^a client asks the health address$")]
async fn health(world: &mut E2eWorld) {
    request(world, "/health", None).await;
}

#[when(regex = r"^a request comes to the Mac's network address instead of the loopback address$")]
async fn network_request(world: &mut E2eWorld) {
    let port = start(world).await;
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    let address = socket
        .connect("192.0.2.1:9")
        .and_then(|()| socket.local_addr())
        .map(|local| local.ip())
        .ok()
        .filter(|ip| !ip.is_loopback() && !ip.is_unspecified());
    assert_loopback_only(port);
    world.answer.refused = match address {
        Some(ip) => {
            let target = format!("{ip}:{port}");
            get(&target, &target, "/health").await.is_err()
        }
        None => true,
    };
}

#[then(regex = r"^the connection is refused$")]
fn connection_refused(world: &mut E2eWorld) {
    assert!(world.answer.refused);
}

#[then(
    regex = r"^the Mac opens (?:(?:exactly )?that IntelliJ link|the IntelliJ link with exactly those characters, as one link)$"
)]
fn opens_that_link(world: &mut E2eWorld) {
    assert_eq!(
        world.home.calls("open"),
        [vec![world.link.clone().unwrap()]]
    );
}

#[then(regex = r"^no command other than the link opener runs$")]
async fn only_the_opener_runs(world: &mut E2eWorld) {
    tokio::time::sleep(Duration::from_millis(600)).await;
    assert_eq!(world.home.calls("open").len(), 1);
    assert_eq!(world.home.calls("launchctl").len(), 0);
    for call in world.home.calls("osascript") {
        assert_eq!(call[0], "-e");
        assert!(!call[1].contains(world.link.as_deref().unwrap()));
    }
}

#[then(regex = r#"^the browser shows the "(Opened|Wrong key|Not found)" page$"#)]
fn shows_page(world: &mut E2eWorld, title: String) {
    let status = match title.as_str() {
        "Opened" => 200,
        "Wrong key" => 403,
        _ => 404,
    };
    assert_eq!(world.answer.status, status, "{}", world.answer.body);
    assert!(world.answer.body.contains(&title), "{}", world.answer.body);
}

#[then(regex = r"^the server asks the browsers to close the tabs that show its open address$")]
async fn closes_tabs(world: &mut E2eWorld) {
    let prefix = format!("http://127.0.0.1:{}/open?", world.port.unwrap());
    for _ in 0..200 {
        if let Some(call) = world.home.calls("osascript").first() {
            assert!(call[1].contains(&prefix), "{}", call[1]);
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("no browser was asked to close a tab");
}

#[then(regex = r"^the server asks no browser to close a tab$")]
async fn closes_no_tab(world: &mut E2eWorld) {
    tokio::time::sleep(Duration::from_millis(600)).await;
    assert_eq!(world.home.calls("osascript").len(), 0);
}

#[then(regex = r"^nothing opens$")]
fn nothing_opens(world: &mut E2eWorld) {
    assert_eq!(world.home.calls("open").len(), 0);
}

#[then(regex = r"^the browser shows a page saying the (Slack scheme|scheme) is not allowed$")]
fn not_allowed(world: &mut E2eWorld, which: String) {
    let title = if which == "scheme" {
        "Scheme not allowed"
    } else {
        "Scheme not allowed: slack"
    };
    assert_eq!(world.answer.status, 400);
    assert!(world.answer.body.contains(title), "{}", world.answer.body);
}

#[then(regex = r"^the browser shows a page saying the link is malformed$")]
fn malformed(world: &mut E2eWorld) {
    assert_eq!(world.answer.status, 400);
    assert!(
        world.answer.body.contains("Malformed link"),
        "{}",
        world.answer.body
    );
}

#[then(regex = r"^the server answers that the host is not served$")]
fn host_not_served(world: &mut E2eWorld) {
    assert_eq!(world.answer.status, 421);
}

#[then(regex = r"^the server answers that it is running$")]
fn running(world: &mut E2eWorld) {
    assert_eq!(
        (world.answer.status, world.answer.body.as_str()),
        (200, "ok")
    );
}

#[then(regex = r"^the browser shows a page saying the app could not be opened$")]
fn could_not_open(world: &mut E2eWorld) {
    assert_eq!(world.answer.status, 502);
    assert!(
        world.answer.body.contains("Could not open the app"),
        "{}",
        world.answer.body
    );
}

fn last_log_line(world: &E2eWorld) -> String {
    world.home.request_log().lines().last().unwrap().to_owned()
}

#[then(regex = r#"^the request log ends with (?:a line holding )?the result "(.+)"$"#)]
async fn log_ends_with_result(world: &mut E2eWorld, result: String) {
    if world.home.fails("osascript") {
        for _ in 0..200 {
            if !world.home.calls("osascript").is_empty() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        assert!(
            !world.home.calls("osascript").is_empty(),
            "no tab close was tried"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let line = last_log_line(world);
    assert!(line.contains(&result), "{line}");
}

#[then(
    regex = r#"^the request log ends with a line holding the time, the result "(.+)" and that IntelliJ link$"#
)]
fn log_ends_with_line(world: &mut E2eWorld, result: String) {
    let line = last_log_line(world);
    let fields: Vec<&str> = line.split('\t').collect();
    assert_eq!(fields.len(), 3, "{line}");
    assert!(
        fields[0].starts_with("20") && fields[0].ends_with('Z'),
        "{line}"
    );
    assert_eq!(fields[1], result, "{line}");
    assert_eq!(fields[2], world.link.as_deref().unwrap(), "{line}");
}

#[then(regex = r"^neither the secret key nor the wrong key appears in the log$")]
fn no_key_in_log(world: &mut E2eWorld) {
    let log = world.home.request_log();
    assert!(!log.contains(&world.home.key()), "{log}");
    assert!(!log.contains(WRONG_KEY), "{log}");
}
