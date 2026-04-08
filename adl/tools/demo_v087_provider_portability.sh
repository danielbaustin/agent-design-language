#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(provider_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v087/provider_portability}"
EXAMPLE="adl/examples/v0-7-provider-portability-http-profile.adl.yaml"

mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87 provider-portability demo..."
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- \
  instrument provider-substrate "$EXAMPLE" \
  >"$OUT_DIR/provider_substrate_manifest.v1.json"

cargo run -q --manifest-path adl/Cargo.toml --bin adl -- \
  "$EXAMPLE" \
  --print-plan \
  >"$OUT_DIR/print_plan.txt"

provider_demo_write_readme \
  "$OUT_DIR" \
  "v0.87 Demo D2 - Provider portability substrate" \
  "bash adl/tools/demo_v087_provider_portability.sh" \
  "artifacts/v087/provider_portability/provider_substrate_manifest.v1.json" \
  $'artifacts/v087/provider_portability/print_plan.txt\n'"$EXAMPLE" \
  "This demo proves the bounded provider-substrate normalization surface without requiring a live remote provider call."

provider_demo_print_proof_surfaces \
  "$OUT_DIR/provider_substrate_manifest.v1.json" \
  "$OUT_DIR/print_plan.txt"
