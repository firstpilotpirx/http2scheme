//! Steps of the `app-distribution` capability on the built bundle and disk image.
#![allow(
    clippy::needless_pass_by_value,
    reason = "cucumber hands captures over by value"
)]

use std::path::{Path, PathBuf};
use std::process::Command;

use cucumber::{given, then, when};

use crate::E2eWorld;
use crate::support::{bundle_app, bundle_dmg};

fn info(key: &str) -> String {
    let plist = bundle_app().join("Contents/Info.plist");
    let output = Command::new("/usr/bin/plutil")
        .args(["-extract", key, "raw", "-o", "-"])
        .arg(&plist)
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn workspace_version() -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../app/Cargo.toml");
    std::fs::read_to_string(manifest)
        .unwrap()
        .split("[workspace.package]")
        .nth(1)
        .and_then(|table| {
            table
                .lines()
                .find_map(|line| line.strip_prefix("version = "))
        })
        .unwrap()
        .trim_matches('"')
        .to_owned()
}

fn record(world: &mut E2eWorld, output: &std::process::Output) {
    world.printed.code = output.status.code();
    world.printed.out = String::from_utf8_lossy(&output.stdout).into_owned();
    world.printed.err = String::from_utf8_lossy(&output.stderr).into_owned();
}

#[given(regex = r"^the app bundle is built$")]
fn bundle_built(_world: &mut E2eWorld) {
    assert!(bundle_app().join("Contents/Info.plist").is_file());
}

#[when(regex = r"^the bundle's information is read$")]
fn read_info(world: &mut E2eWorld) {
    world.printed.out = info("CFBundleIdentifier");
}

#[then(regex = r#"^it names the bundle identifier "(.+)" and the workspace version$"#)]
fn names_identifier(world: &mut E2eWorld, identifier: String) {
    assert_eq!(world.printed.out, identifier);
    assert_eq!(info("CFBundleShortVersionString"), workspace_version());
    assert_eq!(info("CFBundleVersion"), workspace_version());
}

#[then(regex = r"^it marks the app as a menu bar app with no Dock icon$")]
fn menu_bar_app(_world: &mut E2eWorld) {
    assert_eq!(info("LSUIElement"), "true");
}

#[then(regex = r"^it gives the reason for controlling a browser$")]
fn browser_reason(_world: &mut E2eWorld) {
    assert!(info("NSAppleEventsUsageDescription").contains("browser tab"));
}

#[when(regex = r"^its signature is checked strictly$")]
fn check_signature(world: &mut E2eWorld) {
    let output = Command::new("/usr/bin/codesign")
        .args(["--verify", "--deep", "--strict"])
        .arg(bundle_app())
        .output()
        .unwrap();
    record(world, &output);
}

#[then(regex = r"^the check passes$")]
fn check_passes(world: &mut E2eWorld) {
    assert_eq!(world.printed.code, Some(0), "{}", world.printed.err);
}

#[given(regex = r"^no code-signing certificate is named$")]
fn no_certificate(_world: &mut E2eWorld) {}

#[when(regex = r"^the release checks its signing identity$")]
fn release_checks(world: &mut E2eWorld) {
    let justfile = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../justfile");
    let output = Command::new("just")
        .arg("--justfile")
        .arg(justfile)
        .arg("_require-release-identity")
        .env("HTTP2SCHEME_SIGNING_IDENTITY", "")
        .output()
        .unwrap();
    record(world, &output);
}

#[then(regex = r"^it stops and names the variable to set$")]
fn stops(world: &mut E2eWorld) {
    assert_eq!(world.printed.code, Some(1));
    assert!(
        world.printed.err.contains("HTTP2SCHEME_SIGNING_IDENTITY"),
        "{}",
        world.printed.err
    );
}

#[given(regex = r"^the disk image(?: and its checksum file)? (?:is|are) built$")]
fn image_built(_world: &mut E2eWorld) {
    assert!(bundle_dmg().is_file());
}

#[when(regex = r"^the image is opened$")]
fn open_image(world: &mut E2eWorld) {
    let mount = world.home.path().join("volume");
    let attach = Command::new("/usr/bin/hdiutil")
        .args([
            "attach",
            "-nobrowse",
            "-readonly",
            "-noautoopen",
            "-mountpoint",
        ])
        .arg(&mount)
        .arg(bundle_dmg())
        .output()
        .unwrap();
    assert!(
        attach.status.success(),
        "{}",
        String::from_utf8_lossy(&attach.stderr)
    );
    let app = mount
        .join("http2scheme.app/Contents/MacOS/http2scheme")
        .is_file();
    let link = std::fs::read_link(mount.join("Applications")).unwrap_or_default();
    let detach = Command::new("/usr/bin/hdiutil")
        .arg("detach")
        .arg(&mount)
        .status()
        .unwrap();
    assert!(detach.success());
    world.printed.out = format!("{app} {}", link.display());
}

#[then(regex = r#"^it holds "http2scheme.app" and a link to "(.+)"$"#)]
fn holds(world: &mut E2eWorld, target: String) {
    assert_eq!(
        world.printed.out,
        format!("true {}", PathBuf::from(target).display())
    );
}

#[when(regex = r"^the checksum is checked$")]
fn check_checksum(world: &mut E2eWorld) {
    let dmg = bundle_dmg();
    let name = dmg.file_name().unwrap().to_string_lossy().into_owned();
    let output = Command::new("/usr/bin/shasum")
        .args(["-a", "256", "-c", &format!("{name}.sha256")])
        .current_dir(dmg.parent().unwrap())
        .output()
        .unwrap();
    record(world, &output);
}
