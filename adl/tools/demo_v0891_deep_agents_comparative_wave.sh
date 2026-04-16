#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0891/deep_agents_comparative_wave}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-89-1-deep-agents-comparative-governance-wave"
PORT="${ADL_DEEP_AGENTS_FOLLOW_ON_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_server.log"
EXAMPLE="adl/examples/v0-89-1-deep-agents-comparative-governance-wave.adl.yaml"
GENERATED_EXAMPLE="$OUT_DIR/v0-89-1-deep-agents-comparative-governance-wave.runtime.adl.yaml"
WAVE_DIR="$OUT_DIR/comparative_wave"
POSITIONS_DIR="$WAVE_DIR/positions"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"

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
mkdir -p "$STEP_OUT" "$WAVE_DIR" "$POSITIONS_DIR"

python3 "$ROOT_DIR/adl/tools/mock_deep_agents_follow_on_provider.py" \
  "$PORT" \
  --port-file "$PORT_FILE" \
  >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!
cleanup() {
  if kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

PORT="$(provider_demo_wait_for_port "$PORT_FILE")"

python3 - "$EXAMPLE" "$GENERATED_EXAMPLE" "$PORT" <<'PY'
import sys
from pathlib import Path

source, target, port = sys.argv[1:4]
text = Path(source).read_text(encoding="utf-8")
text = text.replace("http://127.0.0.1:8794/chatgpt", f"http://127.0.0.1:{port}/chatgpt")
text = text.replace("http://127.0.0.1:8794/claude", f"http://127.0.0.1:{port}/claude")
text = text.replace("http://127.0.0.1:8794/gemini", f"http://127.0.0.1:{port}/gemini")
text = text.replace("http://127.0.0.1:8794/synthesis", f"http://127.0.0.1:{port}/synthesis")
Path(target).write_text(text, encoding="utf-8")
PY

python3 - "$PORT" <<'PY'
import json
import sys
import time
import urllib.request

port = int(sys.argv[1])
url = f"http://127.0.0.1:{port}/health"
deadline = time.time() + 10.0
last_error = None
while time.time() < deadline:
    try:
        with urllib.request.urlopen(url, timeout=1.0) as resp:
            payload = json.load(resp)
        if payload.get("ok") is True:
            raise SystemExit(0)
    except Exception as exc:
        last_error = exc
        time.sleep(0.1)
raise SystemExit(f"provider shim failed health check: {last_error}")
PY

cd "$ROOT_DIR"

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$GENERATED_EXAMPLE" \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    >"$OUT_DIR/run_log.txt" 2>&1

cp "$STEP_OUT/discussion/01-chatgpt-position.md" "$POSITIONS_DIR/chatgpt.md"
cp "$STEP_OUT/discussion/02-claude-position.md" "$POSITIONS_DIR/claude.md"
cp "$STEP_OUT/discussion/03-gemini-position.md" "$POSITIONS_DIR/gemini.md"
cp "$STEP_OUT/discussion/04-synthesis.md" "$WAVE_DIR/synthesis.md"

cat >"$WAVE_DIR/comparison_claims.md" <<'EOF'
# Comparison Claims

This packet deepens the bounded `v0.88` comparative proof rather than replacing it.

The comparative claim is intentionally narrow:

- ADL makes multi-agent behavior more reviewable and governable
- ADL gives the reviewer clearer packet, trace, and operator surfaces
- the filesystem-first style still retains some appeal for fast exploratory work

This is not a benchmark ladder or winner/loser packet.
EOF

cat >"$WAVE_DIR/reviewer_checklist.md" <<'EOF'
# Reviewer Checklist

Review this packet in order:

1. `comparative_wave/comparison_claims.md`
2. `comparative_wave/adl_surface_map.json`
3. `comparative_wave/filesystem_surface_map.json`
4. `comparative_wave/positions/chatgpt.md`
5. `comparative_wave/positions/claude.md`
6. `comparative_wave/positions/gemini.md`
7. `comparative_wave/synthesis.md`
8. `comparative_wave/public_positioning_summary.md`
9. `runtime/runs/v0-89-1-deep-agents-comparative-governance-wave/run_summary.json`

What to verify:

- the claim is comparative, bounded, and respectful
- governance and review surfaces are visible rather than implied
- operator visibility is explicit enough to explain accountability quickly
EOF

cat >"$WAVE_DIR/adl_surface_map.json" <<'EOF'
{
  "schema_version": "adl.deep_agents_surface_map.v1",
  "surface_kind": "adl_runtime_reviewable",
  "surfaces": [
    "packet manifest and comparison claims",
    "explicit reviewer checklist",
    "position-by-position provider outputs",
    "runtime trace and run summary",
    "operator visibility and governance summary"
  ]
}
EOF

cat >"$WAVE_DIR/filesystem_surface_map.json" <<'EOF'
{
  "schema_version": "adl.deep_agents_surface_map.v1",
  "surface_kind": "filesystem_first_reference",
  "surfaces": [
    "visible files and role outputs",
    "lighter-weight packet structure",
    "weaker reviewer guidance",
    "less explicit provenance and governance surfacing"
  ]
}
EOF

cat >"$WAVE_DIR/operator_visibility.md" <<'EOF'
# Operator Visibility

In this comparative packet, the operator role is legible because ADL makes the
review surface explicit:

- the packet has a declared comparative claim
- the reviewer checklist states what should be verified
- the runtime emits trace and run-summary artifacts
- the synthesis names governance and coordination cost directly

That is the difference this packet is trying to show. The operator is not just
the person who ran the demo; the operator is part of a visible review contract.
EOF

cat >"$WAVE_DIR/governance_snapshot.json" <<'EOF'
{
  "schema_version": "adl.deep_agents_governance_snapshot.v1",
  "claims": [
    "review surfaces are explicit",
    "operator obligations are visible",
    "trace-backed packet lineage exists",
    "comparative language remains bounded"
  ],
  "non_goals": [
    "benchmark ranking",
    "winner-loser scorecard",
    "claiming universal superiority"
  ]
}
EOF

cat >"$WAVE_DIR/public_positioning_summary.md" <<'EOF'
# Public Positioning Summary

The right public story is not that ADL makes louder deep-agent demos.

The right public story is that ADL makes multi-agent behavior easier to inspect:

- stronger reviewer orientation
- clearer governance and packet boundaries
- more explicit operator role visibility
- more legible lineage from packet to trace

That is a calmer and more defensible claim than "our agents are smarter."
EOF

python3 - "$MANIFEST" "$RUN_ID" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.deep_agents_comparative_wave_follow_on_demo.v1",
    "demo_id": "v0.89.1.deep_agents_comparative_governance_wave",
    "title": "v0.89.1 deep-agents comparative governance wave demo",
    "execution_mode": "runtime_http_compatibility_demo",
    "claim": "ADL can host a bounded comparative wave whose runtime, governance, and operator surfaces make the review difference legible in minutes.",
    "artifacts": {
        "comparison_claims": "comparative_wave/comparison_claims.md",
        "reviewer_checklist": "comparative_wave/reviewer_checklist.md",
        "adl_surface_map": "comparative_wave/adl_surface_map.json",
        "filesystem_surface_map": "comparative_wave/filesystem_surface_map.json",
        "operator_visibility": "comparative_wave/operator_visibility.md",
        "governance_snapshot": "comparative_wave/governance_snapshot.json",
        "public_positioning_summary": "comparative_wave/public_positioning_summary.md",
        "chatgpt_position": "comparative_wave/positions/chatgpt.md",
        "claude_position": "comparative_wave/positions/claude.md",
        "gemini_position": "comparative_wave/positions/gemini.md",
        "synthesis": "comparative_wave/synthesis.md",
        "run_summary": f"runtime/runs/{sys.argv[2]}/run_summary.json",
        "steps": f"runtime/runs/{sys.argv[2]}/steps.json",
        "trace": f"runtime/runs/{sys.argv[2]}/logs/trace_v1.json"
    }
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<EOF
# v0.89.1 Demo - Deep-Agents Comparative Governance Wave

Canonical command:

\`\`\`bash
bash adl/tools/demo_v0891_deep_agents_comparative_wave.sh
\`\`\`

Primary proof surfaces:
- \`comparative_wave/comparison_claims.md\`
- \`comparative_wave/reviewer_checklist.md\`
- \`comparative_wave/adl_surface_map.json\`
- \`comparative_wave/filesystem_surface_map.json\`
- \`comparative_wave/operator_visibility.md\`
- \`comparative_wave/public_positioning_summary.md\`
- \`comparative_wave/synthesis.md\`
- \`runtime/runs/$RUN_ID/run_summary.json\`

What this proves:
- ADL can deepen the earlier bounded comparative row without turning into a benchmark stunt
- the difference is expressed through governance, reviewer, and operator surfaces
- the final packet stays calm, evidence-first, and publicly usable
EOF

sanitize_generated_artifacts

echo "Deep-agents comparative governance proof surface under the output directory:"
echo "  comparative_wave/comparison_claims.md"
echo "  comparative_wave/reviewer_checklist.md"
echo "  comparative_wave/adl_surface_map.json"
echo "  comparative_wave/filesystem_surface_map.json"
echo "  comparative_wave/operator_visibility.md"
echo "  comparative_wave/public_positioning_summary.md"
echo "  comparative_wave/synthesis.md"
echo "  runtime/runs/$RUN_ID/run_summary.json"
