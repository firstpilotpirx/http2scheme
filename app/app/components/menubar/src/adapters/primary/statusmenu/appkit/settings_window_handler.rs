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
    NSApplication, NSBackingStoreType, NSColor, NSControlStateValueOff, NSControlStateValueOn,
    NSFont, NSSwitch, NSTabView, NSTabViewItem, NSTextField, NSView, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString};

use super::app_kit_config::ready;
use super::history_tab_handler::HistoryTabHandler;
use crate::core::application::use_cases::operate_menu::OperateMenu;
use crate::core::domain::menu_command_enum::MenuCommandEnum;
use crate::core::domain::menu_view_value_object::MenuViewValueObject;

/// The window's title, also how a test finds it.
pub const WINDOW_TITLE: &str = "http2scheme Settings";
const TABS: [&str; 2] = ["General", "History"];
const NAMES: [&str; 2] = ["Serve links", "Start at login"];
const WIDTH: f64 = 720.0;
const HEIGHT: f64 = 480.0;
const INSET: f64 = 12.0;
const MARGIN: f64 = 20.0;
const ROW_HEIGHT: f64 = 56.0;
const LINE_HEIGHT: f64 = 18.0;
const SWITCH_WIDTH: f64 = 44.0;
const NAME_SIZE: f64 = 13.0;
const NOTE_SIZE: f64 = 11.0;
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
    tabs: Retained<NSTabView>,
    rows: [SwitchRow; 2],
}

/// The ivars of the target of both switches.
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
        window.center();
        let frame = rect(INSET, INSET, WIDTH - 2.0 * INSET, HEIGHT - 2.0 * INSET);
        let tabs = NSTabView::initWithFrame(mtm.alloc::<NSTabView>(), frame);
        let area = tabs.contentRect().size;
        let general = NSView::initWithFrame(
            mtm.alloc::<NSView>(),
            rect(0.0, 0.0, area.width, area.height),
        );
        let rows = [0, 1].map(|index| switch_row(mtm, &general, NAMES[index], index));
        // SAFETY: both actions are sender-taking methods of `self`, which outlives the window.
        unsafe {
            rows[0].switch.setTarget(Some(self as &AnyObject));
            rows[0].switch.setAction(Some(sel!(toggleServer:)));
            rows[1].switch.setTarget(Some(self as &AnyObject));
            rows[1].switch.setAction(Some(sel!(toggleLogin:)));
        }
        let history = self.ivars().history.view();
        for (label, view) in TABS.iter().zip([&general, &history]) {
            let item = NSTabViewItem::new();
            item.setLabel(&NSString::from_str(label));
            item.setView(Some(view));
            tabs.addTabViewItem(&item);
        }
        window.setContentView(Some(&tabs));
        Parts { window, tabs, rows }
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

    fn present(&self, tab: usize) {
        let parts = self.parts();
        parts
            .tabs
            .selectTabViewItemAtIndex(isize::try_from(tab).unwrap_or_default());
        self.run(MenuCommandEnum::Show);
        self.ivars().history.show();
        NSApplication::sharedApplication(self.mtm()).activate();
        parts.window.makeKeyAndOrderFront(None);
    }
}

/// Whether the server is still starting after the Serve links switch was turned on.
fn still_starting(view: Option<&MenuViewValueObject>) -> bool {
    view.is_some_and(|view| view.status_line().starts_with(STARTING))
}

fn switch_row(mtm: MainThreadMarker, general: &NSView, name: &str, index: usize) -> SwitchRow {
    let place = f64::from(u32::try_from(index).unwrap_or_default());
    let top = general.frame().size.height - MARGIN - place * ROW_HEIGHT;
    let width = general.frame().size.width - 3.0 * MARGIN - SWITCH_WIDTH;
    let title = label(mtm, name, &NSFont::boldSystemFontOfSize(NAME_SIZE));
    title.setFrame(rect(MARGIN, top - LINE_HEIGHT, width, LINE_HEIGHT));
    let line = label(mtm, "", &NSFont::systemFontOfSize(NOTE_SIZE));
    line.setTextColor(Some(&NSColor::secondaryLabelColor()));
    line.setFrame(rect(MARGIN, top - 2.0 * LINE_HEIGHT, width, LINE_HEIGHT));
    let switch = NSSwitch::new(mtm);
    let switch_x = general.frame().size.width - MARGIN - SWITCH_WIDTH;
    switch.setFrame(rect(
        switch_x,
        top - 1.5 * LINE_HEIGHT - 4.0,
        SWITCH_WIDTH,
        24.0,
    ));
    general.addSubview(&title);
    general.addSubview(&line);
    general.addSubview(&switch);
    SwitchRow { switch, line }
}

const fn rect(x: f64, y: f64, width: f64, height: f64) -> NSRect {
    NSRect::new(NSPoint::new(x, y), NSSize::new(width, height))
}

fn label(mtm: MainThreadMarker, text: &str, font: &NSFont) -> Retained<NSTextField> {
    let field = NSTextField::labelWithString(&NSString::from_str(text), mtm);
    field.setFont(Some(font));
    field
}

fn titled_window(mtm: MainThreadMarker) -> Retained<NSWindow> {
    // SAFETY: a new titled window, kept by the target, so it must not release itself on close.
    let window = unsafe {
        let window = NSWindow::initWithContentRect_styleMask_backing_defer(
            mtm.alloc::<NSWindow>(),
            rect(0.0, 0.0, WIDTH, HEIGHT),
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

    /// Turns the Serve links switch, as a click does.
    pub fn turn_server(&self) {
        self.target.server_turned();
    }

    /// Turns the Start at login switch, as a click does.
    pub fn turn_login(&self) {
        self.target.run(MenuCommandEnum::ToggleLoginItem);
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
