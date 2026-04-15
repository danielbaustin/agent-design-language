#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v089/review_surface}"
ENTRYPOINT_ROOT="$OUT_DIR/proof_entrypoints"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"
INDEX_OUT="$OUT_DIR/index.txt"
CLAIM_MATRIX_OUT="$OUT_DIR/claim_matrix.md"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

bash adl/tools/demo_v089_proof_entrypoints.sh "$ENTRYPOINT_ROOT" >/dev/null

python3 - "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.v089.review_surface.v1",
    "milestone": "v0.89",
    "title": "v0.89 integrated review surface",
    "review_root": "artifacts/v089/review_surface",
    "packages": [
        {
            "package_id": "core_runtime_control",
            "covers_demo_ids": ["D1", "D2", "D3", "D4"],
            "title": "Core runtime-control proof family",
            "command": "bash adl/tools/demo_v089_proof_entrypoints.sh",
            "primary_proof_surface": "proof_entrypoints/demo_manifest.json",
            "secondary_proof_surfaces": [
                "proof_entrypoints/README.md",
                "proof_entrypoints/index.txt"
            ]
        },
        {
            "package_id": "experiment_memory_security",
            "covers_demo_ids": ["D5", "D6", "D7"],
            "title": "Experiment, memory, and security review family",
            "command": "bash adl/tools/demo_v089_proof_entrypoints.sh",
            "primary_proof_surface": "proof_entrypoints/demo_manifest.json",
            "secondary_proof_surfaces": [
                "proof_entrypoints/README.md",
                "proof_entrypoints/index.txt"
            ]
        }
    ]
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<'EOF'
# v0.89 Demo Integration Review Surface

Canonical review command:

```bash
bash adl/tools/demo_v089_review_surface.sh
```

Primary proof surface:
- `artifacts/v089/review_surface/demo_manifest.json`

Secondary proof surfaces:
- `artifacts/v089/review_surface/claim_matrix.md`
- `artifacts/v089/review_surface/proof_entrypoints/demo_manifest.json`
- `artifacts/v089/review_surface/proof_entrypoints/README.md`
- `artifacts/v089/review_surface/proof_entrypoints/index.txt`

Reviewer walkthrough:
- Review the integrated manifest first to understand the two bounded package families.
- Inspect the core runtime-control family for D1 through D4.
- Inspect the experiment, memory, and security family for D5 through D7.
- Use the claim matrix to confirm that each demo row maps to one milestone claim and one primary proof surface.

Important boundary:
- This review surface integrates the `v0.89` proof rows into one reviewer package.
- It does not replace the row-specific commands or claim that all proof rows are executed by one heavyweight script.
- D7 remains a document-driven main-band proof row and should not be confused with the later `v0.89.1` adversarial runtime package.
EOF

cat >"$CLAIM_MATRIX_OUT" <<'EOF'
# v0.89 Claim Matrix

| Demo ID | Milestone claim | Primary proof surface | Reviewer note |
|---|---|---|---|
| D1 | bounded convergence and stop conditions | `control_path/convergence.json` | shares the runtime-control bundle with D2 and D3 |
| D2 | richer Freedom Gate judgment boundary | `learning/freedom_gate.v1.json` | shares the runtime-control bundle with D1 and D3 |
| D3 | decision and action mediation boundary | `control_path/decisions.json` | shares the runtime-control bundle with D1 and D2 |
| D4 | skill invocation contract | `control_path/skill_model.json` | validation-focused row on top of the control-path family |
| D5 | bounded experiment package and adopt/reject reviewability | `runs/<run-id>/godel/experiment_record.v1.json` | canonical experiment package plus inspect summary |
| D6 | evidence-aware ObsMem retrieval and ranking | `obsmem_retrieval_result.json` | ranking reasons matter as much as success |
| D7 | security, posture, and trust-under-adversary main-band contract | `docs/milestones/v0.89/features/SECURITY_AND_THREAT_MODELING.md` | reviewer-legible document proof row for `v0.89` |
EOF

cat >"$INDEX_OUT" <<'EOF'
core_runtime_control proof_entrypoints/demo_manifest.json
experiment_memory_security proof_entrypoints/demo_manifest.json
claim_matrix claim_matrix.md
EOF

echo "v0.89 review surface:"
echo "  artifacts/v089/review_surface/demo_manifest.json"
echo "  artifacts/v089/review_surface/claim_matrix.md"
echo "  artifacts/v089/review_surface/proof_entrypoints/demo_manifest.json"
