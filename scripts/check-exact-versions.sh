#!/usr/bin/env bash
# R-127: every external version a build or a script depends on is one exact version.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/arch/lib.sh"

exact='^=[0-9]+\.[0-9]+\.[0-9]+([-+][0-9A-Za-z.-]+)?$'

# Dependency requirements in every manifest.
while IFS= read -r manifest; do
  while IFS=$'\t' read -r line name requirement; do
    fail R-127 "$(relative "$manifest"):$line $name is \"$requirement\", not one exact version"
  done < <(awk -v exact="$exact" '
    /^\[/ { deps = ($0 ~ /dependencies\]$/) ; next }
    !deps || /^[[:space:]]*(#|$)/ { next }
    {
      name = $0; sub(/[[:space:]]*=.*/, "", name)
      if ($0 ~ /workspace[[:space:]]*=[[:space:]]*true/ || ($0 ~ /path[[:space:]]*=/ && $0 !~ /version[[:space:]]*=/)) next
      if ($0 ~ /git[[:space:]]*=/) { if ($0 !~ /rev[[:space:]]*=[[:space:]]*"[0-9a-f]{40}"/) printf "%d\t%s\tgit without a commit\n", NR, name; next }
      requirement = $0
      if (requirement ~ /version[[:space:]]*=/) sub(/.*version[[:space:]]*=[[:space:]]*"/, "", requirement)
      else sub(/^[^=]*=[[:space:]]*"/, "", requirement)
      sub(/".*/, "", requirement)
      if (requirement !~ exact) printf "%d\t%s\t%s\n", NR, name, requirement
    }' "$manifest")
done < <(find "$ROOT" -name Cargo.toml ! -path '*/target/*' ! -path '*/gate_fixtures/*' 2>/dev/null | sort)

# Toolchain channels.
while IFS= read -r toolchain; do
  grep -qE '^channel[[:space:]]*=[[:space:]]*"[0-9]+\.[0-9]+\.[0-9]+"' "$toolchain" \
    || fail R-127 "$(relative "$toolchain") pins no exact toolchain version"
done < <(find "$ROOT" -name 'rust-toolchain*' ! -path '*/target/*' ! -path '*/gate_fixtures/*' 2>/dev/null | sort)

# Tool installs in recipes and scripts.
while IFS= read -r file; do
  while IFS= read -r line; do
    fail R-127 "$(relative "$file"):${line%%:*} installs a tool with no --version"
  done < <(grep -nE 'cargo (binstall|install) ' "$file" | grep -vE -- '--version ([0-9]+\.[0-9]+\.[0-9]+|" \+ [a-z_]+_version)|--path ')
  # A pin held in a just variable is checked where it is written.
  while IFS= read -r line; do
    fail R-127 "$(relative "$file"):${line%%:*} pins a tool to no exact version"
  done < <(grep -nE '^[a-z_]+_version := ' "$file" | grep -vE ':= "[0-9]+\.[0-9]+\.[0-9]+"$')
done < <(find "$ROOT" \( -name justfile -o -name '*.sh' -o -name '*.yml' -o -name '*.yaml' -o -name 'README.md' \) ! -path '*/target/*' ! -path '*/gate_fixtures/*' ! -path '*/node_modules/*' ! -path '*/openspec/*' 2>/dev/null | sort)
finish exact-versions
