//! Steps of the `background-service` capability, driven through the agent and serve commands.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::fs;

use cucumber::{given, then, when};

use super::home::{
    LABEL, PROGRAM, agents_dir, answers, config, free_port, reset_output, serve_in_background,
    serve_once, settings_dir, write_key, write_settings,
};
use crate::LinkserverWorld;

fn definition(world: &mut LinkserverWorld) -> std::path::PathBuf {
    agents_dir(world).join(format!("{LABEL}.plist"))
}

async fn install(world: &mut LinkserverWorld) {
    reset_output(world);
    let command = config(world).install_command().await;
    let (mut out, mut err) = (world.printed.out.clone(), world.printed.err.clone());
    world.printed.code = Some(command.run(&mut out, &mut err).await);
}

#[given(regex = r"^the agent is not installed$")]
fn not_installed(world: &mut LinkserverWorld) {
    assert!(!definition(world).exists());
}

#[given(regex = r"^the agent is installed(?: and loaded)?$")]
async fn installed(world: &mut LinkserverWorld) {
    write_settings(world, free_port(), &["idea"]);
    write_key(world, 0o600);
    install(world).await;
    assert!(world.fakes.supervisor.is_loaded_now());
}

#[given(regex = r"^the agent is installed and loaded and the server answers$")]
async fn healthy(world: &mut LinkserverWorld) {
    install(world).await;
    world.fakes.health.set_answering(true);
}

#[given(regex = r"^the agent is installed but not loaded and the server does not answer$")]
async fn stopped(world: &mut LinkserverWorld) {
    install(world).await;
    world.fakes.supervisor.set_loaded(false);
    world.fakes.health.set_answering(false);
}

/// The program an earlier version installed from, which a second install replaces.
const OLD_PROGRAM: &str = "/old/http2scheme";

#[when(regex = r"^the user installs the agent( again)?$")]
async fn user_installs(world: &mut LinkserverWorld, again: String) {
    if !again.is_empty() {
        let text = fs::read_to_string(definition(world)).unwrap();
        fs::write(definition(world), text.replace(PROGRAM, OLD_PROGRAM)).unwrap();
    }
    install(world).await;
}

#[when(regex = r"^the user uninstalls the agent$")]
async fn user_uninstalls(world: &mut LinkserverWorld) {
    reset_output(world);
    let command = config(world).uninstall_command();
    let (mut out, mut err) = (world.printed.out.clone(), world.printed.err.clone());
    world.printed.code = Some(command.run(&mut out, &mut err).await);
}

#[when(regex = r"^the user (starts|stops) the agent$")]
async fn user_starts_or_stops(world: &mut LinkserverWorld, verb: String) {
    reset_output(world);
    let root = config(world);
    let (mut out, mut err) = (world.printed.out.clone(), world.printed.err.clone());
    let code = if verb == "starts" {
        root.start_command().run(&mut out, &mut err).await
    } else {
        root.stop_command().run(&mut out, &mut err).await
    };
    world.printed.code = Some(code);
}

#[when(regex = r"^the user asks for the status$")]
async fn user_asks_status(world: &mut LinkserverWorld) {
    reset_output(world);
    let command = config(world).status_command().await;
    let (mut out, mut err) = (world.printed.out.clone(), world.printed.err.clone());
    world.printed.code = Some(command.run(&mut out, &mut err).await);
}

#[then(
    regex = r"^the agent definition exists and asks to run at login and to restart after an exit$"
)]
fn definition_keeps_alive(world: &mut LinkserverWorld) {
    let text = fs::read_to_string(definition(world)).unwrap();
    assert!(text.contains("<key>RunAtLoad</key>\n    <true/>"), "{text}");
    assert!(text.contains("<key>KeepAlive</key>\n    <true/>"), "{text}");
}

#[then(regex = r"^the agent is loaded$")]
fn is_loaded(world: &mut LinkserverWorld) {
    assert!(world.fakes.supervisor.is_loaded_now());
}

#[then(regex = r"^the command says the agent is installed and running$")]
fn says_installed(world: &mut LinkserverWorld) {
    let text = world.printed.out.text();
    assert!(
        text.contains("installed and running on http://127.0.0.1:7777"),
        "{text}"
    );
    assert_eq!(world.printed.code, Some(0));
}

