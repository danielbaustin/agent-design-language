#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v089/proof_entrypoints}"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"
INDEX_OUT="$OUT_DIR/index.txt"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

python3 - "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.v089.proof_entrypoints.v1",
    "milestone": "v0.89",
    "title": "v0.89 demo scaffolding and proof entrypoints",
    "review_root": "artifacts/v089/proof_entrypoints",
    "rows": [
        {
            "demo_id": "D1",
            "title": "AEE convergence walkthrough",
            "command": "cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture",
            "primary_proof_surface": "control_path/convergence.json",
            "secondary_surfaces": [
                "control_path/summary.txt",
                "learning/freedom_gate.v1.json"
            ],
            "status": "READY",
            "notes": "Shares the bounded runtime-control proof run with D2 and D3."
        },
        {
            "demo_id": "D2",
            "title": "Freedom Gate v2 judgment demo",
            "command": "cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture",
            "primary_proof_surface": "learning/freedom_gate.v1.json",
            "secondary_surfaces": [
                "control_path/final_result.json",
                "control_path/summary.txt"
            ],
            "status": "READY",
            "notes": "Use the same runtime-control artifact bundle as D1 and D3."
        },
        {
            "demo_id": "D3",
            "title": "Decision + action mediation proof",
            "command": "cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture",
            "primary_proof_surface": "control_path/decisions.json",
            "secondary_surfaces": [
                "control_path/action_proposals.json",
                "control_path/mediation.json",
                "control_path/summary.txt"
            ],
            "status": "READY",
            "notes": "This row proves route-selection, reframing, and commitment-gate visibility from one bounded runtime-control run."
        },
        {
            "demo_id": "D4",
            "title": "Skill invocation contract demo",
            "command": "cargo test --manifest-path adl/Cargo.toml cli_artifact_validate_control_path_ -- --nocapture",
            "primary_proof_surface": "control_path/skill_model.json",
            "secondary_surfaces": [
                "control_path/skill_execution_protocol.json",
                "control_path/summary.txt"
            ],
            "status": "READY",
            "notes": "The validator path is the bounded proof surface here, not a hidden runtime shortcut."
        },
        {
            "demo_id": "D5",
            "title": "Godel experiment package demo",
            "command": "cargo run --manifest-path adl/Cargo.toml -- godel run --run-id run-745-a --workflow-id wf-godel-loop --failure-code tool_failure --failure-summary \"step failed with deterministic parse error\" --evidence-ref runs/run-745-a/run_status.json --evidence-ref runs/run-745-a/logs/activation_log.json --runs-dir \"$tmp_root\" && cargo run --manifest-path adl/Cargo.toml -- godel inspect --run-id run-745-a --runs-dir \"$tmp_root\"",
            "primary_proof_surface": "runs/<run-id>/godel/experiment_record.v1.json",
            "secondary_surfaces": [
                "runs/<run-id>/godel/evaluation_plan.v1.json",
                "runs/<run-id>/godel/canonical_evidence_view.v1.json"
            ],
            "status": "READY",
            "notes": "Review the inspect summary alongside the canonical package."
        },
        {
            "demo_id": "D6",
            "title": "ObsMem evidence and ranking walkthrough",
            "command": "cargo run --manifest-path adl/Cargo.toml -- demo demo-f-obsmem-retrieval --run --trace --out ./out",
            "primary_proof_surface": "obsmem_retrieval_result.json",
            "secondary_surfaces": [
                "runs/_shared/obsmem_store.v1.json"
            ],
            "status": "READY",
            "notes": "Reviewer focus is explanation-bearing ordering, not just successful execution."
        },
        {
            "demo_id": "D7",
            "title": "Security / trust / posture walkthrough",
            "command": "Review docs/milestones/v0.89/features/SECURITY_AND_THREAT_MODELING.md, docs/milestones/v0.89/features/ADL_SECURITY_POSTURE_MODEL.md, and docs/milestones/v0.89/features/ADL_TRUST_MODEL_UNDER_ADVERSARY.md together",
            "primary_proof_surface": "docs/milestones/v0.89/features/SECURITY_AND_THREAT_MODELING.md",
            "secondary_surfaces": [
                "docs/milestones/v0.89/features/ADL_SECURITY_POSTURE_MODEL.md",
                "docs/milestones/v0.89/features/ADL_TRUST_MODEL_UNDER_ADVERSARY.md"
            ],
            "status": "READY",
            "notes": "This is intentionally a reviewer-legible document proof row for v0.89, not the later adversarial runtime demo band."
        }
    ]
}

Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<'EOF'
# v0.89 Demo Scaffolding And Proof Entrypoints

Canonical review command:

```bash
bash adl/tools/demo_v089_proof_entrypoints.sh
```

Reviewer flow:
- inspect `demo_manifest.json` first
- start with D1-D3 if you want the bounded runtime-control story in one place
- use D4 for the explicit skill invocation contract validation path
- use D5 and D6 for the experiment and retrieval proof rows
- treat D7 as a bounded document walkthrough, not as the `v0.89.1` adversarial runtime band

Important boundary:
- D1, D2, and D3 intentionally share one bounded runtime-control proof run
- D7 is reviewer-legible by design and should not be overclaimed as a live adversarial demo
EOF

cat >"$INDEX_OUT" <<'EOF'
D1 control_path/convergence.json
D2 learning/freedom_gate.v1.json
D3 control_path/decisions.json
D4 control_path/skill_model.json
D5 runs/<run-id>/godel/experiment_record.v1.json
D6 obsmem_retrieval_result.json
D7 docs/milestones/v0.89/features/SECURITY_AND_THREAT_MODELING.md
EOF

echo "v0.89 proof entrypoints:"
echo "  artifacts/v089/proof_entrypoints/demo_manifest.json"
echo "  artifacts/v089/proof_entrypoints/README.md"
echo "  artifacts/v089/proof_entrypoints/index.txt"
