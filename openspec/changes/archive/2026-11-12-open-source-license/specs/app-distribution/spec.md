## ADDED Requirements

### Requirement: The app bundle carries the license texts

The build SHALL copy the repository's `LICENSE-MIT` and `LICENSE-APACHE` unchanged into `http2scheme.app/Contents/Resources`, so every disk image carries the license.

#### Scenario: The bundle holds both license texts

- **GIVEN** the app bundle is built
- **WHEN** the bundle's resources are listed
- **THEN** they hold "LICENSE-MIT" and "LICENSE-APACHE" equal to the repository's files
