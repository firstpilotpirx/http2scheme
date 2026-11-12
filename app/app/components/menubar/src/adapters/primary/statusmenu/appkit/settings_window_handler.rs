//! The Settings window: the General tab with the Serve links and Start at login switches, and the
//! History tab with the latest requests (design D19, after the settings window of `VoicePuncher`).

#![expect(unsafe_code, reason = "ADR-001: AppKit FFI")]

use std::cell::OnceCell;
use std::sync::Arc;
use std::time::{Duration, Instant};

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSApplication, NSBackingStoreType, NSControlStateValueOff, NSControlStateValueOn,
    NSSegmentedControl, NSSwitch, NSTabView, NSTextField, NSView, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSString};

use super::app_kit_config::ready;
use super::history_tab_handler::{HistoryTabHandler, click};
use super::history_table_config::rect;
use super::settings_layout_config::{
    BAR, GENERAL_HEIGHT, WIDTH, switch_row, tab_bar, tab_height, tab_view,
};
use crate::core::application::use_cases::operate_menu::OperateMenu;
use crate::core::domain::menu_command_enum::MenuCommandEnum;
use crate::core::domain::menu_view_value_object::MenuViewValueObject;

/// The window's title, also how a test finds it.
pub const WINDOW_TITLE: &str = "http2scheme Settings";
const STARTING: &str = "Starting";
const POLL: Duration = Duration::from_millis(50);
const POLL_LIMIT: Duration = Duration::from_secs(1);

/// One switch of the General tab with the line under its name.
struct SwitchRow {
    switch: Retained<NSSwitch>,
    line: Retained<NSTextField>,
}

impl SwitchRow {
    fn show(&self, on: bool, enabled: bool, line: &str) {
        let states = [NSControlStateValueOff, NSControlStateValueOn];
        self.switch.setState(states[usize::from(on)]);
        self.switch.setEnabled(enabled);
        self.line.setStringValue(&NSString::from_str(line));
    }
}

/// The window's views, built on first use.
struct Parts {
    window: Retained<NSWindow>,
    segments: Retained<NSSegmentedControl>,
    tabs: Retained<NSTabView>,
    rows: [SwitchRow; 2],
}

/// The ivars of the target of both switches and of the tab bar.
struct SettingsIvars {
    menu: Arc<dyn OperateMenu>,
    history: HistoryTabHandler,
    parts: OnceCell<Parts>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "H2SSettingsTarget"]
    #[ivars = SettingsIvars]
    struct SettingsTarget;

    unsafe impl NSObjectProtocol for SettingsTarget {}

    impl SettingsTarget {
        #[unsafe(method(toggleServer:))]
        fn toggle_server(&self, _sender: Option<&AnyObject>) {
            self.server_turned();
        }

        #[unsafe(method(toggleLogin:))]
        fn toggle_login(&self, _sender: Option<&AnyObject>) {
            self.run(MenuCommandEnum::ToggleLoginItem);
        }

        #[unsafe(method(chooseTab:))]
        fn choose_tab(&self, _sender: Option<&AnyObject>) {
            self.tab_chosen();
        }
    }
);

impl SettingsTarget {
    fn server_turned(&self) {
        let mut view = self.run(MenuCommandEnum::ToggleServer);
        let deadline = Instant::now() + POLL_LIMIT;
        while still_starting(view.as_ref()) && Instant::now() < deadline {
            std::thread::sleep(POLL);
            view = self.run(MenuCommandEnum::Show);
        }
    }

    fn tab_chosen(&self) {
        let chosen = self.parts().segments.selectedSegment();
        self.select(usize::try_from(chosen).unwrap_or_default());
    }

    fn new(
        mtm: MainThreadMarker,
        menu: Arc<dyn OperateMenu>,
        history: HistoryTabHandler,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(SettingsIvars {
            menu,
            history,
            parts: OnceCell::new(),
        });
        // SAFETY: `NSObject`'s `init` on a freshly allocated object.
        unsafe { msg_send![super(this), init] }
    }

    fn mtm(&self) -> MainThreadMarker {
        MainThreadMarker::from(self)
    }

    fn parts(&self) -> &Parts {
        self.ivars().parts.get_or_init(|| self.build())
    }

    fn build(&self) -> Parts {
        let mtm = self.mtm();
        let window = titled_window(mtm);
        let height = BAR + GENERAL_HEIGHT;
        let content = NSView::initWithFrame(mtm.alloc::<NSView>(), rect(0.0, 0.0, WIDTH, height));
        let segments = tab_bar(mtm, &content);
        let general =
            NSView::initWithFrame(mtm.alloc::<NSView>(), rect(0.0, 0.0, WIDTH, GENERAL_HEIGHT));
        let rows = [0, 1].map(|index| {
            let (switch, line) = switch_row(mtm, &general, index);
            SwitchRow { switch, line }
        });
        // SAFETY: every action is a sender-taking method of `self`, which outlives the window.
        unsafe {
            rows[0].switch.setTarget(Some(self as &AnyObject));
            rows[0].switch.setAction(Some(sel!(toggleServer:)));
            rows[1].switch.setTarget(Some(self as &AnyObject));
            rows[1].switch.setAction(Some(sel!(toggleLogin:)));
            segments.setTarget(Some(self as &AnyObject));
            segments.setAction(Some(sel!(chooseTab:)));
        }
        let tabs = tab_view(mtm, &general, &self.ivars().history.view());
        content.addSubview(&tabs);
        window.setContentView(Some(&content));
        window.center();
        Parts {
            window,
            segments,
            tabs,
            rows,
        }
    }

