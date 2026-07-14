#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/skills/sprint-conductor/scripts/validate_review_subagent_policy.py"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

python3 "$SCRIPT" \
  --allow-review-subagent-exception true \
  --max-review-subagents 1 \
  --review-subagent-id reviewer-1 >"$TMP/legacy.out"
grep -Fq "review_subagent_policy_ok" "$TMP/legacy.out"

if env -u OPENAI_API_KEY python3 "$SCRIPT" \
  --allow-review-subagent-exception true \
  --max-review-subagents 1 \
  --review-subagent-id reviewer-1 \
  --require-responses-auth >"$TMP/missing.out" 2>"$TMP/missing.err"; then
  echo "expected missing auth to fail" >&2
  exit 1
fi
grep -Fq "missing Codex Responses API authentication context" "$TMP/missing.err"
grep -Fq "OPENAI_API_KEY" "$TMP/missing.err"
if grep -Eq 'sk-[A-Za-z0-9_-]{12,}|Bearer |Authorization:' "$TMP/missing.err"; then
  echo "missing-auth diagnostic leaked credential-shaped content" >&2
  exit 1
fi

OPENAI_API_KEY=fixture-token python3 "$SCRIPT" \
  --allow-review-subagent-exception true \
  --max-review-subagents 1 \
  --review-subagent-id reviewer-1 \
  --require-responses-auth >"$TMP/present.out"
grep -Fq "review_subagent_policy_ok" "$TMP/present.out"

if OPENAI_API_KEY=fixture-token python3 "$SCRIPT" \
  --allow-review-subagent-exception true \
  --max-review-subagents 1 \
  --review-subagent-id reviewer-1 \
  --require-responses-auth \
  --subagent-model-override gpt-test-model >"$TMP/model.out" 2>"$TMP/model.err"; then
  echo "expected model override to fail" >&2
  exit 1
fi
grep -Fq "review subagent model override is forbidden" "$TMP/model.err"
if grep -Fq "gpt-test-model" "$TMP/model.err"; then
  echo "model override diagnostic exposed the supplied override value" >&2
  exit 1
fi

echo "review_subagent_auth_policy_ok"
