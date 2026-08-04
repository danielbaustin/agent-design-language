#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNBOOK="$ROOT_DIR/docs/tooling/OPUS_REVIEW_RUNBOOK.md"

test -f "$RUNBOOK"

for required in \
  'adl-provider-adapter' \
  '--request request.json' \
  '--out result.json' \
  '--log run.jsonl' \
  '"route"' \
  '"model_identity"' \
  '"prompt_contract_ref"' \
  '"lane_ref"' \
  'provider_model_id' \
  'attempt_policy' \
  'timeout_ms' \
  'max_attempts' \
  'input_text' \
  'ANTHROPIC_API_KEY' \
  '$HOME/keys/claude2.key' \
  'git diff --check' \
  'csdlc-review'
do
  if ! grep -F -- "$required" "$RUNBOOK" >/dev/null; then
    echo "runbook is missing required contract text: $required" >&2
    exit 1
  fi
done

request_json="$(awk '/^```json$/{capture=1; next} capture && /^```$/{exit} capture' "$RUNBOOK")"
if ! jq -e '
  .route.provider == "anthropic" and
  .route.provider_model_id == "claude-opus-5" and
  .model_identity.provider_model_id == "claude-opus-5" and
  (.prompt_contract_ref | type == "string") and
  (.lane_ref | type == "string") and
  (.attempt_policy.timeout_ms == 120000) and
  (.attempt_policy.max_attempts == 1) and
  (.input_text | type == "string")
' <<<"$request_json" >/dev/null; then
  echo "runbook JSON example does not satisfy the structured Opus request contract" >&2
  exit 1
fi

for forbidden in \
  '--provider anthropic' \
  '--model claude-opus-5' \
  '--prompt-file' \
  '--max-output-tokens'
do
  if grep -F -- "$forbidden" "$RUNBOOK" >/dev/null; then
    echo "runbook contains retired adapter flag: $forbidden" >&2
    exit 1
  fi
done

if grep -E '(^|[[:space:]`(])/([[:alnum:]_.-]+)/' "$RUNBOOK" >/dev/null; then
  echo "runbook contains machine-local absolute path" >&2
  exit 1
fi

help_output="$(cargo run --quiet --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin adl-provider-adapter -- --help)"
for required in '--request <REQUEST>' '--out <OUT>' '--log <LOG>'; do
  if ! grep -F -- "$required" <<<"$help_output" >/dev/null; then
    echo "adapter help is missing current flag: $required" >&2
    exit 1
  fi
done

echo "PASS test_opus_review_runbook"
