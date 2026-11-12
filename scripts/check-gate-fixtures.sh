#!/usr/bin/env bash
# R-129: every gate check fails its violation tree with the expected message and passes its clean tree.
# A tree is tests/gate_fixtures/_base with the check's clean/ (and violation/) folder laid on top;
# every fixture file ends in .fixture, which is stripped on copy, so the gate never sees a planted breach.
set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FIXTURES="$ROOT/tests/gate_fixtures"

# Every check the gate runs, as name and script.
CHECKS="adr:scripts/arch/adr.sh
layout:scripts/arch/layout.sh
layers:scripts/arch/layers.sh
purity:scripts/arch/purity.sh
naming:scripts/arch/naming.sh
tests:scripts/arch/tests.sh
slices:scripts/arch/slices.sh
features:scripts/arch/features.sh
no-todo:scripts/arch/no-todo.sh
exact-versions:scripts/check-exact-versions.sh
size-limits:scripts/check-size-limits.sh
complexity-ratchet:scripts/check-complexity-ratchet.sh
repo:scripts/check-repo.sh
rule-coverage:scripts/check-rule-coverage.sh
gate-fixtures:scripts/check-gate-fixtures.sh"

failures=()
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

# lay <from> <to>: copies the fixture files of <from> into <to>, without the .fixture ending.
lay() {
  [ -d "$1" ] || return 0
  (cd "$1" && find . -type f -name '*.fixture') | while IFS= read -r file; do
    mkdir -p "$2/$(dirname "$file")"
    cp "$1/$file" "$2/${file%.fixture}"
  done
}

exempt() {
  awk -F '\t' -v name="$1" '$1 == name { found = 1 } END { exit !found }' "$FIXTURES/exempt.tsv" 2>/dev/null
}

while IFS=: read -r name script; do
  dir="$FIXTURES/$name"
  if [ ! -d "$dir" ]; then
    exempt "$name" || failures+=("$name has neither fixtures nor an exemption")
    continue
  fi
  clean="$work/$name/clean" violation="$work/$name/violation"
  lay "$FIXTURES/_base" "$clean" && lay "$dir/clean" "$clean"
  lay "$FIXTURES/_base" "$violation" && lay "$dir/clean" "$violation" && lay "$dir/violation" "$violation"
  if ! output="$(QA_ROOT="$clean" "$ROOT/$script" 2>&1)"; then
    failures+=("$name fails its clean tree: $output")
  fi
  if output="$(QA_ROOT="$violation" "$ROOT/$script" 2>&1)"; then
    failures+=("$name passes its violation tree")
  elif ! grep -qF "$(cat "$dir/expect.txt")" <<<"$output"; then
    failures+=("$name does not say \"$(cat "$dir/expect.txt")\": $output")
  fi
done <<<"$CHECKS"

# Every qa.sh stage runs only the scripts above, or is exempt.
SCRIPT_STAGES="arch arch-tests complexity"
stages="$(sed -n 's/^STAGES="\(.*\)"$/\1/p' "$ROOT/scripts/qa.sh")"
for stage in $stages; do
  grep -qw -- "$stage" <<<"$SCRIPT_STAGES" || exempt "$stage" || failures+=("the stage $stage has neither fixtures nor an exemption")
done

# Every fixture folder and exemption names a check that exists.
for dir in "$FIXTURES"/*/; do
  name="$(basename "$dir")"
  [ "$name" = _base ] && continue
  grep -q "^$name:" <<<"$CHECKS" || failures+=("fixtures $name name no check")
done
while IFS=$'\t' read -r name _; do
  [ -n "$name" ] || continue
  grep -q "^$name:" <<<"$CHECKS" || grep -qw -- "$name" <<<"$stages" ||
    failures+=("exemption $name names no check")
done < "$FIXTURES/exempt.tsv"

if [ "${#failures[@]}" -gt 0 ]; then
  printf 'R-129: %s\n' "${failures[@]}" >&2
  exit 1
fi
