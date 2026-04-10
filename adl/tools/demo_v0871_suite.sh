#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/suite}"
ARTIFACT_ROOT="$(cd "$(dirname "$OUT_DIR")" && pwd)/$(basename "$OUT_DIR")"
MANIFEST="$ARTIFACT_ROOT/demo_manifest.json"
README_OUT="$ARTIFACT_ROOT/README.md"
INDEX="$ARTIFACT_ROOT/index.txt"

rm -rf "$ARTIFACT_ROOT"
mkdir -p "$ARTIFACT_ROOT"

cd "$ROOT_DIR"

echo "Running v0.87.1 milestone demo suite..."

bash adl/tools/demo_v0871_runtime_environment.sh "$ARTIFACT_ROOT/runtime_environment" >/dev/null
bash adl/tools/demo_v0871_lifecycle.sh "$ARTIFACT_ROOT/lifecycle" >/dev/null
bash adl/tools/demo_v0871_trace_runtime.sh "$ARTIFACT_ROOT/trace_runtime" >/dev/null
bash adl/tools/demo_v0871_resilience_failure.sh "$ARTIFACT_ROOT/resilience_failure" >/dev/null
bash adl/tools/demo_v0871_shepherd_recovery.sh "$ARTIFACT_ROOT/shepherd_recovery" >/dev/null
bash adl/tools/demo_v0871_restartability.sh "$ARTIFACT_ROOT/restartability" >/dev/null
bash adl/tools/demo_v0871_integrated_runtime.sh "$ARTIFACT_ROOT/integrated_runtime" >/dev/null
bash adl/tools/demo_v0871_docs_review.sh "$ARTIFACT_ROOT/docs_review" >/dev/null
bash adl/tools/demo_v0871_quality_gate.sh "$ARTIFACT_ROOT/quality_gate" >/dev/null
bash adl/tools/demo_v0871_release_review_package.sh "$ARTIFACT_ROOT/release_review_package" >/dev/null
bash adl/tools/demo_v0871_provider_local_ollama.sh "$ARTIFACT_ROOT/provider_local_ollama" >/dev/null
bash adl/tools/demo_v0871_provider_http.sh "$ARTIFACT_ROOT/provider_http" >/dev/null
bash adl/tools/demo_v0871_provider_mock.sh "$ARTIFACT_ROOT/provider_mock" >/dev/null
bash adl/tools/demo_v0871_provider_chatgpt.sh "$ARTIFACT_ROOT/provider_chatgpt" >/dev/null
V0871_OPERATOR_ROOT="$ARTIFACT_ROOT/operator_surface" \
V0871_RUNTIME_STATE_ROOT="$ARTIFACT_ROOT/runtime_state" \
  bash adl/tools/demo_v0871_review_surface.sh "$ARTIFACT_ROOT/review_surface" >/dev/null
bash adl/tools/demo_v0871_multi_agent_discussion.sh "$ARTIFACT_ROOT/multi_agent_discussion" >/dev/null

python3 - "$ROOT_DIR" "$ARTIFACT_ROOT" "$MANIFEST" <<'PY'
import json
import os
import sys

root_dir, artifact_root, manifest_path = sys.argv[1:4]

def rel(path: str) -> str:
    return os.path.relpath(path, root_dir)

def package(demo_id: str, title: str, root: str, primary: str, secondaries=None):
    return {
        "demo_id": demo_id,
        "title": title,
        "artifact_root": rel(os.path.join(artifact_root, root)),
        "primary_proof_surface": rel(os.path.join(artifact_root, primary)),
        "secondary_proof_surfaces": [
            rel(os.path.join(artifact_root, path)) for path in (secondaries or [])
        ],
    }

