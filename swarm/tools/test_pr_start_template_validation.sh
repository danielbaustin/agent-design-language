#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/swarm/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/swarm/tools/card_paths.sh"
PROMPT_LINT_SRC="$ROOT_DIR/swarm/tools/lint_prompt_spec.sh"
PROMPT_VALIDATOR_SRC="$ROOT_DIR/swarm/tools/validate_structured_prompt.sh"
INPUT_TPL_SRC="$ROOT_DIR/swarm/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/swarm/templates/cards/output_card_template.md"
STP_CONTRACT_SRC="$ROOT_DIR/swarm/schemas/structured_task_prompt.contract.yaml"
SIP_CONTRACT_SRC="$ROOT_DIR/swarm/schemas/structured_implementation_prompt.contract.yaml"
SOR_CONTRACT_SRC="$ROOT_DIR/swarm/schemas/structured_output_record.contract.yaml"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

origin="$tmpdir/origin.git"
repo="$tmpdir/repo"
mkdir -p "$repo/swarm/tools" "$repo/swarm/templates/cards" "$repo/swarm/schemas"
cp "$PR_SH_SRC" "$repo/swarm/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/swarm/tools/card_paths.sh"
cp "$PROMPT_LINT_SRC" "$repo/swarm/tools/lint_prompt_spec.sh"
cp "$PROMPT_VALIDATOR_SRC" "$repo/swarm/tools/validate_structured_prompt.sh"
cp "$INPUT_TPL_SRC" "$repo/swarm/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/swarm/templates/cards/output_card_template.md"
cp "$STP_CONTRACT_SRC" "$repo/swarm/schemas/structured_task_prompt.contract.yaml"
cp "$SIP_CONTRACT_SRC" "$repo/swarm/schemas/structured_implementation_prompt.contract.yaml"
cp "$SOR_CONTRACT_SRC" "$repo/swarm/schemas/structured_output_record.contract.yaml"
chmod +x "$repo/swarm/tools/pr.sh" "$repo/swarm/tools/lint_prompt_spec.sh" "$repo/swarm/tools/validate_structured_prompt.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add -A
  git commit -q -m "init"
  git branch -M main
  git init --bare -q "$origin"
  git remote add origin "$origin"
  git push -q -u origin main
  git fetch -q origin main
)

assert_contains() {
  local pattern="$1" text="$2"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed: expected to find '$pattern'" >&2
    echo "$text" >&2
    exit 1
  }
}

(
  cd "$repo"
  "$BASH_BIN" swarm/tools/pr.sh start 910 --slug validation-pass --no-fetch-issue >/dev/null

  perl -0pi -e 's/Status: NOT_STARTED/Status: MAYBE/' swarm/templates/cards/output_card_template.md

  set +e
  bad="$("$BASH_BIN" swarm/tools/pr.sh start 911 --slug validation-fail --no-fetch-issue 2>&1)"
  status=$?
  set -e

  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected pr.sh start validation failure" >&2
    exit 1
  }
  assert_contains "output card failed bootstrap validation" "$bad"
)

echo "pr.sh start template validation: ok"
