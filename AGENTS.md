# http2scheme — AI agent instructions

## What this project is

A tiny local HTTP server for macOS, written in Rust. It lets apps that allow only `http://` and `https://` links (for example the Claude chat) open custom URL scheme links (`idea://`, `goland://`, `slack://`, …): a click on an `http://127.0.0.1` link makes the server run `open` on the matching scheme link. It listens only on loopback, needs a secret key in every link and opens only the schemes it allows. One Cargo crate at the repository root: the binary and its library (`src/`), tests in `tests/`, task recipes in `justfile`. The first design is in the original spec saved with the first OpenSpec change.

**Product lifecycle stage:** PoC.

## Workflow — OpenSpec

Every feature, bug fix and behavior change runs through the stages below, in order. Changes without behavior change (typos, formatting, renames, docs, dependency bumps) skip stages 1–3 and 6–7; they still go through a branch, the checks, a PR and a merge.

| # | Stage | How | Done when |
|---|---|---|---|
| 1 | Explore (always) | `/opsx:explore` | the idea is clear and the scope settled; no files written |
| 2 | Propose | `/opsx:new`, then `/opsx:continue` per artifact (proposal → specs → design → tasks) | `kit proposal-lint <name>` and `openspec validate <name>` pass; the `plan-reviewer` agent answers `RESULT: PASS` |
| 3 | Approval | show the proposal to the user | the user approved it — **stop and wait**; no code before this |
| 4 | Branch | `kit branch <type>/<name>` (`feat`, `fix`, `chore`) | |
| 5 | Apply | `/opsx:apply`, test-first (§ TDD) | every task in `tasks.md` is checked; one commit per task group (`kit ship`) |
| 6 | Verify | `/opsx:verify`; then the `spec-test-reviewer` agent once, only for a change that is not small | no CRITICAL issues; for a reviewed change, `RESULT: PASS` |
| 7 | Archive | `/opsx:archive`, then commit | specs merged into `openspec/specs/`, change in `openspec/changes/archive/` |
| 8 | Full check | `kit verify` (§ How to verify a change) | passes on the final commit |
| 9 | Pull request | `kit pr` | the PR body lists the checks that ran |
| 10 | Manual run | the user, only for the areas `manual_run` in `.claude/kit.yaml` lists | the user confirmed it works |
| 11 | Merge | `kit land` | the change is on `main` and the main checkout is updated |
| 12 | Release | the release command of § Release, when `kit release-needed` names a version | tag `vX.Y.Z` and its release exist |

- Stages 4–12 are pre-authorized: commit, push, open the PR, squash-merge, tag and release without asking.
- Skills (agent-kit): `/create-change` files a change for later; `/proceed-change <name>` runs every stage, stopping only at the gates picked at start.

## TDD

Every behavior change in stage 5 is test-first (`tdd` skill):

1. **Red** — write the test for the new behavior and run only it; it fails because the behavior is missing.
2. **Green** — write the smallest code that makes it pass.
3. **Refactor** — clean up with the test green.

- `tasks.md` lists the Red task before the Green task for each behavior.
- A bug fix starts with a test that reproduces the bug.

## Gherkin

Slice and component tests are Gherkin `.feature` files run by cucumber-rs, one per OpenSpec capability, in `tests/features/`, steps in `tests/steps/`. Unit tests (`#[cfg(test)]`) and adapter integration tests stay plain Rust. Each `.feature` scenario matches a spec scenario by meaning (`~/.claude/rules-custom/gherkin.md`); the design of each change names the test level of every scenario.

## How to verify a change

| When | Command |
|---|---|
| At TDD Red and Green, on the new test | `just test-one <name>` |
| Once per change, after the last task group | `just check`: fmt, clippy, cargo-deny, tests (`kit verify` in the background runs the level `kit checks-needed` names) |
| Before merge | `just check` |

No check after an edit or a task group beyond the tests `tdd` writes; the checks run once per change.

## Release

The version is `version` in `Cargo.toml` (SemVer, `~/.claude/rules-custom/semver.md`).

- Every change with behavior change states its bump and target version in the proposal and bumps the version in its last task group; `kit version-check <name>` checks both.
- A change without behavior change states "no release" and leaves the version alone.
- After `kit land`, when `kit release-needed` names a version, run `kit release` (or the project's own release command named here).
