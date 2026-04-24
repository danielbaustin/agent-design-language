#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

plan="$(bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" --print-plan)"

for required in \
  "phase=always_on_authoritative" \
  "phase=proof_heavy_authoritative" \
  "proof_features=slow-proof-tests,slow-finish-tests" \
  "test(access_control)" \
  "test(challenge)" \
  "test(observatory_flagship)" \
  "test(private_state_observatory)" \
  "test(runtime_v2_feature_proof_coverage_runs_runtime_v2_cli_regression_matrix)" \
  "test(runtime_v2_v0903_demo_stdout_uses_repo_relative_output_paths)" \
  "test(real_pr_finish_creates_draft_pr_and_commits_branch_changes)" \
  "test(real_pr_finish_rejects_main_and_reports_no_pr_when_only_local_bundle_sync_changes_exist)" \
  "test(real_pr_finish_rejects_staged_gitignore_changes_without_allow_flag)"
do
  if ! grep -F "$required" <<<"$plan" >/dev/null 2>&1; then
    echo "missing authoritative coverage plan token: $required" >&2
    exit 1
  fi
done

if ! grep -F "base_filter=not (" <<<"$plan" >/dev/null 2>&1; then
  echo "authoritative coverage base filter must exclude the proof-heavy slice" >&2
  exit 1
fi

echo "PASS test_run_authoritative_coverage_lane"
