#!/usr/bin/env bash
# R-131: no file over 300 code lines. R-135: no type with over 35 public members.
# R-137: no struct or variant with over 12 fields. R-138: no directory over 2000 code lines.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/arch/lib.sh"

MAX_FILE=300
MAX_PUBLIC=35
MAX_FIELDS=12
MAX_DIR=2000

# code_lines <file>: lines that are neither blank nor comments.
code_lines() {
  grep -cvE '^[[:space:]]*($|//)' "$1"
}

totals=""
while IFS= read -r file; do
  rel="$(relative "$file")"
  lines="$(code_lines "$file")"
  [ "$lines" -le "$MAX_FILE" ] || fail R-131 "$rel has $lines code lines (budget $MAX_FILE)"
  totals+="$(dirname "$rel")"$'\t'"$lines"$'\n'
  while IFS=$'\t' read -r rule type count; do
    case "$rule" in
      R-135) fail R-135 "$rel: $type has $count public members (budget $MAX_PUBLIC)" ;;
      R-137) fail R-137 "$rel: $type has $count fields (budget $MAX_FIELDS)" ;;
    esac
  done < <(awk -v max_public="$MAX_PUBLIC" -v max_fields="$MAX_FIELDS" '
    /^impl/ { impl = $0; sub(/[[:space:]]*\{.*/, "", impl); sub(/.* for /, "", impl); sub(/^impl(<[^>]*>)? /, "", impl); next }
    /^}/ { impl = ""; record = "" }
    impl != "" && /^    pub (const )?(async )?fn / { members[impl]++ }
    /^pub (\(crate\) )?struct [A-Za-z]+.*\{[[:space:]]*$/ { record = $3; if (record == "struct") record = $4; sub(/[<{].*/, "", record); next }
    /^pub (\(crate\) )?enum / { enumname = $0 }
    /^    [A-Z][A-Za-z]* \{[[:space:]]*$/ { variant = $1; next }
    variant != "" && /^    \}/ { variant = "" }
    record != "" && /^    (pub(\([a-z]+\))? )?[a-z_][a-z0-9_]*: / { fields[record]++ }
    variant != "" && /^        [a-z_][a-z0-9_]*: / { fields[variant]++ }
    END {
      for (t in members) if (members[t] > max_public) printf "R-135\t%s\t%d\n", t, members[t]
      for (t in fields) if (fields[t] > max_fields) printf "R-137\t%s\t%d\n", t, fields[t]
    }' "$file")
done < <(find "$ROOT/app" -name '*.rs' ! -path '*/target/*' 2>/dev/null | sort)
while IFS=$'\t' read -r dir lines; do
  fail R-138 "$dir holds $lines code lines (budget $MAX_DIR)"
done < <(printf '%s' "$totals" | awk -F '\t' -v max="$MAX_DIR" '{ sum[$1] += $2 } END { for (d in sum) if (sum[d] > max) printf "%s\t%d\n", d, sum[d] }')
finish size-limits
