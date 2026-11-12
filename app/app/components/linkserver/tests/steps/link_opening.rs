//! Steps of the `link-opening` capability, driven through the real HTTP adapter and use case.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::net::TcpListener as StdListener;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use cucumber::{given, then, when};
use linkserver::adapters::primary::linkopening::http::axum_config::AxumConfig;
use linkserver::adapters::primary::linkopening::http::open_link_response::OpenLinkResponse;
use linkserver::core::domain::access_key_value_object::AccessKeyValueObject;
use linkserver::core::domain::port_value_object::PortValueObject;
use linkserver::core::domain::scheme_allowlist_policy::SchemeAllowlistPolicy;
use linkserver::core::domain::scheme_name_value_object::SchemeNameValueObject;
use linkserver::core::domain::server_settings_value_object::ServerSettingsValueObject;
use linkserver::linkserver_config::LinkserverConfig;
use linkserver::open_link_ports::OpenLinkPorts;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

use crate::LinkserverWorld;
use crate::fakes::{FakeCloser, FakeOpener};

const KEY: &str = "0707070707070707070707070707070707070707070707070707070707070707";
const WRONG_KEY: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const INTELLIJ_LINE_17: &str = "idea://open?file=/src/TokenController.java&line=17";
const INTELLIJ: &str = "idea://open?file=/src/Main.java&line=1";
const SPECIAL: &str = "idea://open?file=/My%20Project/\"q\";'x'.java&line=1";

fn settings(world: &LinkserverWorld, port: PortValueObject) -> ServerSettingsValueObject {
    let schemes = world
        .allowed
        .iter()
        .map(|name| SchemeNameValueObject::parse(name).unwrap())
        .collect();
    ServerSettingsValueObject::new(port, SchemeAllowlistPolicy::new(schemes), world.close_tab)
}

async fn start(world: &mut LinkserverWorld) -> u16 {
    if let Some(port) = world.port {
        return port;
    }
    let port = StdListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let port_value = PortValueObject::new(port).unwrap();
    let hosts = vec!["127.0.0.1".to_owned(), "localhost".to_owned()];
    let http = AxumConfig::new("127.0.0.1".to_owned(), hosts, port_value);
    let ports = OpenLinkPorts {
        opener: world.fakes.opener.clone(),
        closer: world.fakes.closer.clone(),
        order: world.fakes.order.clone(),
        attempts: world.fakes.log.clone(),
    };
    let pages = OpenLinkResponse::new(
        "/home/settings.toml".to_owned(),
        "/home/requests.log".to_owned(),
    );
    let key = AccessKeyValueObject::parse_hex(KEY).unwrap();
    let router =
        LinkserverConfig::open_link_router(settings(world, port_value), key, ports, &http, pages);
    let listener = http.listen().await.unwrap();
    tokio::spawn(AxumConfig::serve(listener, router, std::future::pending()));
    world.port = Some(port);
    port
}

fn open_path(key: Option<&str>, link: Option<&str>) -> String {
    let mut parameters = Vec::new();
    if let Some(key) = key {
        parameters.push(format!("key={key}"));
    }
    if let Some(link) = link {
        parameters.push(format!(
            "url={}",
            utf8_percent_encode(link, NON_ALPHANUMERIC)
        ));
    }
    format!("/open?{}", parameters.join("&"))
}

async fn request(world: &mut LinkserverWorld, path: &str, host: Option<String>) {
    let port = start(world).await;
    let mut builder = reqwest::Client::new().get(format!("http://127.0.0.1:{port}{path}"));
    if let Some(host) = host {
        builder = builder.header("host", host);
    }
    let response = builder.send().await.unwrap();
    world.answer.status = response.status().as_u16();
    world.answer.body = response.text().await.unwrap();
}

async fn follow(world: &mut LinkserverWorld, key: Option<&str>, link: Option<&str>) {
    world.link = link.map(str::to_owned);
    request(world, &open_path(key, link), None).await;
}

#[given(regex = r"^the server runs with (?:only )?the IntelliJ scheme on the allowed list$")]
fn intellij_only(world: &mut LinkserverWorld) {
    world.allowed = vec!["idea".to_owned()];
}