    fn run(&self, command: MenuCommandEnum) -> Option<MenuViewValueObject> {
        match ready(self.ivars().menu.execute(command)) {
            Ok(view) => {
                let [serve, login] = &self.parts().rows;
                serve.show(view.serve_checked(), true, view.status_line());
                login.show(
                    view.login_checked(),
                    view.login_enabled(),
                    view.login_note(),
                );
                Some(view)
            }
            Err(error) => {
                log::warn!("settings: {error}");
                None
            }
        }
    }

    /// Shows tab `tab` and fits the window to it, keeping its top edge in place.
    fn select(&self, tab: usize) {
        let parts = self.parts();
        let index = isize::try_from(tab).unwrap_or_default();
        parts.tabs.selectTabViewItemAtIndex(index);
        parts.segments.setSelectedSegment(index);
        let height = tab_height(tab);
        let window = &parts.window;
        let old = window.contentRectForFrameRect(window.frame());
        let top = old.origin.y + old.size.height;
        let fitted = rect(old.origin.x, top - height, old.size.width, height);
        let frame = window.frameRectForContentRect(fitted);
        window.setFrame_display_animate(frame, true, window.isVisible());
    }

    fn present(&self, tab: usize) {
        self.select(tab);
        self.run(MenuCommandEnum::Show);
        self.ivars().history.show();
        NSApplication::sharedApplication(self.mtm()).activate();
        self.parts().window.makeKeyAndOrderFront(None);
    }
}

/// Whether the server is still starting after the Serve links switch was turned on.
fn still_starting(view: Option<&MenuViewValueObject>) -> bool {
    view.is_some_and(|view| view.status_line().starts_with(STARTING))
}

fn titled_window(mtm: MainThreadMarker) -> Retained<NSWindow> {
    // SAFETY: a new titled window, kept by the target, so it must not release itself on close.
    let window = unsafe {
        let window = NSWindow::initWithContentRect_styleMask_backing_defer(
            mtm.alloc::<NSWindow>(),
            rect(0.0, 0.0, WIDTH, BAR + GENERAL_HEIGHT),
            NSWindowStyleMask::Titled | NSWindowStyleMask::Closable,
            NSBackingStoreType::Buffered,
            false,
        );
        window.setReleasedWhenClosed(false);
        window
    };
    window.setTitle(&NSString::from_str(WINDOW_TITLE));
    window
}

/// The Settings window, built on first use and brought to the front after.
pub struct SettingsWindowHandler {
    target: Retained<SettingsTarget>,
}

impl SettingsWindowHandler {
    /// Prepares the window over the menu's `menu` use case with the `history` tab; nothing
    /// shows until [`Self::show_general`] or [`Self::show_history`].
    #[must_use]
    pub fn install(
        mtm: MainThreadMarker,
        menu: Arc<dyn OperateMenu>,
        history: HistoryTabHandler,
    ) -> Self {
        Self {
            target: SettingsTarget::new(mtm, menu, history),
        }
    }

    /// Brings the window to the front on the General tab, as Settings… does.
    pub fn show_general(&self) {
        self.target.present(0);
    }

    /// Brings the window to the front on the History tab, as Link History… does.
    pub fn show_history(&self) {
        self.target.present(1);
    }

    /// The label of the selected tab.
    #[must_use]
    pub fn selected_tab(&self) -> String {
        self.target
            .parts()
            .tabs
            .selectedTabViewItem()
            .map(|item| item.label().to_string())
            .unwrap_or_default()
    }

    /// Whether the Serve links and the Start at login switches are on.
    #[must_use]
    pub fn switches(&self) -> [bool; 2] {
        self.target
            .parts()
            .rows
            .each_ref()
            .map(|row| row.switch.state() == NSControlStateValueOn)
    }

    /// Whether the Start at login switch can be turned.
    #[must_use]
    pub fn login_enabled(&self) -> bool {
        self.target.parts().rows[1].switch.isEnabled()
    }

    /// The lines under the two switches: the status line and the login note.
    #[must_use]
    pub fn lines(&self) -> [String; 2] {
        self.target
            .parts()
            .rows
            .each_ref()
            .map(|row| row.line.stringValue().to_string())
    }

    /// Clicks the Serve links switch.
    pub fn turn_server(&self) {
        click(&self.target.parts().rows[0].switch);
    }

    /// Clicks the Start at login switch; a disabled switch does nothing.
    pub fn turn_login(&self) {
        click(&self.target.parts().rows[1].switch);
    }

    /// The window's title.
    #[must_use]
    pub fn title(&self) -> String {
        self.target.parts().window.title().to_string()
    }

    /// Clicks segment `index` of the tab bar: 0 General, 1 History.
    pub fn choose_tab(&self, index: usize) {
        let segments = &self.target.parts().segments;
        segments.setSelectedSegment(isize::try_from(index).unwrap_or_default());
        click(segments);
    }

    /// The height of the window's content: the tab bar and the shown tab.
    #[must_use]
    pub fn content_height(&self) -> f64 {
        let window = &self.target.parts().window;
        window.contentRectForFrameRect(window.frame()).size.height
    }

    /// The History tab.
    #[must_use]
    pub fn history(&self) -> &HistoryTabHandler {
        &self.target.ivars().history
    }

    /// Whether the window is on the screen.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.target
            .ivars()
            .parts
            .get()
            .is_some_and(|parts| parts.window.isVisible())
    }
}

impl Drop for SettingsWindowHandler {
    fn drop(&mut self) {
        if let Some(parts) = self.target.ivars().parts.get() {
            parts.window.orderOut(None);
        }
    }
}

#[cfg(test)]
#[path = "settings_window_handler_test.rs"]
mod settings_window_handler_test;