payload = {
    "suite_version": "adl.v0871.demo_suite.v1",
    "milestone": "v0.87.1",
    "suite_id": "WP-13",
    "primary_proof_surface": rel(manifest_path),
    "review_readme": rel(os.path.join(artifact_root, "README.md")),
    "demo_packages": [
        package(
            "D1",
            "Runtime Environment Bring-Up",
            "runtime_environment",
            "runtime_environment/runtime/runtime_environment.json",
            [
                "runtime_environment/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json",
                "runtime_environment/runtime/runs/v0-4-demo-deterministic-replay/run_status.json",
                "runtime_environment/runtime/runs/v0-4-demo-deterministic-replay/logs/trace_v1.json",
            ],
        ),
        package(
            "D2",
            "Lifecycle Phases And Boundaries",
            "lifecycle",
            "lifecycle/lifecycle_summary.json",
            [
                "lifecycle/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json",
                "lifecycle/runtime/runs/v0-4-demo-deterministic-replay/logs/trace_v1.json",
                "lifecycle/README.md",
            ],
        ),
        package(
            "D3",
            "Trace-Aligned Runtime Execution",
            "trace_runtime",
            "trace_runtime/trace_bundle_manifest.json",
            [
                "trace_runtime/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json",
                "trace_runtime/runtime/runs/v0-4-demo-deterministic-replay/logs/trace_v1.json",
                "trace_runtime/README.md",
            ],
        ),
        package(
            "D4",
            "Local Failure Handling",
            "resilience_failure",
            "resilience_failure/failure_summary.json",
            [
                "resilience_failure/runtime/runs/v0-87-1-failure-missing-file/run_status.json",
                "resilience_failure/runtime/runs/v0-87-1-failure-missing-file/logs/trace_v1.json",
                "resilience_failure/README.md",
            ],
        ),
        package(
            "D4A",
            "Shepherd Preservation And Recovery",
            "shepherd_recovery",
            "shepherd_recovery/shepherd_recovery_summary.json",
            [
                "shepherd_recovery/runtime/runs/v0-6-hitl-pause-demo/run_status.json",
                "shepherd_recovery/runtime/runs/v0-6-hitl-pause-demo/pause_state.json",
                "shepherd_recovery/README.md",
            ],
        ),
        package(
            "D5",
            "Restartability And Recovery",
            "restartability",
            "restartability/restartability_summary.json",
            [
                "restartability/runtime/runs/v0-6-hitl-pause-demo/run_status.json",
                "restartability/runtime/runs/v0-6-hitl-pause-demo/pause_state.json",
                "restartability/README.md",
            ],
        ),
        package(
            "D9",
            "Integrated runtime path",
            "integrated_runtime",
            "integrated_runtime/demo_manifest.json",
            [
                "integrated_runtime/d3_trace_runtime/trace_bundle_manifest.json",
                "integrated_runtime/d8_review_surface/demo_manifest.json",
                "integrated_runtime/README.md",
            ],
        ),
        package(
            "D10",
            "Docs-to-runtime consistency check",
            "docs_review",
            "docs_review/docs_review_manifest.json",
            [
                "docs_review/integrated_runtime/demo_manifest.json",
                "docs_review/README.md",
            ],
        ),
        package(
            "D11",
            "Quality gate walkthrough",
            "quality_gate",
            "quality_gate/quality_gate_record.json",
            [
                "quality_gate/fmt.log",
                "quality_gate/runtime_rows.log",
                "quality_gate/README.md",
            ],
        ),
        package(
            "D12",
            "Release review package",
            "release_review_package",
            "release_review_package/release_review_package_manifest.json",
            [
                "release_review_package/docs_review/docs_review_manifest.json",
                "release_review_package/quality_gate/quality_gate_record.json",
                "release_review_package/README.md",
            ],
        ),
        package(
            "P1",
            "Local Ollama provider family",
            "provider_local_ollama",
            "provider_local_ollama/runtime/runs/v0-87-1-provider-local-ollama-demo/run_summary.json",
            [
                "provider_local_ollama/runtime/runs/v0-87-1-provider-local-ollama-demo/run_status.json",
                "provider_local_ollama/runtime/runs/v0-87-1-provider-local-ollama-demo/logs/trace_v1.json",
                "provider_local_ollama/README.md",
            ],
        ),
        package(
            "P2",
            "Bounded HTTP provider family",
            "provider_http",
            "provider_http/runtime/runs/v0-87-1-provider-http-demo/run_summary.json",
            [
                "provider_http/runtime/runs/v0-87-1-provider-http-demo/run_status.json",
                "provider_http/runtime/runs/v0-87-1-provider-http-demo/logs/trace_v1.json",
                "provider_http/README.md",
            ],
        ),
        package(
            "P3",
            "Mock provider family",
            "provider_mock",
            "provider_mock/runtime/runs/v0-87-1-provider-mock-demo/run_summary.json",
            [
                "provider_mock/runtime/runs/v0-87-1-provider-mock-demo/run_status.json",
                "provider_mock/runtime/runs/v0-87-1-provider-mock-demo/logs/trace_v1.json",
                "provider_mock/README.md",
            ],
        ),
        package(
            "P4",
            "ChatGPT provider family",
            "provider_chatgpt",
            "provider_chatgpt/runtime/runs/v0-87-1-provider-chatgpt-demo/run_summary.json",
            [
                "provider_chatgpt/runtime/runs/v0-87-1-provider-chatgpt-demo/run_status.json",
                "provider_chatgpt/runtime/runs/v0-87-1-provider-chatgpt-demo/logs/trace_v1.json",
                "provider_chatgpt/README.md",
            ],
        ),
        package(
            "D8",
            "Runtime review surface walkthrough",
            "review_surface",
            "review_surface/demo_manifest.json",
            [
                "operator_surface/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json",
                "runtime_state/runtime/runs/v0-6-hitl-pause-demo/run_status.json",
                "review_surface/README.md",
            ],
        ),
        package(
            "D13",
            "Claude + ChatGPT tea discussion",
            "multi_agent_discussion",
            "multi_agent_discussion/transcript.md",
            [
                "multi_agent_discussion/demo_manifest.json",
                "multi_agent_discussion/runtime/runs/v0-87-1-multi-agent-tea-discussion/run_summary.json",
                "multi_agent_discussion/runtime/runs/v0-87-1-multi-agent-tea-discussion/logs/trace_v1.json",
            ],
        ),
    ],
}

