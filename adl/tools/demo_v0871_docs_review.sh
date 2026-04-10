#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/docs_review}"
MANIFEST="$OUT_DIR/docs_review_manifest.json"
README="$OUT_DIR/README.md"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

bash "$ROOT_DIR/adl/tools/demo_v0871_integrated_runtime.sh" "$OUT_DIR/integrated_runtime" >/dev/null

python3 - "$ROOT_DIR" "$OUT_DIR" "$MANIFEST" <<'PY'
import json
import os
import sys

root_dir, out_dir, manifest_path = sys.argv[1:4]
doc_pairs = [
    ("D1", "docs/milestones/v0.87.1/features/ADL_RUNTIME_ENVIRONMENT.md", "integrated_runtime/d1_runtime_environment/runtime/runtime_environment.json"),
    ("D2", "docs/milestones/v0.87.1/features/AGENT_LIFECYCLE.md", "integrated_runtime/d2_lifecycle/lifecycle_summary.json"),
    ("D3", "docs/milestones/v0.87.1/features/EXECUTION_BOUNDARIES.md", "integrated_runtime/d3_trace_runtime/trace_bundle_manifest.json"),
    ("D4", "docs/milestones/v0.87.1/features/LOCAL_RUNTIME_RESILIENCE.md", "integrated_runtime/d4_resilience_failure/failure_summary.json"),
    ("D4A", "docs/milestones/v0.87.1/features/SHEPHERD_RUNTIME_MODEL.md", "integrated_runtime/d4a_shepherd_recovery/shepherd_recovery_summary.json"),
]
payload = {
    "demo_id": "D10",
    "manifest_version": "adl.v0871.docs_review.v1",
    "doc_to_proof_map": [
        {
            "demo_id": demo_id,
            "doc": doc,
            "proof_surface": os.path.relpath(os.path.join(out_dir, proof), root_dir),
        }
        for demo_id, doc, proof in doc_pairs
    ],
}
with open(manifest_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

cat >"$README" <<'EOF'
# v0.87.1 Demo D10 - Docs-To-Runtime Consistency Check

Canonical command:

```bash
bash adl/tools/demo_v0871_docs_review.sh
```

Primary proof surface:
- `artifacts/v0871/docs_review/docs_review_manifest.json`

Success signal:
- the manifest maps the promoted runtime feature docs to the corresponding integrated proof surfaces without requiring chat-history reconstruction
EOF