#[given(regex = r"^the server runs(?: with tab closing on)?$")]
fn server_runs(world: &mut LinkserverWorld) {
    world.close_tab = true;
}

#[given(regex = r"^the server runs with tab closing on and the browser refuses to close tabs$")]
fn closer_refuses(world: &mut LinkserverWorld) {
    world.close_tab = true;
    world.fakes.closer = Arc::new(FakeCloser::failing());
    world.close_must_be_tried = true;
}

#[given(regex = r"^the settings turn tab closing off$")]
fn tab_closing_off(world: &mut LinkserverWorld) {
    world.close_tab = false;
}

#[given(regex = r"^the settings list https and file as allowed schemes$")]
fn web_and_file_allowed(world: &mut LinkserverWorld) {
    world.allowed = vec!["idea".to_owned(), "https".to_owned(), "file".to_owned()];
}

#[given(regex = r"^the server runs and the Mac's link opener fails for the IntelliJ link$")]
fn opener_fails(world: &mut LinkserverWorld) {
    world.fakes.opener = Arc::new(FakeOpener::failing());
}

#[when(
    regex = r"^the browser follows an open link with the right key and an IntelliJ link to a file at line 17$"
)]
async fn follow_line_17(world: &mut LinkserverWorld) {
    follow(world, Some(KEY), Some(INTELLIJ_LINE_17)).await;
}

#[when(
    regex = r"^the browser follows an open link whose IntelliJ link names a file path with spaces written as %20, quotes and a semicolon$"
)]
async fn follow_special(world: &mut LinkserverWorld) {
    follow(world, Some(KEY), Some(SPECIAL)).await;
}

#[when(
    regex = r"^the browser follows a keyed open link to (an IntelliJ link|that IntelliJ link|a Slack link|a web page|a local file)$"
)]
async fn follow_keyed(world: &mut LinkserverWorld, target: String) {
    let link = match target.as_str() {
        "a Slack link" => "slack://channel?id=C123",
        "a web page" => "https://example.com/",
        "a local file" => "file:///etc/hosts",
        _ => INTELLIJ,
    };
    follow(world, Some(KEY), Some(link)).await;
}

#[when(regex = r"^the browser follows an open link with a wrong key$")]
async fn follow_wrong_key(world: &mut LinkserverWorld) {
    follow(world, Some(WRONG_KEY), Some(INTELLIJ)).await;
}

#[when(regex = r"^the browser follows an open link without a key$")]
async fn follow_without_key(world: &mut LinkserverWorld) {
    follow(world, None, Some(INTELLIJ)).await;
}

#[when(regex = r"^the browser follows a keyed open link whose scheme link is (.+)$")]
async fn follow_malformed(world: &mut LinkserverWorld, bad: String) {
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
    follow(world, Some(KEY), link).await;
}

#[when(
    regex = r"^a keyed open link arrives under a host name other than the loopback address or localhost$"
)]
async fn follow_foreign_host(world: &mut LinkserverWorld) {
    let port = start(world).await;
    request(
        world,
        &open_path(Some(KEY), Some(INTELLIJ)),
        Some(format!("evil.example:{port}")),
    )
    .await;
}

#[when(regex = r"^the browser asks for an address the server does not serve$")]
async fn unknown_address(world: &mut LinkserverWorld) {
    request(world, "/nope", None).await;
}

#[when(regex = r"^a client asks the health address$")]
async fn health(world: &mut LinkserverWorld) {
    request(world, "/health", None).await;
}

#[then(
    regex = r"^the Mac opens (?:(?:exactly )?that IntelliJ link|the IntelliJ link with exactly those characters, as one link)$"
)]
fn opens_that_link(world: &mut LinkserverWorld) {
    assert_eq!(world.fakes.opener.links(), [world.link.clone().unwrap()]);
}

#[then(regex = r"^no command other than the link opener runs$")]
fn only_the_opener_runs(world: &mut LinkserverWorld) {
    assert_eq!(world.fakes.opener.links().len(), 1);
    assert!(
        world
            .fakes
            .closer
            .ports()
            .iter()
            .all(|port| Some(*port) == world.port)
    );
}

