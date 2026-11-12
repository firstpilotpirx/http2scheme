//! The app server's state, read from the lines its `serve` command writes.

use std::io::{self, Write};
use std::sync::{Arc, Mutex, PoisonError};

use async_trait::async_trait;

use crate::core::application::services::server_state_service::ServerStateService;
use crate::core::domain::server_state_enum::ServerStateEnum;

const LISTENING: &str = "listening on ";
const FAIL_MARKS: [&str; 2] = ["[fail] ", "✗ "];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stream {
    Out,
    Err,
}

/// One state shared by the writers handed to `serve` and the menu that reads it.
#[derive(Debug, Clone)]
pub struct ServeOutputServerStateGateway {
    state: Arc<Mutex<ServerStateEnum>>,
    stream: Stream,
    pending: Vec<u8>,
}

impl Default for ServeOutputServerStateGateway {
    fn default() -> Self {
        Self::new()
    }
}

impl ServeOutputServerStateGateway {
    /// A gateway whose server is starting.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ServerStateEnum::Starting)),
            stream: Stream::Out,
            pending: Vec::new(),
        }
    }

    /// The writer for `serve`'s standard output: the banner line means running.
    #[must_use]
    pub fn out(&self) -> Self {
        self.writer(Stream::Out)
    }

    /// The writer for `serve`'s error output: a line means stopped, with that reason.
    #[must_use]
    pub fn err(&self) -> Self {
        self.writer(Stream::Err)
    }

    fn writer(&self, stream: Stream) -> Self {
        Self {
            state: Arc::clone(&self.state),
            stream,
            pending: Vec::new(),
        }
    }

    fn state_of(&self, line: &str) -> Option<ServerStateEnum> {
        let line = line.trim();
        match self.stream {
            Stream::Out => {
                let (_, rest) = line.split_once(LISTENING)?;
                let address = rest.split_whitespace().next()?;
                Some(ServerStateEnum::Running(address.to_owned()))
            }
            Stream::Err if line.is_empty() => None,
            Stream::Err => {
                let reason = FAIL_MARKS
                    .iter()
                    .find_map(|mark| line.strip_prefix(mark))
                    .unwrap_or(line);
                Some(ServerStateEnum::Stopped(reason.to_owned()))
            }
        }
    }

    fn take_lines(&mut self) {
        while let Some(end) = self.pending.iter().position(|&byte| byte == b'\n') {
            let line: Vec<u8> = self.pending.drain(..=end).collect();
            if let Some(state) = self.state_of(&String::from_utf8_lossy(&line)) {
                *self.state.lock().unwrap_or_else(PoisonError::into_inner) = state;
            }
        }
    }
}

impl Write for ServeOutputServerStateGateway {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.pending.extend_from_slice(bytes);
        self.take_lines();
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl ServerStateService for ServeOutputServerStateGateway {
    async fn current(&self) -> ServerStateEnum {
        self.state
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

#[cfg(test)]
#[path = "serve_output_server_state_gateway_test.rs"]
mod serve_output_server_state_gateway_test;
