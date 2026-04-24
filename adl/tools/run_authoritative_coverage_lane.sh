#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
PRINT_PLAN=false

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_authoritative_coverage_lane.sh [--print-plan]

Run the authoritative coverage lane in two bounded phases:
1. always-on authoritative coverage for the base workspace
2. proof-heavy authoritative slices that need opt-in features or dominate lane variance

The two cargo-llvm-cov nextest runs accumulate into one final coverage report.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --print-plan)
      PRINT_PLAN=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

build_expression() {
  python3 - "$@" <<'PY'
import sys

tokens = [token for token in sys.argv[1:] if token]
if not tokens:
    raise SystemExit(1)
print(" or ".join(f"test({token})" for token in tokens))
PY
}

readonly PROOF_FEATURES="slow-proof-tests,slow-finish-tests"
readonly -a PROOF_TOKENS=(
  "access_control"
  "challenge"
  "observatory_flagship"
  "private_state_observatory"
  "runtime_v2_v0903_demo_stdout_uses_repo_relative_output_paths"
  "runtime_v2_csm_invalid_action_rejection"
  "runtime_v2_csm_observatory"
  "runtime_v2_csm_quarantine"
  "runtime_v2_csm_recovery_eligibility"
  "runtime_v2_csm_hardening"
  "runtime_v2_csm_wake_continuity"
  "runtime_v2_private_state_sanctuary"
  "runtime_v2_private_state_envelope"
  "runtime_v2_private_state_write_to_root_materializes_authority_and_projection"
  "runtime_v2_private_state_anti_equivocation_write_to_root_materializes_fixtures"
  "runtime_v2_private_state_witness_write_to_root_materializes_fixtures"
  "runtime_v2_private_state_sealing_write_to_root_materializes_fixtures"
  "runtime_v2_feature_proof_coverage_runs_runtime_v2_cli_regression_matrix"
  "real_pr_finish_creates_draft_pr_and_commits_branch_changes"
  "real_pr_finish_rejects_main_and_reports_no_pr_when_only_local_bundle_sync_changes_exist"
  "real_pr_finish_rejects_staged_gitignore_changes_without_allow_flag"
)

proof_expr="$(build_expression "${PROOF_TOKENS[@]}")"
base_expr="not (${proof_expr})"

if [ "$PRINT_PLAN" = true ]; then
  printf 'phase=always_on_authoritative\n'
  printf 'base_filter=%s\n' "$base_expr"
  printf 'phase=proof_heavy_authoritative\n'
  printf 'proof_features=%s\n' "$PROOF_FEATURES"
  printf 'proof_filter=%s\n' "$proof_expr"
  exit 0
fi

cd "$ADL_DIR"

echo "Authoritative coverage phase: always_on_authoritative"
echo "Base filter: $base_expr"
cargo llvm-cov nextest \
  --workspace \
  --status-level all \
  --final-status-level slow \
  --no-report \
  -E "$base_expr"

echo "Authoritative coverage phase: proof_heavy_authoritative"
echo "Proof features: $PROOF_FEATURES"
echo "Proof filter: $proof_expr"
cargo llvm-cov nextest \
  --workspace \
  --features "$PROOF_FEATURES" \
  --status-level all \
  --final-status-level slow \
  --no-report \
  -E "$proof_expr"

cargo llvm-cov report --json --summary-only --output-path coverage-summary.json
