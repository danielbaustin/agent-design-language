#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VALIDATOR="$ROOT/swarm/tools/validate_structured_prompt.rb"
GOOD="$ROOT/docs/tooling/examples/workflow-state/good_output_record.md"
BAD="$ROOT/docs/tooling/examples/workflow-state/bad_output_record.md"

ruby "$VALIDATOR" --type sor --phase completed --input "$GOOD" >/dev/null

if ruby "$VALIDATOR" --type sor --phase completed --input "$BAD" >/dev/null 2>&1; then
  echo "assertion failed: invalid completed output record unexpectedly passed" >&2
  exit 1
fi

echo "completed output validation fixtures passed"
