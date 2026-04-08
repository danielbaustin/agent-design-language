#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT_ROOT="$ROOT_DIR/artifacts/v0871/codex_ollama_skills"
FIXTURE_ROOT="$ROOT_DIR/demos/fixtures/codex_ollama_operational_skills_demo/workspace"
CAPABILITY_FILE="$ROOT_DIR/adl/tools/local_model_capabilities.v1.json"
CODEX_BIN="${CODEX_BIN:-codex}"
MODEL="${CODEX_OLLAMA_MODEL:-gpt-oss:latest}"
LOCAL_PROVIDER="${CODEX_LOCAL_PROVIDER:-ollama}"
OLLAMA_HOST_URL="${OLLAMA_HOST_URL:-${OLLAMA_HOST:-http://127.0.0.1:11434}}"
OLLAMA_GENERATE_TIMEOUT_SECS="${ADL_OLLAMA_GENERATE_TIMEOUT_SECS:-90}"
FORCE_SEMANTIC_FALLBACK="${ADL_DEMO_FORCE_SEMANTIC_FALLBACK:-0}"
SEMANTIC_FALLBACK_RESPONSE_FILE="${ADL_SEMANTIC_FALLBACK_RESPONSE_FILE:-}"
DRY_RUN=0

usage() {
  cat <<'EOF'
Usage:
  bash adl/tools/demo_codex_ollama_operational_skills.sh [--artifact-root <path>] [--model <ollama-model>] [--local-provider ollama|lmstudio] [--dry-run]

Purpose:
  Install the tracked operational skills into a demo-local CODEX_HOME, copy a
  prepared local issue bundle fixture into an artifact workspace, and run Codex
  CLI against local OSS provider settings to use the editor skills on that
  bounded task.

Notes:
  - Default model: gpt-oss:latest
  - Default provider: ollama
  - Default Ollama API: http://127.0.0.1:11434
  - Default semantic-fallback timeout: 90 seconds
  - Non-tool local models are routed through semantic tool fallback instead of native Codex tool calling.
  - --dry-run prepares the workspace, prompt, and manifest but does not invoke Codex.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --artifact-root)
      ARTIFACT_ROOT="$2"
      shift 2
      ;;
    --model)
      MODEL="$2"
      shift 2
      ;;
    --local-provider)
      LOCAL_PROVIDER="$2"
      shift 2
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "ERROR: unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

mkdir -p "$ARTIFACT_ROOT"
WORKSPACE="$ARTIFACT_ROOT/workspace"
CODEX_HOME_DEMO="$ARTIFACT_ROOT/codex_home"
PROMPT_FILE="$ARTIFACT_ROOT/codex_prompt.md"
FALLBACK_PROMPT_FILE="$ARTIFACT_ROOT/semantic_tool_fallback_prompt.md"
MANIFEST_FILE="$ARTIFACT_ROOT/demo_manifest.json"
EVENTS_FILE="$ARTIFACT_ROOT/codex_events.jsonl"
LAST_MESSAGE_FILE="$ARTIFACT_ROOT/codex_last_message.md"
RUN_LOG="$ARTIFACT_ROOT/codex_stdout.log"
FALLBACK_RESPONSE_RAW="$ARTIFACT_ROOT/semantic_tool_fallback_raw.json"
FALLBACK_RESPONSE_PARSED="$ARTIFACT_ROOT/semantic_tool_fallback_parsed.json"
FALLBACK_ERROR_LOG="$ARTIFACT_ROOT/semantic_tool_fallback_error.log"

rm -rf "$WORKSPACE" "$CODEX_HOME_DEMO"
mkdir -p "$WORKSPACE" "$CODEX_HOME_DEMO"
cp -R "$FIXTURE_ROOT/." "$WORKSPACE/"

CODEX_INSTALL_MODE="${ADL_OPERATIONAL_SKILLS_INSTALL_MODE:-symlink}"
(
  cd "$ROOT_DIR"
  CODEX_HOME="$CODEX_HOME_DEMO" ADL_OPERATIONAL_SKILLS_INSTALL_MODE="$CODEX_INSTALL_MODE" \
    bash adl/tools/install_adl_operational_skills.sh >/dev/null
)

STP_PATH="$WORKSPACE/.adl/v0.87.1/tasks/issue-9001__v0-87-1-tools-demo-codex-cli-ollama-operational-skills/stp.md"
SIP_PATH="$WORKSPACE/.adl/v0.87.1/tasks/issue-9001__v0-87-1-tools-demo-codex-cli-ollama-operational-skills/sip.md"
SOURCE_PROMPT_PATH="$WORKSPACE/.adl/v0.87.1/bodies/issue-9001-v0-87-1-tools-demo-codex-cli-ollama-operational-skills.md"
STP_SKILL="$CODEX_HOME_DEMO/skills/stp-editor/SKILL.md"
SIP_SKILL="$CODEX_HOME_DEMO/skills/sip-editor/SKILL.md"
REL_STP_PATH=".adl/v0.87.1/tasks/issue-9001__v0-87-1-tools-demo-codex-cli-ollama-operational-skills/stp.md"
REL_SIP_PATH=".adl/v0.87.1/tasks/issue-9001__v0-87-1-tools-demo-codex-cli-ollama-operational-skills/sip.md"
REL_SOURCE_PROMPT_PATH=".adl/v0.87.1/bodies/issue-9001-v0-87-1-tools-demo-codex-cli-ollama-operational-skills.md"

