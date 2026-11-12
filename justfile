# http2scheme task recipes. `just --list` shows them all.

set shell := ["bash", "-euo", "pipefail", "-c"]

app := "app/app"
e2e := "app/e2e"
manifests := app + "/Cargo.toml " + e2e + "/Cargo.toml"
real_program_tests := "--test launch_services_scheme_link_opener_gateway_integration_test --test launchd_agent_supervisor_gateway_integration_test --test apple_script_browser_tab_closer_gateway_integration_test"

# Pinned analyzers (tech-matrix-rust.md § Static Analysis Stack).
rca_version := "0.0.25"
deny_version := "0.20.2"
machete_version := "0.9.2"
geiger_version := "0.14.0"
llvm_cov_version := "0.9.1"
modules_version := "0.28.0"
typos_version := "1.51.1"
shellcheck_version := "0.11.0"
jscpd_version := "5.4.2"

# List the recipes.
default:
    @just --list

# The whole gate: every stage of scripts/qa.sh (pass its flags after `--`).
check *flags:
    scripts/qa.sh {{flags}}

# Run every unit and integration test (both workspaces build).
test:
    cargo test --workspace --manifest-path {{app}}/Cargo.toml

# Run one test by name, or one cucumber scenario with "scenario: <name>".
test-one name:
    #!/usr/bin/env bash
    set -euo pipefail
    name="{{name}}"
    if [[ "$name" == scenario:* ]]; then
      cargo test --manifest-path {{app}}/Cargo.toml -p linkserver --test cucumber_integration -- --name "${name#scenario: }"
    else
      cargo test --workspace --manifest-path {{app}}/Cargo.toml -- "$name"
    fi

# Unit tests: the sibling tests inside the crates.
unit:
    cargo test --workspace --manifest-path {{app}}/Cargo.toml --lib --bins

# Integration, slice and component tests, then the tests that run the real macOS programs.
integration:
    cargo test --workspace --manifest-path {{app}}/Cargo.toml --tests
    cargo test --manifest-path {{app}}/Cargo.toml -p linkserver {{real_program_tests}} -- --ignored

# Build the release binary and run the E2E scenarios against it.
e2e:
    cargo build --release --manifest-path {{app}}/Cargo.toml -p http2scheme
    SUT_BIN="$PWD/{{app}}/target/release/http2scheme" cargo test --manifest-path {{e2e}}/Cargo.toml --test cucumber_e2e -- --tags "@e2e and not @wip"

# The release build.
build:
    cargo build --release --manifest-path {{app}}/Cargo.toml --workspace

# Check formatting in both workspaces.
fmt:
    for manifest in {{manifests}}; do cargo fmt --manifest-path "$manifest" --all -- --check; done

# Clippy with the lint table, every finding an error.
lint:
    for manifest in {{manifests}}; do cargo clippy --manifest-path "$manifest" --workspace --all-targets --all-features -- -D warnings; done

# Format and apply clippy's fixes in both workspaces.
fix:
    for manifest in {{manifests}}; do cargo fmt --manifest-path "$manifest" --all; cargo clippy --manifest-path "$manifest" --workspace --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings; done

# Type-check both workspaces.
typecheck:
    for manifest in {{manifests}}; do cargo check --manifest-path "$manifest" --workspace --all-targets; done

# Cyclomatic complexity per function against metrics/cyclomatic.tsv (R-136, R-141).
complexity: _require-rust-code-analysis
    scripts/check-complexity-ratchet.sh

# Unused dependencies.
machete: _require-cargo-machete
    cargo machete {{app}} {{e2e}}

# The binary and linkserver forbid unsafe code in every entry point; menubar keeps its unsafe
# blocks and impls per file at the counts in metrics/unsafe.tsv (R-144, ADR-001).
# The E2E crate has only test targets, which cargo-geiger does not scan; its [lints] forbid unsafe code.
unsafe-count: _require-cargo-geiger
    #!/usr/bin/env bash
    set -euo pipefail
    report="$(cargo geiger --forbid-only --manifest-path "$PWD/{{app}}/apps/http2scheme/Cargo.toml" 2>/dev/null)"
    for name in http2scheme linkserver; do
      grep -qE "^:\) [│├└─ ]*$name " <<<"$report" || { echo "$name does not forbid unsafe code in every entry point"; exit 1; }
    done
    actual="$(grep -rlE 'unsafe (\{|impl)' {{app}}/components/menubar/src | sort | while IFS= read -r file; do
      printf '%s\t%s\n' "$file" "$(grep -oE 'unsafe (\{|impl)' "$file" | wc -l | tr -d ' ')"
    done)"
    if [ "$actual" != "$(cat metrics/unsafe.tsv)" ]; then
      echo "unsafe uses differ from metrics/unsafe.tsv (a higher count needs ADR-001 updated; write a lower one back):"
      diff <(cat metrics/unsafe.tsv) <(echo "$actual") || true
      exit 1
    fi

# No cycle between the modules of a component (R-148).
modules: _require-cargo-modules
    #!/usr/bin/env bash
    set -euo pipefail
    for dir in {{app}}/components/*/; do
      package="$(basename "$dir")"
      cargo modules dependencies --manifest-path {{app}}/Cargo.toml --package "$package" --lib \
        --no-fns --no-types --no-traits --no-externs --no-sysroot --no-owns 2>/dev/null \
        | sed -n 's/^ *"\([^"]*\)" -> "\([^"]*\)".*/\1 \2/p' | awk '$1 != $2' > "$TMPDIR/modules.$$"
      cycle="$(tsort "$TMPDIR/modules.$$" 2>&1 > /dev/null || true)"
      rm -f "$TMPDIR/modules.$$"
      if [ -n "$cycle" ]; then echo "$package has a module cycle:"; echo "$cycle"; exit 1; fi
    done

