//! Steps of the `agent-skill` capability, driven through the skill commands over a temporary home.

use cucumber::{given, then, when};

use super::home::{PROGRAM, config, reset_output};
use crate::LinkserverWorld;

#[given(regex = r"^the program is installed at a known path$")]
fn program_at_known_path(_world: &mut LinkserverWorld) {
    assert!(PROGRAM.starts_with('/'), "the scenarios' program is a full path");
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
    assert!(header.contains("description: ") && header.contains("Use whenever"), "{header}");
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
