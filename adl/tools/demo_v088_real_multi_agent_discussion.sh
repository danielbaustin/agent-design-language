#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v088/real_multi_agent_discussion}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-88-real-multi-agent-tea-discussion"
TRANSCRIPT="$OUT_DIR/transcript.md"
TRANSCRIPT_CONTRACT="$OUT_DIR/transcript_contract.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
README_OUT="$OUT_DIR/README.md"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai.key}"
ANTHROPIC_KEY_FILE="${ADL_ANTHROPIC_KEY_FILE:-$HOME/keys/claude.key}"

load_key() {
  local env_name="$1"
  local key_file="$2"
  if [[ -n "${!env_name:-}" ]]; then
    return 0
  fi
  if [[ ! -s "$key_file" ]]; then
    echo "missing required key file for $env_name: $key_file" >&2
    return 1
  fi
  local key_value=""
  while IFS= read -r line || [[ -n "$line" ]]; do
    line="${line//$'\r'/}"
    [[ -z "$line" || "$line" == \#* ]] && continue
    if [[ "$line" == "$env_name="* ]]; then
      key_value="${line#*=}"
    else
      key_value="$line"
    fi
    key_value="${key_value%\"}"
    key_value="${key_value#\"}"
    key_value="${key_value%\'}"
    key_value="${key_value#\'}"
    break
  done <"$key_file"
  if [[ -z "$key_value" ]]; then
    echo "empty required key file for $env_name: $key_file" >&2
    return 1
  fi
  export "$env_name=$key_value"
}

load_key OPENAI_API_KEY "$OPENAI_KEY_FILE"
load_key ANTHROPIC_API_KEY "$ANTHROPIC_KEY_FILE"

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT"

cd "$ROOT_DIR"

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
ADL_PROVIDER_INVOCATIONS_PATH="$INVOCATIONS" \
  bash adl/tools/pr.sh run adl/examples/v0-88-real-multi-agent-tea-discussion.adl.yaml \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    >"$OUT_DIR/run_log.txt" 2>&1

cat >"$TRANSCRIPT" <<'EOF'
# Claude + ChatGPT Multi-Agent Tea Discussion Transcript

This transcript is assembled from the runtime-written step outputs under `out/discussion/`.
EOF

for file in \
  "$STEP_OUT/discussion/01-chatgpt-opening.md" \
  "$STEP_OUT/discussion/02-claude-reply.md" \
  "$STEP_OUT/discussion/03-chatgpt-reflection.md" \
  "$STEP_OUT/discussion/04-claude-refinement.md" \
  "$STEP_OUT/discussion/05-chatgpt-toast.md"; do
  printf '\n\n---\n\n' >>"$TRANSCRIPT"
  cat "$file" >>"$TRANSCRIPT"
done

cat >"$TRANSCRIPT_CONTRACT" <<EOF
{
  "schema_version": "multi_agent_discussion_transcript.v1",
  "transcript_path": "transcript.md",
  "turn_count": 5,
  "turns": [
    {"turn_id": "turn_01", "ordinal": 1, "speaker": "ChatGPT", "heading": "# Turn 1 - ChatGPT", "source_output": "out/discussion/01-chatgpt-opening.md"},
    {"turn_id": "turn_02", "ordinal": 2, "speaker": "Claude", "heading": "# Turn 2 - Claude", "source_output": "out/discussion/02-claude-reply.md"},
    {"turn_id": "turn_03", "ordinal": 3, "speaker": "ChatGPT", "heading": "# Turn 3 - ChatGPT", "source_output": "out/discussion/03-chatgpt-reflection.md"},
    {"turn_id": "turn_04", "ordinal": 4, "speaker": "Claude", "heading": "# Turn 4 - Claude", "source_output": "out/discussion/04-claude-refinement.md"},
    {"turn_id": "turn_05", "ordinal": 5, "speaker": "ChatGPT", "heading": "# Turn 5 - ChatGPT", "source_output": "out/discussion/05-chatgpt-toast.md"}
  ],
  "companion_artifacts": {
    "demo_manifest": "demo_manifest.json",
    "run_summary": "runtime/runs/$RUN_ID/run_summary.json",
    "trace": "runtime/runs/$RUN_ID/logs/trace_v1.json"
  }
}
EOF

cat >"$MANIFEST" <<EOF
{
  "demo_id": "v0.88.real_multi_agent_discussion",
  "title": "Rust-native live ChatGPT + Claude multi-agent tea discussion demo",
  "execution_mode": "runtime_native_live_providers",
  "provider_mode": "rust_native_openai_and_anthropic",
  "credential_policy": "operator_env_or_home_keys_no_secret_material_recorded",
  "steps": 5,
  "proof_surfaces": {
    "transcript": "$TRANSCRIPT",
    "transcript_contract": "$TRANSCRIPT_CONTRACT",
    "provider_invocations": "$INVOCATIONS",
    "run_summary": "$RUNS_ROOT/$RUN_ID/run_summary.json",
    "trace": "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json"
  }
}
EOF

cat >"$README_OUT" <<EOF
# v0.88 Demo - Rust-Native Live ChatGPT + Claude Multi-Agent Tea Discussion

Canonical command:

\`\`\`bash
bash adl/tools/demo_v088_real_multi_agent_discussion.sh
\`\`\`

Credential loading:
- Uses \`OPENAI_API_KEY\` and \`ANTHROPIC_API_KEY\` when already set.
- Otherwise reads local operator-managed keys from \`\\\$HOME/keys/openai.key\` and \`\\\$HOME/keys/claude.key\`.
- Secret values and raw Authorization headers are not written to generated artifacts.

What this proves:
- one ADL runtime workflow with two named live provider families
- direct Rust-native OpenAI and Anthropic provider invocation
- five sequential turns with saved-state handoff, runtime conversation metadata, and transcript contract validation

Primary proof surfaces:
- \`$TRANSCRIPT\`
- \`$INVOCATIONS\`
- \`$RUNS_ROOT/$RUN_ID/run_summary.json\`
EOF

python3 "$ROOT_DIR/adl/tools/validate_multi_agent_transcript.py" \
  "$TRANSCRIPT" \
  --contract "$TRANSCRIPT_CONTRACT" \
  >/dev/null

echo "Rust-native live multi-agent discussion proof surface:"
echo "  $TRANSCRIPT"
echo "  $INVOCATIONS"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"
echo "  $RUNS_ROOT/$RUN_ID/logs/trace_v1.json"
