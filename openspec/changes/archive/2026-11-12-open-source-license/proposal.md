## Why

http2scheme is published as public open source, but the repository holds no license text: `Cargo.toml` says only `license = "MIT"`, and the app bundle and the disk image carry no license at all. The owner chose the Rust community's dual license, `MIT OR Apache-2.0`, as the kit's tech matrix now states for every public repository.

## What Changes

- `LICENSE-MIT` (2026, Aleksandr Volkov) and `LICENSE-APACHE` (the official Apache License 2.0 text) at the repository root.
- `license = "MIT OR Apache-2.0"` in `app/app/Cargo.toml` (inherited by every crate) and `app/e2e/Cargo.toml`.
- The README ends with a `## License` section with the standard contribution clause.
- `just bundle` copies both texts into `http2scheme.app/Contents/Resources`, so the released disk image carries them.
- `.gitignore` gets one entry per path, with comments, and ignores `.DS_Store`.

SemVer: PATCH — the released app now carries its license; no behavior change in the app.
Target version: 0.1.1.
Current stage: MVP. Target stage: MVP.
Scope: the license files, the manifests, README, `.gitignore`, the `bundle` recipe, the `app-distribution` spec.
Acceptance bar: `kit verify` passes, the `@bundle` scenario finds both texts in the bundle, and release v0.1.1 holds them.
Out of scope: changing the app's behavior; an About window showing the license.
Interface: none — files only; the menu and windows do not change.

## Capabilities

### Modified Capabilities

- `app-distribution`: the app bundle carries both license texts.

## Impact

- `LICENSE-MIT`, `LICENSE-APACHE`, `app/app/Cargo.toml`, `app/e2e/Cargo.toml`, `README.md`, `.gitignore`, `justfile`, `app/e2e/tests/features/app_distribution.feature`, `app/e2e/tests/steps/app_distribution.rs`.

## Open points

- None.
