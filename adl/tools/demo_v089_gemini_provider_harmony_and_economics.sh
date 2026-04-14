#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v089/gemini_provider_harmony_and_economics}"
FIXTURE_DIR="$ROOT_DIR/demos/fixtures/gemini_in_the_loop"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-89-gemini-provider-harmony-and-economics"
EXAMPLE="adl/examples/v0-89-gemini-in-the-loop.adl.yaml"
GENERATED_EXAMPLE="$OUT_DIR/v0-89-gemini-provider-harmony-and-economics.runtime.adl.yaml"
PACKET_DIR="$OUT_DIR/packet"
PROVIDER_DIR="$OUT_DIR/provider_selection"
REVIEW_DIR="$OUT_DIR/review_artifacts"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"
PORT="${ADL_GEMINI_ECON_PORT:-8790}"
RESPONSE_FILE="${ADL_GEMINI_RESPONSE_FILE:-$FIXTURE_DIR/valid_response.json}"
SERVER_LOG="$OUT_DIR/mock_server.log"
TMP_PROVIDER_OUT="$OUT_DIR/provider_setup"

sanitize_generated_artifacts() {
  export ADL_SANITIZE_OUT_DIR="$OUT_DIR"
  export ADL_SANITIZE_OUT_REAL
  ADL_SANITIZE_OUT_REAL="$(cd "$OUT_DIR" && pwd -P)"
  export ADL_SANITIZE_ROOT_DIR="$ROOT_DIR"
  export ADL_SANITIZE_ROOT_REAL
  ADL_SANITIZE_ROOT_REAL="$(cd "$ROOT_DIR" && pwd -P)"
  find "$OUT_DIR" -type f \( -name '*.json' -o -name '*.md' -o -name '*.txt' -o -name '*.yaml' \) -print0 |
    xargs -0 perl -0pi -e '
      for my $name (qw(ADL_SANITIZE_OUT_REAL ADL_SANITIZE_OUT_DIR ADL_SANITIZE_ROOT_REAL ADL_SANITIZE_ROOT_DIR)) {
        my $value = $ENV{$name} // "";
        next if $value eq "";
        my $replacement = $name =~ /ROOT/ ? "<repo_root>" : "<output_dir>";
        s/\Q$value\E/$replacement/g;
      }
    '
}

require_fixture() {
  local path="$1"
  [[ -f "$path" ]] || {
    echo "missing required Gemini fixture: $path" >&2
    exit 1
  }
}

validate_output() {
  local input="$1"
  local output="$2"
  python3 - <<'PY' "$input" "$output"
from pathlib import Path
import json, sys
payload = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if payload.get("schema_version") != "adl.gemini.review.v1":
    raise SystemExit("unexpected schema_version")
if payload.get("provider_family") != "gemini":
    raise SystemExit("provider_family mismatch")
if payload.get("execution_mode") != "bounded_http_profile":
    raise SystemExit("execution_mode mismatch")
if payload.get("verdict") not in {"accept", "revise"}:
    raise SystemExit("invalid verdict")
findings = payload.get("findings")
if not isinstance(findings, list) or not findings:
    raise SystemExit("findings must be non-empty")
Path(sys.argv[2]).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
PY
}

render_findings() {
  local input="$1"
  local output="$2"
  python3 - <<'PY' "$input" "$output"
from pathlib import Path
import json, sys
payload = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
lines = [
    "# Gemini Reviewer Artifact",
    "",
    f"Verdict: `{payload['verdict']}`",
    "",
    "## Findings",
]
for finding in payload["findings"]:
    lines.extend([
        "",
        f"### {finding['id']} - {finding['title']}",
        f"- Severity: `{finding['severity']}`",
        "",
        finding["detail"],
    ])
lines.extend(["", "## Synthesis", "", payload["synthesis"], ""])
Path(sys.argv[2]).write_text("\n".join(lines), encoding="utf-8")
PY
}

require_fixture "$FIXTURE_DIR/review_packet.md"
require_fixture "$RESPONSE_FILE"

rm -rf "$OUT_DIR"
mkdir -p "$PACKET_DIR" "$PROVIDER_DIR" "$REVIEW_DIR" "$TMP_PROVIDER_OUT"
cp "$FIXTURE_DIR/review_packet.md" "$PACKET_DIR/review_packet.md"

cat >"$PACKET_DIR/packet_manifest.json" <<'EOF'
{
  "schema_version": "adl.provider_demo.packet_manifest.v1",
  "packet_id": "gemini_provider_harmony_packet.v1",
  "task_class": "bounded_review_packet",
  "reviewer_goal": "Evaluate one bounded packet using a provider-fit decision that remains legible after the run.",
  "inputs": [
    "review_packet.md"
  ]
}
EOF

cat >"$PROVIDER_DIR/candidate_providers.json" <<'EOF'
{
  "schema_version": "adl.provider_candidates.v1",
  "candidates": [
    {
      "provider_family": "gemini",
      "fit_for_packet": "high",
      "cost_class": "economical",
      "latency_class": "fast",
      "strength": "bounded structured synthesis"
    },
    {
      "provider_family": "chatgpt",
      "fit_for_packet": "high",
      "cost_class": "premium",
      "latency_class": "medium",
      "strength": "strong general reasoning"
    },
    {
      "provider_family": "claude",
      "fit_for_packet": "high",
      "cost_class": "premium",
      "latency_class": "medium",
      "strength": "careful critique"
    }
  ]
}
EOF

