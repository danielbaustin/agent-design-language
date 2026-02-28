#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR/swarm"

release_notes="../docs/milestones/v0.2/RELEASE_NOTES_v0.2.md"
invalid_cmd="cargo run --example coordinator -- examples/v0-2-coordinator-agents-sdk.adl.yaml --print-plan"
quickstart_adl="cargo run --bin adl -- examples/v0-2-coordinator-agents-sdk.adl.yaml --print-plan"
quickstart_swarm="cargo run --bin swarm -- examples/v0-2-coordinator-agents-sdk.adl.yaml --print-plan"

if grep -Fq "$invalid_cmd" "$release_notes"; then
  echo "invalid command present in $release_notes: $invalid_cmd" >&2
  exit 1
fi

has_adl=0
has_swarm=0
if grep -Fq "$quickstart_adl" "$release_notes"; then
  has_adl=1
fi
if grep -Fq "$quickstart_swarm" "$release_notes"; then
  has_swarm=1
fi

if [ "$has_adl" -eq 0 ] && [ "$has_swarm" -eq 0 ]; then
  echo "missing quickstart command in $release_notes: expected one of:" >&2
  echo "  - $quickstart_adl" >&2
  echo "  - $quickstart_swarm" >&2
  exit 1
fi

# Always execute the canonical binary for validation, even when historical docs
# still show the compatibility shim command.
cargo run --bin adl -- examples/v0-2-coordinator-agents-sdk.adl.yaml --print-plan >/dev/null

echo "release-notes command check: ok"
