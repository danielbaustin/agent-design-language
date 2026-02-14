#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "• Running tooling sanity checks (batched)…"
bash -n "$ROOT/swarm/tools/codex_pr.sh"
bash -n "$ROOT/swarm/tools/codexw.sh"
sh "$ROOT/swarm/tools/codex_pr.sh" --help >/dev/null
sh "$ROOT/swarm/tools/codexw.sh" --help >/dev/null

echo "• Running swarm checks (batched)…"
(
  cd "$ROOT/swarm"
  cargo fmt
  cargo clippy --all-targets -- -D warnings
  cargo test
)
