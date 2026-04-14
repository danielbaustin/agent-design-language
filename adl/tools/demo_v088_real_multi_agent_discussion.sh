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
SYNTHESIS="$OUT_DIR/synthesis.md"
MANIFEST="$OUT_DIR/demo_manifest.json"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
README_OUT="$OUT_DIR/README.md"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-}"
ANTHROPIC_KEY_FILE="${ADL_ANTHROPIC_KEY_FILE:-}"

TURN_FILES=(
  "discussion/01-chatgpt-opening.md"
  "discussion/02-claude-opening-response.md"
  "discussion/03-chatgpt-clarification.md"
  "discussion/04-claude-challenge.md"
  "discussion/05-chatgpt-defense.md"
  "discussion/06-claude-refinement.md"
  "discussion/07-chatgpt-extension.md"
  "discussion/08-claude-narrowing-question.md"
  "discussion/09-chatgpt-reframe.md"
  "discussion/10-claude-caution.md"
  "discussion/11-chatgpt-practical-proposal.md"
  "discussion/12-claude-governance-concern.md"
  "discussion/13-chatgpt-concession.md"
  "discussion/14-claude-acknowledgement.md"
  "discussion/15-chatgpt-synthesis-path.md"
  "discussion/16-claude-synthesis-test.md"
  "discussion/17-chatgpt-revised-principles.md"
  "discussion/18-claude-partial-acceptance.md"
  "discussion/19-chatgpt-shared-synthesis.md"
  "discussion/20-claude-closing.md"
)

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

for file in "${TURN_FILES[@]}"; do
  printf '\n\n---\n\n' >>"$TRANSCRIPT"
  cat "$STEP_OUT/$file" >>"$TRANSCRIPT"
done

cat >"$SYNTHESIS" <<EOF
# Shared Synthesis

This synthesis surface highlights the convergence work from the flagship 20-turn ChatGPT + Claude tea discussion.

## Primary synthesis turn