#[then(regex = r#"^the browser shows the "(Opened|Wrong key|Not found)" page$"#)]
fn shows_page(world: &mut LinkserverWorld, title: String) {
    let status = match title.as_str() {
        "Opened" => 200,
        "Wrong key" => 403,
        _ => 404,
    };
    assert_eq!(world.answer.status, status, "{}", world.answer.body);
    assert!(world.answer.body.contains(&title), "{}", world.answer.body);
    if world.close_must_be_tried {
        assert_eq!(world.fakes.closer.ports(), [world.port.unwrap()]);
    }
}

#[then(regex = r"^the server asks the browsers to close the tabs that show its open address$")]
fn closes_tabs(world: &mut LinkserverWorld) {
    assert_eq!(world.fakes.closer.ports(), [world.port.unwrap()]);
}

#[then(regex = r"^the server asks no browser to close a tab$")]
fn closes_no_tab(world: &mut LinkserverWorld) {
    assert_eq!(world.fakes.closer.ports().len(), 0);
}

#[then(regex = r"^nothing opens$")]
fn nothing_opens(world: &mut LinkserverWorld) {
    assert_eq!(world.fakes.opener.links().len(), 0);
}

#[then(regex = r"^the browser shows a page saying the Slack scheme is not allowed$")]
fn slack_not_allowed(world: &mut LinkserverWorld) {
    assert_eq!(world.answer.status, 400);
    assert!(
        world.answer.body.contains("Scheme not allowed: slack"),
        "{}",
        world.answer.body
    );
}

#[then(regex = r"^the browser shows a page saying the scheme is not allowed$")]
fn scheme_not_allowed(world: &mut LinkserverWorld) {
    assert_eq!(world.answer.status, 400);
    assert!(
        world.answer.body.contains("Scheme not allowed"),
        "{}",
        world.answer.body
    );
}

#[then(regex = r"^the browser shows a page saying the link is malformed$")]
fn malformed(world: &mut LinkserverWorld) {
    assert_eq!(world.answer.status, 400);
    assert!(
        world.answer.body.contains("Malformed link"),
        "{}",
        world.answer.body
    );
}

#[then(regex = r"^the server answers that the host is not served$")]
fn host_not_served(world: &mut LinkserverWorld) {
    assert_eq!(world.answer.status, 421);
}

#[then(regex = r"^the server answers that it is running$")]
fn running(world: &mut LinkserverWorld) {
    assert_eq!(
        (world.answer.status, world.answer.body.as_str()),
        (200, "ok")
    );
}

#[then(regex = r"^the browser shows a page saying the app could not be opened$")]
fn could_not_open(world: &mut LinkserverWorld) {
    assert_eq!(world.answer.status, 502);
    assert!(
        world.answer.body.contains("Could not open the app"),
        "{}",
        world.answer.body
    );
}

#[then(regex = r#"^the request log ends with (?:a line holding )?the result "(.+)"$"#)]
fn log_ends_with_result(world: &mut LinkserverWorld, result: String) {
    let events = world.fakes.log.events();
    assert_eq!(events.last().unwrap().outcome().label(), result);
}

#[then(
    regex = r#"^the request log ends with a line holding the time, the result "(.+)" and that IntelliJ link$"#
)]
fn log_ends_with_line(world: &mut LinkserverWorld, result: String) {
    let events = world.fakes.log.events();
    let last = events.last().unwrap();
    assert_eq!(last.link_text(), world.link.clone().unwrap());
    assert_eq!(last.outcome().label(), result);
    let age = SystemTime::now().duration_since(last.at()).unwrap();
    assert!(age < Duration::from_secs(60), "{age:?}");
}

#[then(regex = r"^neither the secret key nor the wrong key appears in the log$")]
fn no_key_in_log(world: &mut LinkserverWorld) {
    for event in world.fakes.log.events() {
        let text = format!("{event:?}");
        assert!(!text.contains(KEY) && !text.contains(WRONG_KEY), "{text}");
    }
}
