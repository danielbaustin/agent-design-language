#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0891/gemini_provider_harmony_roundtable}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-89-1-gemini-provider-harmony-roundtable"
PORT="${ADL_GEMINI_ROUNDTABLE_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_server.log"
EXAMPLE="adl/examples/v0-89-1-gemini-provider-harmony-roundtable.adl.yaml"
GENERATED_EXAMPLE="$OUT_DIR/v0-89-1-gemini-provider-harmony-roundtable.runtime.adl.yaml"
PACKET_DIR="$OUT_DIR/packet"
PROVIDER_DIR="$OUT_DIR/provider_selection"
ROUNDTABLE_DIR="$OUT_DIR/roundtable"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"
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

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT" "$PACKET_DIR" "$PROVIDER_DIR" "$ROUNDTABLE_DIR" "$TMP_PROVIDER_OUT"

cat >"$PACKET_DIR/topic.md" <<'EOF'
# Provider-fit question

Why is Gemini the right participant for a bounded reviewer-facing packet, and
what tradeoffs should still remain visible when ADL makes that choice?
EOF

cat >"$PACKET_DIR/packet_manifest.json" <<'EOF'
{
  "schema_version": "adl.provider_demo.packet_manifest.v1",
  "packet_id": "gemini_provider_roundtable_packet.v1",
  "task_class": "bounded_provider_fit_roundtable",
  "reviewer_goal": "Inspect why Gemini was selected, how peer providers respond, and whether the final rationale remains calm and legible.",
  "inputs": [
    "topic.md"
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
      "reviewability": "high",
      "strength": "bounded structured synthesis"
    },
    {
      "provider_family": "chatgpt",
      "fit_for_packet": "high",
      "cost_class": "premium",
      "reviewability": "high",
      "strength": "broader exploratory depth"
    },
    {
      "provider_family": "claude",
      "fit_for_packet": "high",
      "cost_class": "premium",
      "reviewability": "high",
      "strength": "careful critique and governance caution"
    }
  ]
}
EOF

cat >"$PROVIDER_DIR/provider_selection_manifest.json" <<'EOF'
{
  "schema_version": "adl.provider_selection_manifest.v1",
  "demo_id": "v0.89.1.gemini_provider_harmony_roundtable",
  "packet_id": "gemini_provider_roundtable_packet.v1",
  "selected_provider_family": "gemini",
  "selection_dimensions": [
    "capability_fit",
    "cost_class",
    "reviewability",
    "provider_harmony",
    "bounded_scope_match"
  ],
  "selection_result": "Gemini is selected as the best-fit participant for this bounded reviewer-facing packet while peer provider tradeoffs remain visible."
}
EOF

cat >"$PROVIDER_DIR/capability_and_cost_reasoning.md" <<'EOF'
# Capability And Cost Reasoning

This follow-on is not a benchmark contest.

It is a bounded provider-fit packet where Gemini is selected because:

- the task is reviewer-facing and structurally bounded
- the expected artifact is synthesis-heavy rather than autonomy-heavy
- the cost-class can honestly be described as economical rather than premium
- the runtime can still preserve peer commentary and visible tradeoffs afterward

ChatGPT and Claude remain respected alternatives.
The value of this packet is that ADL can explain the selection calmly instead of
smuggling it in as mere preference.
EOF

cat >"$PROVIDER_DIR/provider_fit_scorecard.json" <<'EOF'
{
  "schema_version": "adl.provider_fit_scorecard.v1",
  "packet_id": "gemini_provider_roundtable_packet.v1",
  "rows": [
    {"provider_family": "gemini", "capability_fit": "high", "cost_class": "economical", "reviewability": "high", "selected": true},
    {"provider_family": "chatgpt", "capability_fit": "high", "cost_class": "premium", "reviewability": "high", "selected": false},
    {"provider_family": "claude", "capability_fit": "high", "cost_class": "premium", "reviewability": "high", "selected": false}
  ]
}
EOF

python3 "$ROOT_DIR/adl/tools/mock_gemini_provider_roundtable.py" \
  "$PORT" \
  --port-file "$PORT_FILE" >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!
