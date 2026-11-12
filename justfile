# http2scheme task recipes. `just --list` shows them all.

set shell := ["bash", "-euo", "pipefail", "-c"]

app := "app/app"
e2e := "app/e2e"

# List the recipes.
default:
    @just --list

# Run every unit and integration test (both workspaces build).
test:
    cargo test --workspace --manifest-path {{app}}/Cargo.toml

# Run one test by name, or one cucumber scenario with "scenario: <name>".
test-one name:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ "{{name}}" == scenario:* ]]; then
      scenario="{{name}}"; scenario="${scenario#scenario: }"
      cargo test --manifest-path {{app}}/Cargo.toml -p linkserver --test cucumber_integration -- --name "$scenario"
    else
      cargo test --workspace --manifest-path {{app}}/Cargo.toml -- "{{name}}"
    fi

# Build the release binary and run the E2E scenarios against it.
e2e:
    cargo build --release --manifest-path {{app}}/Cargo.toml -p http2scheme
    SUT_BIN="$PWD/{{app}}/target/release/http2scheme" cargo test --manifest-path {{e2e}}/Cargo.toml --test cucumber_e2e -- --tags "@e2e and not @wip"

# Check formatting in both workspaces.
fmt:
    cargo fmt --manifest-path {{app}}/Cargo.toml --all -- --check
    cargo fmt --manifest-path {{e2e}}/Cargo.toml --all -- --check

# Clippy with the lint table, every finding an error.
lint:
    cargo clippy --manifest-path {{app}}/Cargo.toml --workspace --all-targets --all-features -- -D warnings
    cargo clippy --manifest-path {{e2e}}/Cargo.toml --all-targets --all-features -- -D warnings

# Advisories, licenses, bans and sources.
deny: _require-cargo-deny
    cargo deny --manifest-path {{app}}/Cargo.toml check
    cargo deny --manifest-path {{e2e}}/Cargo.toml check

_require-cargo-deny:
    @cargo deny --version | grep -q '0.20.2' || { echo "install: cargo install cargo-deny --version 0.20.2 --locked"; exit 1; }
