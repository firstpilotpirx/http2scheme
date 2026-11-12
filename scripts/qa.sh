#!/usr/bin/env bash
# The gate: every check of the repository, in the R-091 order, across both workspaces.
set -uo pipefail

STAGES="fmt lint check complexity machete unsafe-count modules typos recipes dup deny arch unit integration arch-tests e2e build coverage"

usage() {
  cat <<'USAGE'
Usage: scripts/qa.sh [options]

Runs every stage of the gate in order and prints one line per stage.
With no option it only checks; it changes no file.

Options:
  --fix            format and apply clippy's fixes first, then check
  --clean          delete the build outputs first
  --stage <name>   run only this stage (repeatable)
  --skip <name>    leave this stage out (repeatable)
  --json           print one JSON object per stage, no color
  --no-color       print without color (also NO_COLOR, TERM=dumb)
  --help           show this help

Stages: fmt lint check complexity machete unsafe-count modules typos recipes
        dup deny arch unit integration arch-tests e2e build coverage
USAGE
}

fix=0 clean=0 json=0 color=1 only="" skip=""
[ -t 1 ] || color=0
[ -n "${NO_COLOR:-}" ] && color=0
[ "${TERM:-}" = dumb ] && color=0
[ -n "${FORCE_COLOR:-}" ] && color=1
while [ $# -gt 0 ]; do
  case "$1" in
    --fix) fix=1 ;;
    --clean) clean=1 ;;
    --json) json=1 color=0 ;;
    --no-color) color=0 ;;
    --stage) only="$only $2"; shift ;;
    --skip) skip="$skip $2"; shift ;;
    --help|-h) usage; exit 0 ;;
    *) echo "qa.sh: unknown option $1; see --help" >&2; exit 2 ;;
  esac
  shift
done
for name in $only $skip; do
  grep -qw -- "$name" <<<"$STAGES" || { echo "qa.sh: unknown stage $name; see --help" >&2; exit 2; }
done

cd "$(dirname "$0")/.." || exit 2

# recipe <stage>: the just recipe that runs the stage.
recipe() {
  case "$1" in
    check) echo typecheck ;;
    *) echo "$1" ;;
  esac
}

paint() {
  if [ "$color" -eq 1 ]; then printf '\033[%sm%s\033[0m' "$1" "$2"; else printf '%s' "$2"; fi
}

report() {
  local verdict="$1" stage="$2" seconds="$3"
  if [ "$json" -eq 1 ]; then
    printf '{"stage":"%s","verdict":"%s","seconds":%d}\n' "$stage" "$verdict" "$seconds"
    return
  fi
  case "$verdict" in
    OK) printf '[ %s ] %s (%ds)\n' "$(paint 32 OK)" "$stage" "$seconds" ;;
    FAIL) printf '[ %s ] %s (%ds)\n' "$(paint 31 FAIL)" "$stage" "$seconds" ;;
    WARN) printf '[ %s ] %s (%ds)\n' "$(paint 33 WARN)" "$stage" "$seconds" ;;
  esac
}

if [ "$clean" -eq 1 ]; then
  just clean > /dev/null 2>&1 || { echo "qa.sh: clean failed" >&2; exit 1; }
fi
if [ "$fix" -eq 1 ]; then
  just fix > /dev/null 2>&1 || echo "qa.sh: some fixes could not be applied; the stages below name them" >&2
fi

failed=0
log="$(mktemp)"
trap 'rm -f "$log"' EXIT
for stage in $STAGES; do
  if [ -n "$only" ] && ! grep -qw -- "$stage" <<<"$only"; then continue; fi
  if grep -qw -- "$stage" <<<"$skip"; then
    report WARN "$stage" 0
    continue
  fi
  start="$(date +%s)"
  if just "$(recipe "$stage")" > "$log" 2>&1; then
    report OK "$stage" "$(( $(date +%s) - start ))"
  else
    report FAIL "$stage" "$(( $(date +%s) - start ))"
    if [ "$json" -eq 1 ]; then cat "$log" >&2; else cat "$log"; fi
    failed=1
  fi
done
exit "$failed"
