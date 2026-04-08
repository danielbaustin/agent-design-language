#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT_ROOT="$ROOT_DIR/artifacts/v0871/codex_ollama_skills"
FIXTURE_ROOT="$ROOT_DIR/demos/fixtures/codex_ollama_operational_skills_demo/workspace"
CODEX_BIN="${CODEX_BIN:-codex}"
MODEL="${CODEX_OLLAMA_MODEL:-deepseek-r1:8b}"
LOCAL_PROVIDER="${CODEX_LOCAL_PROVIDER:-ollama}"
OLLAMA_HOST_URL="${OLLAMA_HOST_URL:-${OLLAMA_HOST:-http://127.0.0.1:11434}}"
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
  - Default model: deepseek-r1:8b
  - Default provider: ollama
  - Default Ollama API: http://127.0.0.1:11434
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
MANIFEST_FILE="$ARTIFACT_ROOT/demo_manifest.json"
EVENTS_FILE="$ARTIFACT_ROOT/codex_events.jsonl"
LAST_MESSAGE_FILE="$ARTIFACT_ROOT/codex_last_message.md"
RUN_LOG="$ARTIFACT_ROOT/codex_stdout.log"

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

cat >"$PROMPT_FILE" <<EOF
Use \$stp-editor at $STP_SKILL and \$sip-editor at $SIP_SKILL.

Repository root: $ROOT_DIR
Working demo fixture root: $WORKSPACE

Task:
- Read the source prompt at $SOURCE_PROMPT_PATH.
- Tighten the STP at $STP_PATH so it has clearer deliverables, acceptance criteria, and validation scope without changing issue intent.
- Normalize the SIP at $SIP_PATH so it is truthful, concrete, and bounded to local card cleanup.
- Keep the task docs-only and limited to this fixture bundle.
- Do not create branches, worktrees, PRs, or implementation claims.
- Stop after editing only those two files and summarize the changes briefly.
EOF

bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type stp --input "$STP_PATH" >/dev/null
bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type sip --input "$SIP_PATH" >/dev/null

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

command -v "$CODEX_BIN" >/dev/null 2>&1 || {
  echo "ERROR: codex CLI not found in PATH" >&2
  exit 1
}
"$CODEX_BIN" exec --help >/dev/null 2>&1 || {
  echo "ERROR: codex exec --help failed" >&2
  exit 1
}

if [[ "$LOCAL_PROVIDER" == "ollama" ]]; then
  curl -fsS "${OLLAMA_HOST_URL%/}/api/tags" >/dev/null 2>&1 || {
    echo "ERROR: Ollama API not reachable at ${OLLAMA_HOST_URL%/}/api/tags; make sure the local Ollama service is running" >&2
    exit 1
  }
fi

CODEX_HOME="$CODEX_HOME_DEMO" "$CODEX_BIN" exec \
  --full-auto \
  --oss \
  --local-provider "$LOCAL_PROVIDER" \
  --model "$MODEL" \
  --cd "$WORKSPACE" \
  --skip-git-repo-check \
  --add-dir "$ROOT_DIR" \
  --output-last-message "$LAST_MESSAGE_FILE" \
  --json \
  "$(cat "$PROMPT_FILE")" | tee "$EVENTS_FILE" "$RUN_LOG"

bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type stp --input "$STP_PATH"
bash "$ROOT_DIR/adl/tools/validate_structured_prompt.sh" --type sip --input "$SIP_PATH"

printf '%s\n' "OK: Codex CLI + $LOCAL_PROVIDER demo completed"
printf '%s\n' "Artifacts: $ARTIFACT_ROOT"
