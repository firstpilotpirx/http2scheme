#!/usr/bin/env bash
# R-073: every primary adapter entry (a *_command or *_handler type) is driven by a slice test.
# The slice steps reach an entry through the composition root, so the root method that builds
# the entry must be called from the component's tests/steps.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/lib.sh"

check_component() {
  local root_file
  root_file="$(find "$1/src" -maxdepth 1 -name '*_config.rs' ! -name '*_test.rs' | head -n 1)"
  while IFS= read -r file; do
    local type
    type="$(grep -oE '^pub struct [A-Za-z]+' "$file" | awk '{print $3}' | head -n 1)"
    [ -n "$type" ] || continue
    local methods=""
    [ -n "$root_file" ] && methods="$(awk -v type="$type" '
      match($0, /pub (const )?(async )?fn [a-z_]+/) { name = substr($0, RSTART, RLENGTH); sub(/.* fn /, "", name) }
      name != "" && index($0, type) && !/^use / { print name }' "$root_file" | sort -u)"
    local driven=0
    for method in $methods; do
      grep -rqE "(\.|::)$method\(" "$1/tests/steps" 2>/dev/null && driven=1
    done
    [ "$driven" -eq 1 ] || fail R-073 "$(relative "$file") ($type) is driven by no slice test"
  done < <(sources "$1/src/adapters/primary" | grep -E '_(command|handler)\.rs$')
}

for component in $(components); do
  check_component "$component"
done
finish slices
