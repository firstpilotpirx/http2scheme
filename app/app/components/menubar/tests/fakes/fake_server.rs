//! A fake server for the app's launcher: it prints the banner and runs until its shutdown.

use std::fmt;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use menubar::menubar_hooks::{ServeLauncher, ServeShutdown};

/// Every run the launcher started; a finished run is `None`.
#[derive(Clone, Default)]
pub struct FakeServer {
    runs: Arc<Mutex<Vec<Option<ServeShutdown>>>>,
}

impl fmt::Debug for FakeServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FakeServer").finish_non_exhaustive()
    }
}

impl FakeServer {
    /// The launcher the app hands the component: a run that listens on `port`.
    pub fn launcher(&self, port: u16) -> ServeLauncher {
        let runs = Arc::clone(&self.runs);
        Arc::new(move |shutdown, mut out, _| {
            let banner = format!("[..] http2scheme 0.1.0 listening on http://127.0.0.1:{port}\n");
            out.write_all(banner.as_bytes()).unwrap();
            runs.lock().unwrap().push(Some(shutdown));
        })
    }

    /// Whether the latest run still serves: its shutdown has not completed.
    pub fn answers(&self) -> bool {
        let mut runs = self.runs.lock().unwrap();
        let Some(latest) = runs.last_mut() else {
            return false;
        };
        if let Some(shutdown) = latest {
            let mut context = Context::from_waker(Waker::noop());
            if shutdown.as_mut().poll(&mut context) == Poll::Ready(()) {
                *latest = None;
            }
        }
        latest.is_some()
    }

    /// How many runs the launcher started.
    pub fn starts(&self) -> usize {
        self.runs.lock().unwrap().len()
    }
}
