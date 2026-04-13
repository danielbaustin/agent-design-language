#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v088/instinct_review_surface}"
STATE_DIR="$OUT_DIR/state"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"

rm -rf "$OUT_DIR"
mkdir -p "$STATE_DIR"

cd "$ROOT_DIR"
cargo run --quiet --manifest-path adl/Cargo.toml -- identity instinct --out "$STATE_DIR/instinct_model_v1.json" >/dev/null
cargo run --quiet --manifest-path adl/Cargo.toml -- identity instinct-runtime --out "$STATE_DIR/instinct_runtime_surface_v1.json" >/dev/null

python3 - "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.v088.instinct_review_surface.v1",
    "milestone": "v0.88",
    "rows": [
        {
            "demo_id": "D6",
            "focus": "instinct declaration and influence",
            "command": "bash adl/tools/demo_v088_instinct_review_surface.sh",
            "primary_proof_surface": "state/instinct_model_v1.json",
            "secondary_proof_surfaces": ["state/instinct_runtime_surface_v1.json"],
            "claim": "ADL exposes instinct explicitly as a bounded declared runtime input rather than hidden mood or persona drift."
        },
        {
            "demo_id": "D7",
            "focus": "bounded agency under instinct",
            "command": "bash adl/tools/demo_v088_instinct_review_surface.sh",
            "primary_proof_surface": "state/instinct_runtime_surface_v1.json",
            "secondary_proof_surfaces": ["state/instinct_model_v1.json"],
            "claim": "ADL can show a deterministic bounded case where instinct changes candidate selection without escaping risk and policy limits."
        }
    ]
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<'EOF'
# v0.88 Demo Package - Instinct Review Surface

Canonical command:

```bash
bash adl/tools/demo_v088_instinct_review_surface.sh
```

Primary proof surfaces:
- `state/instinct_model_v1.json`
- `state/instinct_runtime_surface_v1.json`
EOF

echo "Instinct review surface under the output directory:"
echo "  demo_manifest.json"
echo "  state/instinct_model_v1.json"
echo "  state/instinct_runtime_surface_v1.json"
