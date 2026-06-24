#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
PR_DELEGATE_SRC="$ROOT_DIR/adl/tools/pr_delegate.sh"
PR_USAGE_SRC="$ROOT_DIR/adl/tools/pr_usage.sh"
PR_CARDS_SRC="$ROOT_DIR/adl/tools/pr_cards.sh"
OBSERVABILITY_SRC="$ROOT_DIR/adl/tools/observability.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
INPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/output_card_template.md"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$PR_DELEGATE_SRC" "$repo/adl/tools/pr_delegate.sh"
cp "$PR_USAGE_SRC" "$repo/adl/tools/pr_usage.sh"
cp "$PR_CARDS_SRC" "$repo/adl/tools/pr_cards.sh"
cp "$OBSERVABILITY_SRC" "$repo/adl/tools/observability.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
chmod +x "$repo/adl/tools/pr.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add -A
  git commit -q -m "init"
)

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

(
  cd "$repo"

  help_out="$("$BASH_BIN" adl/tools/pr.sh help)"
  assert_contains "Commands:" "$help_out" "help alias"
  assert_contains "Compatibility / maintenance commands:" "$help_out" "compatibility section"
  assert_contains "  run     <issue>" "$help_out" "canonical run help"
  assert_contains "  doctor  <issue>" "$help_out" "canonical doctor help"
  if grep -Fq "  new     " <<<"$help_out"; then
    echo "assertion failed: help should not advertise retired pr new command" >&2
    exit 1
  fi

  card_help_out="$("$BASH_BIN" adl/tools/pr.sh card --help)"
  assert_contains "Usage:" "$card_help_out" "card --help"

  in_path="$("$BASH_BIN" adl/tools/pr.sh card 271 input --no-fetch-issue --slug demo-title)"
  [[ "$in_path" == *"/.adl/v0.86/tasks/issue-0271__demo-title/sip.md" ]] || {
    echo "assertion failed: unexpected input card path: $in_path" >&2
    exit 1
  }
  [[ -f ".adl/v0.86/tasks/issue-0271__demo-title/sip.md" ]] || {
    echo "assertion failed: expected input card file to exist" >&2
    exit 1
  }
  [[ -L ".adl/cards/271/input_271.md" ]] || {
    echo "assertion failed: expected input compatibility link to exist" >&2
    exit 1
  }

  out_path="$("$BASH_BIN" adl/tools/pr.sh card 271 output --no-fetch-issue --slug demo-title)"
  [[ "$out_path" == *"/.adl/v0.86/tasks/issue-0271__demo-title/sor.md" ]] || {
    echo "assertion failed: unexpected output card path via card command: $out_path" >&2
    exit 1
  }
  [[ -f ".adl/v0.86/tasks/issue-0271__demo-title/sor.md" ]] || {
    echo "assertion failed: expected output card file to exist" >&2
    exit 1
  }
  [[ -L ".adl/cards/271/output_271.md" ]] || {
    echo "assertion failed: expected output compatibility link to exist" >&2
    exit 1
  }

  out_path2="$("$BASH_BIN" adl/tools/pr.sh output 271 output --no-fetch-issue --slug demo-title)"
  [[ "$out_path2" == *"/.adl/v0.86/tasks/issue-0271__demo-title/sor.md" ]] || {
    echo "assertion failed: unexpected output card path via output command: $out_path2" >&2
    exit 1
  }

  in_path2="$("$BASH_BIN" adl/tools/pr.sh output 271 input --no-fetch-issue --slug demo-title)"
  [[ "$in_path2" == *"/.adl/v0.86/tasks/issue-0271__demo-title/sip.md" ]] || {
    echo "assertion failed: unexpected input card path via output command: $in_path2" >&2
    exit 1
  }

  set +e
  bad_out="$("$BASH_BIN" adl/tools/pr.sh card 271 nope 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected non-zero status for unknown card arg" >&2
    exit 1
  }
  assert_contains "Usage:" "$bad_out" "card unknown arg usage"

  mv adl/tools/pr_cards.sh adl/tools/pr_cards.sh.missing
  set +e
  missing_helper_out="$("$BASH_BIN" adl/tools/pr.sh card --help 2>&1)"
  missing_helper_status=$?
  set -e
  mv adl/tools/pr_cards.sh.missing adl/tools/pr_cards.sh
  if [[ "$missing_helper_status" -eq 0 ]]; then
    echo "assertion failed: expected missing pr_cards helper to fail closed" >&2
    exit 1
  fi
  assert_contains "missing pr.sh helper: pr_cards.sh" "$missing_helper_out" "missing cards helper message"
  assert_contains "stage=source-helper result=missing helper=pr_cards.sh" "$missing_helper_out" "missing cards helper observability"
)

echo "pr.sh help + card positional open flows: ok"
