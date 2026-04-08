#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/review_surface}"
OPERATOR_ROOT="$ROOT_DIR/artifacts/v0871/operator_surface"
RUNTIME_STATE_ROOT="$ROOT_DIR/artifacts/v0871/runtime_state"
MANIFEST_FILE="$OUT_DIR/demo_manifest.json"
README_FILE="$OUT_DIR/README.md"
INDEX_FILE="$OUT_DIR/index.txt"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

cd "$ROOT_DIR"

echo "Running v0.87.1 review-surface walkthrough..."
bash adl/tools/demo_v0871_operator_surface.sh "$OPERATOR_ROOT" >/dev/null
bash adl/tools/demo_v0871_runtime_state.sh "$RUNTIME_STATE_ROOT" >/dev/null

python3 - "$ROOT_DIR" "$OUT_DIR" "$OPERATOR_ROOT" "$RUNTIME_STATE_ROOT" "$MANIFEST_FILE" <<'PY'
import json
import os
import sys

root_dir, out_dir, operator_root, runtime_state_root, manifest_file = sys.argv[1:]

def rel(path: str) -> str:
    return os.path.relpath(path, root_dir)

manifest = {
    "review_surface_version": "adl.runtime_review_surface.v1",
    "milestone": "v0.87.1",
    "demo_id": "D8",
    "review_root": rel(out_dir),
    "review_readme": rel(os.path.join(out_dir, "README.md")),
    "primary_proof_surface": rel(manifest_file),
    "demo_packages": [
        {
            "demo_id": "D6",
            "title": "Operator Invocation Surface",
            "review_readme": rel(os.path.join(operator_root, "README.md")),
            "primary_proof_surface": rel(os.path.join(
                operator_root,
                "runtime/runs/v0-4-demo-deterministic-replay/run_summary.json",
            )),
            "secondary_proof_surfaces": [
                rel(os.path.join(operator_root, "runtime/runtime_environment.json")),
                rel(os.path.join(
                    operator_root,
                    "runtime/runs/v0-4-demo-deterministic-replay/run_status.json",
                )),
                rel(os.path.join(
                    operator_root,
                    "runtime/runs/v0-4-demo-deterministic-replay/logs/trace_v1.json",
                )),
            ],
        },
        {
            "demo_id": "D7",
            "title": "Runtime State / Persistence Discipline",
            "review_readme": rel(os.path.join(runtime_state_root, "README.md")),
            "primary_proof_surface": rel(os.path.join(
                runtime_state_root,
                "runtime/runs/v0-6-hitl-pause-demo/run_status.json",
            )),
            "secondary_proof_surfaces": [
                rel(os.path.join(
                    runtime_state_root,
                    "runtime/runs/v0-6-hitl-pause-demo/pause_state.json",
                )),
                rel(os.path.join(
                    runtime_state_root,
                    "runtime/runs/v0-6-hitl-pause-demo/logs/trace_v1.json",
                )),
                rel(os.path.join(
                    runtime_state_root,
                    "runtime/runs/v0-6-hitl-no-pause-demo/run_status.json",
                )),
                rel(os.path.join(
                    runtime_state_root,
                    "runtime/runs/v0-6-hitl-no-pause-demo/logs/trace_v1.json",
                )),
                rel(os.path.join(runtime_state_root, "runtime/runtime_environment.json")),
            ],
        },
    ],
}

with open(manifest_file, "w", encoding="utf-8") as fh:
    json.dump(manifest, fh, indent=2)
    fh.write("\n")
PY

cat >"$README_FILE" <<EOF
# v0.87.1 Demo D8 - Runtime Review Surface Walkthrough

Canonical review command:

\`\`\`bash
bash adl/tools/demo_v0871_review_surface.sh
\`\`\`

Primary proof surface:
- \`artifacts/v0871/review_surface/demo_manifest.json\`

Secondary proof surfaces:
- \`artifacts/v0871/review_surface/README.md\`
- \`artifacts/v0871/operator_surface/README.md\`
- \`artifacts/v0871/runtime_state/README.md\`

Reviewer walkthrough:
- Review D6 first for the canonical operator entrypoint.
- Then inspect D7 for persistence, pause-state, and continuity evidence.
- Use the manifest to jump directly to the primary proof surface for each bounded demo.

This review surface proves the bounded reviewer entry contract for \`v0.87.1\`:
- one canonical command gives the reviewer the runtime proof package
- operator, trace, and persistence surfaces are linked from one stable manifest
- the reviewer does not need to reconstruct proof roots by hand
EOF

cat >"$INDEX_FILE" <<EOF
D6 artifacts/v0871/operator_surface/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json
D7 artifacts/v0871/runtime_state/runtime/runs/v0-6-hitl-pause-demo/run_status.json
EOF

echo "Review-surface proof package:"
echo "  artifacts/v0871/review_surface/demo_manifest.json"
echo "  artifacts/v0871/review_surface/README.md"
echo "  artifacts/v0871/operator_surface/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json"
echo "  artifacts/v0871/runtime_state/runtime/runs/v0-6-hitl-pause-demo/run_status.json"
