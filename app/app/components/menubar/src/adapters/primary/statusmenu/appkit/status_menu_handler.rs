//! The status item in the menu bar and its menu: the title line, the status line, Serve Links,
//! Start at Login, Settings…, Link History… and Quit (design D14, D18, D19).

#![expect(unsafe_code, reason = "ADR-001: AppKit FFI")]

use std::cell::OnceCell;
use std::sync::Arc;

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject, Sel};
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSApplication, NSControlStateValueOff, NSControlStateValueOn, NSImage, NSMenu, NSMenuDelegate,
    NSMenuItem, NSStatusBar, NSStatusItem, NSVariableStatusItemLength,
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSString};

use super::app_kit_config::ready;
use crate::core::application::use_cases::operate_menu::OperateMenu;
use crate::core::domain::menu_command_enum::MenuCommandEnum;
use crate::core::domain::menu_view_value_object::MenuViewValueObject;

const SYMBOL: &str = "link";
const QUIT: &str = "Quit http2scheme";
const TITLE: isize = 0;
const STATUS: isize = 1;
const SERVE: isize = 3;
const LOGIN: isize = 4;

/// The ivars of the target of every menu item.
struct TargetIvars {
    use_case: Arc<dyn OperateMenu>,
    show_settings: Box<dyn Fn()>,
    show_history: Box<dyn Fn()>,
    menu: OnceCell<Retained<NSMenu>>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "H2SMenuTarget"]
    #[ivars = TargetIvars]
    struct MenuTarget;

    unsafe impl NSObjectProtocol for MenuTarget {}

    unsafe impl NSMenuDelegate for MenuTarget {
        #[unsafe(method(menuNeedsUpdate:))]
        fn menu_needs_update(&self, menu: &NSMenu) {
            self.run(menu, MenuCommandEnum::Show);
        }
    }

    impl MenuTarget {
        #[unsafe(method(toggleServer:))]
        fn toggle_server(&self, _sender: Option<&AnyObject>) {
            self.chosen(MenuCommandEnum::ToggleServer);
        }

        #[unsafe(method(showSettings:))]
        fn show_settings(&self, _sender: Option<&AnyObject>) {
            (self.ivars().show_settings)();
        }

        #[unsafe(method(showHistory:))]
        fn show_history(&self, _sender: Option<&AnyObject>) {
            (self.ivars().show_history)();
        }

        #[unsafe(method(toggleLogin:))]
        fn toggle_login(&self, _sender: Option<&AnyObject>) {
            self.chosen(MenuCommandEnum::ToggleLoginItem);
        }

        #[unsafe(method(quit:))]
        fn quit(&self, _sender: Option<&AnyObject>) {
            NSApplication::sharedApplication(self.mtm()).terminate(None);
        }
    }
);

impl MenuTarget {
    fn new(
        mtm: MainThreadMarker,
        use_case: Arc<dyn OperateMenu>,
        show_settings: Box<dyn Fn()>,
        show_history: Box<dyn Fn()>,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(TargetIvars {
            use_case,
            show_settings,
            show_history,
            menu: OnceCell::new(),
        });
        // SAFETY: `NSObject`'s `init` on a freshly allocated object.
        unsafe { msg_send![super(this), init] }
    }

    fn mtm(&self) -> MainThreadMarker {
        MainThreadMarker::from(self)
    }

    fn chosen(&self, command: MenuCommandEnum) {
        if let Some(menu) = self.ivars().menu.get() {
            self.run(menu, command);
        }
    }

    fn run(&self, menu: &NSMenu, command: MenuCommandEnum) {
        match ready(self.ivars().use_case.execute(command)) {
            Ok(view) => show(menu, &view),
            Err(error) => log::warn!("menu: {error}"),
        }
    }
}

fn show(menu: &NSMenu, view: &MenuViewValueObject) {
    let title = |index, text: &str| {
        if let Some(item) = menu.itemAtIndex(index) {
            item.setTitle(&NSString::from_str(text));
        }
    };
    title(TITLE, view.title());
    title(STATUS, view.status_line());
    title(LOGIN, view.login_title());
    let check = |index, checked: bool| {
        if let Some(item) = menu.itemAtIndex(index) {
            item.setState(if checked {
                NSControlStateValueOn
            } else {
                NSControlStateValueOff
            });
        }
    };
    check(SERVE, view.serve_checked());
    check(LOGIN, view.login_checked());
    if let Some(item) = menu.itemAtIndex(LOGIN) {
        item.setEnabled(view.login_enabled());
    }
}

