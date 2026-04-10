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
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-}"
ANTHROPIC_KEY_FILE="${ADL_ANTHROPIC_KEY_FILE:-}"

load_key() {
  local env_name="$1"
  local key_file="${2:-}"
  if [[ -n "${!env_name:-}" ]]; then
    return 0
  fi
  if [[ -z "$key_file" || ! -s "$key_file" ]]; then
    echo "missing required credential source for $env_name; set $env_name or the matching ADL_*_KEY_FILE override" >&2
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
    echo "empty required credential source for $env_name" >&2
    return 1
  fi
  export "$env_name=$key_value"
}

sanitize_generated_artifacts() {
  export ADL_SANITIZE_OUT_DIR="$OUT_DIR"
  export ADL_SANITIZE_OUT_REAL
  ADL_SANITIZE_OUT_REAL="$(cd "$OUT_DIR" && pwd -P)"
  export ADL_SANITIZE_ROOT_DIR="$ROOT_DIR"
  export ADL_SANITIZE_ROOT_REAL
  ADL_SANITIZE_ROOT_REAL="$(cd "$ROOT_DIR" && pwd -P)"
  export ADL_SANITIZE_OPENAI_KEY_FILE="$OPENAI_KEY_FILE"
  export ADL_SANITIZE_ANTHROPIC_KEY_FILE="$ANTHROPIC_KEY_FILE"
  find "$OUT_DIR" -type f \( -name '*.json' -o -name '*.md' -o -name '*.txt' \) -print0 |
    xargs -0 perl -0pi -e '
      for my $name (qw(
        ADL_SANITIZE_OUT_REAL
        ADL_SANITIZE_OUT_DIR
        ADL_SANITIZE_ROOT_REAL
        ADL_SANITIZE_ROOT_DIR
        ADL_SANITIZE_OPENAI_KEY_FILE
        ADL_SANITIZE_ANTHROPIC_KEY_FILE
      )) {
        my $value = $ENV{$name} // "";
        next if $value eq "";
        my $replacement = $name =~ /KEY_FILE/ ? "<credential_file>" :
          ($name =~ /ROOT/ ? "<repo_root>" : "<output_dir>");
        s/\Q$value\E/$replacement/g;
      }
    '
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
  "credential_policy": "operator_env_or_explicit_key_file_no_secret_material_recorded",
  "steps": 5,
  "proof_surfaces": {
    "transcript": "transcript.md",
    "transcript_contract": "transcript_contract.json",
    "provider_invocations": "provider_invocations.json",
    "run_summary": "runtime/runs/$RUN_ID/run_summary.json",
    "trace": "runtime/runs/$RUN_ID/logs/trace_v1.json"
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
- Otherwise reads operator-selected key files only when \`ADL_OPENAI_KEY_FILE\` and \`ADL_ANTHROPIC_KEY_FILE\` are set.
- Secret values, key-file paths, and raw Authorization headers are not written to generated artifacts.

What this proves:
- one ADL runtime workflow with two named live provider families
- direct Rust-native OpenAI and Anthropic provider invocation
- five sequential turns with saved-state handoff, runtime conversation metadata, and transcript contract validation

Primary proof surfaces:
- \`transcript.md\`
- \`provider_invocations.json\`
- \`runtime/runs/$RUN_ID/run_summary.json\`
EOF

python3 "$ROOT_DIR/adl/tools/validate_multi_agent_transcript.py" \
  "$TRANSCRIPT" \
  --contract "$TRANSCRIPT_CONTRACT" \
  >/dev/null

sanitize_generated_artifacts

echo "Rust-native live multi-agent discussion proof surface under the output directory:"
echo "  transcript.md"
echo "  provider_invocations.json"
echo "  runtime/runs/$RUN_ID/run_summary.json"
echo "  runtime/runs/$RUN_ID/logs/trace_v1.json"
