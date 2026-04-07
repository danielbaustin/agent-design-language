#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
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

cat >"$OUT_DIR/README.md" <<EOF
# v0.87 Demo D2 - Provider portability substrate

Command:

\`\`\`bash
bash adl/tools/demo_v087_provider_portability.sh
\`\`\`

Primary proof surface:
- \`artifacts/v087/provider_portability/provider_substrate_manifest.v1.json\`

Secondary proof surfaces:
- \`artifacts/v087/provider_portability/print_plan.txt\`
- \`$EXAMPLE\`

This demo proves the bounded provider-substrate normalization surface without
requiring a live remote provider call.
EOF

echo "Demo proof surface:"
echo "  $OUT_DIR/provider_substrate_manifest.v1.json"
echo "  $OUT_DIR/print_plan.txt"