fn item(
    mtm: MainThreadMarker,
    target: &MenuTarget,
    title: &str,
    action: Option<Sel>,
    key: &str,
) -> Retained<NSMenuItem> {
    // SAFETY: `action` is a sender-taking method of `target`, which lives as long as the menu.
    let item = unsafe {
        let item = NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &NSString::from_str(title),
            action,
            &NSString::from_str(key),
        );
        item.setTarget(Some(target as &AnyObject));
        item
    };
    item.setEnabled(action.is_some());
    item
}

fn status_item(mtm: MainThreadMarker) -> Retained<NSStatusItem> {
    let item = NSStatusBar::systemStatusBar().statusItemWithLength(NSVariableStatusItemLength);
    if let Some(button) = item.button(mtm) {
        let image = NSImage::imageWithSystemSymbolName_accessibilityDescription(
            &NSString::from_str(SYMBOL),
            Some(&NSString::from_str("http2scheme")),
        );
        if let Some(image) = image {
            image.setTemplate(true);
            button.setImage(Some(&image));
        }
    }
    item
}

/// The status item and its menu; the item leaves the menu bar when this is dropped.
pub struct StatusMenuHandler {
    item: Retained<NSStatusItem>,
    menu: Retained<NSMenu>,
    target: Retained<MenuTarget>,
}

impl StatusMenuHandler {
    /// Puts the link icon with its menu in the menu bar; the menu reads `use_case` each time it
    /// opens; Settings… calls `show_settings` and Link History… calls `show_history`.
    #[must_use]
    pub fn install(
        mtm: MainThreadMarker,
        use_case: Arc<dyn OperateMenu>,
        show_settings: Box<dyn Fn()>,
        show_history: Box<dyn Fn()>,
    ) -> Self {
        let target = MenuTarget::new(mtm, use_case, show_settings, show_history);
        let menu = NSMenu::new(mtm);
        menu.setAutoenablesItems(false);
        menu.setDelegate(Some(ProtocolObject::from_ref(&*target)));
        let entries = [
            item(mtm, &target, "http2scheme", None, ""),
            item(mtm, &target, "", None, ""),
            NSMenuItem::separatorItem(mtm),
            item(mtm, &target, "Serve Links", Some(sel!(toggleServer:)), ""),
            item(mtm, &target, "Start at Login", Some(sel!(toggleLogin:)), ""),
            NSMenuItem::separatorItem(mtm),
            item(mtm, &target, "Settings…", Some(sel!(showSettings:)), ","),
            item(mtm, &target, "Link History…", Some(sel!(showHistory:)), ""),
            NSMenuItem::separatorItem(mtm),
            item(mtm, &target, QUIT, Some(sel!(quit:)), "q"),
        ];
        for entry in &entries {
            menu.addItem(entry);
        }
        let item = status_item(mtm);
        item.setMenu(Some(&menu));
        let _ = target.ivars().menu.set(menu.clone());
        target.run(&menu, MenuCommandEnum::Show);
        Self { item, menu, target }
    }

    /// The menu's items as they read after it opens: `(title, enabled, checked)`; a separator reads `-`.
    #[must_use]
    pub fn items(&self) -> Vec<(String, bool, bool)> {
        self.target.run(&self.menu, MenuCommandEnum::Show);
        (0..self.menu.numberOfItems())
            .filter_map(|index| self.menu.itemAtIndex(index))
            .map(|entry| {
                let title = if entry.isSeparatorItem() {
                    "-".to_owned()
                } else {
                    entry.title().to_string()
                };
                (
                    title,
                    entry.isEnabled(),
                    entry.state() == NSControlStateValueOn,
                )
            })
            .collect()
    }

    /// Chooses the item that reads `title`, as a click does.
    pub fn choose(&self, title: &str) {
        let titles = self.items();
        if let Some(index) = titles.iter().position(|(text, _, _)| text == title) {
            self.menu
                .performActionForItemAtIndex(isize::try_from(index).unwrap_or_default());
        }
    }
}

impl Drop for StatusMenuHandler {
    fn drop(&mut self) {
        NSStatusBar::systemStatusBar().removeStatusItem(&self.item);
    }
}

#[cfg(test)]
#[path = "status_menu_handler_test.rs"]
mod status_menu_handler_test;
