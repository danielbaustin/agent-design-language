#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v088/phi_review_surface}"
STATE_DIR="$OUT_DIR/state"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"

rm -rf "$OUT_DIR"
mkdir -p "$STATE_DIR"

cd "$ROOT_DIR"
cargo run --quiet --manifest-path adl/Cargo.toml -- identity phi --out "$STATE_DIR/phi_integration_metrics_v1.json" >/dev/null

cat >"$MANIFEST" <<'EOF'
{
  "schema_version": "adl.v088.phi_review_surface.v1",
  "demo_id": "D5",
  "focus": "PHI-style integration metrics",
  "command": "bash adl/tools/demo_v088_phi_review_surface.sh",
  "primary_proof_surface": "state/phi_integration_metrics_v1.json",
  "claim": "ADL can present low / medium / high integration profiles with explicit dimensions and no metaphysical overclaim."
}
EOF

cat >"$README_OUT" <<'EOF'
# v0.88 Demo Package - PHI Review Surface

Canonical command:

```bash
bash adl/tools/demo_v088_phi_review_surface.sh
```

Primary proof surface:
- `state/phi_integration_metrics_v1.json`
EOF

echo "PHI review surface under the output directory:"
echo "  demo_manifest.json"
echo "  state/phi_integration_metrics_v1.json"
