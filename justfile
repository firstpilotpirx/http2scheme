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
    SUT_BIN="$PWD/{{app}}/target/release/http2scheme" cargo test --manifest-path {{e2e}}/Cargo.toml --test cucumber_e2e -- --tags "@e2e and not @wip and not @bundle"

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
# The E2E crate has only test targets, which cargo-geiger does not scan; its crate root forbids unsafe code.
unsafe-count: _require-cargo-geiger
    #!/usr/bin/env bash
    set -euo pipefail
    # The geiger verdict depends only on the lock file and the entry point files: reuse it while
    # they are unchanged (it scans every dependency's sources, about two minutes).
    stamp="$(cat {{app}}/Cargo.lock {{app}}/apps/http2scheme/src/main.rs {{app}}/components/linkserver/src/lib.rs \
      {{app}}/components/linkserver/tests/cli_fixture/main.rs | shasum -a 256 | cut -d' ' -f1) {{geiger_version}}"
    cache="{{app}}/target/unsafe-count.geiger"
    if [ "$(cat "$cache" 2>/dev/null)" != "$stamp" ]; then
      report="$(cargo geiger --forbid-only --manifest-path "$PWD/{{app}}/apps/http2scheme/Cargo.toml" 2>/dev/null)"
      for name in http2scheme linkserver; do
        grep -qE "^:\) [│├└─ ]*$name " <<<"$report" || { echo "$name does not forbid unsafe code in every entry point"; exit 1; }
      done
      mkdir -p "$(dirname "$cache")" && echo "$stamp" >"$cache"
    fi
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

# Release the version in app/app/Cargo.toml from a clean origin/main, signed with the certificate: the gate
# (which builds the disk image), the tag and the GitHub release with the image and its .sha256 (design D12).
release: _require-release-identity
    #!/usr/bin/env bash
    set -euo pipefail
    git fetch origin main
    [ -z "$(git status --porcelain)" ] || { echo "release: the checkout has uncommitted changes"; exit 1; }
    [ "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)" ] || { echo "release: HEAD is not origin/main"; exit 1; }
    tag="v{{app_release}}"
    git rev-parse -q --verify "refs/tags/$tag" > /dev/null && { echo "release: $tag exists"; exit 1; }
    scripts/qa.sh
    dmg={{quote(dist_dir)}}/http2scheme-$tag.dmg
    [ -f "$dmg" ] && [ -f "$dmg.sha256" ] || { echo "release: the gate built no $dmg"; exit 1; }
    git tag -a "$tag" -m "http2scheme $tag"
    git push origin "$tag"
    gh release create "$tag" "$dmg" "$dmg.sha256" --title "http2scheme $tag" --generate-notes

# ── App bundle and disk image (design D17) ──

dist_dir := justfile_directory() / "dist"
app_bundle := dist_dir / "http2scheme.app"
packaging_dir := justfile_directory() / "app/packaging"
app_release := shell("sed -n '/^\\[workspace.package\\]/,/^\\[/s/^version = \"\\(.*\\)\"/\\1/p' \"$1\"", justfile_directory() / "app/app/Cargo.toml")
# The self-signed code-signing certificate of README § Install; empty signs ad-hoc (no stable identity)
signing_identity := env_var_or_default("HTTP2SCHEME_SIGNING_IDENTITY", "")

# Build the signed dist/http2scheme.app (release build): the program, the icon, the information and the license texts
bundle: _require-signing-identity
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --release --manifest-path {{app}}/Cargo.toml -p http2scheme
    app={{quote(app_bundle)}}
    rm -rf "$app" && mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
    cp {{app}}/target/release/http2scheme "$app/Contents/MacOS/http2scheme"
    iconutil -c icns {{quote(packaging_dir / "icons/http2scheme.iconset")}} -o "$app/Contents/Resources/http2scheme.icns"
    cp {{quote(packaging_dir / "Info.plist")}} "$app/Contents/Info.plist"
    cp {{quote(justfile_directory() / "LICENSE-MIT")}} {{quote(justfile_directory() / "LICENSE-APACHE")}} "$app/Contents/Resources/"
    for key in CFBundleShortVersionString CFBundleVersion; do plutil -replace "$key" -string {{quote(app_release)}} "$app/Contents/Info.plist"; done
    {{just_executable()}} --justfile {{quote(justfile())}} _sign "$app"
    echo "✓ bundle $app"

# Build the disk image dist/http2scheme-vX.Y.Z.dmg (the bundle, an Applications link, the background picture)
# and its .sha256; create-dmg lays the window out through the Finder
dmg: _require-create-dmg bundle
    #!/usr/bin/env bash
    set -euo pipefail
    dist={{quote(dist_dir)}}
    name="http2scheme-v{{app_release}}"
    stage="$dist/stage"
    rm -rf "$stage" "$dist/$name.dmg" "$dist/$name.dmg.sha256" && mkdir -p "$stage"
    cp -R {{quote(app_bundle)}} "$stage/"
    create-dmg --volname "http2scheme {{app_release}}" --background {{quote(packaging_dir / "dmg-background.png")}} \
        --window-pos 200 120 --window-size 600 400 --icon-size 128 --icon http2scheme.app 150 200 \
        --hide-extension http2scheme.app --app-drop-link 450 200 --no-internet-enable \
        "$dist/$name.dmg" "$stage"
    (cd "$dist" && shasum -a 256 "$name.dmg" > "$name.dmg.sha256")
    rm -rf "$stage"
    echo "✓ dmg $dist/$name.dmg"

