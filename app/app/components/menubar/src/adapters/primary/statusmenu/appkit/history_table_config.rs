//! The History tab's table as the approved mock draws it: a rounded, bordered box with a header
//! row over a line, striped rows, the result in green or red and the link in a fixed-pitch font
//! (design D19).

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSBox, NSBoxType, NSColor, NSFont, NSLineBreakMode, NSTextField,
    NSTitlePosition, NSView,
};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

/// The height of a row and of the header.
pub(super) const ROW_HEIGHT: f64 = 26.0;
/// The text size of the table.
pub(super) const FONT_SIZE: f64 = 12.0;
const PAD: f64 = 8.0;
const GAP: f64 = 8.0;
const TIME_WIDTH: f64 = 130.0;
const RESULT_WIDTH: f64 = 110.0;
const BUTTON_WIDTH: f64 = 36.0;
const BUTTON_HEIGHT: f64 = 20.0;
const TEXT_HEIGHT: f64 = 16.0;
const CORNER: f64 = 5.0;
const HEADERS: [&str; 3] = ["Time", "Result", "Link"];

/// The x and the width of the time, result and link columns and of the ▶ button in a row
/// `width` wide.
pub(super) fn columns(width: f64) -> [(f64, f64); 4] {
    let result_x = PAD + TIME_WIDTH + GAP;
    let link_x = result_x + RESULT_WIDTH + GAP;
    let button_x = width - PAD - BUTTON_WIDTH;
    [
        (PAD, TIME_WIDTH),
        (result_x, RESULT_WIDTH),
        (link_x, button_x - GAP - link_x),
        (button_x, BUTTON_WIDTH),
    ]
}

/// A rectangle from its origin and size.
pub(super) const fn rect(x: f64, y: f64, width: f64, height: f64) -> NSRect {
    NSRect::new(NSPoint::new(x, y), NSSize::new(width, height))
}

/// A label with `text` in `font`, cut in the middle when it does not fit.
pub(super) fn label(mtm: MainThreadMarker, text: &str, font: &NSFont) -> Retained<NSTextField> {
    let field = NSTextField::labelWithString(&NSString::from_str(text), mtm);
    field.setFont(Some(font));
    field.setLineBreakMode(NSLineBreakMode::ByTruncatingMiddle);
    field
}

/// The rounded, bordered box that holds the header and the rows; its content view is answered.
pub(super) fn framed(mtm: MainThreadMarker, frame: NSRect) -> (Retained<NSBox>, Retained<NSView>) {
    let frame_box = filled(mtm, frame, &NSColor::controlBackgroundColor());
    frame_box.setCornerRadius(CORNER);
    frame_box.setBorderWidth(1.0);
    frame_box.setBorderColor(&NSColor::tertiaryLabelColor());
    let content = frame_box.contentView().unwrap_or_else(|| NSView::new(mtm));
    (frame_box, content)
}

/// The header row of `width` at the top of `content`, with the line under it; answers the height
/// left for the rows.
pub(super) fn header(mtm: MainThreadMarker, content: &NSView, width: f64) -> f64 {
    let top = content.frame().size.height;
    let bold = NSFont::boldSystemFontOfSize(FONT_SIZE);
    for (text, (x, column_width)) in HEADERS.iter().zip(columns(width)) {
        let cell = label(mtm, text, &bold);
        cell.setFrame(centered(x, top - ROW_HEIGHT, column_width));
        content.addSubview(&cell);
    }
    let line = NSBox::initWithFrame(
        mtm.alloc::<NSBox>(),
        rect(0.0, top - ROW_HEIGHT - 1.0, width, 1.0),
    );
    line.setBoxType(NSBoxType::Separator);
    content.addSubview(&line);
    top - ROW_HEIGHT - 1.0
}

/// The time, result and link cells of a row whose top is `y`; every second row is striped.
pub(super) fn row(
    mtm: MainThreadMarker,
    document: &NSView,
    row: &[String; 3],
    opened: bool,
    index: usize,
) {
    let width = document.frame().size.width;
    let y = document.frame().size.height - (place(index) + 1.0) * ROW_HEIGHT;
    if index % 2 == 1 {
        let stripe = NSColor::alternatingContentBackgroundColors()
            .lastObject()
            .unwrap_or_else(NSColor::controlBackgroundColor);
        document.addSubview(&filled(mtm, rect(0.0, y, width, ROW_HEIGHT), &stripe));
    }
    let text = NSFont::systemFontOfSize(FONT_SIZE);
    let fixed = NSFont::userFixedPitchFontOfSize(FONT_SIZE).unwrap_or_else(|| text.clone());
    let result = if opened {
        NSColor::systemGreenColor()
    } else {
        NSColor::systemRedColor()
    };
    let colors = [NSColor::labelColor(), result, NSColor::labelColor()];
    let fonts = [&text, &text, &fixed];
    for (((value, font), color), (x, column_width)) in
        row.iter().zip(fonts).zip(colors).zip(columns(width))
    {
        let cell = label(mtm, value, font);
        cell.setTextColor(Some(&color));
        cell.setToolTip(Some(&NSString::from_str(value)));
        cell.setFrame(centered(x, y, column_width));
        document.addSubview(&cell);
    }
}

/// Where the ▶ button of row `index` goes in a document `width` wide and `height` high.
pub(super) fn button_frame(width: f64, height: f64, index: usize) -> NSRect {
    let (x, button_width) = columns(width)[3];
    let y = height - (place(index) + 1.0) * ROW_HEIGHT;
    rect(
        x,
        y + (ROW_HEIGHT - BUTTON_HEIGHT) / 2.0,
        button_width,
        BUTTON_HEIGHT,
    )
}

/// Keeps `view` at the top of its superview when the superview changes height.
pub(super) fn pin_to_top(view: &NSView) {
    view.setAutoresizingMask(NSAutoresizingMaskOptions::ViewMinYMargin);
}

fn filled(mtm: MainThreadMarker, frame: NSRect, color: &NSColor) -> Retained<NSBox> {
    let filled = NSBox::initWithFrame(mtm.alloc::<NSBox>(), frame);
    filled.setBoxType(NSBoxType::Custom);
    filled.setTitlePosition(NSTitlePosition::NoTitle);
    filled.setBorderWidth(0.0);
    filled.setContentViewMargins(NSSize::new(0.0, 0.0));
    filled.setFillColor(color);
    filled
}

fn centered(x: f64, y: f64, width: f64) -> NSRect {
    rect(x, y + (ROW_HEIGHT - TEXT_HEIGHT) / 2.0, width, TEXT_HEIGHT)
}

fn place(index: usize) -> f64 {
    f64::from(u32::try_from(index).unwrap_or(u32::MAX))
}

#[cfg(test)]
#[path = "history_table_config_test.rs"]
mod history_table_config_test;
