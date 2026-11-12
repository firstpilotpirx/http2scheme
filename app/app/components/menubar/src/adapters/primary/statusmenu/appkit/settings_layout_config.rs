//! The layout of the Settings window as the approved mock draws it: a General and History tab bar
//! over a line, the General rows with their switches, and a window as high as the shown tab
//! (design D19).

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSBox, NSBoxType, NSColor, NSFont, NSSegmentSwitchTracking,
    NSSegmentedControl, NSSwitch, NSTabView, NSTabViewItem, NSTabViewType, NSTextField, NSView,
};
use objc2_foundation::{NSSize, NSString};

use super::history_tab_handler;
use super::history_table_config::{pin_to_top, rect};

/// The width of the window.
pub(super) const WIDTH: f64 = 720.0;
/// The tab bar: the segmented control and the line under it.
pub(super) const BAR: f64 = 40.0;
const MARGIN: f64 = 20.0;
const ROW_HEIGHT: f64 = 56.0;
/// The General tab is as high as its two rows, with no empty space under them.
pub(super) const GENERAL_HEIGHT: f64 = 2.0 * MARGIN + 2.0 * ROW_HEIGHT - 14.0;
const TABS: [&str; 2] = ["General", "History"];
const NAMES: [&str; 2] = ["Serve links", "Start at login"];
const ROW_WIDTH: f64 = 360.0;
const NAME_HEIGHT: f64 = 20.0;
const LINE_HEIGHT: f64 = 16.0;
const SWITCH_SIZE: NSSize = NSSize::new(38.0, 22.0);
const NAME_SIZE: f64 = 13.0;
const NOTE_SIZE: f64 = 12.0;

/// The height of the window's content while tab `tab` shows: the tab bar and that tab.
pub(super) fn tab_height(tab: usize) -> f64 {
    BAR + [GENERAL_HEIGHT, history_tab_handler::HEIGHT][tab.min(1)]
}

/// The General and History segments centered at the top of `content`, with a line under them
/// across the window, as the mock's tab bar.
pub(super) fn tab_bar(mtm: MainThreadMarker, content: &NSView) -> Retained<NSSegmentedControl> {
    let top = content.frame().size.height;
    let segments = NSSegmentedControl::new(mtm);
    segments.setSegmentCount(2);
    for (index, label) in (0..).zip(TABS) {
        segments.setLabel_forSegment(&NSString::from_str(label), index);
    }
    segments.setTrackingMode(NSSegmentSwitchTracking::SelectOne);
    segments.setSelectedSegment(0);
    segments.sizeToFit();
    let size = segments.frame().size;
    segments.setFrame(rect(
        (WIDTH - size.width) / 2.0,
        top - BAR + (BAR - size.height) / 2.0,
        size.width,
        size.height,
    ));
    segments.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewMinYMargin
            | NSAutoresizingMaskOptions::ViewMinXMargin
            | NSAutoresizingMaskOptions::ViewMaxXMargin,
    );
    let line = NSBox::initWithFrame(mtm.alloc::<NSBox>(), rect(0.0, top - BAR, WIDTH, 1.0));
    line.setBoxType(NSBoxType::Separator);
    line.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMinYMargin,
    );
    content.addSubview(&segments);
    content.addSubview(&line);
    segments
}

/// The tab view under the tab bar: General and History, with no tabs or border of its own.
pub(super) fn tab_view(
    mtm: MainThreadMarker,
    general: &NSView,
    history: &NSView,
) -> Retained<NSTabView> {
    let tabs = NSTabView::initWithFrame(
        mtm.alloc::<NSTabView>(),
        rect(0.0, 0.0, WIDTH, GENERAL_HEIGHT),
    );
    tabs.setTabViewType(NSTabViewType::NoTabsNoBorder);
    tabs.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );
    for (label, view) in TABS.iter().zip([general, history]) {
        let item = NSTabViewItem::new();
        item.setLabel(&NSString::from_str(label));
        item.setView(Some(view));
        tabs.addTabViewItem(&item);
    }
    tabs
}

/// The name, the line under it and the switch of row `index` of the General tab.
pub(super) fn switch_row(
    mtm: MainThreadMarker,
    general: &NSView,
    index: usize,
) -> (Retained<NSSwitch>, Retained<NSTextField>) {
    let place = f64::from(u32::try_from(index).unwrap_or_default());
    let top = general.frame().size.height - MARGIN - place * ROW_HEIGHT;
    let width = ROW_WIDTH - SWITCH_SIZE.width - MARGIN;
    let title = label(
        mtm,
        NAMES[index.min(1)],
        &NSFont::systemFontOfSize(NAME_SIZE),
    );
    title.setFrame(rect(MARGIN, top - NAME_HEIGHT, width, NAME_HEIGHT));
    let line = label(mtm, "", &NSFont::systemFontOfSize(NOTE_SIZE));
    line.setTextColor(Some(&NSColor::secondaryLabelColor()));
    line.setFrame(rect(
        MARGIN,
        top - NAME_HEIGHT - 4.0 - LINE_HEIGHT,
        2.0 * width,
        LINE_HEIGHT,
    ));
    let switch = NSSwitch::new(mtm);
    let switch_x = MARGIN + ROW_WIDTH - SWITCH_SIZE.width;
    switch.setFrame(rect(
        switch_x,
        top - SWITCH_SIZE.height,
        SWITCH_SIZE.width,
        SWITCH_SIZE.height,
    ));
    pin_to_top(&title);
    pin_to_top(&line);
    pin_to_top(&switch);
    general.addSubview(&title);
    general.addSubview(&line);
    general.addSubview(&switch);
    (switch, line)
}

fn label(mtm: MainThreadMarker, text: &str, font: &NSFont) -> Retained<NSTextField> {
    let field = NSTextField::labelWithString(&NSString::from_str(text), mtm);
    field.setFont(Some(font));
    field
}

#[cfg(test)]
#[path = "settings_layout_config_test.rs"]
mod settings_layout_config_test;
