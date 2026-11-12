#!/usr/bin/env bash
# Checks whose subject is the repository itself: its files, its gate script and its recipes.
# R-001, R-002, R-009, R-010, R-090, R-091, R-092, R-093, R-094, R-095, R-112, R-121, R-122, R-123, R-139.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/arch/lib.sh"

QA="$ROOT/scripts/qa.sh"
JUST="$ROOT/justfile"
MANIFESTS="app/app/Cargo.toml app/e2e/Cargo.toml"

[ -x "$QA" ] || fail R-001 "scripts/qa.sh is missing or not executable"

for section in "## Install" "## Run (dev)" "## Run (production)" "## Test"; do
  grep -qxF "$section" "$ROOT/README.md" 2>/dev/null || fail R-002 "README.md has no \"$section\" section"
done

tracked="$(git -C "$ROOT" ls-files 2>/dev/null)"
for manifest in $MANIFESTS; do
  grep -qxF "${manifest%Cargo.toml}Cargo.lock" <<<"$tracked" || fail R-009 "${manifest%Cargo.toml}Cargo.lock is not committed"
done
while IFS= read -r file; do
  [ -n "$file" ] && fail R-010 "$file is a committed binary or archive"
done < <(grep -E '\.(tar\.gz|tgz|zip|exe|dylib|so|a|dmg|pkg)$' <<<"$tracked")

for flag in --fix --clean --stage --skip --json --no-color --help; do
  grep -qE -- "^ +${flag}[)|]" "$QA" 2>/dev/null || fail R-090 "scripts/qa.sh does not take $flag"
done

for verdict in OK FAIL WARN; do
  if ! grep -qE "^ +$verdict\) printf '\[ %s \] %s \(%ds\)" "$QA"; then
    fail R-092 "scripts/qa.sh does not print [ $verdict ] <stage> (<n>s)"
  fi
done

order="fmt lint check complexity machete unsafe-count modules typos recipes dup deny arch unit integration arch-tests e2e build coverage"
stages="$(sed -n 's/^STAGES="\(.*\)"$/\1/p' "$QA" 2>/dev/null)"
[ "$stages" = "$order" ] || fail R-091 "scripts/qa.sh runs \"$stages\", not \"$order\""
# R-121: one runner, one stage at a time; E2E runs alone after every test stage.
[ "$(tr ' ' '\n' <<<"$stages" | grep -n '^e2e$' | cut -d: -f1)" = 16 ] || fail R-121 "scripts/qa.sh does not run e2e after the other test stages"

for manifest in $MANIFESTS; do
  grep -qE "^[a-z0-9_]+ := \"${manifest%/Cargo.toml}\"" "$JUST" || fail R-093 "the recipes never run on $manifest"
done
grep -qF 'for manifest in {{manifests}}' "$JUST" || fail R-093 "no recipe loops over every workspace"

for tool in "cargo fmt" "cargo clippy" "cargo check" "rust-code-analysis-cli" "cargo machete" "cargo geiger" \
  "cargo modules" "typos" "shellcheck" "jscpd" "cargo deny" "cargo llvm-cov"; do
  grep -qF "$tool" "$JUST" || fail R-094 "the recipes never run $tool"
done

grep -nE -- '--no-verify|-A warnings|--cap-lints|allow\(warnings\)' "$QA" "$JUST" >/dev/null &&
  fail R-095 "scripts/qa.sh or the justfile switches warnings or hooks off"
grep -qE -- '-D warnings' "$JUST" || fail R-095 "clippy does not run with -D warnings"

color_libs='^(colored|termcolor|ansi_term|nu-ansi-term|yansi|console|crossterm|anstream|owo-colors)\b'
count="$(cat "$ROOT"/app/app/Cargo.toml "$ROOT"/app/e2e/Cargo.toml 2>/dev/null | grep -cE "$color_libs")"
[ "$count" -le 1 ] || fail R-112 "the manifests name $count color libraries"
while IFS= read -r file; do
  grep -qE '\\x1b\[|\\u\{1b\}\[|\\033\[' "$file" && fail R-112 "$(relative "$file") writes raw escape codes"
done < <(sources "$APP")

lints() {
  sed -n '/^\[\(workspace\.\)\{0,1\}lints\./,/^\[[a-z]/p' "$1" | grep -vE '^\[' | sort
}
[ "$(lints "$ROOT/app/app/Cargo.toml")" = "$(lints "$ROOT/app/e2e/Cargo.toml")" ] ||
  fail R-122 "app/app and app/e2e do not share one lint table"

toolchains="$(find "$ROOT" -name 'rust-toolchain*' ! -path '*/target/*' ! -path '*/gate_fixtures/*' 2>/dev/null)"
[ "$(wc -l <<<"$toolchains" | tr -d ' ')" -eq 1 ] && [ -n "$toolchains" ] ||
  fail R-123 "the workspaces do not share one toolchain file: ${toolchains:-none}"

for recipe in complexity machete unsafe-count modules typos recipes dup deny coverage; do
  grep -qE "^$recipe: _require-" "$JUST" || fail R-139 "just $recipe runs before its _require-<tool> check"
done
grep -A8 '^_require tool' "$JUST" | grep -qE '^\s+(cargo install|brew install|npm install)' &&
  fail R-139 "a _require recipe installs a tool"
finish repo
