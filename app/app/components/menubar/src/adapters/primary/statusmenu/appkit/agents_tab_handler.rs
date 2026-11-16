//! The Agents tab of the Settings window: the Claude Code folder with Choose…, the skill's state
//! and place with Install or Update, and Copy skill text (design D9, D10, D11).

#![expect(unsafe_code, reason = "ADR-001: AppKit FFI")]

use std::cell::{OnceCell, RefCell};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSButton, NSColor, NSModalResponseOK, NSOpenPanel, NSPasteboard, NSPasteboardTypeString, NSView,
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSString, NSURL};

use super::agents_row_config::{BUTTONS, GAP, Row, row, skill_name};
use super::app_kit_config::ready;
use super::history_tab_handler::click;
use super::history_table_config::rect;
use super::settings_layout_config::{AGENTS_HEIGHT as HEIGHT, WIDTH};
use crate::core::application::use_cases::manage_skill::ManageSkill;
use crate::core::domain::agents_command_enum::AgentsCommandEnum;
use crate::core::domain::agents_view_value_object::AgentsViewValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

/// The ivars of the target of the three buttons.
struct AgentsIvars {
    use_case: Arc<dyn ManageSkill>,
    view: Retained<NSView>,
    rows: OnceCell<[Row; 3]>,
    shown: RefCell<Option<AgentsViewValueObject>>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "H2SAgentsTarget"]
    #[ivars = AgentsIvars]
    struct AgentsTarget;

    unsafe impl NSObjectProtocol for AgentsTarget {}

    impl AgentsTarget {
        #[unsafe(method(choose:))]
        fn choose_clicked(&self, _sender: Option<&AnyObject>) {
            if let Some(folder) = self.pick_folder() {
                self.show(Some(folder));
            }
        }

        #[unsafe(method(install:))]
        fn install_clicked(&self, _sender: Option<&AnyObject>) {
            let view = ready(self.ivars().use_case.execute(AgentsCommandEnum::Install(self.folder())));
            self.draw(view);
        }

        #[unsafe(method(copy:))]
        fn copy_clicked(&self, _sender: Option<&AnyObject>) {
            let view = ready(self.ivars().use_case.execute(AgentsCommandEnum::Copy(self.folder())));
            let pasteboard = NSPasteboard::generalPasteboard();
            pasteboard.clearContents();
            let text = NSString::from_str(view.clipboard().unwrap_or_default());
            pasteboard.setString_forType(&text, string_type());
            self.draw(view);
        }
    }
);

impl AgentsTarget {
    fn new(mtm: MainThreadMarker, use_case: Arc<dyn ManageSkill>) -> Retained<Self> {
        let view = NSView::initWithFrame(mtm.alloc::<NSView>(), rect(0.0, 0.0, WIDTH, HEIGHT));
        let this = Self::alloc(mtm).set_ivars(AgentsIvars {
            use_case,
            view,
            rows: OnceCell::new(),
            shown: RefCell::new(None),
        });
        // SAFETY: `NSObject`'s `init` on a freshly allocated object.
        unsafe { msg_send![super(this), init] }
    }

    fn mtm(&self) -> MainThreadMarker {
        MainThreadMarker::from(self)
    }

    fn rows(&self) -> &[Row; 3] {
        self.ivars().rows.get_or_init(|| self.build())
    }

    fn build(&self) -> [Row; 3] {
        let mtm = self.mtm();
        let actions = [sel!(choose:), sel!(install:), sel!(copy:)];
        // SAFETY: every action is a sender-taking method of `self`, which outlives the tab.
        let buttons = unsafe {
            [0, 1, 2].map(|index| {
                NSButton::buttonWithTitle_target_action(
                    &NSString::from_str(BUTTONS[index]),
                    Some(self as &AnyObject),
                    Some(actions[index]),
                    mtm,
                )
            })
        };
        let mut index = 0;
        buttons.map(|button| {
            let row = row(self.mtm(), &self.ivars().view, index, button);
            index += 1;
            row
        })
    }

    /// The folder the tab shows, or the remembered one before the first read.
    fn folder(&self) -> PathBuf {
        let shown = self.ivars().shown.borrow().clone();
        shown.map_or_else(
            || {
                ready(self.ivars().use_case.execute(AgentsCommandEnum::Show(None)))
                    .folder()
                    .to_path_buf()
            },
            |view| view.folder().to_path_buf(),
        )
    }

    fn show(&self, folder: Option<PathBuf>) {
        let view = ready(
            self.ivars()
                .use_case
                .execute(AgentsCommandEnum::Show(folder)),
        );
        self.draw(view);
    }

    /// The folder panel: folders only, hidden ones shown, opening at the shown folder.
    fn folder_panel(&self) -> Retained<NSOpenPanel> {
        let panel = NSOpenPanel::openPanel(self.mtm());
        panel.setCanChooseDirectories(true);
        panel.setCanChooseFiles(false);
        panel.setShowsHiddenFiles(true);
        panel.setPrompt(Some(&NSString::from_str("Choose")));
        if let Some(url) = NSURL::from_directory_path(self.folder()) {
            panel.setDirectoryURL(Some(&url));
        }
        panel
    }

    /// The folder the user picks in the folder panel.
    fn pick_folder(&self) -> Option<PathBuf> {
        let panel = self.folder_panel();
        (panel.runModal() == NSModalResponseOK)
            .then(|| panel.URL().and_then(|url| url.to_file_path()))
            .flatten()
    }

