#!/usr/bin/env bash
# R-006: docs/decisions/ exists; every file is ADR-NNN-*.html with the h2 sections
# Status, Context, Decision, Alternatives and Consequences.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/lib.sh"

DECISIONS="$ROOT/docs/decisions"
SECTIONS="Status Context Decision Alternatives Consequences"

if [ ! -d "$DECISIONS" ]; then
  fail R-006 "docs/decisions/ is missing"
  finish adr
fi
while IFS= read -r file; do
  name="$(basename "$file")"
  if [[ ! "$name" =~ ^ADR-[0-9]{3}-[a-z0-9-]+\.html$ ]]; then
    fail R-006 "$(relative "$file") is not named ADR-NNN-<title>.html"
    continue
  fi
  for section in $SECTIONS; do
    grep -qE "<h2[^>]*>$section</h2>" "$file" || fail R-006 "$(relative "$file") has no $section section"
  done
done < <(find "$DECISIONS" -type f | sort)
finish adr
