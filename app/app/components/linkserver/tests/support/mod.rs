//! Shared helpers for the adapter integration tests.

#![allow(
    dead_code,
    clippy::unwrap_used,
    reason = "each test crate uses a different part of the support module"
)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

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