resolve_model_capabilities() {
  python3 - "$CAPABILITY_FILE" "$LOCAL_PROVIDER" "$MODEL" "$FORCE_SEMANTIC_FALLBACK" <<'PY'
import fnmatch
import json
import sys

capability_path, provider, model, force_flag = sys.argv[1:]
with open(capability_path, "r", encoding="utf-8") as fh:
    data = json.load(fh)

selected = None
for profile in data.get("profiles", []):
    if profile.get("provider") != provider:
        continue
    for pattern in profile.get("model_globs", []):
        if fnmatch.fnmatch(model, pattern):
            selected = profile
            break
    if selected is not None:
        break

if selected is None:
    selected = {
        "profile_id": "unknown",
        "reason": "no_matching_capability_profile",
        "capabilities": {
            "native_tool_calling": False,
            "semantic_tool_fallback": False,
            "structured_output_mode": "unknown",
        },
    }

result = {
    "profile_id": selected["profile_id"],
    "reason": selected["reason"],
    "capabilities": selected["capabilities"],
}

if force_flag == "1":
    result["reason"] = "forced_semantic_tool_fallback"
    result["capabilities"]["native_tool_calling"] = False
    result["capabilities"]["semantic_tool_fallback"] = True
    if result["capabilities"]["structured_output_mode"] == "unknown":
        result["capabilities"]["structured_output_mode"] = "prompt_json"

print(json.dumps(result))
PY
}

build_semantic_fallback_prompt() {
  cat >"$FALLBACK_PROMPT_FILE" <<EOF
You are operating in ADL semantic tool fallback mode because this local model does not support native tool calling.
Do not describe tool calls. Do not emit prose outside the required JSON object.

Return exactly one JSON object with this shape:
{
  "stp_markdown": "<full replacement content for ${REL_STP_PATH}>",
  "sip_markdown": "<full replacement content for ${REL_SIP_PATH}>",
  "summary": "<brief summary of the bounded card cleanup>"
}

Rules:
- Preserve issue intent and milestone context.
- Keep the task docs-only and bounded to local card cleanup.
- Do not create branches, worktrees, PRs, or implementation claims.
- Use repository-relative paths only.
- Do not include markdown fences.
- Output valid JSON only.

Editing guidance derived from the tracked editor skills:
- Tighten the STP summary, goal, deliverables, and acceptance criteria.
- Remove vague placeholders and contradictory instructions.
- Keep the SIP truthful for pre-run card cleanup.
- Use Branch: not bound yet when no execution branch should exist.
- Make target surfaces concrete.
- Give a short, realistic validation plan.
- Do not widen scope beyond these two files.

Source issue prompt (${REL_SOURCE_PROMPT_PATH}):
$(cat "$SOURCE_PROMPT_PATH")

Current STP (${REL_STP_PATH}):
$(cat "$STP_PATH")

Current SIP (${REL_SIP_PATH}):
$(cat "$SIP_PATH")
EOF
}

