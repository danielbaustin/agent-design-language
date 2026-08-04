#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -e adl/tools/validate_structured_prompt.sh ]]; then
  echo "assertion failed: sunset structured-prompt validator was restored" >&2
  exit 1
fi

# This fixture performs sixteen real csdlc-edit CLI requests against an
# issue-bound record, then checks cross-card validation and clean doctor truth.
cargo test --quiet --locked --manifest-path csdlc-v2/Cargo.toml --test gate2 \
  issue_5337_preparation_converts_to_complete_implementation_truth_with_typed_edits -- --exact

# Representative invalid repairs (empty fields, wrong owners, stale CAS,
# missing claims, and incomplete acceptance mappings) must leave record and
# cards byte-for-byte unchanged.
cargo test --quiet --locked --manifest-path csdlc-v2/Cargo.toml --test gate2 \
  planning_replacements_reject_invalid_requests_without_mutation -- --exact

echo "PASS: typed card repair accepts coherent edits and rejects invalid repairs atomically"