cleanup() {
  if kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

PORT="$(provider_demo_wait_for_port "$PORT_FILE")"

python3 - "$EXAMPLE" "$GENERATED_EXAMPLE" "$PORT" "$PACKET_DIR/topic.md" <<'PY'
import sys
from pathlib import Path

source, target, port, topic = sys.argv[1:5]
text = Path(source).read_text(encoding="utf-8")
text = text.replace("http://127.0.0.1:8793/gemini", f"http://127.0.0.1:{port}/gemini")
text = text.replace("http://127.0.0.1:8793/chatgpt", f"http://127.0.0.1:{port}/chatgpt")
text = text.replace("http://127.0.0.1:8793/claude", f"http://127.0.0.1:{port}/claude")
text = text.replace("http://127.0.0.1:8793/synthesis", f"http://127.0.0.1:{port}/synthesis")
text = text.replace("@file:packets/v0-89-1-gemini-provider-roundtable-topic.md", f"@file:{topic}")
Path(target).write_text(text, encoding="utf-8")
PY

export GEMINI_API_KEY="${GEMINI_API_KEY:-demo-gemini-token}"
export OPENAI_API_KEY="${OPENAI_API_KEY:-demo-openai-token}"
export ANTHROPIC_API_KEY="${ANTHROPIC_API_KEY:-demo-anthropic-token}"

cd "$ROOT_DIR"
cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- provider setup gemini --out "$TMP_PROVIDER_OUT" --force >/dev/null

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$GENERATED_EXAMPLE" \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    >"$OUT_DIR/run_log.txt" 2>&1

cp "$STEP_OUT/discussion/01-gemini-opening.md" "$ROUNDTABLE_DIR/01-gemini-opening.md"
cp "$STEP_OUT/discussion/02-chatgpt-response.md" "$ROUNDTABLE_DIR/02-chatgpt-response.md"
cp "$STEP_OUT/discussion/03-claude-response.md" "$ROUNDTABLE_DIR/03-claude-response.md"
cp "$STEP_OUT/discussion/04-synthesis.md" "$ROUNDTABLE_DIR/synthesis.md"

cat >"$ROUNDTABLE_DIR/provider_participation_summary.json" <<'EOF'
{
  "schema_version": "adl.provider_participation_summary.v1",
  "participants": [
    {"provider_family": "gemini", "role": "selected primary participant"},
    {"provider_family": "chatgpt", "role": "peer response"},
    {"provider_family": "claude", "role": "governance caution"},
    {"provider_family": "adl", "role": "selection and synthesis host"}
  ]
}
EOF

python3 - "$MANIFEST" "$RUN_ID" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.gemini_provider_harmony_follow_on_demo.v1",
    "demo_id": "v0.89.1.gemini_provider_harmony_roundtable",
    "title": "v0.89.1 Gemini provider harmony roundtable",
    "execution_mode": "runtime_http_roundtable_demo",
    "selected_provider_family": "gemini",
    "claim": "ADL can host Gemini as a first-class bounded participant while recording a calm provider-fit and cost-class rationale plus visible peer tradeoffs.",
    "artifacts": {
        "topic": "packet/topic.md",
        "packet_manifest": "packet/packet_manifest.json",
        "candidate_providers": "provider_selection/candidate_providers.json",
        "selection_manifest": "provider_selection/provider_selection_manifest.json",
        "scorecard": "provider_selection/provider_fit_scorecard.json",
        "reasoning": "provider_selection/capability_and_cost_reasoning.md",
        "gemini_opening": "roundtable/01-gemini-opening.md",
        "chatgpt_response": "roundtable/02-chatgpt-response.md",
        "claude_response": "roundtable/03-claude-response.md",
        "synthesis": "roundtable/synthesis.md",
        "participation_summary": "roundtable/provider_participation_summary.json",
        "provider_setup": "provider_setup/provider.adl.yaml",
        "run_summary": f"runtime/runs/{sys.argv[2]}/run_summary.json",
        "steps": f"runtime/runs/{sys.argv[2]}/steps.json",
        "trace": f"runtime/runs/{sys.argv[2]}/logs/trace_v1.json"
    }
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<EOF
# v0.89.1 Demo - Gemini Provider Harmony Roundtable

Canonical command:

\`\`\`bash
bash adl/tools/demo_v0891_gemini_provider_harmony_roundtable.sh
\`\`\`

What this proves:
- ADL can explain why Gemini was selected for one bounded packet
- Gemini can participate as a first-class participant without benchmark-war framing
- ChatGPT and Claude can remain visible as peer voices instead of disappearing behind the routing choice

Primary proof surfaces:
- \`provider_selection/provider_selection_manifest.json\`
- \`provider_selection/capability_and_cost_reasoning.md\`
- \`roundtable/synthesis.md\`
- \`roundtable/provider_participation_summary.json\`
- \`runtime/runs/$RUN_ID/run_summary.json\`
EOF

sanitize_generated_artifacts

echo "Gemini provider harmony roundtable proof surface under the output directory:"
echo "  provider_selection/provider_selection_manifest.json"
echo "  provider_selection/capability_and_cost_reasoning.md"
echo "  roundtable/synthesis.md"
echo "  roundtable/provider_participation_summary.json"
echo "  runtime/runs/$RUN_ID/run_summary.json"
echo "  runtime/runs/$RUN_ID/logs/trace_v1.json"
