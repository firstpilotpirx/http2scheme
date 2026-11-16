//! Steps of the `menu-bar-app` capability for the Agents tab of the Settings window.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::fs;
use std::path::{Path, PathBuf};

use cucumber::{given, then, when};
use menubar::adapters::primary::statusmenu::appkit::agents_tab_handler::AgentsTabHandler;
use menubar::core::domain::agents_view_value_object::COPIED_NOTE;
use menubar::menubar_config::MenubarConfig;

use super::menu_bar_app::config;
use crate::MenubarWorld;
use crate::fakes::SKILL_TEXT;

const AGENTS: usize = 2;
const OLDER: &str = "---\nname: http2scheme\n---\nan older text\n";

fn default_folder(world: &MenubarWorld) -> PathBuf {
    world.home.path().join(".claude")
}

fn skill_file(folder: &Path) -> PathBuf {
    folder.join("skills/http2scheme/SKILL.md")
}

fn write_skill(folder: &Path, text: &str) {
    let file = skill_file(folder);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, text).unwrap();
}

/// Opens the Agents tab through the tab bar, does `act` on it and reads what it shows.
fn on_agents_tab(world: &mut MenubarWorld, act: impl FnOnce(&AgentsTabHandler)) {
    let mtm = MenubarConfig::main_thread().unwrap();
    let config = config(world);
    let window = config.settings_window(
        mtm,
        config.menu(),
        config.history_tab(mtm),
        config.agents_tab(mtm),
    );
    window.show_general();
    window.choose_tab(AGENTS);
    world.tab = window.selected_tab();
    let agents = window.agents();
    act(agents);
    world.agents.names = agents.names();
    world.agents.lines = agents.lines();
    world.agents.button = agents.button();
    world.agents.clipboard = agents.clipboard();
}

#[given(regex = r"^the user has never installed the skill$")]
fn never_installed(world: &mut MenubarWorld) {
    assert!(!default_folder(world).exists());
}

#[given(
    regex = r"^the Agents tab (?:shows the default folder|shows a folder without the skill|is shown)$"
)]
fn tab_shown(world: &mut MenubarWorld) {
    on_agents_tab(world, |_| {});
    assert_eq!(world.tab, "Agents");
    assert_eq!(world.agents.lines[0], "~/.claude");
}

#[given(regex = r"^the Claude Code folder's skills folder is a link to a folder elsewhere$")]
fn linked_skills(world: &mut MenubarWorld) {
    let elsewhere = world.home.path().join("elsewhere/skills");
    fs::create_dir_all(&elsewhere).unwrap();
    fs::create_dir_all(default_folder(world)).unwrap();
    std::os::unix::fs::symlink(&elsewhere, default_folder(world).join("skills")).unwrap();
}

#[given(regex = r"^the shown folder holds an older version of the skill$")]
fn older_skill(world: &mut MenubarWorld) {
    write_skill(&default_folder(world), OLDER);
    on_agents_tab(world, |_| {});
    assert_eq!(world.agents.button, "Update");
}

#[when(regex = r"^the user opens the Agents tab$")]
fn opens_tab(world: &mut MenubarWorld) {
    on_agents_tab(world, |_| {});
}

#[when(regex = r"^the user chooses a work Claude Code folder that holds the current skill$")]
fn chooses_work(world: &mut MenubarWorld) {
    let work = world.home.path().join("work-claude");
    write_skill(&work, SKILL_TEXT);
    on_agents_tab(world, |agents| agents.choose_folder(&work));
}

#[when(regex = r"^the user clicks (Install|Update)$")]
fn clicks_install(world: &mut MenubarWorld, title: String) {
    on_agents_tab(world, |agents| {
        assert_eq!(agents.button(), title);
        agents.install_skill();
    });
}

#[when(regex = r"^the user clicks Copy skill text$")]
fn clicks_copy(world: &mut MenubarWorld) {
    on_agents_tab(world, AgentsTabHandler::copy);
}

