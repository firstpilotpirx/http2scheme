//! The app server's state, read from the lines its `serve` command writes, and its switch.

use std::fmt;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, PoisonError};

use async_trait::async_trait;
use tokio::sync::oneshot;

use crate::core::application::services::server_state_service::ServerStateService;
use crate::core::application::services::server_switch_service::ServerSwitchService;
use crate::core::domain::server_state_enum::ServerStateEnum;
use crate::menubar_hooks::ServeLauncher;

const LISTENING: &str = "listening on ";
const FAIL_MARKS: [&str; 2] = ["[fail] ", "✗ "];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stream {
    Out,
    Err,
}

/// One state shared by the writers handed to `serve` and the menu that reads it; it also starts
/// the server through the app's launcher and stops the run it started.
#[derive(Clone)]
pub struct ServeOutputServerStateGateway {
    state: Arc<Mutex<ServerStateEnum>>,
    stream: Stream,
    pending: Vec<u8>,
    launcher: Option<ServeLauncher>,
    stop: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl fmt::Debug for ServeOutputServerStateGateway {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServeOutputServerStateGateway")
            .field("state", &self.state)
            .field("stream", &self.stream)
            .finish_non_exhaustive()
    }
}

impl Default for ServeOutputServerStateGateway {
    fn default() -> Self {
        Self::new()
    }
}

impl ServeOutputServerStateGateway {
    /// A gateway whose server is starting and that cannot start it again.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ServerStateEnum::Starting)),
            stream: Stream::Out,
            pending: Vec::new(),
            launcher: None,
            stop: Arc::default(),
        }
    }

    /// A gateway that starts the server through `launcher`.
    #[must_use]
    pub fn with_launcher(launcher: ServeLauncher) -> Self {
        Self {
            launcher: Some(launcher),
            ..Self::new()
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
            stream,
            pending: Vec::new(),
            ..self.clone()
        }
    }

    fn set(&self, state: ServerStateEnum) {
        *self.state.lock().unwrap_or_else(PoisonError::into_inner) = state;
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
                self.follow(state);
            }
        }
    }

    /// Takes the state a line tells, unless the user turned the server off.
    fn follow(&self, state: ServerStateEnum) {
        let mut current = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if current.is_serving() {
            *current = state;
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

#[async_trait]
impl ServerSwitchService for ServeOutputServerStateGateway {
    async fn turn_on(&self) {
        let (sender, receiver) = oneshot::channel();
        let previous = self
            .stop
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .replace(sender);
        if let Some(previous) = previous {
            let _ = previous.send(());
        }
        self.set(ServerStateEnum::Starting);
        if let Some(launcher) = &self.launcher {
            let shutdown = Box::pin(async move {
                let _ = receiver.await;
            });
            launcher(shutdown, Box::new(self.out()), Box::new(self.err()));
        }
    }

    async fn turn_off(&self) {
        self.set(ServerStateEnum::Off);
        let sender = self
            .stop
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        if let Some(sender) = sender {
            let _ = sender.send(());
        }
    }
}

#[cfg(test)]
#[path = "serve_output_server_state_gateway_test.rs"]
mod serve_output_server_state_gateway_test;
