//! The shared `AppKit` application: a menu bar app with no Dock icon, run on the main thread.

use objc2::MainThreadMarker;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};

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
