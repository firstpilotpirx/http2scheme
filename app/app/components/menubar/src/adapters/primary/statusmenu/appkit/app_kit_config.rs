//! The shared `AppKit` application: a menu bar app with no Dock icon, run on the main thread.

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use objc2::MainThreadMarker;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};

/// Drives a future whose ports never wait: the menu and the window run outside any tokio runtime.
pub(crate) fn ready<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        std::thread::yield_now();
    }
}

/// Starts and runs the application loop.
pub struct AppKitConfig;

impl AppKitConfig {
    /// The main-thread marker, when called on the main thread.
    #[must_use]
    pub fn main_thread() -> Option<MainThreadMarker> {
        MainThreadMarker::new()
    }

    /// Runs the application as an accessory (no Dock icon, no menu of its own) until it terminates.
    pub fn run(mtm: MainThreadMarker) {
        let app = NSApplication::sharedApplication(mtm);
        app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
        app.run();
    }
}

#[cfg(test)]
#[path = "app_kit_config_test.rs"]
mod app_kit_config_test;
