#!/usr/bin/env bash
# Core purity: R-032 to R-035.
set -uo pipefail
# shellcheck source=scripts/arch/lib.sh
source "$(dirname "$0")/lib.sh"

checks=(
  'R-032;\b(axum|hyper|reqwest|ureq|config|clap|owo_colors)::;a framework'
  'R-033;\bserde\b;serialization'
  'R-034;\b(log|tracing|env_logger)::;a logger'
  'R-035;\b(std::(fs|net|process|env)|tokio::(fs|net|process|io))\b;I/O'
)
for component in $(components); do
  while IFS= read -r file; do
    for check in "${checks[@]}"; do
      IFS=';' read -r rule pattern what <<<"$check"
      grep -qE "$pattern" "$file" && fail "$rule" "$(relative "$file") uses $what in core"
    done
  done < <(sources "$component/src/core")
done
for component in $(components); do
  while IFS= read -r file; do
    grep -qwE 'f32|f64' "$file" && fail R-088 "$(relative "$file") uses floating point in core"
  done < <(sources "$component/src/core")
  # R-083: hosts and ports come from the settings, never from literals.
  while IFS= read -r file; do
    grep -vE '^[[:space:]]*//' "$file" | grep -qE '"[^"]*(127\.0\.0\.1|localhost|0\.0\.0\.0|:[0-9]{4,5}\b)' &&
      fail R-083 "$(relative "$file") holds a host or a port literal"
  done < <(sources "$component/src/core"; sources "$component/src/adapters")
done
# R-085: no dynamic-type escape hatches anywhere.
while IFS= read -r file; do
  grep -qE '\bdyn Any\b|std::any::Any\b|\btransmute\b|downcast' "$file" && fail R-085 "$(relative "$file") escapes the type system"
done < <(sources "$APP")
finish purity