# Redraw the app icon and the disk image background, then scale the icon into the ten iconset sizes
icons:
    #!/usr/bin/env bash
    set -euo pipefail
    cd {{quote(packaging_dir)}}
    swift draw-artwork.swift
    sips -s dpiWidth 144 -s dpiHeight 144 dmg-background.png > /dev/null
    for size in 16 32 128 256 512; do
      sips -z "$size" "$size" icons/icon-1024.png --out "icons/http2scheme.iconset/icon_${size}x${size}.png" > /dev/null
      sips -z "$((size * 2))" "$((size * 2))" icons/icon-1024.png --out "icons/http2scheme.iconset/icon_${size}x${size}@2x.png" > /dev/null
    done

# The `bundle` stage of the gate: the disk image built (ad-hoc signed when no certificate is named), then the
# @bundle scenarios on the bundle and the image
bundle-check: dmg
    BUNDLE_APP={{quote(app_bundle)}} BUNDLE_DMG={{quote(dist_dir)}}/http2scheme-v{{app_release}}.dmg cargo test --manifest-path {{e2e}}/Cargo.toml --test cucumber_e2e -- --tags "@bundle and not @wip"

# Sign a bundle with the signing identity, or ad-hoc with a notice when none is named, then verify it strictly.
# No hardened runtime: it needs a Developer ID and entitlements
_sign path:
    #!/usr/bin/env bash
    set -euo pipefail
    if identity={{quote(signing_identity)}}; [[ -z "$identity" ]]; then
        echo "● HTTP2SCHEME_SIGNING_IDENTITY is unset: signing ad-hoc — the bundle has no stable identity, so macOS asks again after each build (README § Install)" >&2
        identity="-"
    fi
    codesign --force --sign "$identity" --timestamp=none {{quote(path)}}
    codesign --verify --deep --strict --verbose=2 {{quote(path)}}

# A named signing identity must be in the keychain; none named is allowed (ad-hoc)
_require-signing-identity:
    #!/usr/bin/env bash
    set -euo pipefail
    identity={{quote(signing_identity)}}
    [[ -n "$identity" ]] || exit 0
    if ! security find-identity -v -p codesigning 2>/dev/null | grep -q "\"$identity\""; then
        echo "✗ signing identity '$identity' (HTTP2SCHEME_SIGNING_IDENTITY) is not in the keychain" >&2
        {{just_executable()}} --justfile {{quote(justfile())}} _signing-identity-steps >&2
        exit 1
    fi

# A release is always signed with the certificate: the identity must be named and in the keychain
_require-release-identity:
    #!/usr/bin/env bash
    set -euo pipefail
    if identity={{quote(signing_identity)}}; [[ -z "$identity" ]]; then
        echo "✗ HTTP2SCHEME_SIGNING_IDENTITY is unset — a release is signed with the self-signed certificate, so that updates keep the browser permission" >&2
        {{just_executable()}} --justfile {{quote(justfile())}} _signing-identity-steps >&2
        exit 1
    fi
    {{just_executable()}} --justfile {{quote(justfile())}} _require-signing-identity

_signing-identity-steps:
    @echo "  Create the certificate once: 'just signing-identity' (macOS asks for your password to trust it),"
    @echo "  then set HTTP2SCHEME_SIGNING_IDENTITY=http2scheme (README § Install)"

# Creates the self-signed code-signing certificate `http2scheme` (10 years) in the login keychain, once per
# machine, and trusts it for code signing; macOS asks for the user's password. Does nothing when it exists
signing-identity name="http2scheme":
    #!/usr/bin/env bash
    set -euo pipefail
    if security find-identity -v -p codesigning | grep -q "\"{{name}}\""; then
        echo "✓ code-signing identity \"{{name}}\" already in the keychain"; exit 0
    fi
    work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
    /usr/bin/openssl req -x509 -newkey rsa:2048 -nodes -days 3650 -subj "/CN={{name}}" \
        -keyout "$work/key.pem" -out "$work/cert.pem" \
        -addext "keyUsage=critical,digitalSignature" -addext "extendedKeyUsage=critical,codeSigning" \
        -addext "basicConstraints=critical,CA:false" 2>/dev/null
    /usr/bin/openssl pkcs12 -export -inkey "$work/key.pem" -in "$work/cert.pem" -out "$work/id.p12" -passout pass:http2scheme
    keychain=$(security default-keychain | tr -d ' "')
    security import "$work/id.p12" -k "$keychain" -P http2scheme -T /usr/bin/codesign
    echo "  macOS now asks for your password to trust the certificate for code signing"
    security add-trusted-cert -r trustRoot -p codeSign -k "$keychain" "$work/cert.pem"
    security find-identity -v -p codesigning | grep "\"{{name}}\""
    echo "✓ created \"{{name}}\" — set HTTP2SCHEME_SIGNING_IDENTITY={{name}}"

_require-create-dmg:
    #!/usr/bin/env bash
    set -euo pipefail
    wanted=1.3.0
    got=$(create-dmg --version 2>/dev/null | head -1 || true)
    if [[ "$got" != *"$wanted"* ]]; then
        echo "✗ create-dmg $wanted not found (got '${got}') — run 'brew install create-dmg'" >&2
        exit 1
    fi
