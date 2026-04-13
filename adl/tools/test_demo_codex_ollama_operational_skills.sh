#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

ARTIFACT_ROOT="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  OLLAMA_HOST=192.168.68.73 \
  bash adl/tools/demo_codex_ollama_operational_skills.sh --dry-run --artifact-root "$ARTIFACT_ROOT" >/dev/null
)

PROMPT_FILE="$ARTIFACT_ROOT/codex_prompt.md"
MANIFEST_FILE="$ARTIFACT_ROOT/demo_manifest.json"
STP_SKILL="$ARTIFACT_ROOT/codex_home/skills/stp-editor/SKILL.md"
SIP_SKILL="$ARTIFACT_ROOT/codex_home/skills/sip-editor/SKILL.md"
STP_PATH="$ARTIFACT_ROOT/workspace/.adl/v0.87.1/tasks/issue-9001__v0-87-1-tools-demo-codex-cli-ollama-operational-skills/stp.md"
SIP_PATH="$ARTIFACT_ROOT/workspace/.adl/v0.87.1/tasks/issue-9001__v0-87-1-tools-demo-codex-cli-ollama-operational-skills/sip.md"

[[ -f "$PROMPT_FILE" ]] || {
  echo "assertion failed: prompt file missing" >&2
  exit 1
}
[[ -f "$MANIFEST_FILE" ]] || {
  echo "assertion failed: manifest missing" >&2
  exit 1
}
[[ -f "$STP_PATH" ]] || {
  echo "assertion failed: copied STP fixture missing" >&2
  exit 1
}
[[ -f "$SIP_PATH" ]] || {
  echo "assertion failed: copied SIP fixture missing" >&2
  exit 1
}
[[ -f "$STP_SKILL" ]] || {
  echo "assertion failed: installed stp-editor skill missing" >&2
  exit 1
}
[[ -f "$SIP_SKILL" ]] || {
  echo "assertion failed: installed sip-editor skill missing" >&2
  exit 1
}

grep -Fq '$stp-editor' "$PROMPT_FILE" || {
  echo "assertion failed: prompt does not reference stp-editor" >&2
  exit 1
}
grep -Fq '$sip-editor' "$PROMPT_FILE" || {
  echo "assertion failed: prompt does not reference sip-editor" >&2
  exit 1
}
grep -Fq 'gpt-oss:latest' "$MANIFEST_FILE" || {
  echo "assertion failed: manifest does not record default tool-capable model" >&2
  exit 1
}
grep -Fq '"codex_working_root"' "$MANIFEST_FILE" || {
  echo "assertion failed: manifest does not record codex working root" >&2
  exit 1
}
grep -Fq '"capability_manifest"' "$MANIFEST_FILE" || {
  echo "assertion failed: manifest does not record capability manifest path" >&2
  exit 1
}
grep -Fq '"execution_mode": "native_tool_calling"' "$MANIFEST_FILE" || {
  echo "assertion failed: manifest does not record native tool execution mode" >&2
  exit 1
}
grep -Fq '"ollama_host_url": "http://192.168.68.73:11434"' "$MANIFEST_FILE" || {
  echo "assertion failed: manifest did not normalize raw OLLAMA_HOST into the expected remote URL" >&2
  exit 1
}
grep -Fq 'Do not use absolute host paths.' "$PROMPT_FILE" || {
  echo "assertion failed: prompt does not reinforce relative-path execution" >&2
  exit 1
}
grep -Fq 'Edit only those two files with apply_patch.' "$PROMPT_FILE" || {
  echo "assertion failed: prompt does not direct explicit bounded editing" >&2
  exit 1
}

echo "demo_codex_ollama_operational_skills dry-run: ok"
