//! Steps of the `agent-skill` capability, driven through the skill commands over a temporary home.

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use cucumber::{given, then, when};
use linkserver::core::domain::agent_skill_value_object::AgentSkillValueObject;

use super::home::{PROGRAM, config, home, reset_output};
use crate::LinkserverWorld;

#[given(regex = r"^the program is installed at a known path$")]
fn program_at_known_path(_world: &mut LinkserverWorld) {
    assert!(
        PROGRAM.starts_with('/'),
        "the scenarios' program is a full path"
    );
}

#[when(regex = r"^the user asks the program for the skill text$")]
async fn ask_for_skill(world: &mut LinkserverWorld) {
    reset_output(world);
    let command = config(world).skill_command();
    let (mut out, mut err) = (world.printed.out.clone(), world.printed.err.clone());
    world.printed.code = Some(command.run(&mut out, &mut err).await);
}

#[then(
    regex = r"^the text starts with a header naming the skill http2scheme and saying when to use it$"
)]
fn header_names_skill(world: &mut LinkserverWorld) {
    let text = world.printed.out.text();
    assert!(text.starts_with("---\nname: http2scheme\n"), "{text}");
    let header = text.split("\n---\n").next().unwrap();
    assert!(
        header.contains("description: ") && header.contains("Use whenever"),
        "{header}"
    );
    assert_eq!(world.printed.code, Some(0));
}

#[then(regex = r"^it names that program path for the link command$")]
fn names_program(world: &mut LinkserverWorld) {
    let text = world.printed.out.text();
    assert!(text.contains(&format!("'{PROGRAM}' link '")), "{text}");
}

#[then(regex = r"^its example shows a file link followed by a link labelled IDE$")]
fn example_has_ide_link(world: &mut LinkserverWorld) {
    let text = world.printed.out.text();
    assert!(text.contains(":42) [IDE](http://127.0.0.1:"), "{text}");
}

/// The scenario's default Claude Code folder, `~/.claude` of the temporary home.
fn default_dir(world: &mut LinkserverWorld) -> PathBuf {
    home(world).join(".claude")
}

/// The second Claude Code folder kept for work.
fn work_dir(world: &mut LinkserverWorld) -> PathBuf {
    home(world).join("work-claude")
}

/// The folder a linked skills folder points to.
fn elsewhere(world: &mut LinkserverWorld) -> PathBuf {
    home(world).join("elsewhere")
}

fn skill_file(dir: &Path) -> PathBuf {
    dir.join("skills/http2scheme/SKILL.md")
}

fn current_text() -> String {
    AgentSkillValueObject::for_program(PROGRAM)
        .text()
        .to_owned()
}

async fn install(world: &mut LinkserverWorld, dir: Option<PathBuf>) {
    reset_output(world);
    let command = config(world).install_skill_command();
    let (mut out, mut err) = (world.printed.out.clone(), world.printed.err.clone());
    world.printed.code = Some(command.run(dir.as_deref(), &mut out, &mut err).await);
}

#[given(regex = r"^the user's Claude Code folder has no skill of this program$")]
fn no_skill(world: &mut LinkserverWorld) {
    assert!(!skill_file(&default_dir(world)).exists());
}

#[given(regex = r"^the user keeps a second Claude Code folder for work$")]
fn work_folder(world: &mut LinkserverWorld) {
    fs::create_dir_all(work_dir(world)).unwrap();
}

#[given(
    regex = r"^the Claude Code folder holds an older text of the skill and another skill beside it$"
)]
fn older_and_sibling(world: &mut LinkserverWorld) {
    let dir = default_dir(world);
    fs::create_dir_all(skill_file(&dir).parent().unwrap()).unwrap();
    fs::write(skill_file(&dir), "an older skill").unwrap();
    let other = dir.join("skills/other/SKILL.md");
    fs::create_dir_all(other.parent().unwrap()).unwrap();
    fs::write(other, "another skill").unwrap();
}

#[given(regex = r"^the Claude Code folder's skills folder is a link to a folder elsewhere$")]
fn linked_skills(world: &mut LinkserverWorld) {
    let (dir, target) = (default_dir(world), elsewhere(world));
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&target).unwrap();
    symlink(&target, dir.join("skills")).unwrap();
}

#[when(regex = r"^the user installs the skill( without naming a folder)?$")]
async fn install_default(world: &mut LinkserverWorld) {
    install(world, None).await;
}

#[when(regex = r"^the user installs the skill naming that folder$")]
async fn install_work(world: &mut LinkserverWorld) {
    let dir = work_dir(world);
    install(world, Some(dir)).await;
}

#[then(regex = r"^the skill file appears in the default Claude Code folder's skills$")]
fn file_in_default(world: &mut LinkserverWorld) {
    let file = skill_file(&default_dir(world));
    assert_eq!(fs::read_to_string(file).unwrap(), current_text());
}

#[then(regex = r"^the command prints the path of that file and succeeds$")]
fn prints_path(world: &mut LinkserverWorld) {
    let file = skill_file(&default_dir(world)).display().to_string();
    let text = world.printed.out.text();
    assert!(text.contains(&file), "{text}");
    assert_eq!(world.printed.code, Some(0));
}

#[then(regex = r"^the skill file appears in that folder's skills$")]
fn file_in_work(world: &mut LinkserverWorld) {
    let file = skill_file(&work_dir(world));
    assert_eq!(fs::read_to_string(file).unwrap(), current_text());
}

#[then(regex = r"^the default Claude Code folder stays untouched$")]
fn default_untouched(world: &mut LinkserverWorld) {
    assert!(!default_dir(world).exists());
}

#[then(regex = r"^the skill file holds the current text$")]
fn holds_current(world: &mut LinkserverWorld) {
    file_in_default(world);
}

#[then(regex = r"^the other skill is unchanged$")]
fn other_unchanged(world: &mut LinkserverWorld) {
    let other = default_dir(world).join("skills/other/SKILL.md");
    assert_eq!(fs::read_to_string(other).unwrap(), "another skill");
}

#[then(regex = r"^the skill file appears in the folder the link points to$")]
fn file_in_target(world: &mut LinkserverWorld) {
    let file = elsewhere(world).join("http2scheme/SKILL.md");
    assert_eq!(fs::read_to_string(file).unwrap(), current_text());
}

#[then(regex = r"^the command also prints that real folder$")]
fn prints_real_folder(world: &mut LinkserverWorld) {
    let real = fs::canonicalize(elsewhere(world)).unwrap();
    let text = world.printed.out.text();
    assert!(
        text.contains(&format!("lands in {}", real.display())),
        "{text}"
    );
}
