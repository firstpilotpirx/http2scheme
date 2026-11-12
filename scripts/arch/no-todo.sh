#!/usr/bin/env bash
# R-080: no comment inside a body. R-081: no TODO, FIXME or commented-out code in committed source.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/lib.sh"

while IFS= read -r file; do
  while IFS= read -r line; do
    fail R-081 "$(relative "$file"):${line%%:*} holds a TODO or FIXME"
  done < <(grep -nE '\b(TODO|FIXME)\b' "$file")
  # A plain comment (not a doc comment) ending like a statement or a block is code left behind.
  while IFS= read -r line; do
    fail R-081 "$(relative "$file"):${line%%:*} holds commented-out code"
  done < <(grep -nE '^[[:space:]]*//[^/!].*(;|\{|\})[[:space:]]*$' "$file")
  # R-080: a plain comment inside a body is a name not given or a step not extracted;
  # a one-line SAFETY: or WHY: justification is allowed, at most three a file.
  while IFS= read -r line; do
    fail R-080 "$(relative "$file"):${line%%:*} holds a comment inside a body"
  done < <([[ "$file" == *.rs ]] && grep -nE '^[[:space:]]{4,}//([^/!]|$)' "$file" | grep -vE '^[0-9]+:[[:space:]]*// (SAFETY|WHY): ')
  justifications="$(grep -cE '^[[:space:]]*// (SAFETY|WHY): ' "$file")"
  [ "$justifications" -le 3 ] || fail R-080 "$(relative "$file") holds $justifications SAFETY or WHY justifications, more than 3"
done < <(find "$APP" "$ROOT/app/e2e" "$ROOT/scripts" \( -name '*.rs' -o -name '*.sh' \) ! -path '*/target/*' 2>/dev/null | grep -v '/scripts/arch/no-todo.sh$' | sort)
finish no-todo
