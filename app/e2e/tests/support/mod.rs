//! A temporary home with recording system programs, and the built binary run inside it.

pub mod answer;
pub mod printed;

use std::fs;
use std::net::TcpListener as StdListener;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpStream;
use tokio::process::{Child, Command};

/// The agent label the binary uses.
pub const LABEL: &str = "io.github.firstpilotpirx.http2scheme";

const RECORDER: &str = r#"#!/bin/bash
dir="$(dirname "$0")"; name="$(basename "$0")"
{ printf '%s\x1f' "$@"; printf '\x1e'; } >> "$dir/$name.calls"
if [ -e "$dir/$name.fail" ]; then echo "recorded failure" >&2; exit 1; fi
if [ "$name" = launchctl ]; then
  case "$1" in
    bootstrap) [ -e "$dir/loaded" ] && exit 5; touch "$dir/loaded" ;;
    bootout) [ -e "$dir/loaded" ] || exit 3; rm "$dir/loaded" ;;
    print) [ -e "$dir/loaded" ] || exit 113 ;;
  esac
fi
exit 0
"#;

/// The binary under test.
pub fn sut_bin() -> PathBuf {
    PathBuf::from(std::env::var("SUT_BIN").unwrap())
}

/// A port no program holds now.
pub fn free_port() -> u16 {
    StdListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

/// A temporary home whose `bin` folder holds recording `open`, `launchctl` and `osascript`.
#[derive(Debug)]
pub struct Home {
    dir: tempfile::TempDir,
}

impl Home {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let bin = dir.path().join("bin");
        fs::create_dir_all(&bin).unwrap();
        for name in ["open", "launchctl", "osascript"] {
            let path = bin.join(name);
            fs::write(&path, RECORDER).unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        }
        Self { dir }
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    fn bin(&self, name: &str) -> PathBuf {
        self.path().join("bin").join(name)
    }

    pub fn settings_dir(&self) -> PathBuf {
        self.path().join("Library/Application Support/http2scheme")
    }

    pub fn settings_text(&self) -> String {
        fs::read_to_string(self.settings_dir().join("settings.toml")).unwrap()
    }

    pub fn key(&self) -> String {
        fs::read_to_string(self.settings_dir().join("key"))
            .unwrap()
            .trim()
            .to_owned()
    }

    pub fn request_log(&self) -> String {
        fs::read_to_string(self.path().join("Library/Logs/http2scheme/requests.log"))
            .unwrap_or_default()
    }

    pub fn agent_file(&self) -> PathBuf {
        self.path()
            .join("Library/LaunchAgents")
            .join(format!("{LABEL}.plist"))
    }

    pub fn write_settings(&self, port: u16, schemes: &[&str], close_tab: bool) {
        fs::create_dir_all(self.settings_dir()).unwrap();
        let schemes: Vec<String> = schemes.iter().map(|name| format!("\"{name}\"")).collect();
        let text = format!(
            "port = {port}\nallowed_schemes = [{}]\nclose_tab = {close_tab}\n",
            schemes.join(", ")
        );
        fs::write(self.settings_dir().join("settings.toml"), text).unwrap();
    }

    pub fn write_key(&self, key: &str, mode: u32) {
        let path = self.settings_dir().join("key");
        fs::create_dir_all(self.settings_dir()).unwrap();
        fs::write(&path, key).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(mode)).unwrap();
    }

    /// Makes the recording program `name` fail from now on.
    pub fn make_fail(&self, name: &str) {
        fs::write(self.bin(&format!("{name}.fail")), "").unwrap();
    }

    /// Whether the recording `launchctl` holds the agent loaded.
    pub fn loaded(&self) -> bool {
        self.bin("loaded").exists()
    }

    pub fn unload(&self) {
        let _ = fs::remove_file(self.bin("loaded"));
    }

    /// The arguments of every call of the recording program `name`.
    pub fn calls(&self, name: &str) -> Vec<Vec<String>> {
        let text = fs::read_to_string(self.bin(&format!("{name}.calls"))).unwrap_or_default();
        text.split('\u{1e}')
            .filter(|call| !call.is_empty())
            .map(|call| {
                call.split('\u{1f}')
                    .filter(|argument| !argument.is_empty())
                    .map(str::to_owned)
                    .collect()
            })
            .collect()
    }

    /// The binary with this home and the recording programs.
    pub fn command(&self, arguments: &[&str]) -> Command {
        let mut command = Command::new(sut_bin());
        command
            .args(arguments)
            .env("HOME", self.path())
            .env("HTTP2SCHEME_OPEN_PROGRAM", self.bin("open"))
            .env("HTTP2SCHEME_LAUNCHCTL_PROGRAM", self.bin("launchctl"))
            .env("HTTP2SCHEME_OSASCRIPT_PROGRAM", self.bin("osascript"))
            .env("NO_COLOR", "1")
            .env_remove("FORCE_COLOR")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        command
    }

    /// Runs one command to its end: exit status, standard output, standard error.
    pub async fn run(&self, arguments: &[&str]) -> (i32, String, String) {
        let output = self.command(arguments).output().await.unwrap();
        let text = |bytes: Vec<u8>| String::from_utf8(bytes).unwrap();
        (
            output.status.code().unwrap(),
            text(output.stdout),
            text(output.stderr),
        )
    }

    /// Starts `serve` in the background.
    pub fn spawn_serve(&self) -> Child {
        self.command(&["serve"]).spawn().unwrap()
    }
}

/// Sends one GET with this Host header and returns the status and the body.
pub async fn get(address: &str, host: &str, path: &str) -> std::io::Result<(u16, String)> {
    let mut stream = TcpStream::connect(address).await?;
    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await?;
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).await?;
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let (head, body) = text.split_once("\r\n\r\n").unwrap_or((&text, ""));
    let status = head
        .split(' ')
        .nth(1)
        .and_then(|code| code.parse().ok())
        .unwrap_or(0);
    Ok((status, body.to_owned()))
}

/// Whether the health address on `port` answers.
pub async fn answers(port: u16) -> bool {
    let host = format!("127.0.0.1:{port}");
    matches!(get(&host, &host, "/health").await, Ok((200, _)))
}

/// Waits until `port` answers or `child` ends; true when it answers.
pub async fn wait_until_answering(child: &mut Child, port: Option<u16>) -> bool {
    for _ in 0..500 {
        if let Some(port) = port
            && answers(port).await
        {
            return true;
        }
        if child.try_wait().unwrap().is_some() {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("the server neither answered nor ended");
}
