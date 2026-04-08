#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

ARTIFACT_ROOT="$TMPDIR_ROOT/artifacts"
RESPONSE_FIXTURE="$ROOT_DIR/demos/fixtures/codex_ollama_operational_skills_demo/semantic_fallback_response.json"

(
  cd "$ROOT_DIR"
  ADL_SEMANTIC_FALLBACK_RESPONSE_FILE="$RESPONSE_FIXTURE" \
  bash adl/tools/demo_codex_ollama_operational_skills.sh \
    --artifact-root "$ARTIFACT_ROOT" \
    --model deepseek-r1:latest >/dev/null
)

MANIFEST_FILE="$ARTIFACT_ROOT/demo_manifest.json"
LAST_MESSAGE_FILE="$ARTIFACT_ROOT/codex_last_message.md"
STP_PATH="$ARTIFACT_ROOT/workspace/.adl/v0.87.1/tasks/issue-9001__v0-87-1-tools-demo-codex-cli-ollama-operational-skills/stp.md"
SIP_PATH="$ARTIFACT_ROOT/workspace/.adl/v0.87.1/tasks/issue-9001__v0-87-1-tools-demo-codex-cli-ollama-operational-skills/sip.md"

[[ -f "$MANIFEST_FILE" ]] || {
  echo "assertion failed: manifest missing" >&2
  exit 1
}
[[ -f "$LAST_MESSAGE_FILE" ]] || {
  echo "assertion failed: last message missing" >&2
  exit 1
}

grep -Fq '"execution_mode": "semantic_tool_fallback"' "$MANIFEST_FILE" || {
  echo "assertion failed: manifest did not record semantic fallback mode" >&2
  exit 1
}
grep -Fq '"capability_profile_id": "ollama_deepseek_semantic_fallback"' "$MANIFEST_FILE" || {
  echo "assertion failed: manifest did not record deepseek fallback profile" >&2
  exit 1
}
grep -Fq 'Branch: not bound yet' "$SIP_PATH" || {
  echo "assertion failed: fallback did not normalize the SIP branch state" >&2
  exit 1
}
grep -Fq 'Refine the STP to provide clearer deliverables and validation scope.' "$STP_PATH" || {
  echo "assertion failed: fallback did not tighten the STP summary" >&2
  exit 1
}

bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type stp --input "$STP_PATH" >/dev/null
bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type sip --phase bootstrap --input "$SIP_PATH" >/dev/null

echo "demo_codex_ollama_operational_skills semantic fallback: ok"