# Spelling in code, comments and documents.
typos: _require-typos
    typos

# Shell scripts and the bash recipe bodies of this justfile (R-150).
recipes: _require-shellcheck
    #!/usr/bin/env bash
    set -euo pipefail
    shellcheck -x scripts/*.sh scripts/arch/*.sh
    for recipe in $(just --summary); do
      body="$(just --show "$recipe")"
      grep -q '^    #!/usr/bin/env bash' <<<"$body" || continue
      sed -n '/^    #!\/usr\/bin\/env bash/,$p' <<<"$body" | sed 's/^    //; s/{{{{[^}]*}}/value/g' \
        | shellcheck --shell=bash - || { echo "in the recipe $recipe"; exit 1; }
    done

# Copy-pasted code may not grow above the threshold in .jscpd.json (percent of lines; R-151).
dup: _require-jscpd
    jscpd --config .jscpd.json {{app}}/components {{app}}/apps {{e2e}}/tests

# Advisories, licenses, bans and sources.
deny: _require-cargo-deny
    for manifest in {{manifests}}; do cargo deny --manifest-path "$manifest" check; done

# The architecture rules (scripts/arch) and the repository checks.
arch:
    for check in scripts/arch/*.sh; do [ "$check" = scripts/arch/lib.sh ] || "$check"; done
    scripts/check-exact-versions.sh
    scripts/check-size-limits.sh
    scripts/check-repo.sh
    kit spec-coverage

# The checks of the checks: every check can fail, every rule has a verdict.
arch-tests:
    scripts/check-gate-fixtures.sh
    scripts/check-rule-coverage.sh

# Line coverage of both test levels.
coverage: _require-cargo-llvm-cov
    cargo llvm-cov --manifest-path {{app}}/Cargo.toml --workspace --summary-only --fail-under-lines "$(cat metrics/coverage.txt)"

# Delete the build outputs of both workspaces.
clean:
    for manifest in {{manifests}}; do cargo clean --manifest-path "$manifest"; done

# Fail when a pinned tool is missing or at another version; installs nothing (R-139).
_require tool version install:
    #!/usr/bin/env bash
    found="$({{tool}} --version 2>/dev/null | head -n 2 | tr '\n' ' ')"
    if ! grep -qF "{{version}}" <<<"$found"; then
      echo "{{tool}}: found '${found:-nothing}', pinned {{version}}; install: {{install}}" >&2
      exit 1
    fi

_require-rust-code-analysis: (_require "rust-code-analysis-cli" rca_version "cargo install rust-code-analysis-cli --version " + rca_version + " --locked")
_require-cargo-deny: (_require "cargo deny" deny_version "cargo install cargo-deny --version " + deny_version + " --locked")
_require-cargo-machete: (_require "cargo machete" machete_version "cargo install cargo-machete --version " + machete_version + " --locked")
_require-cargo-geiger: (_require "cargo geiger" geiger_version "cargo install cargo-geiger --version " + geiger_version + " --locked")
_require-cargo-llvm-cov: (_require "cargo llvm-cov" llvm_cov_version "cargo install cargo-llvm-cov --version " + llvm_cov_version + " --locked")
_require-cargo-modules: (_require "cargo modules" modules_version "cargo install cargo-modules --version " + modules_version + " --locked")
_require-typos: (_require "typos" typos_version "cargo install typos-cli --version " + typos_version + " --locked")
_require-shellcheck: (_require "shellcheck" shellcheck_version "brew install shellcheck (pinned " + shellcheck_version + ")")
_require-jscpd: (_require "jscpd" jscpd_version "npm install --global jscpd@" + jscpd_version)

# Release the version in app/app/Cargo.toml from a clean origin/main: the gate, a release build,
# the tarball with its checksum, the tag and the GitHub release (design D12).
release:
    #!/usr/bin/env bash
    set -euo pipefail
    git fetch origin main
    [ -z "$(git status --porcelain)" ] || { echo "release: the checkout has uncommitted changes"; exit 1; }
    [ "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)" ] || { echo "release: HEAD is not origin/main"; exit 1; }
    version="$(sed -n '/^\[workspace.package\]/,/^\[/s/^version = "\(.*\)"/\1/p' {{app}}/Cargo.toml)"
    tag="v$version"
    git rev-parse -q --verify "refs/tags/$tag" > /dev/null && { echo "release: $tag exists"; exit 1; }
    scripts/qa.sh
    cargo build --release --manifest-path {{app}}/Cargo.toml -p http2scheme
    dist="$(mktemp -d)"
    archive="http2scheme-$tag-aarch64-apple-darwin.tar.gz"
    tar -czf "$dist/$archive" -C {{app}}/target/release http2scheme
    (cd "$dist" && shasum -a 256 "$archive" > "$archive.sha256")
    git tag -a "$tag" -m "http2scheme $tag"
    git push origin "$tag"
    gh release create "$tag" "$dist/$archive" "$dist/$archive.sha256" --title "http2scheme $tag" --generate-notes
