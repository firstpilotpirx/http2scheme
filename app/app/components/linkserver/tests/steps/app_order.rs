//! Steps of the `link-opening` capability about the app order after a click.

use std::sync::Arc;

use cucumber::{given, then};

use crate::LinkserverWorld;
use crate::fakes::FakeAppOrder;

#[given(regex = r"^the user left the Claude app for the browser$")]
fn left_claude(world: &mut LinkserverWorld) {
    world.close_tab = true;
    world.fakes.order = Arc::new(FakeAppOrder::watching(world.fakes.opener.clone()));
}

#[given(regex = r"^the app order cannot be changed$")]
fn order_fails(world: &mut LinkserverWorld) {
    world.close_tab = true;
    world.fakes.order = Arc::new(FakeAppOrder::failing());
}

#[then(regex = r"^the server brings the Claude app to the front$")]
fn brings_back(world: &mut LinkserverWorld) {
    assert_eq!(world.fakes.order.calls().len(), 1);
}

#[then(regex = r"^then the Mac opens that IntelliJ link$")]
fn opens_after(world: &mut LinkserverWorld) {
    assert_eq!(world.fakes.opener.links(), [world.link.clone().unwrap()]);
    assert_eq!(
        world.fakes.order.calls(),
        [0],
        "the app came back after the link opened"
    );
}

#[then(regex = r"^the server brings no app to the front$")]
fn brings_none(world: &mut LinkserverWorld) {
    assert_eq!(world.fakes.order.calls(), Vec::<usize>::new());
    assert_eq!(world.fakes.opener.links(), Vec::<String>::new());
}
