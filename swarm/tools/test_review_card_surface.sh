#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE_DIR="$ROOT/docs/tooling/examples/reviewer-surface/issue-661"
ACTUAL="$(mktemp -t review_surface_XXXXXX.yaml)"
trap 'rm -f "$ACTUAL"' EXIT

ruby "$ROOT/swarm/tools/review_card_surface.rb" \
  --input "$ROOT/docs/tooling/examples/reviewer-regression/issue-661/input_661.md" \
  --output "$ROOT/docs/tooling/examples/reviewer-regression/issue-661/output_661.md" \
  > "$ACTUAL"

diff -u "$FIXTURE_DIR/expected_review_surface_output_661.yaml" "$ACTUAL"
echo "review_card_surface fixture passed"
