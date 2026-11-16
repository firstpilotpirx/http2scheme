//! Steps of the `menu-bar-app` capability that reach the server: ▶ on a History tab row runs
//! the root's replay handler, whose newest logged attempt is the tab's first row; the Agents tab
//! runs the root's skill handlers.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::time::SystemTime;

use cucumber::{given, then, when};
use linkserver::core::application::repositories::open_attempt_repository::OpenAttemptRepository;
use linkserver::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;
use linkserver::core::domain::link_rejected_error::LinkRejectedError;
use linkserver::core::domain::open_outcome_enum::OpenOutcomeEnum;

use super::agent_skill::{current_text, skill_file, work_dir};
use super::home::{config, free_port, write_settings};
use crate::LinkserverWorld;

const INTELLIJ: &str = "idea://open?file=/src/Main.java&line=1";
const SLACK: &str = "slack://channel?id=C1";

#[given(
    regex = r#"^the History tab lists (an opened IntelliJ|a Slack) link(?: with the result "scheme not allowed")?$"#
)]
async fn window_lists(world: &mut LinkserverWorld, row: String) {
    write_settings(world, free_port(), &["idea"]);
    let (link, outcome) = if row == "a Slack" {
        let scheme = "slack".to_owned();
        (
            SLACK,
            OpenOutcomeEnum::Rejected(LinkRejectedError::SchemeNotAllowed { scheme }),
        )
    } else {
        (INTELLIJ, OpenOutcomeEnum::Opened)
    };
    let earlier = LinkOpenAttemptedDomainEvent::new(SystemTime::now(), link.to_owned(), outcome);
    world.fakes.log.record(&earlier).await.unwrap();
    world.link = Some(link.to_owned());
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
    assert_eq!(events.len(), 2, "the earlier row and the new attempt");
    let newest = events.last().unwrap();
    assert_eq!(newest.link_text(), world.link.clone().unwrap());
    assert_eq!(newest.outcome().label(), result);
    assert!(newest.at() >= events[0].at());
    assert_eq!(
        world.fakes.closer.ports(),
        Vec::<u16>::new(),
        "no tab closes"
    );
}

#[given(regex = r"^a work Claude Code folder holds the current skill$")]
fn work_holds_current(world: &mut LinkserverWorld) {
    let file = skill_file(&work_dir(world));
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(file, current_text()).unwrap();
}

#[when(regex = r"^the Agents tab asks the server (about|to install the skill into) that folder$")]
async fn agents_tab_asks(world: &mut LinkserverWorld, request: String) {
    let dir = work_dir(world);
    let root = config(world);
    let answer = if request == "about" {
        root.inspect_skill_handler().inspect(&dir).await
    } else {
        root.install_skill_handler().install(&dir).await
    };
    let (file, real_file, state) = answer.unwrap();
    assert_eq!(real_file, None);
    world.answer.body = format!("{state} {}", file.display());
}

#[then(regex = r"^the server answers that the skill is installed in that folder$")]
fn answers_installed(world: &mut LinkserverWorld) {
    let file = skill_file(&work_dir(world));
    assert_eq!(world.answer.body, format!("installed {}", file.display()));
}

#[when(regex = r"^the Agents tab asks the server for the skill text$")]
async fn asks_for_text(world: &mut LinkserverWorld) {
    world.answer.body = config(world).skill_text_handler().text().await;
}

#[then(regex = r"^the answer is the text the skill command prints$")]
async fn answer_is_printed_text(world: &mut LinkserverWorld) {
    let command = config(world).skill_command();
    let (mut out, mut err) = (world.printed.out.clone(), world.printed.err.clone());
    assert_eq!(command.run(&mut out, &mut err).await, 0);
    assert_eq!(world.answer.body, world.printed.out.text());
    assert_eq!(world.answer.body, current_text());
}