\`\`\`markdown
$(cat "$STEP_OUT/discussion/19-chatgpt-shared-synthesis.md")
\`\`\`

## Closing reflection

\`\`\`markdown
$(cat "$STEP_OUT/discussion/20-claude-closing.md")
\`\`\`
EOF

cat >"$TRANSCRIPT_CONTRACT" <<'EOF'
{
  "schema_version": "multi_agent_discussion_transcript.v1",
  "transcript_path": "transcript.md",
  "turn_count": 20,
  "turns": [
    {"turn_id": "turn_01", "ordinal": 1, "speaker": "ChatGPT", "heading": "# Turn 1 - ChatGPT", "source_output": "out/discussion/01-chatgpt-opening.md"},
    {"turn_id": "turn_02", "ordinal": 2, "speaker": "Claude", "heading": "# Turn 2 - Claude", "source_output": "out/discussion/02-claude-opening-response.md"},
    {"turn_id": "turn_03", "ordinal": 3, "speaker": "ChatGPT", "heading": "# Turn 3 - ChatGPT", "source_output": "out/discussion/03-chatgpt-clarification.md"},
    {"turn_id": "turn_04", "ordinal": 4, "speaker": "Claude", "heading": "# Turn 4 - Claude", "source_output": "out/discussion/04-claude-challenge.md"},
    {"turn_id": "turn_05", "ordinal": 5, "speaker": "ChatGPT", "heading": "# Turn 5 - ChatGPT", "source_output": "out/discussion/05-chatgpt-defense.md"},
    {"turn_id": "turn_06", "ordinal": 6, "speaker": "Claude", "heading": "# Turn 6 - Claude", "source_output": "out/discussion/06-claude-refinement.md"},
    {"turn_id": "turn_07", "ordinal": 7, "speaker": "ChatGPT", "heading": "# Turn 7 - ChatGPT", "source_output": "out/discussion/07-chatgpt-extension.md"},
    {"turn_id": "turn_08", "ordinal": 8, "speaker": "Claude", "heading": "# Turn 8 - Claude", "source_output": "out/discussion/08-claude-narrowing-question.md"},
    {"turn_id": "turn_09", "ordinal": 9, "speaker": "ChatGPT", "heading": "# Turn 9 - ChatGPT", "source_output": "out/discussion/09-chatgpt-reframe.md"},
    {"turn_id": "turn_10", "ordinal": 10, "speaker": "Claude", "heading": "# Turn 10 - Claude", "source_output": "out/discussion/10-claude-caution.md"},
    {"turn_id": "turn_11", "ordinal": 11, "speaker": "ChatGPT", "heading": "# Turn 11 - ChatGPT", "source_output": "out/discussion/11-chatgpt-practical-proposal.md"},
    {"turn_id": "turn_12", "ordinal": 12, "speaker": "Claude", "heading": "# Turn 12 - Claude", "source_output": "out/discussion/12-claude-governance-concern.md"},
    {"turn_id": "turn_13", "ordinal": 13, "speaker": "ChatGPT", "heading": "# Turn 13 - ChatGPT", "source_output": "out/discussion/13-chatgpt-concession.md"},
    {"turn_id": "turn_14", "ordinal": 14, "speaker": "Claude", "heading": "# Turn 14 - Claude", "source_output": "out/discussion/14-claude-acknowledgement.md"},
    {"turn_id": "turn_15", "ordinal": 15, "speaker": "ChatGPT", "heading": "# Turn 15 - ChatGPT", "source_output": "out/discussion/15-chatgpt-synthesis-path.md"},
    {"turn_id": "turn_16", "ordinal": 16, "speaker": "Claude", "heading": "# Turn 16 - Claude", "source_output": "out/discussion/16-claude-synthesis-test.md"},
    {"turn_id": "turn_17", "ordinal": 17, "speaker": "ChatGPT", "heading": "# Turn 17 - ChatGPT", "source_output": "out/discussion/17-chatgpt-revised-principles.md"},
    {"turn_id": "turn_18", "ordinal": 18, "speaker": "Claude", "heading": "# Turn 18 - Claude", "source_output": "out/discussion/18-claude-partial-acceptance.md"},
    {"turn_id": "turn_19", "ordinal": 19, "speaker": "ChatGPT", "heading": "# Turn 19 - ChatGPT", "source_output": "out/discussion/19-chatgpt-shared-synthesis.md"},
    {"turn_id": "turn_20", "ordinal": 20, "speaker": "Claude", "heading": "# Turn 20 - Claude", "source_output": "out/discussion/20-claude-closing.md"}
  ],
  "companion_artifacts": {
    "demo_manifest": "demo_manifest.json",
    "synthesis": "synthesis.md",
    "run_summary": "runtime/runs/v0-88-real-multi-agent-tea-discussion/run_summary.json",
    "trace": "runtime/runs/v0-88-real-multi-agent-tea-discussion/logs/trace_v1.json"
  }
}
EOF

cat >"$MANIFEST" <<EOF
{
  "demo_id": "v0.88.real_multi_agent_discussion",
  "title": "Rust-native live ChatGPT + Claude long-form tea discussion demo",
  "execution_mode": "runtime_native_live_providers",
  "provider_mode": "rust_native_openai_and_anthropic",
  "credential_policy": "operator_env_or_explicit_key_file_no_secret_material_recorded",
  "steps": 20,
  "structure": {
    "acts": [
      "opening_positions",
      "first_exchange",
      "deepening",
      "convergence_work",
      "closing"
    ],
    "roles": {
      "ChatGPT": "Builder",
      "Claude": "Reflective Critic"
    }
  },
  "proof_surfaces": {
    "transcript": "transcript.md",
    "transcript_contract": "transcript_contract.json",
    "synthesis": "synthesis.md",
    "provider_invocations": "provider_invocations.json",
    "run_summary": "runtime/runs/$RUN_ID/run_summary.json",
    "trace": "runtime/runs/$RUN_ID/logs/trace_v1.json"
  }
}
EOF

cat >"$README_OUT" <<EOF
# v0.88 Demo - Rust-Native Live ChatGPT + Claude Long-Form Tea Discussion

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
- a 20-turn act-structured discussion with explicit role framing, visible disagreement, and usable synthesis

Primary proof surfaces:
- \`transcript.md\`
- \`synthesis.md\`
- \`provider_invocations.json\`
- \`runtime/runs/$RUN_ID/run_summary.json\`
EOF

python3 "$ROOT_DIR/adl/tools/validate_multi_agent_transcript.py" \
  "$TRANSCRIPT" \
  --contract "$TRANSCRIPT_CONTRACT" \
  >/dev/null

sanitize_generated_artifacts

echo "Rust-native long-form multi-agent discussion proof surface under the output directory:"
echo "  transcript.md"
echo "  synthesis.md"
echo "  provider_invocations.json"
echo "  runtime/runs/$RUN_ID/run_summary.json"
echo "  runtime/runs/$RUN_ID/logs/trace_v1.json"
