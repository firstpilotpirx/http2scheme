## 1. Dual license in the repository and the bundle

- [x] 1.1 Add `LICENSE-MIT` and the official `LICENSE-APACHE` at the root
- [x] 1.2 Set `license = "MIT OR Apache-2.0"` in `app/app/Cargo.toml` and `app/e2e/Cargo.toml`
- [x] 1.3 Add the README `## License` section; tidy `.gitignore`
- [x] 1.4 Add the `@bundle` scenario "The bundle holds both license texts" and its steps
- [x] 1.5 Copy both texts into the bundle in `just bundle`
- [x] 1.6 Bump the workspace version to 0.1.1 — `kit version-check open-source-license` passes
- [x] 1.7 Commit the group with `kit ship`
