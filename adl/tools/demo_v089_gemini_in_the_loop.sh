#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v089/gemini_in_the_loop}"
FIXTURE_DIR="$ROOT_DIR/demos/fixtures/gemini_in_the_loop"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-89-gemini-in-the-loop"
EXAMPLE="adl/examples/v0-89-gemini-in-the-loop.adl.yaml"
PACKET_DIR="$OUT_DIR/packet"
REVIEW_DIR="$OUT_DIR/review_artifacts"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"
PORT="${ADL_GEMINI_LOOP_PORT:-8789}"
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
if payload.get("packet_id") != "gemini_review_packet.v1":
    raise SystemExit("packet_id mismatch")
if payload.get("provider_family") != "gemini":
    raise SystemExit("provider_family mismatch")
if payload.get("execution_mode") != "bounded_http_profile":
    raise SystemExit("execution_mode mismatch")
if payload.get("verdict") not in {"accept", "revise"}:
    raise SystemExit("invalid verdict")
findings = payload.get("findings")
if not isinstance(findings, list) or not findings:
    raise SystemExit("findings must be a non-empty list")
for idx, finding in enumerate(findings, start=1):
    if not isinstance(finding, dict):
      raise SystemExit(f"finding {idx} must be an object")
    for key in ("id", "severity", "title", "detail"):
      if not isinstance(finding.get(key), str) or not finding[key]:
        raise SystemExit(f"finding {idx} missing {key}")
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
    "# Gemini Findings",
    "",
    f"Packet ID: `{payload['packet_id']}`",
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
mkdir -p "$PACKET_DIR" "$REVIEW_DIR"
cp "$FIXTURE_DIR/review_packet.md" "$PACKET_DIR/review_packet.md"

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
mkdir -p "$TMP_PROVIDER_OUT"
cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- provider setup gemini --out "$TMP_PROVIDER_OUT" --force >/dev/null

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$EXAMPLE" \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    >"$OUT_DIR/run_log.txt" 2>&1

validate_output "$STEP_OUT/review/01-gemini-review.json" "$REVIEW_DIR/validated_review.json"
render_findings "$REVIEW_DIR/validated_review.json" "$REVIEW_DIR/findings.md"

python3 - "$MANIFEST" "$RUN_ID" <<'PY'
import json, sys
from pathlib import Path
manifest = {
    "schema_version": "adl.gemini_in_the_loop_demo.v1",
    "demo_id": "v0.89.gemini_in_the_loop",
    "title": "Gemini in the Loop bounded provider demo",
    "execution_mode": "bounded_http_profile",
    "provider_family": "gemini",
    "provider_profile": "http:gemini-2.0-flash",
    "claim": "ADL can host Gemini as a first-class bounded participant by packaging a review packet, invoking Gemini through a provider-aware path, validating the output, and writing stable reviewer-facing artifacts.",
    "artifacts": {
        "packet": "packet/review_packet.md",
        "provider_setup": "provider_setup/provider.adl.yaml",
        "raw_output": "out/review/01-gemini-review.json",
        "validated_output": "review_artifacts/validated_review.json",
        "findings": "review_artifacts/findings.md",
        "run_summary": f"runtime/runs/{sys.argv[2]}/run_summary.json",
        "steps": f"runtime/runs/{sys.argv[2]}/steps.json",
        "trace": f"runtime/runs/{sys.argv[2]}/logs/trace_v1.json"
    }
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<EOF
# v0.89 Demo - Gemini in the Loop

Canonical command:

\`\`\`bash
bash adl/tools/demo_v089_gemini_in_the_loop.sh
\`\`\`

Primary proof surfaces:
- \`demo_manifest.json\`
- \`packet/review_packet.md\`
- \`review_artifacts/validated_review.json\`
- \`review_artifacts/findings.md\`
- \`runtime/runs/$RUN_ID/run_summary.json\`
- \`runtime/runs/$RUN_ID/logs/trace_v1.json\`

What this proves:
- ADL can package a bounded packet for Gemini
- Gemini can contribute through a provider-aware HTTP profile path
- output is validated before it is accepted as a review artifact
- the runtime, not Gemini, stays responsible for packet construction and artifact writing
EOF

sanitize_generated_artifacts

echo "Gemini in the Loop proof surface under the output directory:"
echo "  demo_manifest.json"
echo "  review_artifacts/findings.md"
echo "  review_artifacts/validated_review.json"
echo "  runtime/runs/$RUN_ID/run_summary.json"
echo "  runtime/runs/$RUN_ID/logs/trace_v1.json"
