//! The rows of the Agents tab: their names, buttons and layout (design D9).

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSButton, NSColor, NSFont, NSTextField, NSView};

use super::history_table_config::{pin_to_top, rect};
use super::settings_layout_config::{
    AGENTS_HEIGHT as HEIGHT, LINE_HEIGHT, MARGIN, NAME_HEIGHT, NAME_SIZE, NOTE_SIZE, ROW_HEIGHT,
    WIDTH, label,
};
use crate::core::domain::skill_state_enum::SkillStateEnum;

pub(super) const NAMES: [&str; 3] = ["Claude Code folder", "Skill", "Skill text"];
pub(super) const BUTTONS: [&str; 3] = ["Choose…", "Install", "Copy skill text"];
pub(super) const SEPARATOR: &str = " ·";
const BUTTON_WIDTH: f64 = 140.0;
const BUTTON_HEIGHT: f64 = 24.0;
pub(super) const GAP: f64 = 4.0;

/// One row: its name, the coloured state after the name (the skill row only), the grey line under
/// the name and the button on the right.
pub(super) struct Row {
    pub(super) name: Retained<NSTextField>,
    pub(super) state: Retained<NSTextField>,
    pub(super) line: Retained<NSTextField>,
    pub(super) button: Retained<NSButton>,
}

pub(super) fn row(
    mtm: MainThreadMarker,
    view: &NSView,
    index: usize,
    button: Retained<NSButton>,
) -> Row {
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

/// The skill row's name: `Skill ·` when the state follows it, else `Skill`.
pub(super) fn skill_name(state: Option<SkillStateEnum>) -> String {
    match state {
        Some(_) => format!("{}{SEPARATOR}", NAMES[1]),
        None => NAMES[1].to_owned(),
    }
}

#[cfg(test)]
#[path = "agents_row_config_test.rs"]
mod agents_row_config_test;
