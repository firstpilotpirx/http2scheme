#!/usr/bin/env bash
# Component layout: R-011 to R-018.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/lib.sh"

layers=(core/domain core/application adapters/primary adapters/secondary)
for component in $(components); do
  name="$(basename "$component")"
  src="$component/src"
  for layer in "${layers[@]}"; do
    [ -d "$src/$layer" ] || fail R-011 "$(relative "$src") has no $layer folder"
  done
  while IFS= read -r dir; do
    fail R-013 "$(relative "$dir") is a ports folder; ports live in core/application"
  done < <(find "$src" -type d -name ports)
  while IFS= read -r dir; do
    fail R-014 "$(relative "$dir") nests a folder inside core/domain"
  done < <(find "$src/core/domain" -mindepth 1 -type d 2>/dev/null)
  while IFS= read -r dir; do
    base="$(basename "$dir")"
    case "$base" in
      use_cases|repositories|services|exceptions) ;;
      *) fail R-015 "$(relative "$dir") is not one of use_cases, repositories, services, exceptions" ;;
    esac
    case "$base" in *s) ;; *) fail R-016 "$(relative "$dir") is not a plural name" ;; esac
  done < <(find "$src/core/application" -mindepth 1 -maxdepth 1 -type d 2>/dev/null)
  while IFS= read -r file; do
    depth="$(relative "$file" | tr -cd '/' | wc -c)"
    base_depth="$(relative "$src/adapters/primary/x/y/z.rs" | tr -cd '/' | wc -c)"
    [ "$depth" -eq "$base_depth" ] || fail R-012 "$(relative "$file") is not in adapters/<side>/<feature>/<tech>/"
  done < <(sources "$src/adapters")
  [ -f "$src/${name}_config.rs" ] || fail R-017 "$(relative "$src") has no ${name}_config.rs at its root"
  while IFS= read -r file; do
    case "$file" in
      "$src/${name}_config.rs"|"$src/adapters/"*/*/*/*_config.rs) ;;
      *) fail R-018 "$(relative "$file") is a config outside its tech folder" ;;
    esac
  done < <(find "$src" -name '*_config.rs' 2>/dev/null)
done
finish layout
