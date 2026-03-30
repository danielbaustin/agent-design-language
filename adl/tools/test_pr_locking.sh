#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
PROMPT_LINT_SRC="$ROOT_DIR/adl/tools/lint_prompt_spec.sh"
PROMPT_VALIDATOR_SRC="$ROOT_DIR/adl/tools/validate_structured_prompt.sh"
INPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/output_card_template.md"
STP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_task_prompt.contract.yaml"
SIP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_implementation_prompt.contract.yaml"
SOR_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_output_record.contract.yaml"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

if rg -n '\$\(acquire_repo_lock ' "$PR_SH_SRC" >/dev/null 2>&1; then
  echo "assertion failed: acquire_repo_lock should not be invoked via command substitution" >&2
  exit 1
fi

origin="$tmpdir/origin.git"
repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards" "$repo/adl/schemas"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$PROMPT_LINT_SRC" "$repo/adl/tools/lint_prompt_spec.sh"
cp "$PROMPT_VALIDATOR_SRC" "$repo/adl/tools/validate_structured_prompt.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
cp "$STP_CONTRACT_SRC" "$repo/adl/schemas/structured_task_prompt.contract.yaml"
cp "$SIP_CONTRACT_SRC" "$repo/adl/schemas/structured_implementation_prompt.contract.yaml"
cp "$SOR_CONTRACT_SRC" "$repo/adl/schemas/structured_output_record.contract.yaml"
chmod +x "$repo/adl/tools/pr.sh" "$repo/adl/tools/lint_prompt_spec.sh" "$repo/adl/tools/validate_structured_prompt.sh"

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

  start_out="$("$BASH_BIN" adl/tools/pr.sh start 980 --slug lock-smoke --no-fetch-issue)"
  [[ "$start_out" == *"STATE  FULLY_STARTED"* ]] || {
    echo "assertion failed: expected legacy start to complete" >&2
    echo "$start_out" >&2
    exit 1
  }
  [[ -d "$repo/.adl/locks" ]] || {
    echo "assertion failed: expected shared lock root to exist under .adl/locks" >&2
    exit 1
  }

  cards_out="$("$BASH_BIN" adl/tools/pr.sh cards 981 --no-fetch-issue --version v0.86)"
  [[ "$cards_out" == *"STATE=ISSUE_AND_CARDS_READY"* ]] || {
    echo "assertion failed: expected cards command to complete" >&2
    echo "$cards_out" >&2
    exit 1
  }
)

echo "pr.sh lock acquisition and bootstrap smoke: ok"
