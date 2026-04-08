#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/provider_demo_common"
README_FILE="$OUT_DIR/README.md"

source "$ROOT_DIR/adl/tools/provider_demo_common.sh"

provider_demo_write_readme \
  "$OUT_DIR" \
  "Provider Demo Harness Test" \
  "bash adl/tools/demo_example.sh" \
  "artifacts/example/run_summary.json" \
  $'artifacts/example/run_status.json\nartifacts/example/trace_v1.json' \
  "the run succeeds and leaves stable artifacts"

[[ -f "$README_FILE" ]] || {
  echo "assertion failed: README missing" >&2
  exit 1
}

grep -Fq '# Provider Demo Harness Test' "$README_FILE" || {
  echo "assertion failed: README title missing" >&2
  exit 1
}
grep -Fq 'bash adl/tools/demo_example.sh' "$README_FILE" || {
  echo "assertion failed: canonical command missing" >&2
  exit 1
}
grep -Fq 'artifacts/example/run_summary.json' "$README_FILE" || {
  echo "assertion failed: primary proof surface missing" >&2
  exit 1
}
grep -Fq 'artifacts/example/run_status.json' "$README_FILE" || {
  echo "assertion failed: secondary proof surface missing" >&2
  exit 1
}
grep -Fq 'the run succeeds and leaves stable artifacts' "$README_FILE" || {
  echo "assertion failed: success signal missing" >&2
  exit 1
}

echo "provider_demo_common: ok"
