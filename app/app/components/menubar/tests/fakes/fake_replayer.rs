//! A fake replayer: opens `idea` links only and appends every attempt to the request log, as the
//! server's replay does.

use std::fmt;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use menubar::menubar_hooks::LinkReplayer;

/// The links it opened, and the log it appends to.
#[derive(Clone)]
pub struct FakeReplayer {
    log: PathBuf,
    opened: Arc<Mutex<Vec<String>>>,
}

impl fmt::Debug for FakeReplayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FakeReplayer")
            .field("log", &self.log)
            .finish_non_exhaustive()
    }
}

impl FakeReplayer {
    /// A replayer appending to `log`.
    pub fn new(log: PathBuf) -> Self {
        Self {
            log,
            opened: Arc::default(),
        }
    }

    /// The hook the app hands the component.
    pub fn replayer(&self) -> LinkReplayer {
        let this = self.clone();
        Arc::new(move |link| {
            let result = if link.starts_with("idea://") {
                this.opened.lock().unwrap().push(link.to_owned());
                "opened"
            } else {
                "scheme not allowed"
            };
            this.append("2099-01-01T00:00:00Z", result, link);
            result.to_owned()
        })
    }

    /// Appends one request line to the log.
    pub fn append(&self, at: &str, result: &str, link: &str) {
        std::fs::create_dir_all(self.log.parent().unwrap()).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log)
            .unwrap();
        writeln!(file, "{at}\t{result}\t{link}").unwrap();
    }

    /// Appends a line that is no request.
    pub fn append_damaged(&self) {
        self.append("not a time", "opened", "idea://damaged");
    }

    /// The links opened, in order.
    pub fn opened(&self) -> Vec<String> {
        self.opened.lock().unwrap().clone()
    }
}
