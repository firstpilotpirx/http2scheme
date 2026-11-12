#!/usr/bin/env bash
# Shared helpers of the architecture checks: the tree to check and the failure list.
# Every check reads QA_ROOT in place of the repository root (R-129).

ROOT="${QA_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
APP="$ROOT/app/app"
FAILURES=()

# fail <rule> <message>: records one breach.
fail() {
  FAILURES+=("$1: $2")
}

# components: every component crate root.
components() {
  find "$APP/components" -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort
}

# sources <dir>: implementation files under <dir>, tests and module roots left out.
sources() {
  find "$1" -name '*.rs' ! -path '*/target/*' ! -name '*_test.rs' ! -name 'mod.rs' ! -name 'lib.rs' ! -name 'main.rs' 2>/dev/null | sort
}

# relative <path>: the path inside the checked tree.
relative() {
  printf '%s' "${1#"$ROOT"/}"
}

# finish <check>: prints the breaches and exits 1, or exits 0.
finish() {
  if [ "${#FAILURES[@]}" -eq 0 ]; then
    exit 0
  fi
  printf '%s\n' "${FAILURES[@]}" >&2
  printf '%s: %d breach(es)\n' "$1" "${#FAILURES[@]}" >&2
  exit 1
}