run_semantic_tool_fallback() {
  if [[ "$LOCAL_PROVIDER" != "ollama" ]]; then
    echo "ERROR: semantic tool fallback is currently implemented only for the ollama local provider" >&2
    exit 1
  fi

  build_semantic_fallback_prompt

  if [[ -n "$SEMANTIC_FALLBACK_RESPONSE_FILE" ]]; then
    cp "$SEMANTIC_FALLBACK_RESPONSE_FILE" "$FALLBACK_RESPONSE_RAW"
  else
    python3 - "$MODEL" "$FALLBACK_PROMPT_FILE" >"$ARTIFACT_ROOT/semantic_tool_fallback_request.json" <<'PY'
import json
import sys

model = sys.argv[1]
prompt_path = sys.argv[2]
with open(prompt_path, "r", encoding="utf-8") as fh:
    prompt = fh.read()
json.dump(
    {
        "model": model,
        "prompt": prompt,
        "stream": False,
        "options": {
            "temperature": 0,
            "num_predict": 1600,
        },
    },
    sys.stdout,
)
PY

    if ! curl -fsS \
      --connect-timeout 5 \
      --max-time "$OLLAMA_GENERATE_TIMEOUT_SECS" \
      "${OLLAMA_HOST_URL%/}/api/generate" \
      -H 'Content-Type: application/json' \
      --data-binary @"$ARTIFACT_ROOT/semantic_tool_fallback_request.json" \
      >"$FALLBACK_RESPONSE_RAW" 2>"$FALLBACK_ERROR_LOG"; then
      if grep -qi "timed out" "$FALLBACK_ERROR_LOG" 2>/dev/null; then
        echo "ERROR: semantic tool fallback timed out after ${OLLAMA_GENERATE_TIMEOUT_SECS}s while waiting for ${MODEL} via ${LOCAL_PROVIDER}" >&2
      else
        echo "ERROR: semantic tool fallback request failed for ${MODEL} via ${LOCAL_PROVIDER}" >&2
        if [[ -s "$FALLBACK_ERROR_LOG" ]]; then
          cat "$FALLBACK_ERROR_LOG" >&2
        fi
      fi
      exit 1
    fi
  fi

  python3 - "$FALLBACK_RESPONSE_RAW" "$FALLBACK_RESPONSE_PARSED" <<'PY'
import json
import re
import sys

raw_path, parsed_path = sys.argv[1:]
with open(raw_path, "r", encoding="utf-8") as fh:
    data = json.load(fh)

if isinstance(data, dict) and "stp_markdown" in data and "sip_markdown" in data:
    parsed = data
else:
    response = data.get("response", "")
    response = response.strip()
    if response.startswith("```"):
        response = re.sub(r"^```(?:json)?\s*", "", response)
        response = re.sub(r"\s*```$", "", response)
    try:
        parsed = json.loads(response)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"semantic fallback output was not valid JSON: {exc}")

required = ("stp_markdown", "sip_markdown", "summary")
for key in required:
    value = parsed.get(key)
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"semantic fallback output missing required string field: {key}")

with open(parsed_path, "w", encoding="utf-8") as fh:
    json.dump(parsed, fh, indent=2)
PY

  python3 - "$FALLBACK_RESPONSE_PARSED" "$STP_PATH" "$SIP_PATH" "$LAST_MESSAGE_FILE" "$EVENTS_FILE" "$RUN_LOG" <<'PY'
import json
import sys

parsed_path, stp_path, sip_path, last_message_path, events_path, run_log_path = sys.argv[1:]
with open(parsed_path, "r", encoding="utf-8") as fh:
    parsed = json.load(fh)

with open(stp_path, "w", encoding="utf-8") as fh:
    fh.write(parsed["stp_markdown"])
    if not parsed["stp_markdown"].endswith("\n"):
        fh.write("\n")

with open(sip_path, "w", encoding="utf-8") as fh:
    fh.write(parsed["sip_markdown"])
    if not parsed["sip_markdown"].endswith("\n"):
        fh.write("\n")

summary = parsed["summary"].strip()
with open(last_message_path, "w", encoding="utf-8") as fh:
    fh.write(summary + "\n")

with open(events_path, "w", encoding="utf-8") as fh:
    fh.write(json.dumps({"type": "semantic_fallback.started"}) + "\n")
    fh.write(json.dumps({"type": "semantic_fallback.completed", "summary": summary}) + "\n")

with open(run_log_path, "a", encoding="utf-8") as fh:
    fh.write("semantic_tool_fallback: applied parsed model output\n")
PY
}

cat >"$PROMPT_FILE" <<EOF
Use \$stp-editor at $STP_SKILL and \$sip-editor at $SIP_SKILL.

You are already running inside the copied demo fixture workspace.
Your current working directory is the writable fixture root.
Use repository-relative paths from that workspace only.
Do not use absolute host paths.

Task:
- Read the source prompt at $REL_SOURCE_PROMPT_PATH.
- Tighten the STP at $REL_STP_PATH so it has clearer deliverables, acceptance criteria, and validation scope without changing issue intent.
- Normalize the SIP at $REL_SIP_PATH so it is truthful, concrete, and bounded to local card cleanup.
- Keep the task docs-only and limited to this fixture bundle.
- Do not create branches, worktrees, PRs, or implementation claims.
- Read files with shell commands that use relative paths.
- Edit only those two files with apply_patch.
- Stop after editing only those two files and summarize the changes briefly.
EOF

bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type stp --input "$STP_PATH" >/dev/null
bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type sip --phase bootstrap --input "$SIP_PATH" >/dev/null