    fn draw(&self, view: AgentsViewValueObject) {
        let [folder, skill, text] = self.rows();
        folder
            .line
            .setStringValue(&NSString::from_str(&view.folder_line()));
        let state = view.state();
        skill
            .name
            .setStringValue(&NSString::from_str(&skill_name(state)));
        skill.name.sizeToFit();
        let width = skill.name.frame().size.width;
        let frame = skill.name.frame();
        skill.state.setFrame(rect(
            frame.origin.x + width + GAP,
            frame.origin.y,
            WIDTH,
            frame.size.height,
        ));
        skill
            .state
            .setStringValue(&NSString::from_str(state.map_or("", |s| s.label())));
        skill.state.setTextColor(Some(&state_color(state)));
        skill
            .line
            .setStringValue(&NSString::from_str(&view.skill_line()));
        let alert = view.error().is_some().then(NSColor::systemRedColor);
        skill.line.setTextColor(Some(&grey_unless(alert)));
        skill.button.setTitle(&NSString::from_str(view.button()));
        skill
            .button
            .setKeyEquivalent(&NSString::from_str(key_equivalent(state)));
        text.line
            .setStringValue(&NSString::from_str(view.copy_line()));
        let done = view.is_copied().then(NSColor::systemGreenColor);
        text.line.setTextColor(Some(&grey_unless(done)));
        *self.ivars().shown.borrow_mut() = Some(view);
    }
}

/// Return presses Update only when an older skill is installed.
const fn key_equivalent(state: Option<SkillStateEnum>) -> &'static str {
    match state {
        Some(SkillStateEnum::Outdated) => "\r",
        Some(SkillStateEnum::NotInstalled | SkillStateEnum::Installed) | None => "",
    }
}

/// `color`, else the grey of a secondary line.
fn grey_unless(color: Option<Retained<NSColor>>) -> Retained<NSColor> {
    color.unwrap_or_else(NSColor::secondaryLabelColor)
}

fn state_color(state: Option<SkillStateEnum>) -> Retained<NSColor> {
    match state {
        Some(SkillStateEnum::Installed) => NSColor::systemGreenColor(),
        Some(SkillStateEnum::Outdated) => NSColor::systemOrangeColor(),
        Some(SkillStateEnum::NotInstalled) | None => NSColor::secondaryLabelColor(),
    }
}

fn string_type() -> &'static NSString {
    // SAFETY: an AppKit constant, set before the app runs and never changed.
    unsafe { NSPasteboardTypeString }
}

/// The Agents tab's view and its rows; the Settings window holds the view.
pub struct AgentsTabHandler {
    target: Retained<AgentsTarget>,
}

impl AgentsTabHandler {
    /// Prepares the tab over `use_case`; it reads the skill's state at [`Self::show`].
    #[must_use]
    pub fn install(mtm: MainThreadMarker, use_case: Arc<dyn ManageSkill>) -> Self {
        let target = AgentsTarget::new(mtm, use_case);
        target.rows();
        Self { target }
    }

    /// The view the Agents tab holds.
    #[must_use]
    pub fn view(&self) -> Retained<NSView> {
        self.target.ivars().view.clone()
    }

    /// Reads the shown folder's state again, the remembered folder before the first read.
    pub fn show(&self) {
        let shown = self.target.ivars().shown.borrow().clone();
        self.target
            .show(shown.map(|view| view.folder().to_path_buf()));
    }

    /// Shows `folder`, as Choose… does once the user picks it in the folder panel.
    pub fn choose_folder(&self, folder: &Path) {
        self.target.show(Some(folder.to_path_buf()));
    }

    /// Clicks Install, or Update.
    pub fn install_skill(&self) {
        click(&self.target.rows()[1].button);
    }

    /// Clicks Copy skill text.
    pub fn copy(&self) {
        click(&self.target.rows()[2].button);
    }

    /// The rows' names; the skill row's ends with its state: `Skill · Installed`.
    #[must_use]
    pub fn names(&self) -> [String; 3] {
        self.target.rows().each_ref().map(|row| {
            let name = row.name.stringValue().to_string();
            let state = row.state.stringValue().to_string();
            if state.is_empty() {
                name
            } else {
                format!("{name} {state}")
            }
        })
    }

    /// The lines under the rows' names.
    #[must_use]
    pub fn lines(&self) -> [String; 3] {
        self.target
            .rows()
            .each_ref()
            .map(|row| row.line.stringValue().to_string())
    }

    /// The install button's title: `Install` or `Update`.
    #[must_use]
    pub fn button(&self) -> String {
        self.target.rows()[1].button.title().to_string()
    }

    /// The text on the general pasteboard, where Copy skill text puts it.
    #[must_use]
    pub fn clipboard(&self) -> Option<String> {
        NSPasteboard::generalPasteboard()
            .stringForType(string_type())
            .map(|text| text.to_string())
    }

    /// The folder panel as Choose… opens it: whether it picks folders, files and hidden ones,
    /// and the folder it starts in.
    #[must_use]
    pub fn folder_panel(&self) -> ([bool; 3], Option<PathBuf>) {
        let panel = self.target.folder_panel();
        let picks = [
            panel.canChooseDirectories(),
            panel.canChooseFiles(),
            panel.showsHiddenFiles(),
        ];
        (
            picks,
            panel.directoryURL().and_then(|url| url.to_file_path()),
        )
    }
}

#[cfg(test)]
#[path = "agents_tab_handler_test.rs"]
mod agents_tab_handler_test;
