#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ADL_TOOLING_MANIFEST_ROOT="$ROOT"
VERIFY="$ROOT/adl/tools/verify_review_output_provenance.rb"
GOOD="$ROOT/docs/tooling/examples/reviewer-provenance/good_review_output_661.yaml"
BAD="$ROOT/docs/tooling/examples/reviewer-provenance/bad_review_output_661.yaml"

"$VERIFY" --review "$GOOD" >/dev/null

if "$VERIFY" --review "$BAD" >/dev/null 2>&1; then
  echo "assertion failed: invalid review provenance artifact unexpectedly passed" >&2
  exit 1
fi

echo "review output provenance fixtures passed"
