#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR/adl"

release_notes="../docs/milestones/v0.86/RELEASE_NOTES_v0.86.md"
demo_matrix_ref="DEMO_MATRIX_v0.86.md"
tag_ref='Tag: `v0.86`'
release_gate_ref='Release date: `Pending manual release ceremony`'

if ! grep -Fq "$demo_matrix_ref" "$release_notes"; then
  echo "missing demo-matrix reference in $release_notes: $demo_matrix_ref" >&2
  exit 1
fi

if ! grep -Fq "$tag_ref" "$release_notes"; then
  echo "missing tag reference in $release_notes: $tag_ref" >&2
  exit 1
fi

if ! grep -Fq "$release_gate_ref" "$release_notes"; then
  echo "missing pre-ceremony release-date note in $release_notes: $release_gate_ref" >&2
  exit 1
fi

echo "release-notes command check: ok"
