#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v088/review_surface}"
TEMPORAL_ROOT="$OUT_DIR/temporal"
PHI_ROOT="$OUT_DIR/phi"
INSTINCT_ROOT="$OUT_DIR/instinct"
PAPER_ROOT="$OUT_DIR/paper_sonata"
COMPARATIVE_ROOT="$OUT_DIR/deep_agents_comparative"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"
INDEX_OUT="$OUT_DIR/index.txt"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

bash adl/tools/demo_v088_temporal_review_surface.sh "$TEMPORAL_ROOT" >/dev/null
bash adl/tools/demo_v088_phi_review_surface.sh "$PHI_ROOT" >/dev/null
bash adl/tools/demo_v088_instinct_review_surface.sh "$INSTINCT_ROOT" >/dev/null
bash adl/tools/demo_v088_paper_sonata.sh "$PAPER_ROOT" >/dev/null
bash adl/tools/demo_v088_deep_agents_comparative_proof.sh "$COMPARATIVE_ROOT" >/dev/null

python3 - "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.v088.review_surface.v1",
    "milestone": "v0.88",
    "title": "v0.88 integrated review surface",
    "review_root": "artifacts/v088/review_surface",
    "rows": [
        {
            "demo_id": "D1-D4",
            "package": "temporal",
            "command": "bash adl/tools/demo_v088_temporal_review_surface.sh",
            "primary_proof_surface": "temporal/demo_manifest.json"
        },
        {
            "demo_id": "D5",
            "package": "phi",
            "command": "bash adl/tools/demo_v088_phi_review_surface.sh",
            "primary_proof_surface": "phi/state/phi_integration_metrics_v1.json"
        },
        {
            "demo_id": "D6-D7",
            "package": "instinct",
            "command": "bash adl/tools/demo_v088_instinct_review_surface.sh",
            "primary_proof_surface": "instinct/demo_manifest.json"
        },
        {
            "demo_id": "D8",
            "package": "paper_sonata",
            "command": "bash adl/tools/demo_v088_paper_sonata.sh",
            "primary_proof_surface": "paper_sonata/demo_manifest.json"
        },
        {
            "demo_id": "D9",
            "package": "deep_agents_comparative",
            "command": "bash adl/tools/demo_v088_deep_agents_comparative_proof.sh",
            "primary_proof_surface": "deep_agents_comparative/comparative_manifest.json"
        }
    ]
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<'EOF'
# v0.88 Demo D10 - Integrated Review Surface

Canonical review command:

```bash
bash adl/tools/demo_v088_review_surface.sh
```

Reviewer flow:
- inspect `demo_manifest.json`
- review the temporal package for D1-D4
- review the PHI package for D5
- review the instinct package for D6-D7
- review the Paper Sonata flagship package for D8
- review the comparative proof packet for D9
EOF

cat >"$INDEX_OUT" <<'EOF'
D1-D4 temporal/demo_manifest.json
D5 phi/state/phi_integration_metrics_v1.json
D6-D7 instinct/demo_manifest.json
D8 paper_sonata/demo_manifest.json
D9 deep_agents_comparative/comparative_manifest.json
EOF

echo "v0.88 review surface:"
echo "  artifacts/v088/review_surface/demo_manifest.json"
echo "  artifacts/v088/review_surface/temporal/demo_manifest.json"
echo "  artifacts/v088/review_surface/phi/state/phi_integration_metrics_v1.json"
echo "  artifacts/v088/review_surface/instinct/demo_manifest.json"
echo "  artifacts/v088/review_surface/paper_sonata/demo_manifest.json"
echo "  artifacts/v088/review_surface/deep_agents_comparative/comparative_manifest.json"
