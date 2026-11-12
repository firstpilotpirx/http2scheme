//! The History tab of the Settings window: the latest requests newest first, each with a ▶ that
//! opens its link again (design D18, D19, after the dictation history window of `VoicePuncher`).

#![expect(unsafe_code, reason = "ADR-001: AppKit FFI")]

use std::cell::RefCell;
use std::sync::Arc;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{NSButton, NSControl, NSFont, NSScrollView, NSTextAlignment, NSView};
use objc2_foundation::{NSDate, NSObject, NSObjectProtocol, NSPoint, NSString, NSTimeZone};

use super::app_kit_config::ready;
use super::history_table_config::{
    FONT_SIZE, ROW_HEIGHT, button_frame, framed, header, label, pin_to_top, rect, row,
};
use crate::core::application::use_cases::browse_history::BrowseHistory;
use crate::core::domain::history_command_enum::HistoryCommandEnum;
use crate::core::domain::history_view_value_object::HistoryViewValueObject;
use crate::core::domain::link_record_value_object::OPENED;

const WIDTH: f64 = 720.0;
/// The height of the History tab.
pub(super) const HEIGHT: f64 = 400.0;
const MARGIN: f64 = 14.0;
const NOTE_HEIGHT: f64 = 22.0;
const REPLAY: &str = "▶";

/// One row as the window shows it: time, result, link.
type ShownRow = [String; 3];

/// The ivars of the target of every ▶ button.
struct HistoryIvars {
    use_case: Arc<dyn BrowseHistory>,
    view: Retained<NSView>,
    content: RefCell<Option<Retained<NSView>>>,
    shown: RefCell<Vec<ShownRow>>,
    note: RefCell<Option<String>>,
    buttons: RefCell<Vec<Retained<NSButton>>>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "H2SHistoryTarget"]
    #[ivars = HistoryIvars]
    struct HistoryTarget;

    unsafe impl NSObjectProtocol for HistoryTarget {}

    impl HistoryTarget {
        #[unsafe(method(replay:))]
        fn replay_clicked(&self, sender: Option<&NSButton>) {
            if let Some(index) = sender.and_then(|button| usize::try_from(button.tag()).ok()) {
                self.replay(index);
            }
        }
    }
);

impl HistoryTarget {
    fn new(mtm: MainThreadMarker, use_case: Arc<dyn BrowseHistory>) -> Retained<Self> {
        let view = NSView::initWithFrame(mtm.alloc::<NSView>(), rect(0.0, 0.0, WIDTH, HEIGHT));
        let this = Self::alloc(mtm).set_ivars(HistoryIvars {
            use_case,
            view,
            content: RefCell::new(None),
            shown: RefCell::new(Vec::new()),
            note: RefCell::new(None),
            buttons: RefCell::new(Vec::new()),
        });
        // SAFETY: `NSObject`'s `init` on a freshly allocated object.
        unsafe { msg_send![super(this), init] }
    }

    fn mtm(&self) -> MainThreadMarker {
        MainThreadMarker::from(self)
    }

    fn run(&self, command: HistoryCommandEnum) {
        let view = match ready(self.ivars().use_case.execute(command)) {
            Ok(view) => view,
            Err(error) => {
                log::warn!("history: {error}");
                *self.ivars().note.borrow_mut() =
                    Some(format!("The link history could not be read: {error}"));
                self.ivars().shown.borrow_mut().clear();
                self.redraw();
                return;
            }
        };
        *self.ivars().note.borrow_mut() = view.note().map(str::to_owned);
        *self.ivars().shown.borrow_mut() = shown_rows(&view);
        self.redraw();
    }

    fn replay(&self, index: usize) {
        let link = self
            .ivars()
            .shown
            .borrow()
            .get(index)
            .map(|row| row[2].clone());
        if let Some(link) = link {
            self.run(HistoryCommandEnum::Replay(link));
        }
    }

    fn click_row(&self, index: usize) {
        let button = self.ivars().buttons.borrow().get(index).cloned();
        if let Some(button) = button {
            click(&button);
        }
    }

