//! Steps of the `agent-skill` capability on the built binary.

use cucumber::{given, then, when};

use crate::E2eWorld;
use crate::support::sut_bin;

fn skill_file(world: &E2eWorld) -> std::path::PathBuf {
    world
        .home
        .path()
        .join(".claude/skills/http2scheme/SKILL.md")
}

#[given(regex = r"^the program is installed at a known path$")]
fn known_path(_world: &mut E2eWorld) {
    assert!(sut_bin().is_absolute(), "{}", sut_bin().display());
}

#[given(regex = r"^the user's Claude Code folder has no skill of this program$")]
fn no_skill(world: &mut E2eWorld) {
    assert!(!skill_file(world).exists());
}

#[when(regex = r"^the user asks the program for the skill text$")]
async fn asks_for_skill(world: &mut E2eWorld) {
    let (code, out, err) = world.home.run(&["skill"]).await;
    (world.printed.code, world.printed.out, world.printed.err) = (Some(code), out, err);
}

#[when(regex = r"^the user installs the skill without naming a folder$")]
async fn installs(world: &mut E2eWorld) {
    let (code, out, err) = world.home.run(&["skill", "install"]).await;
    (world.printed.code, world.printed.out, world.printed.err) = (Some(code), out, err);
}

#[then(
    regex = r"^the text starts with a header naming the skill http2scheme and saying when to use it$"
)]
fn header(world: &mut E2eWorld) {
    let text = &world.printed.out;
    assert!(
        text.starts_with("---\nname: http2scheme\ndescription: "),
        "{text}"
    );
    assert!(text.contains("Use whenever"), "{text}");
    assert_eq!(world.printed.code, Some(0));
}

#[then(regex = r"^it names that program path for the link command$")]
fn names_program(world: &mut E2eWorld) {
    let program = std::fs::canonicalize(sut_bin()).unwrap();
    let call = format!("'{}' link ", program.display());
    let plain = format!("'{}' link ", sut_bin().display());
    let text = &world.printed.out;
    assert!(text.contains(&call) || text.contains(&plain), "{text}");
    assert!(!text.contains("{{program}}"), "{text}");
}

#[then(regex = r"^its example shows a file link followed by a link labelled IDE$")]
fn example(world: &mut E2eWorld) {
    let text = &world.printed.out;
    assert!(
        text.contains("[server.rs:42](app/src/server.rs:42) [IDE](http://127.0.0.1:"),
        "{text}"
    );
}

#[then(regex = r"^the skill file appears in the default Claude Code folder's skills$")]
fn file_appears(world: &mut E2eWorld) {
    let text = std::fs::read_to_string(skill_file(world)).unwrap();
    assert!(text.starts_with("---\nname: http2scheme\n"), "{text}");
}

#[then(regex = r"^the command prints the path of that file and succeeds$")]
fn prints_path(world: &mut E2eWorld) {
    let file = skill_file(world);
    assert!(
        world.printed.out.contains(&file.display().to_string()),
        "{}",
        world.printed.out
    );
    assert_eq!(world.printed.code, Some(0));
}
