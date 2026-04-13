#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v088/temporal_review_surface}"
STATE_DIR="$OUT_DIR/state"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"

rm -rf "$OUT_DIR"
mkdir -p "$STATE_DIR"

cd "$ROOT_DIR"

cargo run --quiet --manifest-path adl/Cargo.toml -- identity foundation --out "$STATE_DIR/chronosense_foundation_v1.json" >/dev/null
cargo run --quiet --manifest-path adl/Cargo.toml -- identity schema --out "$STATE_DIR/temporal_schema_v1.json" >/dev/null
cargo run --quiet --manifest-path adl/Cargo.toml -- identity continuity --out "$STATE_DIR/continuity_semantics_v1.json" >/dev/null
cargo run --quiet --manifest-path adl/Cargo.toml -- identity retrieval --out "$STATE_DIR/temporal_query_retrieval_v1.json" >/dev/null
cargo run --quiet --manifest-path adl/Cargo.toml -- identity commitments --out "$STATE_DIR/commitment_deadline_semantics_v1.json" >/dev/null
cargo run --quiet --manifest-path adl/Cargo.toml -- identity causality --out "$STATE_DIR/temporal_causality_explanation_v1.json" >/dev/null
cargo run --quiet --manifest-path adl/Cargo.toml -- identity cost --out "$STATE_DIR/execution_policy_cost_model_v1.json" >/dev/null

python3 - "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

manifest_path = Path(sys.argv[1])
manifest = {
    "schema_version": "adl.v088.temporal_review_surface.v1",
    "demo_package_id": "temporal_review_surface",
    "milestone": "v0.88",
    "rows": [
        {
            "demo_id": "D1",
            "focus": "temporal schema and anchors",
            "command": "bash adl/tools/demo_v088_temporal_review_surface.sh",
            "primary_proof_surface": "state/temporal_schema_v1.json",
            "secondary_proof_surfaces": [
                "state/chronosense_foundation_v1.json",
            ],
            "claim": "ADL exposes a reviewer-legible temporal schema and bounded chronosense foundation instead of hidden execution timing assumptions.",
        },
        {
            "demo_id": "D2",
            "focus": "continuity and identity",
            "command": "bash adl/tools/demo_v088_temporal_review_surface.sh",
            "primary_proof_surface": "state/continuity_semantics_v1.json",
            "secondary_proof_surfaces": [
                "state/chronosense_foundation_v1.json",
            ],
            "claim": "ADL surfaces continuity and interruption semantics explicitly enough for a reviewer to inspect state transitions and identity persistence.",
        },
        {
            "demo_id": "D3",
            "focus": "temporal retrieval and commitments",
            "command": "bash adl/tools/demo_v088_temporal_review_surface.sh",
            "primary_proof_surface": "state/temporal_query_retrieval_v1.json",
            "secondary_proof_surfaces": [
                "state/commitment_deadline_semantics_v1.json",
            ],
            "claim": "ADL exposes retrieval and commitment semantics as bounded reviewer artifacts rather than hidden internal bookkeeping.",
        },
        {
            "demo_id": "D4",
            "focus": "execution policy and cost",
            "command": "bash adl/tools/demo_v088_temporal_review_surface.sh",
            "primary_proof_surface": "state/execution_policy_cost_model_v1.json",
            "secondary_proof_surfaces": [
                "state/temporal_causality_explanation_v1.json",
            ],
            "claim": "ADL shows requested execution posture, realized cost, and temporal explanation as one explicit review surface.",
        },
    ],
}
manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<'EOF'
# v0.88 Demo Package - Temporal Review Surface

Canonical command:

```bash
bash adl/tools/demo_v088_temporal_review_surface.sh
```

This package proves the four temporal review rows:
- D1 temporal schema and anchors
- D2 continuity and identity
- D3 temporal retrieval and commitments
- D4 execution policy and cost

Primary proof surfaces:
- `state/temporal_schema_v1.json`
- `state/continuity_semantics_v1.json`
- `state/temporal_query_retrieval_v1.json`
- `state/execution_policy_cost_model_v1.json`
EOF

echo "Temporal review surface under the output directory:"
echo "  demo_manifest.json"
echo "  state/temporal_schema_v1.json"
echo "  state/continuity_semantics_v1.json"
echo "  state/temporal_query_retrieval_v1.json"
echo "  state/execution_policy_cost_model_v1.json"