#[then(regex = r"^one agent definition exists, pointing at the current program$")]
fn one_definition(world: &mut LinkserverWorld) {
    assert_eq!(world.printed.code, Some(0), "{}", world.printed.err.text());
    let entries = fs::read_dir(agents_dir(world)).unwrap().count();
    assert_eq!(entries, 1);
    let text = fs::read_to_string(definition(world)).unwrap();
    assert!(
        text.contains(&format!("<string>{PROGRAM}</string>")),
        "{text}"
    );
    assert!(!text.contains(OLD_PROGRAM), "{text}");
}

#[then(regex = r"^the agent is unloaded and its definition is gone$")]
fn unloaded_and_gone(world: &mut LinkserverWorld) {
    assert!(!world.fakes.supervisor.is_loaded_now());
    assert!(!definition(world).exists());
}

#[then(regex = r"^the settings and the key remain$")]
fn settings_remain(world: &mut LinkserverWorld) {
    let dir = settings_dir(world);
    assert!(dir.join("settings.toml").exists());
    assert!(dir.join("key").exists());
}

#[then(regex = r"^the command succeeds and says nothing was installed$")]
fn says_nothing_installed(world: &mut LinkserverWorld) {
    assert_eq!(world.printed.code, Some(0));
    assert!(world.printed.out.text().contains("nothing was installed"));
}

#[then(regex = r"^the agent is unloaded and its definition remains$")]
fn unloaded_and_kept(world: &mut LinkserverWorld) {
    assert!(!world.fakes.supervisor.is_loaded_now());
    assert!(definition(world).exists());
}

#[then(regex = r"^the command fails and says to install the agent first$")]
fn says_install_first(world: &mut LinkserverWorld) {
    assert_eq!(world.printed.code, Some(1));
    assert!(world.printed.err.text().contains("http2scheme install"));
}

#[then(
    regex = r"^the command shows (installed|not installed), (loaded|not loaded) and (answering|not answering)$"
)]
fn shows_status(world: &mut LinkserverWorld, installed: String, loaded: String, answering: String) {
    let mark = |s: &str| ["[ok]", "[fail]"][usize::from(s.starts_with("not "))];
    let line = |s: &str| format!("{} {s}", mark(s));
    let expected = [line(&installed), line(&loaded), line(&answering)].join("\n");
    let text = world.printed.out.text();
    assert!(
        text.starts_with(&format!("{expected} on 127.0.0.1:")),
        "{text}"
    );
}

#[given(regex = r"^the configured port is free$")]
fn port_free(world: &mut LinkserverWorld) {
    let port = free_port();
    write_settings(world, port, &["idea"]);
    world.port = Some(port);
}

#[given(regex = r"^another program holds the configured port$")]
fn port_held(world: &mut LinkserverWorld) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    world.held = Some(listener);
    write_settings(world, port, &["idea"]);
    world.port = Some(port);
}

#[when(regex = r"^the user runs the server in the foreground$")]
async fn runs_foreground(world: &mut LinkserverWorld) {
    let port = world.port.unwrap();
    if world.held.is_some() {
        serve_once(world).await;
    } else {
        serve_in_background(world, port).await;
    }
}

#[then(regex = r"^it prints the loopback address with the port$")]
fn prints_address(world: &mut LinkserverWorld) {
    let port = world.port.unwrap();
    let text = world.printed.out.text();
    assert!(
        text.contains(&format!("listening on http://127.0.0.1:{port}")),
        "{text}"
    );
}

#[then(regex = r"^the health address answers$")]
async fn health_answers(world: &mut LinkserverWorld) {
    assert!(answers(world.port.unwrap()).await);
}

#[then(regex = r"^it fails and says the port is taken$")]
fn says_taken(world: &mut LinkserverWorld) {
    let port = world.port.unwrap();
    assert_eq!(world.printed.code, Some(1));
    assert!(
        world
            .printed
            .err
            .text()
            .contains(&format!("port {port} is taken")),
        "{}",
        world.printed.err.text()
    );
}
