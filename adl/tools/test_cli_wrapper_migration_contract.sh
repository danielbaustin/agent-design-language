#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

for retired in \
  adl/tools/pr.sh \
  adl/tools/skills/workflow-conductor/scripts/route_workflow.py \
  adl/tools/validate_structured_prompt.sh \
  adl/tools/prompt_template.sh; do
  [[ ! -e "$ROOT_DIR/$retired" ]] || {
    echo "retired v1 route unexpectedly exists: $retired" >&2
    exit 1
  }
done

cargo run --quiet --locked --manifest-path "$ROOT_DIR/csdlc-v2/Cargo.toml" --bin csdlc-install -- --help >/dev/null
cargo run --quiet --locked --manifest-path "$ROOT_DIR/csdlc-v2/Cargo.toml" --bin csdlc-bind -- --help >/dev/null

echo "PASS test_cli_wrapper_migration_contract"
