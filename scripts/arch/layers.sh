#!/usr/bin/env bash
# Layer imports: R-021 to R-024 and R-027.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/lib.sh"

# imports <dir> <pattern> <rule> <message>: every file under <dir> that imports <pattern> is a breach.
imports() {
  while IFS= read -r file; do
    grep -qE "$2" "$file" && fail "$3" "$(relative "$file") $4"
  done < <(sources "$1")
}

for component in $(components); do
  src="$component/src"
  imports "$src/adapters/primary" 'crate::adapters::secondary' R-021 "imports a secondary adapter"
  imports "$src/adapters/secondary" 'crate::adapters::primary' R-022 "imports a primary adapter"
  imports "$src/core/application" 'crate::adapters|crate::[a-z_]+_config' R-023 "imports an adapter or the config"
  imports "$src/core/domain" 'crate::(adapters|core::application)|crate::[a-z_]+_config' R-024 "imports out of the domain"
  if grep -qE 'path *= *"\.\./' "$component/Cargo.toml"; then
    fail R-027 "$(relative "$component/Cargo.toml") depends on another component directly"
  fi
done
# R-030, R-031: an app wires a component through its root modules (the composition root and its
# setup) and *_config types only; never through a secondary adapter or the core.
while IFS= read -r file; do
  for component in $(components); do
    name="$(basename "$component")"
    while IFS= read -r path; do
      case "$path" in
        "$name::adapters::secondary::"*) fail R-030 "$(relative "$file") imports $path, a secondary adapter" ;;
        "$name::adapters::"*_config::*) ;;
        "$name::adapters::"*|"$name::core::"*) fail R-031 "$(relative "$file") imports $path, not the component's config" ;;
      esac
    done < <(grep -oE "\b$name::[a-z_:]+" "$file")
  done
done < <(find "$APP/apps" -name '*.rs' ! -path '*/target/*' 2>/dev/null | sort)

# R-036: a primary adapter reaches the core through one use case, never a repository or a service.
for component in $(components); do
  while IFS= read -r file; do
    count="$(grep -cE 'core::application::use_cases::' "$file")"
    [ "$count" -le 1 ] || fail R-036 "$(relative "$file") uses $count use cases"
    grep -qE 'core::application::(repositories|services)::' "$file" &&
      fail R-036 "$(relative "$file") calls a repository or a service, not a use case"
  done < <(sources "$component/src/adapters/primary")
done

# R-067: the E2E suite does not build against the application's sources.
for manifest in "$ROOT"/app/e2e*/Cargo.toml; do
  [ -f "$manifest" ] || continue
  grep -qE 'path *= *"\.\./' "$manifest" && fail R-067 "$(relative "$manifest") depends on the application's sources"
done
finish layers
