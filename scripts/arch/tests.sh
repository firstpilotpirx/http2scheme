#!/usr/bin/env bash
# Tests beside the code: R-071, R-072 sibling unit tests (a superset of R-060, the core types);
# R-061, R-062, R-063 adapter integration tests; R-068 hermetic unit tests.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/lib.sh"

TRIVIAL='_(record|mapper|config|request|response|dto|message|event|error|query|row)$'

for component in $(components); do
  src="$component/src"
  while IFS= read -r file; do
    name="$(basename "$file" .rs)"
    rel="$(relative "$file")"
    if [ ! -f "${file%.rs}_test.rs" ]; then
      fail R-071 "$rel has no sibling ${name}_test.rs"
    elif ! grep -q "#\[path = \"${name}_test.rs\"\]" "$file"; then
      fail R-071 "$rel does not declare ${name}_test.rs"
    fi
    grep -qE '#\[(tokio::)?test\]' "$file" && fail R-072 "$rel holds a test body"
  done < <(sources "$src")
  # Shared fixtures of several sibling tests are named *_fixtures_test.rs and have no implementation.
  while IFS= read -r test; do
    [ -f "${test%_test.rs}.rs" ] || fail R-071 "$(relative "$test") has no implementation beside it"
  done < <(find "$src" -name '*_test.rs' ! -name '*_fixtures_test.rs')
  # R-068: unit tests open no socket, start no program and touch no file outside a temporary folder.
  while IFS= read -r test; do
    grep -qE 'TcpListener|TcpStream|UdpSocket|process::Command|std::fs::|tokio::fs::' "$test" &&
      ! grep -q 'tempfile' "$test" && fail R-068 "$(relative "$test") is not hermetic"
  done < <(find "$src" -name '*_test.rs')
  for side in primary:R-061 secondary:R-062; do
    while IFS= read -r file; do
      name="$(basename "$file" .rs)"
      grep -qE "$TRIVIAL" <<<"$name" && continue
      [ -f "$component/tests/${name}_integration_test.rs" ] ||
        fail "${side#*:}" "$(relative "$file") has no tests/${name}_integration_test.rs"
    done < <(sources "$src/adapters/${side%%:*}")
  done
  while IFS= read -r test; do
    name="$(basename "$test" _integration_test.rs)"
    grep -qE "$TRIVIAL" <<<"$name" && fail R-063 "$(relative "$test") tests a trivial type"
  done < <(find "$component/tests" -maxdepth 1 -name '*_integration_test.rs')
done
finish tests
