#!/usr/bin/env bash
# Naming: R-040 to R-054.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/lib.sh"

# suffix <dir> <rule> <regex> <message>: every file under <dir> whose name misses <regex> is a breach.
suffix() {
  while IFS= read -r file; do
    basename "$file" .rs | grep -qE "$3" || fail "$2" "$(relative "$file") $4"
  done < <(sources "$1")
}

# methods <dir> <rule> <regex> <message>: every trait method under <dir> that misses <regex> is a breach.
methods() {
  while IFS= read -r file; do
    while IFS= read -r method; do
      grep -qE "$3" <<<"$method" || fail "$2" "$(relative "$file") declares $method; $4"
    done < <(awk '/^pub trait /{inside=1; next} inside && /^}/{inside=0} inside && match($0, /fn [a-z_]+/) {print substr($0, RSTART + 3, RLENGTH - 3)}' "$file")
  done < <(sources "$1")
}

check_component() {
  local src="$1/src"
  suffix "$src/core/domain" R-040 '_(value_object|entity|aggregate|policy|domain_event|domain_service|error|enum)$' "has no domain type suffix"
  suffix "$src/core/application/exceptions" R-042 '_error$' "is an error without the _error suffix"
  suffix "$src/core/application/repositories" R-043 '_repository$' "is a repository port without the _repository suffix"
  suffix "$src/core/application/services" R-044 '_service$' "is a service port without the _service suffix"
  for dir in "$src"/adapters/secondary/*/filesystem; do
    [ -d "$dir" ] && suffix "$dir" R-045 '_(repository|record|mapper)$' "is a storage adapter without the _repository suffix"
  done
  for dir in "$src"/adapters/secondary/*/http; do
    [ -d "$dir" ] && suffix "$dir" R-046 '_(client|gateway|mapper)$' "is a remote client without the _client suffix"
  done
  suffix "$src/adapters/secondary" R-048 '_(repository|gateway|client|record|mapper)$' "is a secondary adapter with no role suffix"
  methods "$src/core/application/repositories" R-050 '^(exists|save|delete|record|(read|create|find|list|count)(_[a-z_]+)?)$' "not a repository verb"
  methods "$src/core/application/use_cases" R-051 '^(execute|new)$' "a use case method is execute"
}

for component in $(components); do
  check_component "$component"
done
while IFS= read -r file; do
  name="$(basename "$file" .rs)"
  rel="$(relative "$file")"
  grep -qiE '(manager|helper|helpers|util|utils|impl|data|info)$' <<<"$name" && fail R-047 "$rel has a banned suffix"
  grep -qE '^[a-z][a-z0-9_]*$' <<<"$name" || fail R-052 "$rel is not a snake_case file name"
  grep -qE '^pub trait I[A-Z]|^pub struct [A-Za-z]+Impl\b' "$file" && fail R-049 "$rel declares an I-prefixed trait or an Impl type"
  count="$(grep -cE '^pub (struct|enum|trait) ' "$file")"
  [ "$count" -le 1 ] || fail R-054 "$rel declares $count public types"
done < <(sources "$APP")
while IFS= read -r dir; do
  grep -qE '^[a-z][a-z0-9_]*$' <<<"$(basename "$dir")" || fail R-053 "$(relative "$dir") is not a snake_case folder"
done < <(find "$APP/components" "$APP/apps" -mindepth 1 -type d ! -path '*/target*' 2>/dev/null)
finish naming