with open(manifest_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

cat >"$README_OUT" <<'EOF'
# v0.87.1 Milestone Demo Suite

Canonical command:

```bash
bash adl/tools/demo_v0871_suite.sh
```

Primary proof surface:
- `artifacts/v0871/suite/demo_manifest.json`

What this suite runs:
- D1 runtime environment bring-up
- D2 lifecycle phases and boundaries
- D3 trace-aligned runtime execution
- D4 local failure handling
- D4A Shepherd preservation and recovery
- D5 restartability and recovery
- D9 integrated runtime path
- D10 docs-to-runtime consistency check
- D11 quality-gate walkthrough
- D12 release review package
- local Ollama provider-family proof
- bounded HTTP provider-family proof
- mock provider-family proof
- ChatGPT provider-family proof
- D8 runtime review-surface walkthrough, including D6 and D7 proof roots
- D13 bounded Claude + ChatGPT multi-agent discussion proof

Scope note:
- This suite is the canonical WP-13 integration entrypoint for the bounded,
  locally runnable `v0.87.1` proof surfaces.
- The live-provider D13L proof remains out of scope for the default suite because
  it depends on operator-managed credentials and provider account access.
EOF

python3 - "$ROOT_DIR" "$ARTIFACT_ROOT" "$INDEX" <<'PY'
import os
import sys

root_dir, artifact_root, index_path = sys.argv[1:4]

def rel(path: str) -> str:
    return os.path.relpath(os.path.join(artifact_root, path), root_dir)

rows = [
    ("D1", "runtime_environment/runtime/runtime_environment.json"),
    ("D2", "lifecycle/lifecycle_summary.json"),
    ("D3", "trace_runtime/trace_bundle_manifest.json"),
    ("D4", "resilience_failure/failure_summary.json"),
    ("D4A", "shepherd_recovery/shepherd_recovery_summary.json"),
    ("D5", "restartability/restartability_summary.json"),
    ("D9", "integrated_runtime/demo_manifest.json"),
    ("D10", "docs_review/docs_review_manifest.json"),
    ("D11", "quality_gate/quality_gate_record.json"),
    ("D12", "release_review_package/release_review_package_manifest.json"),
    ("P1", "provider_local_ollama/runtime/runs/v0-87-1-provider-local-ollama-demo/run_summary.json"),
    ("P2", "provider_http/runtime/runs/v0-87-1-provider-http-demo/run_summary.json"),
    ("P3", "provider_mock/runtime/runs/v0-87-1-provider-mock-demo/run_summary.json"),
    ("P4", "provider_chatgpt/runtime/runs/v0-87-1-provider-chatgpt-demo/run_summary.json"),
    ("D8", "review_surface/demo_manifest.json"),
    ("D13", "multi_agent_discussion/transcript.md"),
]

with open(index_path, "w", encoding="utf-8") as fh:
    for demo_id, path in rows:
        fh.write(f"{demo_id} {rel(path)}\n")
PY

echo "v0.87.1 suite proof surface:"
echo "  $MANIFEST"
echo "  $README_OUT"
echo "  $INDEX"
