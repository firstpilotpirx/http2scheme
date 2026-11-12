#!/usr/bin/env bash
# R-136: no function's cyclomatic complexity rises above metrics/cyclomatic.tsv, and no new
# function exceeds the record's highest value. `--write` rewrites the record from the sources.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/arch/lib.sh"

RECORD="$ROOT/metrics/cyclomatic.tsv"

# measure: path, function (impl::name) and cyclomatic value of every non-test function.
measure() {
  local files
  files="$(find "$APP" -path '*/src/*' -name '*.rs' ! -name '*_test.rs' ! -path '*/target/*' 2>/dev/null | sort)"
  [ -n "$files" ] || return 0
  for file in $files; do
    rust-code-analysis-cli -m -O json -p "$file" | jq -r --arg root "$ROOT/" '
      def walk_fns($prefix): .spaces[]? | (if .kind == "function" then [$prefix + .name, .metrics.cyclomatic.sum] else empty end), (if .kind == "impl" or .kind == "trait" then .name + "::" else $prefix end) as $next | walk_fns($next);
      (.name | ltrimstr($root)) as $path | walk_fns("") | [$path, .[0], (.[1] | floor | tostring)] | @tsv'
  done | sort -t $'\t' -k1,1 -k2,2 -k3,3nr | awk -F '\t' '!seen[$1 FS $2]++'
}

if [ "${1:-}" = "--write" ]; then
  mkdir -p "$(dirname "$RECORD")"
  measure > "$RECORD"
  echo "wrote $(wc -l < "$RECORD" | tr -d ' ') functions to metrics/cyclomatic.tsv"
  exit 0
fi
[ -f "$RECORD" ] || { fail R-136 "metrics/cyclomatic.tsv is missing; run scripts/check-complexity-ratchet.sh --write"; finish complexity-ratchet; }

while IFS= read -r line; do
  FAILURES+=("$line")
done < <(measure | awk -F '\t' -v record="$RECORD" '
  BEGIN { while ((getline row < record) > 0) { split(row, f, "\t"); known[f[1] FS f[2]] = f[3]; if (f[3] > top) top = f[3] } }
  {
    key = $1 FS $2
    if (key in known) {
      if ($3 > known[key]) printf "R-136: %s %s rose from %d to %d\n", $1, $2, known[key], $3
      else if ($3 < known[key]) printf "R-136: %s %s fell from %d to %d; run scripts/check-complexity-ratchet.sh --write\n", $1, $2, known[key], $3
    } else if ($3 > top) printf "R-136: %s %s is new at %d, over the record maximum %d\n", $1, $2, $3, top
  }')
finish complexity-ratchet