CAPABILITY_JSON="$(resolve_model_capabilities)"
NATIVE_TOOL_CALLING="$(printf '%s' "$CAPABILITY_JSON" | python3 -c 'import json,sys; print("true" if json.load(sys.stdin)["capabilities"]["native_tool_calling"] else "false")')"
SEMANTIC_TOOL_FALLBACK="$(printf '%s' "$CAPABILITY_JSON" | python3 -c 'import json,sys; print("true" if json.load(sys.stdin)["capabilities"]["semantic_tool_fallback"] else "false")')"
STRUCTURED_OUTPUT_MODE="$(printf '%s' "$CAPABILITY_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin)["capabilities"]["structured_output_mode"])')"
CAPABILITY_PROFILE_ID="$(printf '%s' "$CAPABILITY_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin)["profile_id"])')"
CAPABILITY_REASON="$(printf '%s' "$CAPABILITY_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin)["reason"])')"
EXECUTION_MODE="native_tool_calling"
if [[ "$NATIVE_TOOL_CALLING" != "true" ]]; then
  EXECUTION_MODE="semantic_tool_fallback"
fi

cat >"$MANIFEST_FILE" <<EOF
{
  "demo": "codex_ollama_operational_skills",
  "repo_root": "$ROOT_DIR",
  "artifact_root": "$ARTIFACT_ROOT",
  "workspace": "$WORKSPACE",
  "codex_working_root": "$WORKSPACE",
  "codex_home": "$CODEX_HOME_DEMO",
  "install_mode": "$CODEX_INSTALL_MODE",
  "local_provider": "$LOCAL_PROVIDER",
  "ollama_host_url": "$OLLAMA_HOST_URL",
  "model": "$MODEL",
  "capability_manifest": "$CAPABILITY_FILE",
  "capability_profile_id": "$CAPABILITY_PROFILE_ID",
  "capability_reason": "$CAPABILITY_REASON",
  "execution_mode": "$EXECUTION_MODE",
  "capabilities": $CAPABILITY_JSON,
  "skills": [
    "$STP_SKILL",
    "$SIP_SKILL"
  ],
  "target_files": [
    "$STP_PATH",
    "$SIP_PATH"
  ],
  "source_prompt": "$SOURCE_PROMPT_PATH",
  "dry_run": $( [[ "$DRY_RUN" -eq 1 ]] && echo true || echo false )
}
EOF

if [[ "$DRY_RUN" -eq 1 ]]; then
  printf '%s\n' "DRY_RUN prepared at $ARTIFACT_ROOT"
  printf '%s\n' "PROMPT $PROMPT_FILE"
  printf '%s\n' "MANIFEST $MANIFEST_FILE"
  exit 0
fi

if [[ "$LOCAL_PROVIDER" == "ollama" && -z "$SEMANTIC_FALLBACK_RESPONSE_FILE" ]]; then
  curl -fsS "${OLLAMA_HOST_URL%/}/api/tags" >/dev/null 2>&1 || {
    echo "ERROR: Ollama API not reachable at ${OLLAMA_HOST_URL%/}/api/tags; make sure the local Ollama service is running" >&2
    exit 1
  }
fi

if [[ "$EXECUTION_MODE" == "native_tool_calling" ]]; then
  command -v "$CODEX_BIN" >/dev/null 2>&1 || {
    echo "ERROR: codex CLI not found in PATH" >&2
    exit 1
  }
  "$CODEX_BIN" exec --help >/dev/null 2>&1 || {
    echo "ERROR: codex exec --help failed" >&2
    exit 1
  }

  set +e
  CODEX_HOME="$CODEX_HOME_DEMO" "$CODEX_BIN" exec \
    --full-auto \
    --oss \
    --local-provider "$LOCAL_PROVIDER" \
    --model "$MODEL" \
    --sandbox workspace-write \
    --cd "$WORKSPACE" \
    --skip-git-repo-check \
    --add-dir "$WORKSPACE" \
    --add-dir "$ARTIFACT_ROOT" \
    --add-dir "$ROOT_DIR" \
    --output-last-message "$LAST_MESSAGE_FILE" \
    --json \
    "$(cat "$PROMPT_FILE")" | tee "$EVENTS_FILE" "$RUN_LOG"
  CODEX_EXIT=${PIPESTATUS[0]}
  set -e

  if [[ "$CODEX_EXIT" -ne 0 ]]; then
    if grep -Fq 'does not support tools' "$RUN_LOG"; then
      echo "ERROR: model '$MODEL' is reachable in Ollama but does not support Codex tool calls. Update the capability manifest or use a tool-capable local model such as 'gpt-oss:latest'." >&2
    fi
    exit "$CODEX_EXIT"
  fi
else
  if [[ "$SEMANTIC_TOOL_FALLBACK" != "true" ]]; then
    echo "ERROR: model '$MODEL' is not marked native tool-capable and no semantic fallback is declared in $CAPABILITY_FILE" >&2
    exit 1
  fi
  run_semantic_tool_fallback
fi

bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type stp --input "$STP_PATH"
bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type sip --phase bootstrap --input "$SIP_PATH"

printf '%s\n' "OK: local-model operational-skills demo completed via $EXECUTION_MODE"
printf '%s\n' "Artifacts: $ARTIFACT_ROOT"
