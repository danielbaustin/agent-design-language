#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR/swarm"

release_notes="RELEASE_NOTES_v0.2.md"
invalid_cmd="cargo run --example coordinator -- examples/v0-2-coordinator-agents-sdk.adl.yaml --print-plan"
quickstart_cmd="cargo run -- examples/v0-2-coordinator-agents-sdk.adl.yaml --print-plan"

if grep -Fq "$invalid_cmd" "$release_notes"; then
  echo "invalid command present in $release_notes: $invalid_cmd" >&2
  exit 1
fi

if ! grep -Fq "$quickstart_cmd" "$release_notes"; then
  echo "missing quickstart command in $release_notes: $quickstart_cmd" >&2
  exit 1
fi

cargo run -- examples/v0-2-coordinator-agents-sdk.adl.yaml --print-plan >/dev/null

echo "release-notes command check: ok"