    /// Draws the note or the table at the top of the tab, which the tab view may hold shorter.
    fn redraw(&self) {
        let mtm = self.mtm();
        let bottom = self.ivars().view.frame().size.height - HEIGHT;
        let content =
            NSView::initWithFrame(mtm.alloc::<NSView>(), rect(0.0, bottom, WIDTH, HEIGHT));
        self.ivars().buttons.borrow_mut().clear();
        if let Some(note) = self.ivars().note.borrow().as_deref() {
            let line = label(mtm, note, &NSFont::systemFontOfSize(FONT_SIZE));
            line.setFrame(rect(
                MARGIN,
                HEIGHT / 2.0,
                WIDTH - 2.0 * MARGIN,
                NOTE_HEIGHT,
            ));
            line.setAlignment(NSTextAlignment::Center);
            content.addSubview(&line);
        } else {
            let frame = rect(MARGIN, MARGIN, WIDTH - 2.0 * MARGIN, HEIGHT - 2.0 * MARGIN);
            let (table, inside) = framed(mtm, frame);
            let width = inside.frame().size.width;
            let left = header(mtm, &inside, width);
            inside.addSubview(&self.rows_view(width, left));
            content.addSubview(&table);
        }
        pin_to_top(&content);
        if let Some(old) = self.ivars().content.replace(Some(content.clone())) {
            old.removeFromSuperview();
        }
        self.ivars().view.addSubview(&content);
    }

    fn rows_view(&self, width: f64, visible: f64) -> Retained<NSScrollView> {
        let mtm = self.mtm();
        let shown = self.ivars().shown.borrow();
        let rows = f64::from(u32::try_from(shown.len()).unwrap_or(u32::MAX));
        let height = (rows * ROW_HEIGHT).max(visible);
        let document = NSView::initWithFrame(mtm.alloc::<NSView>(), rect(0.0, 0.0, width, height));
        for (index, shown_row) in shown.iter().enumerate() {
            row(mtm, &document, shown_row, shown_row[1] == OPENED, index);
            let button = replay_button(mtm, self);
            button.setTag(isize::try_from(index).unwrap_or(isize::MAX));
            button.setFrame(button_frame(width, height, index));
            document.addSubview(&button);
            self.ivars().buttons.borrow_mut().push(button);
        }
        let scroll = NSScrollView::initWithFrame(
            mtm.alloc::<NSScrollView>(),
            rect(0.0, 0.0, width, visible),
        );
        scroll.setHasVerticalScroller(true);
        scroll.setAutohidesScrollers(true);
        scroll.setDrawsBackground(false);
        scroll.setDocumentView(Some(&document));
        document.scrollPoint(NSPoint::new(0.0, height));
        scroll
    }
}

fn shown_rows(view: &HistoryViewValueObject) -> Vec<ShownRow> {
    view.rows()
        .iter()
        .map(|row| {
            let at = row
                .at()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            let date = NSDate::dateWithTimeIntervalSince1970(at.as_secs_f64());
            let offset = NSTimeZone::localTimeZone().secondsFromGMTForDate(&date);
            [
                row.local_time(i64::try_from(offset).unwrap_or(0)),
                row.result().to_owned(),
                row.link().to_owned(),
            ]
        })
        .collect()
}

fn replay_button(mtm: MainThreadMarker, target: &HistoryTarget) -> Retained<NSButton> {
    // SAFETY: `replay:` is a sender-taking method of `target`, which outlives the tab's rows.
    unsafe {
        NSButton::buttonWithTitle_target_action(
            &NSString::from_str(REPLAY),
            Some(target as &AnyObject),
            Some(sel!(replay:)),
            mtm,
        )
    }
}

/// Clicks `control` as the user does: an enabled control changes its state and sends its action;
/// a disabled one ignores the click.
pub(super) fn click(control: &NSControl) {
    if control.isEnabled() {
        // SAFETY: a click with no sender on a control whose action takes an optional sender.
        unsafe { control.performClick(None) }
    }
}

/// The History tab's view and its rows; the Settings window holds the view.
pub struct HistoryTabHandler {
    target: Retained<HistoryTarget>,
}

impl HistoryTabHandler {
    /// Prepares the tab over `use_case`; it reads the request log at [`Self::show`].
    #[must_use]
    pub fn install(mtm: MainThreadMarker, use_case: Arc<dyn BrowseHistory>) -> Self {
        Self {
            target: HistoryTarget::new(mtm, use_case),
        }
    }

    /// The view the History tab holds.
    #[must_use]
    pub fn view(&self) -> Retained<NSView> {
        self.target.ivars().view.clone()
    }

    /// Reads the request log again and draws its rows.
    pub fn show(&self) {
        self.target.run(HistoryCommandEnum::Show);
    }

    /// The rows as the tab shows them: `[time, result, link]`, newest first.
    #[must_use]
    pub fn rows(&self) -> Vec<[String; 3]> {
        self.target.ivars().shown.borrow().clone()
    }

    /// The line the tab shows in place of its rows, if any.
    #[must_use]
    pub fn note(&self) -> Option<String> {
        self.target.ivars().note.borrow().clone()
    }

    /// Clicks ▶ on row `index`.
    pub fn replay(&self, index: usize) {
        self.target.click_row(index);
    }
}

#[cfg(test)]
#[path = "history_tab_handler_test.rs"]
mod history_tab_handler_test;