#[then(regex = r"^it shows the default Claude Code folder$")]
fn shows_default(world: &mut MenubarWorld) {
    assert_eq!(world.agents.names[0], "Claude Code folder");
    assert_eq!(world.agents.lines[0], "~/.claude");
}

#[then(regex = r"^(?:it|the tab) says the skill is (not installed|installed)$")]
fn says_state(world: &mut MenubarWorld, state: String) {
    let label = if state == "installed" {
        "Installed"
    } else {
        "Not installed"
    };
    assert_eq!(world.agents.names[1], format!("Skill · {label}"));
}

#[then(regex = r"^the tab shows the work folder$")]
fn shows_work(world: &mut MenubarWorld) {
    assert_eq!(world.agents.lines[0], "~/work-claude");
    assert_eq!(
        world.agents.lines[1],
        "~/work-claude/skills/http2scheme/SKILL.md"
    );
}

#[then(regex = r"^the tab shows the skill file's place inside the folder the link points to$")]
fn shows_real_place(world: &mut MenubarWorld) {
    let real = fs::canonicalize(world.home.path().join("elsewhere/skills")).unwrap();
    let home = fs::canonicalize(world.home.path()).unwrap();
    let short = real.strip_prefix(&home).map_or_else(
        |_| real.display().to_string(),
        |rest| format!("~/{}", rest.display()),
    );
    let expected = format!(
        "Lands in {} (skills is a link)",
        Path::new(&short).join("http2scheme/SKILL.md").display()
    );
    let alternative = format!(
        "Lands in {}/http2scheme/SKILL.md (skills is a link)",
        real.display()
    );
    assert!(
        world.agents.lines[1] == expected || world.agents.lines[1] == alternative,
        "{}",
        world.agents.lines[1]
    );
}

#[then(regex = r"^the skill file (?:appears in that folder's skills|holds the current text)$")]
fn skill_written(world: &mut MenubarWorld) {
    let text = fs::read_to_string(skill_file(&default_folder(world))).unwrap();
    assert_eq!(text, SKILL_TEXT);
}

#[then(regex = r"^the clipboard holds the skill text$")]
fn clipboard_holds(world: &mut MenubarWorld) {
    assert_eq!(world.agents.clipboard.as_deref(), Some(SKILL_TEXT));
}

#[then(regex = r"^the tab says the text was copied$")]
fn says_copied(world: &mut MenubarWorld) {
    assert_eq!(world.agents.lines[2], COPIED_NOTE);
}

fn work_folder(world: &MenubarWorld) -> PathBuf {
    world.home.path().join("work-claude")
}

fn install_into_work(world: &mut MenubarWorld) {
    let work = work_folder(world);
    fs::create_dir_all(&work).unwrap();
    on_agents_tab(world, |agents| {
        agents.choose_folder(&work);
        agents.install_skill();
    });
    assert_eq!(fs::read_to_string(skill_file(&work)).unwrap(), SKILL_TEXT);
}

#[given(
    regex = r"^the user installed the skill from the app into a folder and that folder holds an older text$"
)]
fn installed_then_older(world: &mut MenubarWorld) {
    install_into_work(world);
    write_skill(&work_folder(world), OLDER);
}

#[given(regex = r"^the user installed the skill from the app and later deleted the skill file$")]
fn installed_then_deleted(world: &mut MenubarWorld) {
    install_into_work(world);
    fs::remove_file(skill_file(&work_folder(world))).unwrap();
}

#[when(regex = r"^the app launches$")]
fn app_launches(world: &mut MenubarWorld) {
    config(world).refresh_skill().unwrap();
}

#[then(regex = r"^that folder's skill holds the app's text$")]
fn refreshed(world: &mut MenubarWorld) {
    let text = fs::read_to_string(skill_file(&work_folder(world))).unwrap();
    assert_eq!(text, SKILL_TEXT);
}

#[then(regex = r"^no skill file is written$")]
fn nothing_written(world: &mut MenubarWorld) {
    assert!(!skill_file(&work_folder(world)).exists());
    assert!(!skill_file(&default_folder(world)).exists());
}
