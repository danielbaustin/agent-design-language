#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v087/shared_obsmem}"
DEMO_ID="demo-f-obsmem-retrieval"

mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87 shared-ObsMem demo..."
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- \
  demo "$DEMO_ID" \
  --run \
  --trace \
  --out "$OUT_DIR"

echo "Demo proof surface:"
echo "  $OUT_DIR/$DEMO_ID/obsmem_retrieval_result.json"
echo "  $OUT_DIR/$DEMO_ID/trace.jsonl"

