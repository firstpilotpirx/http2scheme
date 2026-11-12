//! Steps of the `background-service` capability on the built binary.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::fs;

use cucumber::{given, then, when};
use tokio::io::AsyncBufReadExt as _;

use crate::E2eWorld;

const KEY: &str = "0707070707070707070707070707070707070707070707070707070707070707";
use crate::support::{answers, free_port, sut_bin, wait_until_answering};

async fn run(world: &mut E2eWorld, command: &str) {
    let (code, out, err) = world.home.run(&[command]).await;
    (world.printed.code, world.printed.out, world.printed.err) = (Some(code), out, err);
}

fn settings(world: &mut E2eWorld) -> u16 {
    let port = free_port();
    world.home.write_settings(port, &["idea"], true);
    world.port = Some(port);
    port
}

#[given(regex = r"^the agent is not installed$")]
fn not_installed(world: &mut E2eWorld) {
    assert!(!world.home.agent_file().exists());
}

#[given(regex = r"^the agent is installed(?: and loaded)?$")]
async fn installed(world: &mut E2eWorld) {
    settings(world);
    world.home.write_key(KEY, 0o600);
    run(world, "install").await;
    assert_eq!(world.printed.code, Some(0), "{}", world.printed.err);
}

#[given(regex = r"^the agent is installed and loaded and the server answers$")]
async fn healthy(world: &mut E2eWorld) {
    installed(world).await;
    let mut server = world.home.spawn_serve();
    assert!(wait_until_answering(&mut server, world.port).await);
    world.server = Some(server);
}

#[given(regex = r"^the agent is installed but not loaded and the server does not answer$")]
async fn stopped(world: &mut E2eWorld) {
    installed(world).await;
    world.home.unload();
}

#[when(regex = r"^the user (installs|uninstalls|starts|stops) the agent(?: again)?$")]
async fn user_controls(world: &mut E2eWorld, verb: String) {
    let command = match verb.as_str() {
        "installs" => "install",
        "uninstalls" => "uninstall",
        "starts" => "start",
        _ => "stop",
    };
    run(world, command).await;
}

#[when(regex = r"^the user asks for the status$")]
async fn user_asks_status(world: &mut E2eWorld) {
    run(world, "status").await;
}

#[then(
    regex = r"^the agent definition exists and asks to run at login and to restart after an exit$"
)]
fn definition_keeps_alive(world: &mut E2eWorld) {
    let text = fs::read_to_string(world.home.agent_file()).unwrap();
    assert!(text.contains("<key>RunAtLoad</key>\n    <true/>"), "{text}");
    assert!(text.contains("<key>KeepAlive</key>\n    <true/>"), "{text}");
    assert!(text.contains("<string>serve</string>"), "{text}");
}

#[then(regex = r"^the agent is loaded$")]
fn is_loaded(world: &mut E2eWorld) {
    assert!(world.home.loaded(), "{}", world.printed.err);
}

#[then(regex = r"^the command says the agent is installed and running$")]
fn says_installed(world: &mut E2eWorld) {
    let port = world.port.unwrap_or(7777);
    let expected = format!("installed and running on http://127.0.0.1:{port}");
    assert!(
        world.printed.out.contains(&expected),
        "{}",
        world.printed.out
    );
    assert_eq!(world.printed.code, Some(0));
}

#[then(regex = r"^one agent definition exists, pointing at the current program$")]
fn one_definition(world: &mut E2eWorld) {
    assert_eq!(world.printed.code, Some(0), "{}", world.printed.err);
    let verbs: Vec<String> = world
        .home
        .calls("launchctl")
        .iter()
        .filter_map(|call| call.first().cloned())
        .filter(|verb| verb != "print")
        .collect();
    assert!(
        verbs.ends_with(&["bootout".to_owned(), "bootstrap".to_owned()]),
        "{verbs:?}"
    );
    let folder = world.home.agent_file().parent().unwrap().to_path_buf();
    assert_eq!(fs::read_dir(folder).unwrap().count(), 1);
    let text = fs::read_to_string(world.home.agent_file()).unwrap();
    let program = fs::canonicalize(sut_bin()).unwrap();
    assert!(
        text.contains(&format!("<string>{}</string>", program.display())),
        "{text}"
    );
}

#[then(regex = r"^the agent is unloaded and its definition (is gone|remains)$")]
fn unloaded(world: &mut E2eWorld, definition: String) {
    assert!(!world.home.loaded());
    assert_eq!(world.home.agent_file().exists(), definition == "remains");
}

#[then(regex = r"^the settings and the key remain$")]
fn settings_remain(world: &mut E2eWorld) {
    assert!(world.home.settings_text().contains("port = "));
    assert_eq!(world.home.key(), KEY);
}

#[then(regex = r"^the command succeeds and says nothing was installed$")]
fn says_nothing_installed(world: &mut E2eWorld) {
    assert_eq!(world.printed.code, Some(0));
    assert!(
        world.printed.out.contains("nothing was installed"),
        "{}",
        world.printed.out
    );
}

#[then(regex = r"^the command fails and says to install the agent first$")]
fn says_install_first(world: &mut E2eWorld) {
    assert_eq!(world.printed.code, Some(1));
    assert!(
        world.printed.err.contains("http2scheme install"),
        "{}",
        world.printed.err
    );
}

#[then(
    regex = r"^the command shows (installed|not installed), (loaded|not loaded) and (answering|not answering)$"
)]
fn shows_status(world: &mut E2eWorld, installed: String, loaded: String, answering: String) {
    let mark = |s: &str| ["[ok]", "[fail]"][usize::from(s.starts_with("not "))];
    let line = |s: &str| format!("{} {s}", mark(s));
    let expected = [line(&installed), line(&loaded), line(&answering)].join("\n");
    let text = &world.printed.out;
    assert!(
        text.starts_with(&format!("{expected} on 127.0.0.1:")),
        "{text}"
    );
}

#[given(regex = r"^the configured port is free$")]
fn port_free(world: &mut E2eWorld) {
    settings(world);
}

#[given(regex = r"^another program holds the configured port$")]
fn port_held(world: &mut E2eWorld) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    world.held = Some(listener);
    world.home.write_settings(port, &["idea"], true);
    world.port = Some(port);
}

#[when(regex = r"^the user runs the server in the foreground$")]
async fn runs_foreground(world: &mut E2eWorld) {
    if world.held.is_some() {
        run(world, "serve").await;
        return;
    }
    let mut server = world.home.spawn_serve();
    let stdout = server.stdout.take().unwrap();
    let mut line = String::new();
    tokio::io::BufReader::new(stdout)
        .read_line(&mut line)
        .await
        .unwrap();
    world.printed.out = line;
    world.server = Some(server);
}

#[then(regex = r"^it prints the loopback address with the port$")]
fn prints_address(world: &mut E2eWorld) {
    let port = world.port.unwrap();
    let expected = format!("listening on http://127.0.0.1:{port}");
    assert!(
        world.printed.out.contains(&expected),
        "{}",
        world.printed.out
    );
}

#[then(regex = r"^the health address answers$")]
async fn health_answers(world: &mut E2eWorld) {
    assert!(answers(world.port.unwrap()).await);
}

#[then(regex = r"^it fails and says the port is taken$")]
fn says_taken(world: &mut E2eWorld) {
    let port = world.port.unwrap();
    assert_eq!(world.printed.code, Some(1));
    assert!(
        world.printed.err.contains(&format!("port {port} is taken")),
        "{}",
        world.printed.err
    );
}
