#!/usr/bin/env bash
# R-064: a .feature per OpenSpec capability holding every spec scenario by name.
# R-065: every scenario has one capability tag and one level tag; steps use no technical words.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/lib.sh"

# escape <text>: the text as a literal inside an extended regex.
escape() {
  printf '%s' "$1" | sed 's/[][\.*^$+?(){}|]/\\&/g'
}

features() {
  find "$ROOT/app" -path '*/tests/features/*' -name '*.feature' ! -path '*/target/*' 2>/dev/null | sort
}

specs() {
  find "$ROOT/openspec/specs" "$ROOT/openspec/changes" -name spec.md ! -path '*/archive/*' 2>/dev/null | sort
}

while IFS= read -r spec; do
  capability="$(basename "$(dirname "$spec")")"
  file="$(tr '/-' '__' <<<"$capability").feature"
  matches="$(features | grep "/$file$")"
  if [ -z "$matches" ]; then
    fail R-064 "capability $capability has no $file"
    continue
  fi
  while IFS= read -r scenario; do
    # shellcheck disable=SC2086
    grep -qxE "[[:space:]]*Scenario( Outline)?: $(escape "$scenario")[[:space:]]*" $matches \
      || fail R-064 "scenario \"$scenario\" of $capability is in no $file"
  done < <(sed -n 's/^#### Scenario: *//p' "$spec" | sed 's/[[:space:]]*$//')
done < <(specs)

banned='\b(HTTP|POST|GET|SELECT|INSERT|JSON|null|struct|status code)\b|fn |::|[a-z]+://|\b[A-Z][A-Z0-9]*_[A-Z0-9_]+\b'
while IFS= read -r feature; do
  capability="$(basename "$feature" .feature)"
  awk -v capability="$capability" -v file="$(relative "$feature")" '
    function check() {
      if (scenario == "") return
      levels = 0; caps = 0
      n = split(feature_tags " " rule_tags " " tags, all, /[[:space:]]+/)
      for (i = 1; i <= n; i++) {
        if (all[i] ~ /^@(slice|component|e2e)$/) levels++
        if (all[i] == "@" capability) caps++
      }
      if (levels != 1 || caps != 1) printf "R-065: %s scenario \"%s\" needs one @%s tag and one level tag\n", file, scenario, capability
      scenario = ""
    }
    /^[[:space:]]*@/ { pending = pending " " $0; next }
    /^[[:space:]]*Feature:/ { feature_tags = pending; pending = ""; next }
    /^[[:space:]]*Rule:/ { check(); rule_tags = pending; pending = ""; next }
    /^[[:space:]]*Scenario( Outline)?:/ { check(); tags = pending; pending = ""; scenario = $0; sub(/^[[:space:]]*Scenario( Outline)?: */, "", scenario); next }
    END { check() }' "$feature" | while IFS= read -r line; do echo "$line"; done
  grep -nE '^[[:space:]]*(Given|When|Then|And|But) ' "$feature" | grep -E "$banned" | while IFS= read -r line; do
    echo "R-065: $(relative "$feature"):${line%%:*} uses a technical word"
  done
done < <(features) > "${TMPDIR:-/tmp}/features.$$"
while IFS= read -r line; do
  FAILURES+=("$line")
done < "${TMPDIR:-/tmp}/features.$$"
rm -f "${TMPDIR:-/tmp}/features.$$"
finish features
