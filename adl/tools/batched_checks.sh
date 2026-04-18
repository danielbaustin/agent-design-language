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
run_step "repo-packet-builder contract check" bash "$ROOT/adl/tools/test_repo_packet_builder_skill_contracts.sh"
run_step "redaction-and-evidence-auditor contract check" bash "$ROOT/adl/tools/test_redaction_and_evidence_auditor_skill_contracts.sh"
run_step "repo-architecture-review contract check" bash "$ROOT/adl/tools/test_repo_architecture_review_skill_contracts.sh"
run_step "repo-dependency-review contract check" bash "$ROOT/adl/tools/test_repo_dependency_review_skill_contracts.sh"
run_step "repo-diagram-planner contract check" bash "$ROOT/adl/tools/test_repo_diagram_planner_skill_contracts.sh"
run_step "architecture-diagram-reviewer contract check" bash "$ROOT/adl/tools/test_architecture_diagram_reviewer_skill_contracts.sh"
run_step "review-to-test-planner contract check" bash "$ROOT/adl/tools/test_review_to_test_planner_skill_contracts.sh"
run_step "test-generator contract check" bash "$ROOT/adl/tools/test_test_generator_skill_contracts.sh"
run_step "demo-operator contract check" bash "$ROOT/adl/tools/test_demo_operator_skill_contracts.sh"
run_step "medium-article-writer contract check" bash "$ROOT/adl/tools/test_medium_article_writer_skill_contracts.sh"
run_step "arxiv-paper-writer contract check" bash "$ROOT/adl/tools/test_arxiv_paper_writer_skill_contracts.sh"
run_step "diagram-author contract check" bash "$ROOT/adl/tools/test_diagram_author_skill_contracts.sh"
run_step "tracked .adl issue-record residue guard" bash "$ROOT/adl/tools/check_no_tracked_adl_issue_record_residue.sh"
echo "• Running adl checks (batched)…"
(
  cd "$ROOT/adl"
  run_step "cargo fmt --check" cargo fmt --check
  run_step "cargo clippy --all-targets -- -D warnings" cargo clippy --all-targets -- -D warnings
  run_step "cargo test" cargo test
)
