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
    NSButton, NSColor, NSFont, NSModalResponseOK, NSOpenPanel, NSPasteboard,
    NSPasteboardTypeString, NSTextField, NSView,
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSString, NSURL};

use super::app_kit_config::ready;
use super::history_tab_handler::click;
use super::history_table_config::{pin_to_top, rect};
use super::settings_layout_config::{
    GENERAL_HEIGHT, LINE_HEIGHT, MARGIN, NAME_HEIGHT, NAME_SIZE, NOTE_SIZE, ROW_HEIGHT, WIDTH,
    label,
};
use crate::core::application::use_cases::manage_skill::ManageSkill;
use crate::core::domain::agents_view_value_object::AgentsViewValueObject;
use crate::core::domain::skill_state_enum::SkillStateEnum;

/// The height of the Agents tab: three rows in the General tab's style.
pub(super) const HEIGHT: f64 = GENERAL_HEIGHT + ROW_HEIGHT;
const NAMES: [&str; 3] = ["Claude Code folder", "Skill", "Skill text"];
const BUTTONS: [&str; 3] = ["Choose…", "Install", "Copy skill text"];
const SEPARATOR: &str = " ·";
const BUTTON_WIDTH: f64 = 140.0;
const BUTTON_HEIGHT: f64 = 24.0;
const GAP: f64 = 4.0;

/// One row: its name, the coloured state after the name (the skill row only), the grey line under
/// the name and the button on the right.
struct Row {
    name: Retained<NSTextField>,
    state: Retained<NSTextField>,
    line: Retained<NSTextField>,
    button: Retained<NSButton>,
}

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
            let view = ready(self.ivars().use_case.install(self.folder()));
            self.draw(view);
        }

        #[unsafe(method(copy:))]
        fn copy_clicked(&self, _sender: Option<&AnyObject>) {
            let text = ready(self.ivars().use_case.skill_text());
            let pasteboard = NSPasteboard::generalPasteboard();
            pasteboard.clearContents();
            pasteboard.setString_forType(&NSString::from_str(&text), string_type());
            let view = self.ivars().shown.borrow().clone();
            if let Some(view) = view {
                self.draw(view.copied());
            }
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
            let row = self.row(index, button);
            index += 1;
            row
        })
    }

    fn row(&self, index: usize, button: Retained<NSButton>) -> Row {
        let mtm = self.mtm();
        let view = &self.ivars().view;
        let place = f64::from(u32::try_from(index).unwrap_or_default());
        let top = HEIGHT - MARGIN - place * ROW_HEIGHT;
        let button_x = WIDTH - MARGIN - BUTTON_WIDTH;
        let name = label(mtm, NAMES[index], &NSFont::systemFontOfSize(NAME_SIZE));
        name.setFrame(rect(MARGIN, top - NAME_HEIGHT, button_x, NAME_HEIGHT));
        let state = label(mtm, "", &NSFont::systemFontOfSize(NAME_SIZE));
        state.setFrame(rect(MARGIN, top - NAME_HEIGHT, button_x, NAME_HEIGHT));
        let line = label(mtm, "", &NSFont::systemFontOfSize(NOTE_SIZE));
        line.setTextColor(Some(&NSColor::secondaryLabelColor()));
        line.setFrame(rect(
            MARGIN,
            top - NAME_HEIGHT - GAP - LINE_HEIGHT,
            button_x - 2.0 * MARGIN,
            LINE_HEIGHT,
        ));
        button.setFrame(rect(
            button_x,
            top - BUTTON_HEIGHT,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
        ));
        let parts: [&NSView; 4] = [&name, &state, &line, &button];
        for part in parts {
            pin_to_top(part);
            view.addSubview(part);
        }
        Row {
            name,
            state,
            line,
            button,
        }
    }

    /// The folder the tab shows, or the remembered one before the first read.
    fn folder(&self) -> PathBuf {
        let shown = self.ivars().shown.borrow().clone();
        shown.map_or_else(
            || {
                ready(self.ivars().use_case.show(None))
                    .folder()
                    .to_path_buf()
            },
            |view| view.folder().to_path_buf(),
        )
    }

    fn show(&self, folder: Option<PathBuf>) {
        let view = ready(self.ivars().use_case.show(folder));
        self.draw(view);
    }

    /// The folder the user picks in the folder panel, starting at the shown one.
    fn pick_folder(&self) -> Option<PathBuf> {
        let panel = NSOpenPanel::openPanel(self.mtm());
        panel.setCanChooseDirectories(true);
        panel.setCanChooseFiles(false);
        panel.setShowsHiddenFiles(true);
        panel.setPrompt(Some(&NSString::from_str("Choose")));
        if let Some(url) = NSURL::from_directory_path(self.folder()) {
            panel.setDirectoryURL(Some(&url));
        }
        (panel.runModal() == NSModalResponseOK)
            .then(|| panel.URL().and_then(|url| url.to_file_path()))
            .flatten()
    }

    fn draw(&self, view: AgentsViewValueObject) {
        let [folder, skill, text] = self.rows();
        folder
            .line
            .setStringValue(&NSString::from_str(&view.folder_line()));
        let (name, state) = match view.state() {
            Some(state) => (format!("{}{SEPARATOR}", NAMES[1]), Some(state)),
            None => (NAMES[1].to_owned(), None),
        };
        skill.name.setStringValue(&NSString::from_str(&name));
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
        let default = state == Some(SkillStateEnum::Outdated);
        skill
            .button
            .setKeyEquivalent(&NSString::from_str(if default { "\r" } else { "" }));
        text.line
            .setStringValue(&NSString::from_str(view.copy_line()));
        let done = view.is_copied().then(NSColor::systemGreenColor);
        text.line.setTextColor(Some(&grey_unless(done)));
        *self.ivars().shown.borrow_mut() = Some(view);
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
}
