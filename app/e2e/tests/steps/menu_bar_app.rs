//! Steps of the `menu-bar-app` capability on the built app bundle.

use std::time::Duration;

use cucumber::{given, then, when};

use crate::E2eWorld;
use crate::support::{answers, bundle_program, free_port, wait_until_answering};

const INTELLIJ: &str = "idea://open?file=/src/Main.java&line=1";

#[given(
    regex = r"^the app (?:is started from its bundle with no command|runs and its server answers)$"
)]
async fn app_started(world: &mut E2eWorld) {
    let port = free_port();
    world.home.write_settings(port, &["idea", "goland"], true);
    let mut app = world
        .home
        .program_command(&bundle_program(), &[])
        .spawn()
        .unwrap();
    assert!(
        wait_until_answering(&mut app, Some(port)).await,
        "the app did not serve"
    );
    world.port = Some(port);
    world.server = Some(app);
}

#[given(regex = r"^the program inside the app bundle$")]
fn bundle_program_exists(world: &mut E2eWorld) {
    assert!(bundle_program().is_file());
    world.port = Some(free_port());
    world
        .home
        .write_settings(world.port.unwrap(), &["idea"], true);
}

#[when(regex = r"^the user asks it for the open link of an IntelliJ link$")]
async fn asks_link(world: &mut E2eWorld) {
    let output = world
        .home
        .program_command(&bundle_program(), &["link", INTELLIJ])
        .output()
        .await
        .unwrap();
    world.printed.code = output.status.code();
    world.printed.out = String::from_utf8(output.stdout).unwrap();
    world.printed.err = String::from_utf8(output.stderr).unwrap();
}

#[then(regex = r"^it prints one open link to the loopback address$")]
fn prints_link(world: &mut E2eWorld) {
    let prefix = format!("http://127.0.0.1:{}/open?key=", world.port.unwrap());
    let lines: Vec<&str> = world.printed.out.lines().collect();
    assert_eq!(lines.len(), 1, "{}", world.printed.err);
    assert!(lines[0].starts_with(&prefix), "{}", world.printed.err);
    assert_eq!(world.printed.code, Some(0));
}

#[when(regex = r"^the app receives a terminate signal$")]
fn terminate(world: &mut E2eWorld) {
    let pid = world.server.as_ref().unwrap().id().unwrap();
    let status = std::process::Command::new("/bin/kill")
        .args(["-TERM", &pid.to_string()])
        .status()
        .unwrap();
    assert!(status.success());
}

#[then(regex = r"^the app ends$")]
async fn app_ends(world: &mut E2eWorld) {
    let app = world.server.as_mut().unwrap();
    let status = tokio::time::timeout(Duration::from_secs(5), app.wait())
        .await
        .unwrap()
        .unwrap();
    assert!(status.success(), "{status}");
}

#[then(regex = r"^the server no longer answers$")]
async fn no_longer_answers(world: &mut E2eWorld) {
    assert!(!answers(world.port.unwrap()).await);
}
