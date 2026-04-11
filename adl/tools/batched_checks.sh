#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERBOSE="${ADL_CHECKS_VERBOSE:-0}"

if [[ "${1:-}" == "--verbose" ]]; then
  VERBOSE="1"
fi

run_step() {
  local label="$1"
  shift
  if [[ "$VERBOSE" == "1" ]]; then
    echo "  • $label"
    "$@"
    return 0
  fi

  local log
  log="$(mktemp)"
  if "$@" >"$log" 2>&1; then
    echo "  ✓ $label"
    rm -f "$log"
    return 0
  fi

  echo "  ✗ $label"
  cat "$log" >&2
  rm -f "$log"
  return 1
}

echo "• Running tooling sanity checks (batched)…"
bash -n "$ROOT/adl/tools/codex_pr.sh"
bash -n "$ROOT/adl/tools/codexw.sh"
echo "Skipping codex_pr sanity check (no --paths configured)."
sh "$ROOT/adl/tools/codexw.sh" --help >/dev/null 2>&1
run_step "repo-code-review contract check" bash "$ROOT/adl/tools/test_repo_code_review_skill_contracts.sh"
run_step "tracked .adl issue-record residue guard" bash "$ROOT/adl/tools/check_no_tracked_adl_issue_record_residue.sh"

echo "• Running adl checks (batched)…"
(
  cd "$ROOT/adl"
  run_step "cargo fmt --check" cargo fmt --check
  run_step "cargo clippy --all-targets -- -D warnings" cargo clippy --all-targets -- -D warnings
  run_step "cargo test" cargo test
)
