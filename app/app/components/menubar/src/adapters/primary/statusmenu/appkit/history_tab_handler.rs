//! The History tab of the Settings window: the latest requests newest first, each with a ▶ that
//! opens its link again (design D18, D19, after the dictation history window of `VoicePuncher`).

#![expect(unsafe_code, reason = "ADR-001: AppKit FFI")]

use std::cell::RefCell;
use std::sync::Arc;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSButton, NSFont, NSLineBreakMode, NSScrollView, NSTextAlignment, NSTextField, NSView,
};
use objc2_foundation::{
    NSDate, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString, NSTimeZone,
};

use super::app_kit_config::ready;
use crate::core::application::use_cases::browse_history::BrowseHistory;
use crate::core::domain::history_command_enum::HistoryCommandEnum;
use crate::core::domain::history_view_value_object::HistoryViewValueObject;

const WIDTH: f64 = 680.0;
const HEIGHT: f64 = 400.0;
const MARGIN: f64 = 16.0;
const HEADER: f64 = 22.0;
const ROW_HEIGHT: f64 = 26.0;
const SCROLLER: f64 = 16.0;
const TIME_WIDTH: f64 = 150.0;
const RESULT_WIDTH: f64 = 130.0;
const BUTTON_WIDTH: f64 = 40.0;
const GAP: f64 = 8.0;
const FONT_SIZE: f64 = 13.0;
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

    fn redraw(&self) {
        let mtm = self.mtm();
        let content = NSView::initWithFrame(mtm.alloc::<NSView>(), rect(0.0, 0.0, WIDTH, HEIGHT));
        let font = NSFont::systemFontOfSize(FONT_SIZE);
        if let Some(note) = self.ivars().note.borrow().as_deref() {
            let line = label(mtm, note, &font);
            line.setFrame(rect(MARGIN, HEIGHT / 2.0, WIDTH - 2.0 * MARGIN, HEADER));
            line.setAlignment(NSTextAlignment::Center);
            content.addSubview(&line);
        } else {
            let bold = NSFont::boldSystemFontOfSize(FONT_SIZE);
            let width = WIDTH - 2.0 * MARGIN - SCROLLER;
            for (column, (x, column_width)) in ["Time", "Result", "Link"].iter().zip(columns(width))
            {
                let header = label(mtm, column, &bold);
                header.setFrame(rect(
                    MARGIN + x,
                    HEIGHT - MARGIN - HEADER,
                    column_width,
                    HEADER,
                ));
                content.addSubview(&header);
            }
            content.addSubview(&self.rows_view(&font, width));
        }
        if let Some(old) = self.ivars().content.replace(Some(content.clone())) {
            old.removeFromSuperview();
        }
        self.ivars().view.addSubview(&content);
    }

    fn rows_view(&self, font: &NSFont, width: f64) -> Retained<NSScrollView> {
        let mtm = self.mtm();
        let shown = self.ivars().shown.borrow();
        let visible = HEIGHT - 2.0 * MARGIN - HEADER - GAP;
        let rows = f64::from(u32::try_from(shown.len()).unwrap_or(u32::MAX));
        let height = (rows * ROW_HEIGHT).max(visible);
        let document = NSView::initWithFrame(mtm.alloc::<NSView>(), rect(0.0, 0.0, width, height));
        for (index, row) in shown.iter().enumerate() {
            let tag = isize::try_from(index).unwrap_or(isize::MAX);
            let place = f64::from(u32::try_from(index).unwrap_or(u32::MAX));
            let y = height - (place + 1.0) * ROW_HEIGHT;
            for (text, (x, column_width)) in row.iter().zip(columns(width)) {
                let cell = label(mtm, text, font);
                cell.setLineBreakMode(NSLineBreakMode::ByTruncatingMiddle);
                cell.setToolTip(Some(&NSString::from_str(text)));
                cell.setFrame(rect(x, y + 4.0, column_width, ROW_HEIGHT - 6.0));
                document.addSubview(&cell);
            }
            let button = replay_button(mtm, self);
            button.setTag(tag);
            button.setFrame(rect(width - BUTTON_WIDTH, y, BUTTON_WIDTH, ROW_HEIGHT));
            document.addSubview(&button);
        }
        let scroll = NSScrollView::initWithFrame(
            mtm.alloc::<NSScrollView>(),
            rect(MARGIN, MARGIN, width + SCROLLER, visible),
        );
        scroll.setHasVerticalScroller(true);
        scroll.setDocumentView(Some(&document));
        document.scrollPoint(NSPoint::new(0.0, height));
        scroll
    }
}

/// The x and the width of the time, result and link columns in a row `width` wide.
fn columns(width: f64) -> [(f64, f64); 3] {
    let link_x = TIME_WIDTH + RESULT_WIDTH + 2.0 * GAP;
    [
        (0.0, TIME_WIDTH),
        (TIME_WIDTH + GAP, RESULT_WIDTH),
        (link_x, width - link_x - BUTTON_WIDTH - GAP),
    ]
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

const fn rect(x: f64, y: f64, width: f64, height: f64) -> NSRect {
    NSRect::new(NSPoint::new(x, y), NSSize::new(width, height))
}

fn label(mtm: MainThreadMarker, text: &str, font: &NSFont) -> Retained<NSTextField> {
    let field = NSTextField::labelWithString(&NSString::from_str(text), mtm);
    field.setFont(Some(font));
    field.setAlignment(NSTextAlignment::Left);
    field
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

    /// Chooses ▶ on row `index`, as a click does.
    pub fn replay(&self, index: usize) {
        self.target.replay(index);
    }
}

#[cfg(test)]
#[path = "history_tab_handler_test.rs"]
mod history_tab_handler_test;