cat >"$PROVIDER_DIR/provider_selection_manifest.json" <<'EOF'
{
  "schema_version": "adl.provider_selection_manifest.v1",
  "demo_id": "v0.89.gemini_provider_harmony_and_economics",
  "packet_id": "gemini_provider_harmony_packet.v1",
  "selected_provider_family": "gemini",
  "selection_dimensions": [
    "capability_fit",
    "cost_class",
    "latency_class",
    "reviewability",
    "bounded_scope_match"
  ],
  "selection_result": "gemini selected as the calm, economical, bounded-fit participant for this packet"
}
EOF

cat >"$PROVIDER_DIR/capability_and_cost_reasoning.md" <<'EOF'
# Capability And Cost Reasoning

This demo is not a benchmark contest.

It is a bounded provider-fit decision.

For this packet, Gemini is selected because:

- the task is narrow and reviewer-facing
- the expected output is structured and bounded
- the cost-class is represented as economical rather than premium
- the runtime can explain that choice afterward

ChatGPT and Claude remain respected alternatives.
The point is not that Gemini is universally better.
The point is that ADL can make the provider choice legible and fair.
EOF

export GEMINI_API_KEY="${GEMINI_API_KEY:-demo-gemini-token}"

python3 "$ROOT_DIR/adl/tools/mock_gemini_http_provider.py" \
  --port "$PORT" \
  --response-file "$RESPONSE_FILE" >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!
cleanup() {
  kill "$SERVER_PID" >/dev/null 2>&1 || true
}
trap cleanup EXIT
sleep 1

cd "$ROOT_DIR"
cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- provider setup gemini --out "$TMP_PROVIDER_OUT" --force >/dev/null

python3 - "$EXAMPLE" "$GENERATED_EXAMPLE" "$PORT" "$PACKET_DIR/review_packet.md" <<'PY'
import sys
from pathlib import Path

source, target, port, packet = sys.argv[1:5]
text = Path(source).read_text(encoding="utf-8")
text = text.replace("http://127.0.0.1:8789/complete", f"http://127.0.0.1:{port}/complete")
text = text.replace("@file:packets/v0-89-gemini-in-the-loop-review-packet.md", f"@file:{packet}")
text = text.replace('name: "v0-89-gemini-in-the-loop"', 'name: "v0-89-gemini-provider-harmony-and-economics"')
Path(target).write_text(text, encoding="utf-8")
PY

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$GENERATED_EXAMPLE" \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    >"$OUT_DIR/run_log.txt" 2>&1

validate_output "$STEP_OUT/review/01-gemini-review.json" "$REVIEW_DIR/validated_review.json"
render_findings "$REVIEW_DIR/validated_review.json" "$REVIEW_DIR/gemini_artifact.md"

python3 - "$MANIFEST" "$RUN_ID" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.gemini_provider_harmony_demo.v1",
    "demo_id": "v0.89.gemini_provider_harmony_and_economics",
    "title": "v0.89 Gemini provider harmony and economics demo",
    "execution_mode": "bounded_http_profile",
    "selected_provider_family": "gemini",
    "claim": "ADL can host Gemini as a welcomed bounded participant while making the provider-fit decision legible through capability, cost-class, latency, and reviewability artifacts.",
    "artifacts": {
        "packet": "packet/review_packet.md",
        "packet_manifest": "packet/packet_manifest.json",
        "candidate_providers": "provider_selection/candidate_providers.json",
        "provider_selection_manifest": "provider_selection/provider_selection_manifest.json",
        "capability_and_cost_reasoning": "provider_selection/capability_and_cost_reasoning.md",
        "provider_setup": "provider_setup/provider.adl.yaml",
        "validated_output": "review_artifacts/validated_review.json",
        "gemini_artifact": "review_artifacts/gemini_artifact.md",
        "run_summary": f"runtime/runs/{sys.argv[2]}/run_summary.json",
        "steps": f"runtime/runs/{sys.argv[2]}/steps.json",
        "trace": f"runtime/runs/{sys.argv[2]}/logs/trace_v1.json"
    }
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<EOF
# v0.89 Demo - Gemini Provider Harmony And Economics

Canonical command:

\`\`\`bash
bash adl/tools/demo_v089_gemini_provider_harmony_and_economics.sh
\`\`\`

Primary proof surfaces:
- \`provider_selection/provider_selection_manifest.json\`
- \`provider_selection/capability_and_cost_reasoning.md\`
- \`review_artifacts/gemini_artifact.md\`
- \`runtime/runs/$RUN_ID/run_summary.json\`
- \`runtime/runs/$RUN_ID/logs/trace_v1.json\`

What this proves:
- Gemini can participate as a welcomed bounded participant in ADL
- ADL can explain why Gemini was chosen for this packet
- provider choice can be legible without turning into vendor-war theater
EOF

sanitize_generated_artifacts

echo "Gemini provider harmony proof surface under the output directory:"
echo "  provider_selection/provider_selection_manifest.json"
echo "  provider_selection/capability_and_cost_reasoning.md"
echo "  review_artifacts/gemini_artifact.md"
echo "  runtime/runs/$RUN_ID/run_summary.json"
echo "  runtime/runs/$RUN_ID/logs/trace_v1.json"
