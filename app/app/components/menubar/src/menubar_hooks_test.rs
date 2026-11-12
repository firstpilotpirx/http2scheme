use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::MenubarHooks;

#[test]
fn the_hooks_call_what_the_app_hands_in() {
    let started = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&started);
    let hooks = MenubarHooks {
        serve: Arc::new(move |_, _, _| {
            counter.fetch_add(1, Ordering::SeqCst);
        }),
        replay: Arc::new(|link| format!("opened {link}")),
    };
    (hooks.serve)(
        Box::pin(std::future::ready(())),
        Box::new(std::io::sink()),
        Box::new(std::io::sink()),
    );
    assert_eq!(started.load(Ordering::SeqCst), 1);
    assert_eq!((hooks.replay)("idea://x"), "opened idea://x");
}
