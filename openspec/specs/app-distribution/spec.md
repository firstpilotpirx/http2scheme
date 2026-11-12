# app-distribution Specification

## Purpose
Install http2scheme the standard Mac way: a disk image the user opens and drags the app from into Applications, published with each release together with its checksum.

## Requirements

### Requirement: The app bundle carries the app's information

The build SHALL produce `http2scheme.app` whose information names the bundle identifier `io.github.firstpilotpirx.http2scheme`, the version from `[workspace.package]` of `app/app/Cargo.toml`, the program `http2scheme`, its icon, macOS 15.0 as the lowest system, a menu bar app with no Dock icon, and the reason shown when macOS asks to let the app control a browser.

#### Scenario: The bundle's information

- **GIVEN** the app bundle is built
- **WHEN** the bundle's information is read
- **THEN** it names the bundle identifier "io.github.firstpilotpirx.http2scheme" and the workspace version
- **AND** it marks the app as a menu bar app with no Dock icon
- **AND** it gives the reason for controlling a browser

### Requirement: The bundle is signed

The build SHALL sign the bundle with the code-signing certificate named by `HTTP2SCHEME_SIGNING_IDENTITY`, or ad-hoc with a notice when the variable is empty, and the signature SHALL pass a strict check. A named certificate that is not in the keychain SHALL stop the build with the steps to create it. A release SHALL stop when no certificate is named.

#### Scenario: A built bundle passes the signature check

- **GIVEN** the app bundle is built
- **WHEN** its signature is checked strictly
- **THEN** the check passes

#### Scenario: A release without a certificate stops

- **GIVEN** no code-signing certificate is named
- **WHEN** the release checks its signing identity
- **THEN** it stops and names the variable to set

### Requirement: The disk image installs the app

The build SHALL produce `http2scheme-vX.Y.Z.dmg` that holds the app bundle and a link to the Applications folder, with a background picture that shows to drag the app onto the link, and beside it `http2scheme-vX.Y.Z.dmg.sha256` with the image's SHA-256 checksum. The release SHALL publish both files.

#### Scenario: The disk image holds the app and the Applications link

- **GIVEN** the disk image is built
- **WHEN** the image is opened
- **THEN** it holds "http2scheme.app" and a link to "/Applications"

#### Scenario: The checksum file matches the disk image

- **GIVEN** the disk image and its checksum file are built
- **WHEN** the checksum is checked
- **THEN** the check passes

### Requirement: The app bundle carries the license texts

The build SHALL copy the repository's `LICENSE-MIT` and `LICENSE-APACHE` unchanged into `http2scheme.app/Contents/Resources`, so every disk image carries the license.

#### Scenario: The bundle holds both license texts

- **GIVEN** the app bundle is built
- **WHEN** the bundle's resources are listed
- **THEN** they hold "LICENSE-MIT" and "LICENSE-APACHE" equal to the repository's files
