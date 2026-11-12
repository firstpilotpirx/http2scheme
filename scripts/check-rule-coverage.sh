#!/usr/bin/env bash
# R-096: every registry rule has exactly one verdict in scripts/rules.tsv, every verdict names checks
# that exist and name the rule, and every check the gate runs answers at least one rule.
set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REGISTRY="${RULES_REGISTRY:-$HOME/.claude/rules-custom/enforcement-spec.md}"
LEDGER="$ROOT/scripts/rules.tsv"
failures=()

[ -f "$REGISTRY" ] || { echo "R-096: the rule registry $REGISTRY is missing" >&2; exit 1; }
stages="$(sed -n 's/^STAGES="\(.*\)"$/\1/p' "$ROOT/scripts/qa.sh")"

# script <name>: the script that runs the check, or nothing.
script() {
  for path in "$ROOT/scripts/arch/$1.sh" "$ROOT/scripts/check-$1.sh"; do
    [ -f "$path" ] && { echo "$path"; return; }
  done
}

registry="$(sed -n 's/^#### \(R-[0-9]*\) .*/\1/p' "$REGISTRY" | sort)"
ledger="$(grep -oE '^R-[0-9]+' "$LEDGER" | sort)"
while IFS= read -r rule; do
  failures+=("$rule has no verdict in scripts/rules.tsv")
done < <(comm -23 <(uniq <<<"$registry") <(uniq <<<"$ledger"))
while IFS= read -r rule; do
  failures+=("$rule in scripts/rules.tsv is not in the registry")
done < <(comm -13 <(uniq <<<"$registry") <(uniq <<<"$ledger"))
while IFS= read -r rule; do
  failures+=("$rule has more than one verdict")
done < <(uniq -d <<<"$ledger")

used=""
while IFS=$'\t' read -r rule verdict detail; do
  case "$verdict" in
    n/a) [[ "$detail" == "not applicable: "* ]] || failures+=("$rule is not applicable without a reason") ;;
    check)
      for check in ${detail//,/ }; do
        used="$used $check"
        path="$(script "$check")"
        if [ -n "$path" ]; then
          [ -x "$path" ] || failures+=("$rule: $(basename "$path") is not executable")
          grep -qw -- "$rule" "$path" || failures+=("$rule: $(basename "$path") does not name the rule")
        elif ! grep -qw -- "$check" <<<"$stages"; then
          failures+=("$rule: no stage or script is named $check")
        fi
      done ;;
    *) failures+=("$rule has the verdict \"$verdict\", not check or n/a") ;;
  esac
done < <(grep -E '^R-' "$LEDGER")

checks="$stages"
for path in "$ROOT"/scripts/arch/*.sh "$ROOT"/scripts/check-*.sh; do
  name="$(basename "$path" .sh)"
  name="${name#check-}"
  [ "$name" = lib ] || checks="$checks $name"
done
for check in $checks; do
  grep -qw -- "$check" <<<"$used" || failures+=("the check $check answers no rule")
done

if [ "${#failures[@]}" -gt 0 ]; then
  printf 'R-096: %s\n' "${failures[@]}" >&2
  exit 1
fi
