//! A temporary home and the component built over it, with fakes for the system services.

use std::fs;
use std::io::Write;
use std::net::TcpListener as StdListener;
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use linkserver::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use linkserver::linkserver_config::{LinkserverConfig, LinkserverPorts, LinkserverSetup};

use crate::LinkserverWorld;

/// The agent label of the scenarios.
pub const LABEL: &str = "io.example.http2scheme";
/// The program the agent runs in the scenarios.
pub const PROGRAM: &str = "/opt/example/bin/http2scheme";
/// A valid key for files written by hand.
pub const KEY: &str = "0707070707070707070707070707070707070707070707070707070707070707";

/// A writer that keeps what it gets for later reading.
#[derive(Debug, Clone, Default)]
pub struct SharedBuffer(Arc<Mutex<Vec<u8>>>);

impl SharedBuffer {
    /// What was written so far.
    pub fn text(&self) -> String {
        let bytes = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

impl Write for SharedBuffer {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// A port no program holds now.
pub fn free_port() -> u16 {
    StdListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

/// The scenario's home, created on first use.
pub fn home(world: &mut LinkserverWorld) -> PathBuf {
    world
        .home
        .get_or_insert_with(|| tempfile::tempdir().unwrap())
        .path()
        .to_path_buf()
}

/// The folder of the settings and the key.
pub fn settings_dir(world: &mut LinkserverWorld) -> PathBuf {
    home(world).join("settings")
}

/// The folder of the agent definitions.
pub fn agents_dir(world: &mut LinkserverWorld) -> PathBuf {
    home(world).join("agents")
}

/// Writes the settings file by hand.
pub fn write_settings(world: &mut LinkserverWorld, port: u16, schemes: &[&str]) {
    let dir = settings_dir(world);
    fs::create_dir_all(&dir).unwrap();
    let schemes: Vec<String> = schemes.iter().map(|name| format!("\"{name}\"")).collect();
    let text = format!(
        "port = {port}\nallowed_schemes = [{}]\nclose_tab = true\n",
        schemes.join(", ")
    );
    fs::write(dir.join("settings.toml"), text).unwrap();
}

/// Writes the key file by hand with the given permission bits.
pub fn write_key(world: &mut LinkserverWorld, mode: u32) {
    let path = settings_dir(world).join("key");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, KEY).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(mode)).unwrap();
}

/// The settings file text.
pub fn settings_text(world: &mut LinkserverWorld) -> String {
    fs::read_to_string(settings_dir(world).join("settings.toml")).unwrap()
}

/// The key file text, trimmed.
pub fn key(world: &mut LinkserverWorld) -> String {
    fs::read_to_string(settings_dir(world).join("key"))
        .unwrap()
        .trim()
        .to_owned()
}

/// The component over the scenario's home, with the world's fakes for the system services.
pub fn config(world: &mut LinkserverWorld) -> LinkserverConfig {
    let home = home(world);
    let setup = LinkserverSetup {
        settings_dir: home.join("settings"),
        log_dir: home.join("logs"),
        agents_dir: home.join("agents"),
        program: PathBuf::from(PROGRAM),
        label: LABEL.to_owned(),
        host: "127.0.0.1".to_owned(),
        hosts: vec!["127.0.0.1".to_owned(), "localhost".to_owned()],
        version: "0.1.0".to_owned(),
        uid: 501,
        open_program: PathBuf::from("/nonexistent/open"),
        launchctl_program: PathBuf::from("/nonexistent/launchctl"),
        osascript_program: PathBuf::from("/nonexistent/osascript"),
    };
    let mut ports = LinkserverPorts::macos(&setup);
    ports.opener = world.opener.clone();
    ports.closer = world.closer.clone();
    ports.attempts = world.log.clone();
    ports.supervisor = world.supervisor.clone();
    ports.health = world.health.clone();
    LinkserverConfig::new(setup, ports, TerminalStyleConfig::new(false))
}

/// Fresh buffers for the next command.
pub fn reset_output(world: &mut LinkserverWorld) {
    world.out = SharedBuffer::default();
    world.err = SharedBuffer::default();
}

/// Runs the serve command until it is ready to stop at once; the files it prepares stay.
pub async fn serve_once(world: &mut LinkserverWorld) {
    reset_output(world);
    let command = config(world).serve_command();
    let (mut out, mut err) = (world.out.clone(), world.err.clone());
    world.code = Some(
        command
            .run(std::future::ready(()), &mut out, &mut err)
            .await,
    );
}

/// Starts the serve command in the background and waits until its health address answers.
pub async fn serve_in_background(world: &mut LinkserverWorld, port: u16) {
    reset_output(world);
    let command = config(world).serve_command();
    let (sender, receiver) = tokio::sync::oneshot::channel::<()>();
    let (mut out, mut err) = (world.out.clone(), world.err.clone());
    tokio::spawn(async move {
        let stop = async {
            let _ = receiver.await;
        };
        command.run(stop, &mut out, &mut err).await
    });
    world.shutdown = Some(sender);
    world.port = Some(port);
    for _ in 0..200 {
        if answers(port).await {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("the server did not answer: {}", world.err.text());
}

/// Whether the health address on `port` answers.
pub async fn answers(port: u16) -> bool {
    reqwest::get(format!("http://127.0.0.1:{port}/health"))
        .await
        .is_ok_and(|response| response.status().is_success())
}
