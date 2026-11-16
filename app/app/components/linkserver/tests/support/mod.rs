//! Shared helpers for the adapter integration tests.

#![allow(
    dead_code,
    clippy::unwrap_used,
    reason = "each test crate uses a different part of the support module"
)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use linkserver::adapters::primary::agentcontrol::cli::terminal_style_config::TerminalStyleConfig;
use linkserver::linkserver_config::LinkserverConfig;
use linkserver::linkserver_ports::LinkserverPorts;
use linkserver::linkserver_programs::LinkserverPrograms;
use linkserver::linkserver_setup::LinkserverSetup;

/// A program that records each call's arguments and exits with a fixed code.
pub struct RecordingProgram {
    pub program: PathBuf,
    record: PathBuf,
}

impl RecordingProgram {
    /// Writes the program into `dir`; it exits with `exit_code`.
    pub fn new(dir: &Path, name: &str, exit_code: i32) -> Self {
        let program = dir.join(name);
        let record = dir.join(format!("{name}.calls"));
        let script = format!(
            "#!/bin/bash\nprintf '%s\\0' \"$@\" >> '{}'\nprintf '\\n\\0' >> '{}'\necho 'recorded failure' >&2\nexit {exit_code}\n",
            record.display(),
            record.display()
        );
        fs::write(&program, script).unwrap();
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).unwrap();
        Self { program, record }
    }

    /// The arguments of every call, in order.
    pub fn calls(&self) -> Vec<Vec<String>> {
        let Ok(bytes) = fs::read(&self.record) else {
            return Vec::new();
        };
        let text = String::from_utf8(bytes).unwrap();
        let mut calls = vec![Vec::new()];
        for part in text.split('\0').filter(|p| !p.is_empty()) {
            if part == "\n" {
                calls.push(Vec::new());
            } else if let Some(last) = calls.last_mut() {
                last.push(part.to_owned());
            }
        }
        calls.pop();
        calls
    }
}

/// The component's root with the macOS adapters over `home`: the program is
/// `<home>/http2scheme`, the Claude Code folder `<home>/claude`, no color.
pub fn root_in(home: &Path) -> LinkserverConfig {
    let setup = LinkserverSetup {
        settings_dir: home.join("settings"),
        log_dir: home.join("logs"),
        agents_dir: home.join("agents"),
        claude_dir: home.join("claude"),
        program: home.join("http2scheme"),
        label: "test.http2scheme".to_owned(),
        host: "127.0.0.1".to_owned(),
        hosts: vec!["127.0.0.1".to_owned()],
        version: "0.0.0".to_owned(),
        uid: 0,
        programs: LinkserverPrograms {
            open: home.join("open"),
            launchctl: home.join("launchctl"),
            osascript: home.join("osascript"),
            lsappinfo: home.join("lsappinfo"),
        },
    };
    let ports = LinkserverPorts::macos(&setup);
    LinkserverConfig::new(setup, ports, TerminalStyleConfig::new(false))
}

/// The skill file inside a Claude Code folder.
pub fn skill_file(claude_dir: &Path) -> PathBuf {
    claude_dir.join("skills/http2scheme/SKILL.md")
}

/// A Claude Code folder that cannot hold a skill: a plain file stands where it should be.
pub fn blocked_folder(home: &Path) -> PathBuf {
    let dir = home.join("blocked");
    fs::write(&dir, "a file, not a folder").unwrap();
    dir
}
