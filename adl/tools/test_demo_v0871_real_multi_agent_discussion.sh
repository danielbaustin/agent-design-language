#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai.key}"
ANTHROPIC_KEY_FILE="${ADL_ANTHROPIC_KEY_FILE:-$HOME/keys/claude.key}"

emit_disposition() {
  local status="$1"
  local reason="$2"
  python3 - "$status" "$reason" <<'PY'
import json
import sys

status, reason = sys.argv[1:3]
payload = {
    "demo_id": "D13L",
    "validation_disposition": status,
    "reason": reason,
    "credentialed_proof": status == "credentialed_proof_passed",
}
print("ADL_VALIDATION_DISPOSITION=" + json.dumps(payload, sort_keys=True), file=sys.stderr)
PY
}

if [[ -z "${OPENAI_API_KEY:-}" && ! -s "$OPENAI_KEY_FILE" ]]; then
  emit_disposition "skipped_non_proving" "missing_openai_credentials"
  echo "SKIP: missing OPENAI_API_KEY and $OPENAI_KEY_FILE" >&2
  exit 0
fi
if [[ -z "${ANTHROPIC_API_KEY:-}" && ! -s "$ANTHROPIC_KEY_FILE" ]]; then
  emit_disposition "skipped_non_proving" "missing_anthropic_credentials"
  echo "SKIP: missing ANTHROPIC_API_KEY and $ANTHROPIC_KEY_FILE" >&2
  exit 0
fi

TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0871_real_multi_agent_discussion.sh "$OUT_DIR" >/dev/null
)

TRANSCRIPT="$OUT_DIR/transcript.md"
TRANSCRIPT_CONTRACT="$OUT_DIR/transcript_contract.json"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
SUMMARY="$OUT_DIR/runtime/runs/v0-87-1-real-multi-agent-tea-discussion/run_summary.json"
STEPS="$OUT_DIR/runtime/runs/v0-87-1-real-multi-agent-tea-discussion/steps.json"
TRACE="$OUT_DIR/runtime/runs/v0-87-1-real-multi-agent-tea-discussion/logs/trace_v1.json"
LOG_FILE="$OUT_DIR/run_log.txt"

for required in "$TRANSCRIPT" "$TRANSCRIPT_CONTRACT" "$INVOCATIONS" "$MANIFEST" "$SUMMARY" "$STEPS" "$TRACE" "$LOG_FILE"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq '"execution_mode": "runtime_http_live_provider_adapter"' "$MANIFEST" || {
  echo "assertion failed: manifest missing live execution mode" >&2
  exit 1
}
grep -Fq '"conversation"' "$STEPS" || {
  echo "assertion failed: steps artifact missing conversation metadata" >&2
  exit 1
}

python3 - "$INVOCATIONS" <<'PY'
import json
import sys

path = sys.argv[1]
payload = json.load(open(path, encoding="utf-8"))
invocations = payload.get("invocations", [])
families = [item.get("family") for item in invocations]
if families.count("openai") != 3:
    raise SystemExit(f"expected 3 OpenAI invocations, found {families.count('openai')}")
if families.count("anthropic") != 2:
    raise SystemExit(f"expected 2 Anthropic invocations, found {families.count('anthropic')}")
bad_statuses = [item for item in invocations if item.get("http_status") != 200]
if bad_statuses:
    raise SystemExit("expected all live provider invocations to return HTTP 200")
for item in invocations:
    if item.get("prompt_chars", 0) <= 0 or item.get("output_chars", 0) <= 0:
        raise SystemExit("expected non-empty prompt/output character counts")
PY

python3 "$ROOT_DIR/adl/tools/validate_multi_agent_transcript.py" \
  "$TRANSCRIPT" \
  --contract "$TRANSCRIPT_CONTRACT" \
  >/dev/null

if grep -R -F "${OPENAI_API_KEY:-__ADL_NO_OPENAI_ENV_KEY__}" "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: OPENAI_API_KEY value leaked into artifacts" >&2
  exit 1
fi
if grep -R -F "${ANTHROPIC_API_KEY:-__ADL_NO_ANTHROPIC_ENV_KEY__}" "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: ANTHROPIC_API_KEY value leaked into artifacts" >&2
  exit 1
fi
if grep -R -E 'Authorization:|Bearer |x-api-key' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: credential header material leaked into artifacts" >&2
  exit 1
fi

emit_disposition "credentialed_proof_passed" "live_provider_artifacts_verified"
echo "demo_v0871_real_multi_agent_discussion: ok"
