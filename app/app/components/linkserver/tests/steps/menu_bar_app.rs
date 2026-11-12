//! Steps of the `menu-bar-app` capability that reach the server: ▶ on a Link History row runs
//! the root's replay handler, whose newest logged attempt is the window's first row.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use cucumber::{given, then, when};

use super::home::{config, free_port, write_settings};
use crate::LinkserverWorld;

const INTELLIJ: &str = "idea://open?file=/src/Main.java&line=1";
const SLACK: &str = "slack://channel?id=C1";

#[given(
    regex = r#"^the Link History window lists an? (?:opened )?(IntelliJ|Slack) link(?: with the result ".+")?$"#
)]
fn window_lists(world: &mut LinkserverWorld, name: String) {
    write_settings(world, free_port(), &["idea"]);
    world.link = Some(if name == "IntelliJ" { INTELLIJ } else { SLACK }.to_owned());
}

#[when(regex = r"^the user chooses ▶ on its row$")]
async fn replays(world: &mut LinkserverWorld) {
    let link = world.link.clone().unwrap();
    world.answer.body = config(world).replay_handler().replay(&link).await;
}

#[then(regex = r"^the Mac opens nothing$")]
fn opens_nothing(world: &mut LinkserverWorld) {
    assert_eq!(world.fakes.opener.links(), Vec::<String>::new());
}

#[then(regex = r#"^the first row shows that (?:IntelliJ|Slack) link with the result "(.+)"$"#)]
fn first_row(world: &mut LinkserverWorld, result: String) {
    assert_eq!(world.answer.body, result);
    let events = world.fakes.log.events();
    let newest = events.last().unwrap();
    assert_eq!(newest.link_text(), world.link.clone().unwrap());
    assert_eq!(newest.outcome().label(), result);
}
