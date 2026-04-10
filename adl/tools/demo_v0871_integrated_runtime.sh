#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/integrated_runtime}"
MANIFEST="$OUT_DIR/demo_manifest.json"
README="$OUT_DIR/README.md"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

bash "$ROOT_DIR/adl/tools/demo_v0871_runtime_environment.sh" "$OUT_DIR/d1_runtime_environment" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_lifecycle.sh" "$OUT_DIR/d2_lifecycle" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_trace_runtime.sh" "$OUT_DIR/d3_trace_runtime" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_resilience_failure.sh" "$OUT_DIR/d4_resilience_failure" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_shepherd_recovery.sh" "$OUT_DIR/d4a_shepherd_recovery" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_restartability.sh" "$OUT_DIR/d5_restartability" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_operator_surface.sh" "$OUT_DIR/d6_operator_surface" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_runtime_state.sh" "$OUT_DIR/d7_runtime_state" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_review_surface.sh" "$OUT_DIR/d8_review_surface" >/dev/null

python3 - "$ROOT_DIR" "$OUT_DIR" "$MANIFEST" <<'PY'
import json
import os
import sys

root_dir, out_dir, manifest_path = sys.argv[1:4]
rows = [
    ("D1", "Runtime Environment Bring-Up", "d1_runtime_environment/runtime/runtime_environment.json"),
    ("D2", "Lifecycle Phases And Boundaries", "d2_lifecycle/lifecycle_summary.json"),
    ("D3", "Trace-Aligned Runtime Execution", "d3_trace_runtime/trace_bundle_manifest.json"),
    ("D4", "Local Failure Handling", "d4_resilience_failure/failure_summary.json"),
    ("D4A", "Shepherd Preservation And Recovery", "d4a_shepherd_recovery/shepherd_recovery_summary.json"),
    ("D5", "Restartability And Recovery", "d5_restartability/restartability_summary.json"),
    ("D6", "Operator Invocation Surface", "d6_operator_surface/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json"),
    ("D7", "Runtime State / Persistence Discipline", "d7_runtime_state/runtime/runs/v0-6-hitl-pause-demo/run_status.json"),
    ("D8", "Review Surface Walkthrough", "d8_review_surface/demo_manifest.json"),
]
payload = {
    "demo_id": "D9",
    "manifest_version": "adl.v0871.integrated_runtime.v1",
    "packages": [
        {
            "demo_id": demo_id,
            "title": title,
            "primary_proof_surface": os.path.relpath(os.path.join(out_dir, path), root_dir),
        }
        for demo_id, title, path in rows
    ],
}
with open(manifest_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

cat >"$README" <<'EOF'
# v0.87.1 Demo D9 - Integrated Runtime Path

Canonical command:

```bash
bash adl/tools/demo_v0871_integrated_runtime.sh
```

Primary proof surface:
- `artifacts/v0871/integrated_runtime/demo_manifest.json`

Success signal:
- one manifest links the bounded runtime environment, lifecycle, trace, failure, Shepherd, restartability, operator, persistence, and review-surface demos from one integrated root
EOF
