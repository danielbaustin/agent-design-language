#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
POLICY="$ROOT_DIR/adl/tools/ci_path_policy.sh"

assert_has() {
  local haystack="$1"
  local needle="$2"
  if ! grep -Fqx "$needle" <<<"$haystack"; then
    echo "expected path-policy output to contain: $needle" >&2
    echo "actual output:" >&2
    echo "$haystack" >&2
    exit 1
  fi
}

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

(
  cd "$tmp_dir"
  git init -q
  git config user.email "ci-path-policy@example.invalid"
  git config user.name "CI Path Policy Test"

  mkdir -p adl/src docs
  printf 'pub fn baseline() -> bool { true }\n' > adl/src/lib.rs
  printf '# adl baseline\n' > adl/README.md
  mkdir -p .github/workflows
  cat > adl/Cargo.toml <<'EOF'
[package]
name = "adl"
version = "0.90.3"
edition = "2021"
EOF
  cat > adl/Cargo.lock <<'EOF'
version = 4

[[package]]
name = "adl"
version = "0.90.3"
EOF
  printf '# baseline\n' > docs/readme.md
  cat > .github/workflows/ci.yaml <<'EOF'
jobs:
  adl-coverage:
    steps:
      - name: Determine PR fast coverage filters
        id: coverage-impact
        if: github.event_name == 'pull_request' && steps.path-policy.outputs.coverage_required == 'true' && steps.path-policy.outputs.full_coverage_required != 'true'
        run: |
          set -euo pipefail
          bash adl/tools/check_coverage_impact.sh \
            --base "${{ github.event.pull_request.base.sha }}" \
            --head "${{ github.event.pull_request.head.sha }}" \
            --print-risk-nextest-expression > adl/coverage-impact-filter-expression.txt
          if test -s adl/coverage-impact-filter-expression.txt; then
            echo "needs_fast_summary=true" >> "$GITHUB_OUTPUT"
            echo "filter_expression=$(cat adl/coverage-impact-filter-expression.txt)" >> "$GITHUB_OUTPUT"
          else
            echo "needs_fast_summary=false" >> "$GITHUB_OUTPUT"
            echo "filter_expression=" >> "$GITHUB_OUTPUT"
          fi
        working-directory: .
      - name: PR fast coverage summary (json)
        if: github.event_name == 'pull_request' && steps.path-policy.outputs.coverage_required == 'true' && steps.path-policy.outputs.full_coverage_required != 'true' && steps.coverage-impact.outputs.needs_fast_summary == 'true'
        run: |
          rm -rf target/debug target/llvm-cov-target
          COVERAGE_BUILD_ROOT="${RUNNER_TEMP:-/tmp}/adl-pr-fast-coverage"
          mkdir -p "$COVERAGE_BUILD_ROOT/target" "$COVERAGE_BUILD_ROOT/llvm-cov-target"
          export CARGO_TARGET_DIR="$COVERAGE_BUILD_ROOT/target"
          export CARGO_LLVM_COV_TARGET_DIR="$COVERAGE_BUILD_ROOT/llvm-cov-target"
          CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E "${{ steps.coverage-impact.outputs.filter_expression }}"
          cargo llvm-cov report --json --summary-only --output-path coverage-summary.json
        working-directory: .
      - name: Coverage run and summary (json)
        run: bash tools/run_authoritative_coverage_lane.sh
      - name: Coverage summary (text)
        run: cargo llvm-cov report --summary-only | tee coverage-summary.txt
      - name: Upload coverage artifact
        with:
          path: |
            adl/coverage-summary.txt
      - name: Upload coverage to Codecov
        with:
          files: adl/lcov.info
          flags: adl
EOF
  cat > .github/workflows/nightly-coverage-ratchet.yaml <<'EOF'
name: nightly-coverage-ratchet

on:
  workflow_dispatch:
EOF
  git add .
  git commit -q -m baseline
  base_sha="$(git rev-parse HEAD)"

  printf '\nmore docs\n' >> docs/readme.md
  git add docs/readme.md
  git commit -q -m docs-change
  docs_head="$(git rev-parse HEAD)"

  docs_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$docs_head" --ref "refs/pull/1/merge")"
  assert_has "$docs_output" "rust_required=false"
  assert_has "$docs_output" "coverage_required=false"
  assert_has "$docs_output" "full_coverage_required=false"
  assert_has "$docs_output" "demo_smoke_required=false"
  assert_has "$docs_output" "v0913_proof_required=false"
  assert_has "$docs_output" "release_version_only=false"
  assert_has "$docs_output" "ci_contracts_required=false"
  assert_has "$docs_output" "slow_proof_contracts_required=false"
  assert_has "$docs_output" "coverage_lane=skip"
  assert_has "$docs_output" "coverage_authority=not_required"
  assert_has "$docs_output" "proof_validation_scope=not_required"
  assert_has "$docs_output" "validation_profile_selected=docs_diff_check_profile"
  assert_has "$docs_output" "validation_profile_status=ready_to_run"
  assert_has "$docs_output" "validation_profile_escalation_required=false"
  assert_has "$docs_output" "validation_profile_run_lanes=docs_diff_check"
  assert_has "$docs_output" "validation_profile_primary_reason=docs_only_surface_requires_diff_hygiene"

  git checkout -q -b release-version-only "$base_sha"
  python3 - <<'PY'
from pathlib import Path

manifest = Path("adl/Cargo.toml")
manifest.write_text(manifest.read_text().replace('version = "0.90.3"', 'version = "0.90.4"', 1))

lock = Path("adl/Cargo.lock")
lock.write_text(lock.read_text().replace('version = "0.90.3"', 'version = "0.90.4"', 1))

doc = Path("docs/readme.md")
doc.write_text(doc.read_text() + "\nrelease-truth update\n")
PY
  git add adl/Cargo.toml adl/Cargo.lock docs/readme.md
  git commit -q -m release-version-only
  release_version_head="$(git rev-parse HEAD)"

  release_version_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$release_version_head" --ref "refs/pull/1/merge")"
  assert_has "$release_version_output" "rust_required=false"
  assert_has "$release_version_output" "coverage_required=false"
  assert_has "$release_version_output" "full_coverage_required=false"
  assert_has "$release_version_output" "demo_smoke_required=false"
  assert_has "$release_version_output" "v0913_proof_required=false"
  assert_has "$release_version_output" "release_version_only=true"
  assert_has "$release_version_output" "ci_contracts_required=false"
  assert_has "$release_version_output" "coverage_lane=skip"
  assert_has "$release_version_output" "coverage_authority=not_required"
  assert_has "$release_version_output" "proof_validation_scope=not_required"
  assert_has "$release_version_output" "reason=release_version_only_cargo_surface_change_runs_lightweight_validation"

  git checkout -q -b release-version-only-with-adl-readme "$base_sha"
  python3 - <<'PY'
from pathlib import Path

manifest = Path("adl/Cargo.toml")
manifest.write_text(manifest.read_text().replace('version = "0.90.3"', 'version = "0.90.4"', 1))

lock = Path("adl/Cargo.lock")
lock.write_text(lock.read_text().replace('version = "0.90.3"', 'version = "0.90.4"', 1))

Path("README.md").write_text("# repo release truth\n")
Path("CHANGELOG.md").write_text("# changelog\n\n- v0.90.4\n")
Path("adl/README.md").write_text("# adl release truth\n")
doc = Path("docs/readme.md")
doc.write_text(doc.read_text() + "\nrelease-truth update with adl readme\n")
PY
  git add README.md CHANGELOG.md adl/README.md adl/Cargo.toml adl/Cargo.lock docs/readme.md
  git commit -q -m release-version-only-with-adl-readme
  release_version_adl_readme_head="$(git rev-parse HEAD)"

  release_version_adl_readme_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$release_version_adl_readme_head" --ref "refs/pull/1/merge")"
  assert_has "$release_version_adl_readme_output" "rust_required=false"
  assert_has "$release_version_adl_readme_output" "coverage_required=false"
  assert_has "$release_version_adl_readme_output" "full_coverage_required=false"
  assert_has "$release_version_adl_readme_output" "demo_smoke_required=false"
  assert_has "$release_version_adl_readme_output" "v0913_proof_required=false"
  assert_has "$release_version_adl_readme_output" "release_version_only=true"
  assert_has "$release_version_adl_readme_output" "ci_contracts_required=false"
  assert_has "$release_version_adl_readme_output" "coverage_lane=skip"
  assert_has "$release_version_adl_readme_output" "coverage_authority=not_required"
  assert_has "$release_version_adl_readme_output" "proof_validation_scope=not_required"
  assert_has "$release_version_adl_readme_output" "reason=release_version_only_cargo_surface_change_runs_lightweight_validation"

  git checkout -q -b cargo-structural-change "$base_sha"
  cat >> adl/Cargo.toml <<'EOF'

[features]
example = []
EOF
  git add adl/Cargo.toml
  git commit -q -m cargo-structural-change
  cargo_structural_head="$(git rev-parse HEAD)"

  cargo_structural_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$cargo_structural_head" --ref "refs/pull/1/merge")"
  assert_has "$cargo_structural_output" "rust_required=true"
  assert_has "$cargo_structural_output" "coverage_required=true"
  assert_has "$cargo_structural_output" "full_coverage_required=false"
  assert_has "$cargo_structural_output" "demo_smoke_required=true"
  assert_has "$cargo_structural_output" "v0913_proof_required=false"
  assert_has "$cargo_structural_output" "release_version_only=false"
  assert_has "$cargo_structural_output" "ci_contracts_required=true"
  assert_has "$cargo_structural_output" "slow_proof_contracts_required=false"
  assert_has "$cargo_structural_output" "fail_closed=false"
  assert_has "$cargo_structural_output" "coverage_lane=pr_fast"
  assert_has "$cargo_structural_output" "coverage_authority=pr_changed_surface"
  assert_has "$cargo_structural_output" "proof_validation_scope=not_required"
  assert_has "$cargo_structural_output" "reason=manifest_only_rust_wave_runs_focused_nextest"
  assert_has "$cargo_structural_output" "validation_profile_selected=rust_pr_fast_profile"
  assert_has "$cargo_structural_output" "validation_profile_status=ready_to_run"
  assert_has "$cargo_structural_output" "validation_profile_escalation_required=false"
  assert_has "$cargo_structural_output" "validation_profile_run_lanes=rust_pr_fast"

  git checkout -q -b runtime-change "$base_sha"
  printf 'pub fn added_runtime() -> bool { true }\n' >> adl/src/lib.rs
  git add adl/src/lib.rs
  git commit -q -m runtime-change
  runtime_head="$(git rev-parse HEAD)"

  runtime_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$runtime_head" --ref "refs/pull/1/merge")"
  assert_has "$runtime_output" "rust_required=true"
  assert_has "$runtime_output" "coverage_required=true"
  assert_has "$runtime_output" "full_coverage_required=true"
  assert_has "$runtime_output" "demo_smoke_required=true"
  assert_has "$runtime_output" "v0913_proof_required=false"
  assert_has "$runtime_output" "release_version_only=false"
  assert_has "$runtime_output" "ci_contracts_required=true"
  assert_has "$runtime_output" "fail_closed=true"
  assert_has "$runtime_output" "coverage_lane=authoritative_full"
  assert_has "$runtime_output" "coverage_authority=fail_closed"
  assert_has "$runtime_output" "proof_validation_scope=not_required"
  assert_has "$runtime_output" "reason=validation_manager_escalation_requires_authoritative_full_coverage"
  assert_has "$runtime_output" "validation_profile_selected=escalated_1_lane_profile"
  assert_has "$runtime_output" "validation_profile_status=escalation_required"
  assert_has "$runtime_output" "validation_profile_escalation_required=true"
  assert_has "$runtime_output" "validation_profile_run_lanes="
  assert_has "$runtime_output" "validation_profile_primary_reason=no_relevant_rust_surface_detected_for_fast_lane"
  assert_has "$runtime_output" "validation_profile_escalation_lanes=rust_pr_fast"

  git checkout -q -b rust-test-manifest-change "$base_sha"
  mkdir -p adl/src/runtime_v2
  printf 'mod slow_proof_contract;\n' > adl/src/runtime_v2/tests.rs
  git add adl/src/runtime_v2/tests.rs
  git commit -q -m rust-test-manifest-change
  rust_test_manifest_head="$(git rev-parse HEAD)"

  rust_test_manifest_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$rust_test_manifest_head" --ref "refs/pull/1/merge")"
  assert_has "$rust_test_manifest_output" "rust_required=true"
  assert_has "$rust_test_manifest_output" "coverage_required=true"
  assert_has "$rust_test_manifest_output" "full_coverage_required=false"
  assert_has "$rust_test_manifest_output" "demo_smoke_required=true"
  assert_has "$rust_test_manifest_output" "v0913_proof_required=false"
  assert_has "$rust_test_manifest_output" "release_version_only=false"
  assert_has "$rust_test_manifest_output" "ci_contracts_required=true"
  assert_has "$rust_test_manifest_output" "coverage_lane=pr_fast"
  assert_has "$rust_test_manifest_output" "coverage_authority=pr_changed_surface"
  assert_has "$rust_test_manifest_output" "reason=bounded_rust_surface_runs_focused_nextest"

  git checkout -q -b sprint-conductor-surface "$base_sha"
  mkdir -p adl/tools/skills/sprint-conductor/scripts
  printf '# sprint conductor skill\n' > adl/tools/skills/sprint-conductor/SKILL.md
  printf 'print(\"goal metrics\")\n' > adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py
  printf '#!/usr/bin/env bash\nexit 0\n' > adl/tools/test_sprint_conductor_helpers.sh
  printf '#!/usr/bin/env bash\nexit 0\n' > adl/tools/test_install_adl_operational_skills.sh
  printf '\nsprint conductor note\n' >> docs/readme.md
  git add adl/tools/skills/sprint-conductor/SKILL.md \
    adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py \
    adl/tools/test_sprint_conductor_helpers.sh \
    adl/tools/test_install_adl_operational_skills.sh \
    docs/readme.md
  git commit -q -m sprint-conductor-surface
  sprint_conductor_head="$(git rev-parse HEAD)"

  sprint_conductor_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$sprint_conductor_head" --ref "refs/pull/1/merge")"
  assert_has "$sprint_conductor_output" "rust_required=false"
  assert_has "$sprint_conductor_output" "coverage_required=false"
  assert_has "$sprint_conductor_output" "full_coverage_required=false"
  assert_has "$sprint_conductor_output" "demo_smoke_required=false"
  assert_has "$sprint_conductor_output" "v0913_proof_required=false"
  assert_has "$sprint_conductor_output" "release_version_only=false"
  assert_has "$sprint_conductor_output" "ci_contracts_required=false"
  assert_has "$sprint_conductor_output" "fail_closed=false"
  assert_has "$sprint_conductor_output" "coverage_lane=skip"
  assert_has "$sprint_conductor_output" "coverage_authority=not_required"
  assert_has "$sprint_conductor_output" "reason=sprint_conductor_surface_requires_helper_contract_checks"
  assert_has "$sprint_conductor_output" "validation_profile_status=ready_to_run"
  assert_has "$sprint_conductor_output" "validation_profile_escalation_required=false"
  assert_has "$sprint_conductor_output" "validation_profile_run_lanes=docs_diff_check,sprint_conductor_contracts"

  git checkout -q -b sprint-conductor-only "$base_sha"
  mkdir -p adl/tools/skills/sprint-conductor/scripts
  printf '# sprint conductor skill\n' > adl/tools/skills/sprint-conductor/SKILL.md
  printf 'print(\"goal metrics\")\n' > adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py
  printf '#!/usr/bin/env bash\nexit 0\n' > adl/tools/test_sprint_conductor_helpers.sh
  printf '#!/usr/bin/env bash\nexit 0\n' > adl/tools/test_install_adl_operational_skills.sh
  git add adl/tools/skills/sprint-conductor/SKILL.md \
    adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py \
    adl/tools/test_sprint_conductor_helpers.sh \
    adl/tools/test_install_adl_operational_skills.sh
  git commit -q -m sprint-conductor-only
  sprint_conductor_only_head="$(git rev-parse HEAD)"

  sprint_conductor_only_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$sprint_conductor_only_head" --ref "refs/pull/1/merge")"
  assert_has "$sprint_conductor_only_output" "rust_required=false"
  assert_has "$sprint_conductor_only_output" "coverage_required=false"
  assert_has "$sprint_conductor_only_output" "full_coverage_required=false"
  assert_has "$sprint_conductor_only_output" "demo_smoke_required=false"
  assert_has "$sprint_conductor_only_output" "v0913_proof_required=false"
  assert_has "$sprint_conductor_only_output" "release_version_only=false"
  assert_has "$sprint_conductor_only_output" "ci_contracts_required=false"
  assert_has "$sprint_conductor_only_output" "fail_closed=false"
  assert_has "$sprint_conductor_only_output" "coverage_lane=skip"
  assert_has "$sprint_conductor_only_output" "coverage_authority=not_required"
  assert_has "$sprint_conductor_only_output" "reason=sprint_conductor_surface_requires_helper_contract_checks"
  assert_has "$sprint_conductor_only_output" "validation_profile_status=ready_to_run"
  assert_has "$sprint_conductor_only_output" "validation_profile_escalation_required=false"
  assert_has "$sprint_conductor_only_output" "validation_profile_run_lanes=sprint_conductor_contracts"

  git checkout -q -b classifier-followup "$base_sha"
  mkdir -p adl/tools/skills/sprint-conductor/scripts adl/config adl/tools
  printf '# sprint conductor skill\n' > adl/tools/skills/sprint-conductor/SKILL.md
  printf 'print(\"goal metrics\")\n' > adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py
  printf '#!/usr/bin/env bash\nexit 0\n' > adl/tools/test_sprint_conductor_helpers.sh
  printf '#!/usr/bin/env bash\nexit 0\n' > adl/tools/test_install_adl_operational_skills.sh
  printf '#!/usr/bin/env bash\nprintf ok\\n' > adl/tools/ci_path_policy.sh
  printf '#!/usr/bin/env bash\nprintf ok\\n' > adl/tools/test_ci_path_policy.sh
  printf '{\"schema_version\":\"adl.validation_lane_selector.v1\",\"surface_defaults\":{},\"lanes\":[],\"special_surfaces\":{},\"manager_guardrails\":{\"docs_only_forbidden_lane_ids\":[\"rust_pr_fast\"],\"pr_fast\":{\"max_rust_surface_count\":4,\"max_filter_token_count\":4,\"max_family_token_count\":3,\"blocked_modes\":[\"full\",\"contract_only\"]}},\"release_gate_hints\":[],\"rust_path_hints\":[]}\n' > adl/config/validation_lane_selector.v0.91.6.json
  printf '#!/usr/bin/env bash\nprintf ok\\n' > adl/tools/test_validation_manager.sh
  printf '\nclassifier followup note\n' >> docs/readme.md
  git add adl/config/validation_lane_selector.v0.91.6.json \
    adl/tools/ci_path_policy.sh \
    adl/tools/test_ci_path_policy.sh \
    adl/tools/test_validation_manager.sh \
    adl/tools/skills/sprint-conductor/SKILL.md \
    adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py \
    adl/tools/test_sprint_conductor_helpers.sh \
    adl/tools/test_install_adl_operational_skills.sh \
    docs/readme.md
  git commit -q -m classifier-followup
  classifier_followup_head="$(git rev-parse HEAD)"

  classifier_followup_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$classifier_followup_head" --ref "refs/pull/1/merge")"
  assert_has "$classifier_followup_output" "rust_required=false"
  assert_has "$classifier_followup_output" "coverage_required=false"
  assert_has "$classifier_followup_output" "full_coverage_required=false"
  assert_has "$classifier_followup_output" "demo_smoke_required=false"
  assert_has "$classifier_followup_output" "v0913_proof_required=false"
  assert_has "$classifier_followup_output" "release_version_only=false"
  assert_has "$classifier_followup_output" "ci_contracts_required=true"
  assert_has "$classifier_followup_output" "fail_closed=false"
  assert_has "$classifier_followup_output" "coverage_lane=skip"
  assert_has "$classifier_followup_output" "coverage_authority=not_required"
  assert_has "$classifier_followup_output" "reason=ci_policy_surface_requires_path_policy_contract_checks"
  assert_has "$classifier_followup_output" "validation_profile_status=ready_to_run"
  assert_has "$classifier_followup_output" "validation_profile_escalation_required=false"

  git checkout -q -b new-runtime-file "$base_sha"
  printf 'pub fn contract_schema() -> bool { true }\n' > adl/src/contract_schema.rs
  git add adl/src/contract_schema.rs
  git commit -q -m new-runtime-file
  new_runtime_file_head="$(git rev-parse HEAD)"

  new_runtime_file_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$new_runtime_file_head" --ref "refs/pull/1/merge")"
  assert_has "$new_runtime_file_output" "rust_required=true"
  assert_has "$new_runtime_file_output" "coverage_required=true"
  assert_has "$new_runtime_file_output" "full_coverage_required=false"
  assert_has "$new_runtime_file_output" "demo_smoke_required=true"
  assert_has "$new_runtime_file_output" "v0913_proof_required=false"
  assert_has "$new_runtime_file_output" "release_version_only=false"
  assert_has "$new_runtime_file_output" "ci_contracts_required=true"
  assert_has "$new_runtime_file_output" "fail_closed=false"
  assert_has "$new_runtime_file_output" "coverage_lane=pr_fast"
  assert_has "$new_runtime_file_output" "coverage_authority=pr_changed_surface"
  assert_has "$new_runtime_file_output" "proof_validation_scope=not_required"
  assert_has "$new_runtime_file_output" "reason=bounded_rust_surface_runs_focused_nextest"
  assert_has "$new_runtime_file_output" "validation_profile_selected=rust_pr_fast_profile"
  assert_has "$new_runtime_file_output" "validation_profile_status=ready_to_run"
  assert_has "$new_runtime_file_output" "validation_profile_escalation_required=false"
  assert_has "$new_runtime_file_output" "validation_profile_primary_reason=bounded_rust_surface_runs_focused_nextest"
  assert_has "$new_runtime_file_output" "validation_profile_escalation_lanes="

  git checkout -q -b finish-control-plane "$base_sha"
  mkdir -p adl/src/cli/pr_cmd adl/src/cli/tests/pr_cmd_inline/finish docs/milestones/v0.90/milestone_compression
  printf 'pub fn finish_support() -> bool { true }\n' > adl/src/cli/pr_cmd/finish_support.rs
  printf 'use super::*;\n#[test]\nfn finish_path_is_stable() {}\n' > adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs
  printf '# workflow\n' > docs/default_workflow.md
  git add adl/src/cli/pr_cmd/finish_support.rs adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs docs/default_workflow.md
  git commit -q -m finish-control-plane
  finish_control_plane_head="$(git rev-parse HEAD)"

  finish_control_plane_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$finish_control_plane_head")"
  assert_has "$finish_control_plane_output" "rust_required=true"
  assert_has "$finish_control_plane_output" "coverage_required=false"
  assert_has "$finish_control_plane_output" "full_coverage_required=false"
  assert_has "$finish_control_plane_output" "demo_smoke_required=false"
  assert_has "$finish_control_plane_output" "v0913_proof_required=false"
  assert_has "$finish_control_plane_output" "release_version_only=false"
  assert_has "$finish_control_plane_output" "ci_contracts_required=true"
  assert_has "$finish_control_plane_output" "slow_proof_contracts_required=false"
  assert_has "$finish_control_plane_output" "reason=publication_control_plane_change_runs_focused_rust_validation"

  git checkout -q -b policy-surface-change "$base_sha"
  mkdir -p adl/tools
  printf '#!/usr/bin/env bash\nprintf policy\n' > adl/tools/enforce_coverage_gates.sh
  git add adl/tools/enforce_coverage_gates.sh
  git commit -q -m policy-surface-change
  policy_surface_head="$(git rev-parse HEAD)"

  policy_surface_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$policy_surface_head" --ref "refs/pull/1/merge")"
  assert_has "$policy_surface_output" "rust_required=false"
  assert_has "$policy_surface_output" "coverage_required=false"
  assert_has "$policy_surface_output" "full_coverage_required=false"
  assert_has "$policy_surface_output" "demo_smoke_required=false"
  assert_has "$policy_surface_output" "v0913_proof_required=false"
  assert_has "$policy_surface_output" "release_version_only=false"
  assert_has "$policy_surface_output" "ci_contracts_required=true"
  assert_has "$policy_surface_output" "slow_proof_contracts_required=false"
  assert_has "$policy_surface_output" "coverage_lane=skip"
  assert_has "$policy_surface_output" "coverage_authority=not_required"
  assert_has "$policy_surface_output" "proof_validation_scope=not_required"
  assert_has "$policy_surface_output" "reason=coverage_policy_surface_tooling_change_runs_contract_validation"
  assert_has "$policy_surface_output" "validation_profile_selected=validation_none"
  assert_has "$policy_surface_output" "validation_profile_status=escalation_required"
  assert_has "$policy_surface_output" "validation_profile_escalation_required=true"
  assert_has "$policy_surface_output" "validation_profile_run_lanes="
  assert_has "$policy_surface_output" "validation_profile_primary_reason=selector left changed paths without validation-lane coverage"
  assert_has "$policy_surface_output" "validation_profile_escalation_lanes=unmapped_change_surface"

  git checkout -q -b policy-surface-plus-demo-tooling "$base_sha"
  mkdir -p adl/tools docs/milestones/v0.91.4/review/demo_showcase
  printf '#!/usr/bin/env bash\nprintf policy\n' > adl/tools/enforce_coverage_gates.sh
  printf '#!/usr/bin/env bash\nprintf demo\n' > adl/tools/demo_v0914_complete_issue.sh
  printf '# demo note\n' > docs/milestones/v0.91.4/review/demo_showcase/DEMO_NOTE.md
  git add adl/tools/enforce_coverage_gates.sh adl/tools/demo_v0914_complete_issue.sh docs/milestones/v0.91.4/review/demo_showcase/DEMO_NOTE.md
  git commit -q -m policy-surface-plus-demo-tooling
  policy_surface_plus_demo_head="$(git rev-parse HEAD)"

  policy_surface_plus_demo_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$policy_surface_plus_demo_head" --ref "refs/pull/1/merge")"
  assert_has "$policy_surface_plus_demo_output" "rust_required=false"
  assert_has "$policy_surface_plus_demo_output" "coverage_required=false"
  assert_has "$policy_surface_plus_demo_output" "full_coverage_required=false"
  assert_has "$policy_surface_plus_demo_output" "demo_smoke_required=true"
  assert_has "$policy_surface_plus_demo_output" "ci_contracts_required=true"
  assert_has "$policy_surface_plus_demo_output" "coverage_lane=skip"
  assert_has "$policy_surface_plus_demo_output" "coverage_authority=not_required"
  assert_has "$policy_surface_plus_demo_output" "reason=coverage_policy_surface_tooling_change_runs_contract_validation"

  git checkout -q -b pvf-runner-policy-surface "$base_sha"
  mkdir -p adl/tools
  printf '#!/usr/bin/env bash\nprintf pvf-runner\n' > adl/tools/run_pvf_validation_lane.sh
  git add adl/tools/run_pvf_validation_lane.sh
  git commit -q -m pvf-runner-policy-surface
  pvf_runner_policy_head="$(git rev-parse HEAD)"

  pvf_runner_policy_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$pvf_runner_policy_head" --ref "refs/pull/1/merge")"
  assert_has "$pvf_runner_policy_output" "ci_contracts_required=true"

  git checkout -q -b workflow-reporting-only-change "$base_sha"
  python3 - <<'PY'
from pathlib import Path

path = Path(".github/workflows/ci.yaml")
path.write_text(path.read_text().replace("            adl/coverage-summary.txt\n", "            adl/coverage-summary.txt\n            adl/coverage-summary.json\n", 1))
PY
  git add .github/workflows/ci.yaml
  git commit -q -m workflow-reporting-only-change
  workflow_reporting_head="$(git rev-parse HEAD)"

  workflow_reporting_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$workflow_reporting_head" --ref "refs/pull/1/merge")"
  assert_has "$workflow_reporting_output" "rust_required=false"
  assert_has "$workflow_reporting_output" "coverage_required=false"
  assert_has "$workflow_reporting_output" "full_coverage_required=false"
  assert_has "$workflow_reporting_output" "demo_smoke_required=false"
  assert_has "$workflow_reporting_output" "v0913_proof_required=false"
  assert_has "$workflow_reporting_output" "release_version_only=false"
  assert_has "$workflow_reporting_output" "ci_contracts_required=true"
  assert_has "$workflow_reporting_output" "coverage_lane=skip"
  assert_has "$workflow_reporting_output" "coverage_authority=not_required"
  assert_has "$workflow_reporting_output" "proof_validation_scope=not_required"
  assert_has "$workflow_reporting_output" "reason=coverage_reporting_workflow_change_skips_authoritative_coverage"

  git checkout -q -b workflow-pvf-slow-proof-change "$base_sha"
  python3 - <<'PY'
from pathlib import Path

path = Path(".github/workflows/ci.yaml")
text = path.read_text()
text = text.replace(
    "jobs:\n",
    'on:\n  workflow_dispatch:\n  schedule:\n    - cron: "17 10 * * *"\n\njobs:\n',
    1,
)
text = text.replace(
    "      - name: Coverage run and summary (json)\n        run: bash tools/run_authoritative_coverage_lane.sh\n",
    "      - name: slow-proof lane contract\n        if: steps.path-policy.outputs.slow_proof_contracts_required == 'true'\n        run: bash adl/tools/test_slow_proof_lane_contract.sh\n      - name: Coverage run and summary (json)\n        run: bash tools/run_authoritative_coverage_lane.sh\n",
    1,
)
text += """
  adl-slow-proof:
    if: github.event_name == 'push' || github.event_name == 'schedule' || github.event_name == 'workflow_dispatch'
    permissions:
      contents: read
    strategy:
      fail-fast: false
      matrix:
        shard: [1, 2, 3, 4]
    defaults:
      run:
        working-directory: adl
    steps:
      - uses: actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5
        with:
          fetch-depth: 0
      - name: Install cargo-nextest
        uses: taiki-e/install-action@e5c52b603cc5f5e9b52b6a43afad8e9fe0527090
        with:
          tool: nextest
      - name: Slow proof shard
        run: cargo nextest run --features slow-proof-tests --partition count:${{ matrix.shard }}/4 --status-level all --final-status-level slow
"""
path.write_text(text)
PY
  git add .github/workflows/ci.yaml
  git commit -q -m workflow-pvf-slow-proof-change
  workflow_pvf_slow_proof_head="$(git rev-parse HEAD)"

  workflow_pvf_slow_proof_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$workflow_pvf_slow_proof_head" --ref "refs/pull/1/merge")"
  assert_has "$workflow_pvf_slow_proof_output" "rust_required=true"
  assert_has "$workflow_pvf_slow_proof_output" "coverage_required=false"
  assert_has "$workflow_pvf_slow_proof_output" "full_coverage_required=false"
  assert_has "$workflow_pvf_slow_proof_output" "demo_smoke_required=false"
  assert_has "$workflow_pvf_slow_proof_output" "v0913_proof_required=false"
  assert_has "$workflow_pvf_slow_proof_output" "ci_contracts_required=true"
  assert_has "$workflow_pvf_slow_proof_output" "slow_proof_contracts_required=true"
  assert_has "$workflow_pvf_slow_proof_output" "coverage_lane=skip"
  assert_has "$workflow_pvf_slow_proof_output" "coverage_authority=not_required"
  assert_has "$workflow_pvf_slow_proof_output" "reason=pvf_slow_proof_change_runs_contract_validation"

  git checkout -q -b workflow-authoritative-policy-change "$base_sha"
  python3 - <<'PY'
from pathlib import Path

path = Path(".github/workflows/ci.yaml")
path.write_text(path.read_text().replace("run: bash tools/run_authoritative_coverage_lane.sh", "run: bash tools/run_authoritative_coverage_lane.sh --strict", 1))
PY
  git add .github/workflows/ci.yaml
  git commit -q -m workflow-authoritative-policy-change
  workflow_policy_head="$(git rev-parse HEAD)"

  workflow_policy_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$workflow_policy_head" --ref "refs/pull/1/merge")"
  assert_has "$workflow_policy_output" "rust_required=false"
  assert_has "$workflow_policy_output" "coverage_required=false"
  assert_has "$workflow_policy_output" "full_coverage_required=false"
  assert_has "$workflow_policy_output" "demo_smoke_required=false"
  assert_has "$workflow_policy_output" "v0913_proof_required=false"
  assert_has "$workflow_policy_output" "ci_contracts_required=true"
  assert_has "$workflow_policy_output" "coverage_lane=skip"
  assert_has "$workflow_policy_output" "coverage_authority=not_required"
  assert_has "$workflow_policy_output" "proof_validation_scope=not_required"

  git checkout -q -b v0913-proof-surface "$base_sha"
  mkdir -p docs/milestones/v0.91.3/review/merge_readiness
  printf '\nproof note\n' >> docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md
  git add docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md
  git commit -q -m v0913-proof-surface
  v0913_proof_head="$(git rev-parse HEAD)"

  v0913_proof_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$v0913_proof_head" --ref "refs/pull/1/merge")"
  assert_has "$v0913_proof_output" "rust_required=true"
  assert_has "$v0913_proof_output" "coverage_required=false"
  assert_has "$v0913_proof_output" "full_coverage_required=false"
  assert_has "$v0913_proof_output" "demo_smoke_required=false"
  assert_has "$v0913_proof_output" "v0913_proof_required=true"
  assert_has "$v0913_proof_output" "ci_contracts_required=true"
  assert_has "$v0913_proof_output" "proof_validation_scope=v0_91_3"
  assert_has "$v0913_proof_output" "reason=v0913_proof_surface_change_runs_targeted_packet_validation"
  assert_has "$workflow_policy_output" "reason=coverage_policy_surface_tooling_change_runs_contract_validation"

  git checkout -q -b v0913-feature-proof-surface "$base_sha"
  mkdir -p docs/milestones/v0.91.3/features docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo
  printf '# feature proof\n' > docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md
  printf '# tracked bundle\n' > docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/README.md
  git add docs/milestones/v0.91.3/features/CARD_LIFECYCLE_INTEGRATION.md docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/README.md
  git commit -q -m v0913-feature-proof-surface
  v0913_feature_proof_head="$(git rev-parse HEAD)"

  v0913_feature_proof_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$v0913_feature_proof_head" --ref "refs/pull/1/merge")"
  assert_has "$v0913_feature_proof_output" "rust_required=true"
  assert_has "$v0913_feature_proof_output" "coverage_required=false"
  assert_has "$v0913_feature_proof_output" "full_coverage_required=false"
  assert_has "$v0913_feature_proof_output" "demo_smoke_required=false"
  assert_has "$v0913_feature_proof_output" "v0913_proof_required=true"
  assert_has "$v0913_feature_proof_output" "ci_contracts_required=true"
  assert_has "$v0913_feature_proof_output" "proof_validation_scope=v0_91_3"
  assert_has "$v0913_feature_proof_output" "reason=v0913_proof_surface_change_runs_targeted_packet_validation"

  git checkout -q -b v0913-proof-surface-deletion "$base_sha"
  mkdir -p docs/milestones/v0.91.3/review/demo_coverage
  printf '# proof packet\n' > docs/milestones/v0.91.3/review/demo_coverage/DEMO_COVERAGE_PACKET_v0.91.3.md
  git add docs/milestones/v0.91.3/review/demo_coverage/DEMO_COVERAGE_PACKET_v0.91.3.md
  git commit -q -m seed-v0913-proof-surface-deletion
  deletion_base="$(git rev-parse HEAD)"
  rm docs/milestones/v0.91.3/review/demo_coverage/DEMO_COVERAGE_PACKET_v0.91.3.md
  git add -A docs/milestones/v0.91.3/review/demo_coverage/DEMO_COVERAGE_PACKET_v0.91.3.md
  git commit -q -m delete-v0913-proof-surface
  deletion_head="$(git rev-parse HEAD)"

  deletion_output="$("$POLICY" --event-name pull_request --base "$deletion_base" --head "$deletion_head" --ref "refs/pull/1/merge")"
  assert_has "$deletion_output" "rust_required=true"
  assert_has "$deletion_output" "coverage_required=false"
  assert_has "$deletion_output" "full_coverage_required=false"
  assert_has "$deletion_output" "demo_smoke_required=false"
  assert_has "$deletion_output" "v0913_proof_required=true"
  assert_has "$deletion_output" "ci_contracts_required=true"
  assert_has "$deletion_output" "proof_validation_scope=v0_91_3"
  assert_has "$deletion_output" "reason=v0913_proof_surface_change_runs_targeted_packet_validation"

  git checkout -q -b runtime-policy-surface-change "$base_sha"
  mkdir -p adl/tools
  printf '#!/usr/bin/env bash\nprintf policy\n' > adl/tools/enforce_coverage_gates.sh
  printf 'pub fn runtime_with_policy() -> bool { true }\n' >> adl/src/lib.rs
  git add adl/tools/enforce_coverage_gates.sh adl/src/lib.rs
  git commit -q -m runtime-policy-surface-change
  runtime_policy_surface_head="$(git rev-parse HEAD)"

  runtime_policy_surface_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$runtime_policy_surface_head" --ref "refs/pull/1/merge")"
  assert_has "$runtime_policy_surface_output" "rust_required=true"
  assert_has "$runtime_policy_surface_output" "coverage_required=true"
  assert_has "$runtime_policy_surface_output" "full_coverage_required=true"
  assert_has "$runtime_policy_surface_output" "demo_smoke_required=true"
  assert_has "$runtime_policy_surface_output" "ci_contracts_required=true"
  assert_has "$runtime_policy_surface_output" "slow_proof_contracts_required=false"
  assert_has "$runtime_policy_surface_output" "coverage_lane=authoritative_full"
  assert_has "$runtime_policy_surface_output" "coverage_authority=pr_policy_surface_runtime_mixed"
  assert_has "$runtime_policy_surface_output" "reason=coverage_policy_surface_change_with_runtime_surface_runs_full_coverage"

  git checkout -q -b workflow-summary-only-change "$base_sha"
  python3 - <<'PY'
from pathlib import Path

workflow = Path(".github/workflows/ci.yaml")
text = workflow.read_text()
needle = "      - name: Determine PR fast coverage filters\n"
if needle not in text:
    raise SystemExit("expected coverage classifier insertion point missing")
replacement = r"""      - name: Validation profile summary (adl-coverage)
        run: |
          {
            echo "## ADL coverage validation profile"
            echo
            echo "| Field | Value |"
            echo "|---|---|"
            echo "| reason | \`${{ steps.path-policy.outputs.reason }}\` |"
            echo "| selected profile | \`${{ steps.path-policy.outputs.validation_profile_selected }}\` |"
            echo "| profile status | \`${{ steps.path-policy.outputs.validation_profile_status }}\` |"
            echo "| coverage lane | \`${{ steps.path-policy.outputs.coverage_lane }}\` |"
            echo "| coverage authority | \`${{ steps.path-policy.outputs.coverage_authority }}\` |"
            echo "| profile run lanes | \`${{ steps.path-policy.outputs.validation_profile_run_lanes }}\` |"
            echo "| escalation required | \`${{ steps.path-policy.outputs.validation_profile_escalation_required }}\` |"
          } >> "$GITHUB_STEP_SUMMARY"
        working-directory: .

      - name: Determine PR fast coverage filters
"""
workflow.write_text(text.replace(needle, replacement, 1))
PY
  git add .github/workflows/ci.yaml
  git commit -q -m workflow-summary-only-change
  workflow_summary_only_head="$(git rev-parse HEAD)"

  workflow_summary_only_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$workflow_summary_only_head" --ref "refs/pull/1/merge")"
  assert_has "$workflow_summary_only_output" "rust_required=false"
  assert_has "$workflow_summary_only_output" "coverage_required=false"
  assert_has "$workflow_summary_only_output" "full_coverage_required=false"
  assert_has "$workflow_summary_only_output" "demo_smoke_required=false"
  assert_has "$workflow_summary_only_output" "ci_contracts_required=true"
  assert_has "$workflow_summary_only_output" "coverage_lane=skip"
  assert_has "$workflow_summary_only_output" "coverage_authority=not_required"
  assert_has "$workflow_summary_only_output" "reason=validation_profile_summary_workflow_change_skips_authoritative_coverage"

  git checkout -q -b workflow-summary-plus-policy-change "$base_sha"
  python3 - <<'PY'
from pathlib import Path

workflow = Path(".github/workflows/ci.yaml")
text = workflow.read_text()
needle = "      - name: Determine PR fast coverage filters\n"
replacement = r"""      - name: Validation profile summary (adl-coverage)
        run: |
          {
            echo "## ADL coverage validation profile"
            echo
            echo "| Field | Value |"
            echo "|---|---|"
            echo "| reason | \`${{ steps.path-policy.outputs.reason }}\` |"
            echo "| selected profile | \`${{ steps.path-policy.outputs.validation_profile_selected }}\` |"
            echo "| profile status | \`${{ steps.path-policy.outputs.validation_profile_status }}\` |"
            echo "| coverage lane | \`${{ steps.path-policy.outputs.coverage_lane }}\` |"
            echo "| coverage authority | \`${{ steps.path-policy.outputs.coverage_authority }}\` |"
            echo "| profile run lanes | \`${{ steps.path-policy.outputs.validation_profile_run_lanes }}\` |"
            echo "| escalation required | \`${{ steps.path-policy.outputs.validation_profile_escalation_required }}\` |"
          } >> "$GITHUB_STEP_SUMMARY"
        working-directory: .

      - name: Determine PR fast coverage filters
"""
text = text.replace(needle, replacement, 1)
text = text.replace("run: bash tools/run_authoritative_coverage_lane.sh", "run: bash tools/run_authoritative_coverage_lane.sh --strict", 1)
workflow.write_text(text)
PY
  git add .github/workflows/ci.yaml
  git commit -q -m workflow-summary-plus-policy-change
  workflow_summary_plus_policy_head="$(git rev-parse HEAD)"

  workflow_summary_plus_policy_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$workflow_summary_plus_policy_head" --ref "refs/pull/1/merge")"
  assert_has "$workflow_summary_plus_policy_output" "rust_required=false"
  assert_has "$workflow_summary_plus_policy_output" "coverage_required=false"
  assert_has "$workflow_summary_plus_policy_output" "full_coverage_required=false"
  assert_has "$workflow_summary_plus_policy_output" "demo_smoke_required=false"
  assert_has "$workflow_summary_plus_policy_output" "ci_contracts_required=true"
  assert_has "$workflow_summary_plus_policy_output" "coverage_lane=skip"
  assert_has "$workflow_summary_plus_policy_output" "coverage_authority=not_required"
  assert_has "$workflow_summary_plus_policy_output" "reason=coverage_policy_surface_tooling_change_runs_contract_validation"

  git checkout -q -b nightly-schedule-only-change "$base_sha"
  python3 - <<'PY'
from pathlib import Path

nightly = Path(".github/workflows/nightly-coverage-ratchet.yaml")
nightly.write_text(nightly.read_text() + '  schedule:\n    - cron: "43 11 * * *"\n')
PY
  git add .github/workflows/nightly-coverage-ratchet.yaml
  git commit -q -m nightly-schedule-only-change
  nightly_schedule_only_head="$(git rev-parse HEAD)"

  nightly_schedule_only_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$nightly_schedule_only_head" --ref "refs/pull/1/merge")"
  assert_has "$nightly_schedule_only_output" "rust_required=false"
  assert_has "$nightly_schedule_only_output" "coverage_required=false"
  assert_has "$nightly_schedule_only_output" "full_coverage_required=false"
  assert_has "$nightly_schedule_only_output" "demo_smoke_required=false"
  assert_has "$nightly_schedule_only_output" "ci_contracts_required=true"
  assert_has "$nightly_schedule_only_output" "coverage_lane=skip"
  assert_has "$nightly_schedule_only_output" "coverage_authority=not_required"
  assert_has "$nightly_schedule_only_output" "reason=nightly_coverage_schedule_only_change_skips_authoritative_coverage"

  git checkout -q -b validation-summary-policy-bundle "$base_sha"
  python3 - <<'PY'
from pathlib import Path

workflow = Path(".github/workflows/ci.yaml")
text = workflow.read_text()
needle = "      - name: Determine PR fast coverage filters\n"
replacement = r"""      - name: Validation profile summary (adl-coverage)
        run: |
          {
            echo "## ADL coverage validation profile"
            echo
            echo "| Field | Value |"
            echo "|---|---|"
            echo "| reason | \`${{ steps.path-policy.outputs.reason }}\` |"
            echo "| selected profile | \`${{ steps.path-policy.outputs.validation_profile_selected }}\` |"
            echo "| profile status | \`${{ steps.path-policy.outputs.validation_profile_status }}\` |"
            echo "| coverage lane | \`${{ steps.path-policy.outputs.coverage_lane }}\` |"
            echo "| coverage authority | \`${{ steps.path-policy.outputs.coverage_authority }}\` |"
            echo "| profile run lanes | \`${{ steps.path-policy.outputs.validation_profile_run_lanes }}\` |"
            echo "| escalation required | \`${{ steps.path-policy.outputs.validation_profile_escalation_required }}\` |"
          } >> "$GITHUB_STEP_SUMMARY"
        working-directory: .

      - name: Determine PR fast coverage filters
"""
workflow.write_text(text.replace(needle, replacement, 1))
workflow = Path(".github/workflows/ci.yaml")
text = workflow.read_text()
deferred = """      - name: Full workspace gate deferred for bounded authoritative PR
        if: github.event_name == 'pull_request' && steps.path-policy.outputs.full_coverage_required == 'true'
        run: |
          echo "Workspace coverage gate and lcov artifact generation are deferred for pull requests."
          echo "This PR ran the authoritative coverage pass plus changed-source coverage-impact validation."
          echo "Full workspace coverage gating remains required on push-to-main, nightly ratchet, and non-PR fail-closed events."
          echo "Policy reason: ${{ steps.path-policy.outputs.reason }}"
          echo "Coverage lane: ${{ steps.path-policy.outputs.coverage_lane }}"
          echo "Coverage authority: ${{ steps.path-policy.outputs.coverage_authority }}"

"""
if deferred in text:
    text = text.replace(deferred, "", 1)
workflow.write_text(text)

nightly = Path(".github/workflows/nightly-coverage-ratchet.yaml")
nightly.write_text(nightly.read_text() + '  schedule:\n    - cron: "43 11 * * *"\n')

policy = Path("adl/tools/ci_path_policy.sh")
policy.parent.mkdir(parents=True, exist_ok=True)
policy.write_text(
    "#!/usr/bin/env bash\n"
    "is_validation_profile_summary_workflow_change() { :; }\n"
    "is_nightly_coverage_schedule_only_change() { :; }\n"
    "reason=validation_profile_summary_workflow_change_skips_authoritative_coverage\n"
)

Path("adl/tools/test_ci_path_policy.sh").write_text(
    "#!/usr/bin/env bash\n"
    "# workflow-summary-only-change\n"
    "# workflow-summary-plus-policy-change\n"
    "# nightly-schedule-only-change\n"
    "# validation_profile_summary_workflow_change_skips_authoritative_coverage\n"
    "# nightly_coverage_schedule_only_change_skips_authoritative_coverage\n"
)

Path("adl/tools/test_ci_runtime_contracts.sh").write_text(
    "#!/usr/bin/env bash\n"
    "# Validation profile summary (adl-ci)\n"
    "# Validation profile summary (adl-coverage)\n"
    "# GITHUB_STEP_SUMMARY\n"
)

Path("docs/milestones/v0.91.6/review").mkdir(parents=True, exist_ok=True)
Path("docs/milestones/v0.91.6/review/ADL_CI_VALIDATION_COST_REVIEW_4322.md").write_text(
    "# ADL CI Validation Cost Review\n"
)
PY
  git add .github/workflows/ci.yaml .github/workflows/nightly-coverage-ratchet.yaml \
    adl/tools/ci_path_policy.sh adl/tools/test_ci_path_policy.sh adl/tools/test_ci_runtime_contracts.sh \
    docs/milestones/v0.91.6/review/ADL_CI_VALIDATION_COST_REVIEW_4322.md
  git commit -q -m validation-summary-policy-bundle
  validation_summary_policy_bundle_head="$(git rev-parse HEAD)"

  validation_summary_policy_bundle_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$validation_summary_policy_bundle_head" --ref "refs/pull/1/merge")"
  assert_has "$validation_summary_policy_bundle_output" "rust_required=false"
  assert_has "$validation_summary_policy_bundle_output" "coverage_required=false"
  assert_has "$validation_summary_policy_bundle_output" "full_coverage_required=false"
  assert_has "$validation_summary_policy_bundle_output" "demo_smoke_required=false"
  assert_has "$validation_summary_policy_bundle_output" "ci_contracts_required=true"
  assert_has "$validation_summary_policy_bundle_output" "coverage_lane=skip"
  assert_has "$validation_summary_policy_bundle_output" "coverage_authority=not_required"
  assert_has "$validation_summary_policy_bundle_output" "reason=coverage_policy_surface_tooling_change_runs_contract_validation"

  git checkout -q -b runtime-bounded-pr-fast-coverage-policy-change "$base_sha"
  mkdir -p adl/src/cli adl/tools
  cat > adl/src/cli/process_cmd.rs <<'EOF'
pub fn process_status_probe() -> bool { true }
EOF
  python3 - <<'PY'
from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if old not in text:
        raise SystemExit(f"expected workflow snippet missing for {label}")
    return text.replace(old, new, 1)

workflow = Path(".github/workflows/ci.yaml")
text = workflow.read_text()
text = replace_once(
    text,
    '            echo "filter_expression=$(cat adl/coverage-impact-filter-expression.txt)" >> "$GITHUB_OUTPUT"\n',
    '            echo "filter_expression=$(cat adl/coverage-impact-filter-expression.txt)" >> "$GITHUB_OUTPUT"\n            echo "run_cli_smoke_process_status=true" >> "$GITHUB_OUTPUT"\n            echo "run_cli_smoke_basics=true" >> "$GITHUB_OUTPUT"\n',
    "coverage-impact output augmentation",
)
text = replace_once(
    text,
    "          rm -rf target/debug target/llvm-cov-target\n          COVERAGE_BUILD_ROOT=\"${RUNNER_TEMP:-/tmp}/adl-pr-fast-coverage\"\n          mkdir -p \"$COVERAGE_BUILD_ROOT/target\" \"$COVERAGE_BUILD_ROOT/llvm-cov-target\"\n          export CARGO_TARGET_DIR=\"$COVERAGE_BUILD_ROOT/target\"\n          export CARGO_LLVM_COV_TARGET_DIR=\"$COVERAGE_BUILD_ROOT/llvm-cov-target\"\n          CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report -E \"${{ steps.coverage-impact.outputs.filter_expression }}\"\n          cargo llvm-cov report --json --summary-only --output-path coverage-summary.json\n",
    "          rm -rf target/debug target/llvm-cov-target\n          COVERAGE_BUILD_ROOT=\"${RUNNER_TEMP:-/tmp}/adl-pr-fast-coverage\"\n          mkdir -p \"$COVERAGE_BUILD_ROOT/target\" \"$COVERAGE_BUILD_ROOT/llvm-cov-target\"\n          export CARGO_TARGET_DIR=\"$COVERAGE_BUILD_ROOT/target\"\n          export CARGO_LLVM_COV_TARGET_DIR=\"$COVERAGE_BUILD_ROOT/llvm-cov-target\"\n          summary_files=()\n          if [ \"${{ steps.coverage-impact.outputs.run_cli_smoke_process_status }}\" = \"true\" ]; then\n            CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --test cli_smoke process_status --no-report\n            cargo llvm-cov report --json --summary-only --output-path coverage-summary-process-status.json\n            summary_files+=(coverage-summary-process-status.json)\n          fi\n          if [ \"${{ steps.coverage-impact.outputs.run_cli_smoke_basics }}\" = \"true\" ]; then\n            CARGO_INCREMENTAL=0 cargo llvm-cov nextest --workspace --test cli_smoke basics --no-report\n            cargo llvm-cov report --json --summary-only --output-path coverage-summary-cli-basics.json\n            summary_files+=(coverage-summary-cli-basics.json)\n          fi\n          cp \"${summary_files[0]}\" coverage-summary.json\n",
    "coverage summary command split",
)
workflow.write_text(text)

coverage = Path("adl/tools/check_coverage_impact.sh")
coverage.parent.mkdir(parents=True, exist_ok=True)
coverage.write_text(
    "#!/usr/bin/env bash\n"
    "candidate_filter_for_path() {\n"
    "  local path=\"$1\"\n"
    "  case \"$path\" in\n"
    "    adl/src/cli/process_cmd.rs)\n"
    "      printf 'process_status'\n"
    "      ;;\n"
    "    adl/src/cli/mod.rs|adl/src/cli/usage.rs)\n"
    "      printf 'cli_basics'\n"
    "      ;;\n"
    "  esac\n"
    "}\n"
)

coverage_test = Path("adl/tools/test_check_coverage_impact.sh")
coverage_test.write_text("#!/usr/bin/env bash\n# process_status\n")

runtime_contract = Path("adl/tools/test_ci_runtime_contracts.sh")
runtime_contract.write_text("if 'coverage-summary.json' not in fast_summary_step:\n    pass\n")
PY
  git add adl/src/cli/process_cmd.rs .github/workflows/ci.yaml adl/tools/check_coverage_impact.sh adl/tools/test_check_coverage_impact.sh adl/tools/test_ci_runtime_contracts.sh
  git commit -q -m runtime-bounded-pr-fast-coverage-policy-change
  runtime_bounded_pr_fast_head="$(git rev-parse HEAD)"

  runtime_bounded_pr_fast_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$runtime_bounded_pr_fast_head" --ref "refs/pull/1/merge")"
  assert_has "$runtime_bounded_pr_fast_output" "rust_required=true"
  assert_has "$runtime_bounded_pr_fast_output" "coverage_required=true"
  assert_has "$runtime_bounded_pr_fast_output" "full_coverage_required=false"
  assert_has "$runtime_bounded_pr_fast_output" "demo_smoke_required=true"
  assert_has "$runtime_bounded_pr_fast_output" "ci_contracts_required=true"
  assert_has "$runtime_bounded_pr_fast_output" "coverage_lane=pr_fast"
  assert_has "$runtime_bounded_pr_fast_output" "coverage_authority=pr_changed_surface"
  assert_has "$runtime_bounded_pr_fast_output" "reason=bounded_pr_fast_coverage_policy_change_keeps_pr_fast_validation"

  git checkout -q -b feature-branch-before-main-advances "$base_sha"
  mkdir -p adl/tools docs/milestones/v0.91.4/review/demo_showcase
  printf '#!/usr/bin/env bash\nprintf demo\n' > adl/tools/demo_v0914_complete_issue.sh
  printf '# tool demo note\n' > docs/milestones/v0.91.4/review/demo_showcase/DEMO_NOTE.md
  git add adl/tools/demo_v0914_complete_issue.sh docs/milestones/v0.91.4/review/demo_showcase/DEMO_NOTE.md
  git commit -q -m feature-branch-before-main-advances
  stale_feature_head="$(git rev-parse HEAD)"

  git checkout -q -b main-advances-with-policy-surface "$base_sha"
  mkdir -p .github/workflows docs/milestones/v0.91.4/review/demo_showcase
  printf 'name: nightly coverage\n' > .github/workflows/nightly-coverage-ratchet.yaml
  printf '# stale packet\n' > docs/milestones/v0.91.4/review/demo_showcase/STALE_PACKET.md
  git add .github/workflows/nightly-coverage-ratchet.yaml docs/milestones/v0.91.4/review/demo_showcase/STALE_PACKET.md
  git commit -q -m main-advance-seed
  rm docs/milestones/v0.91.4/review/demo_showcase/STALE_PACKET.md
  git add -A docs/milestones/v0.91.4/review/demo_showcase/STALE_PACKET.md
  git commit -q -m main-advances-with-policy-surface
  advanced_main_head="$(git rev-parse HEAD)"

  stale_feature_output="$("$POLICY" --event-name pull_request --base "$advanced_main_head" --head "$stale_feature_head" --ref "refs/pull/1/merge")"
  assert_has "$stale_feature_output" "rust_required=true"
  assert_has "$stale_feature_output" "coverage_required=true"
  assert_has "$stale_feature_output" "full_coverage_required=true"
  assert_has "$stale_feature_output" "demo_smoke_required=true"
  assert_has "$stale_feature_output" "ci_contracts_required=true"
  assert_has "$stale_feature_output" "fail_closed=true"
  assert_has "$stale_feature_output" "coverage_lane=authoritative_full"
  assert_has "$stale_feature_output" "coverage_authority=fail_closed"
  assert_has "$stale_feature_output" "reason=validation_manager_escalation_requires_authoritative_full_coverage"
  assert_has "$stale_feature_output" "validation_profile_selected=docs_diff_check_profile"
  assert_has "$stale_feature_output" "validation_profile_status=escalation_required"
  assert_has "$stale_feature_output" "validation_profile_escalation_required=true"
  assert_has "$stale_feature_output" "validation_profile_run_lanes=docs_diff_check"
  assert_has "$stale_feature_output" "validation_profile_escalation_lanes=unmapped_change_surface"

  main_output="$("$POLICY" --event-name push --ref "refs/heads/main")"
  assert_has "$main_output" "rust_required=true"
  assert_has "$main_output" "coverage_required=true"
  assert_has "$main_output" "full_coverage_required=true"
  assert_has "$main_output" "demo_smoke_required=true"
  assert_has "$main_output" "release_version_only=false"
  assert_has "$main_output" "ci_contracts_required=true"
  assert_has "$main_output" "coverage_lane=authoritative_full"
  assert_has "$main_output" "coverage_authority=push_main"
  assert_has "$main_output" "reason=push_main_runs_authoritative_full_coverage"

  fail_closed_output="$("$POLICY" --event-name pull_request --base "" --head "$runtime_head" --ref "refs/pull/1/merge")"
  assert_has "$fail_closed_output" "rust_required=true"
  assert_has "$fail_closed_output" "coverage_required=true"
  assert_has "$fail_closed_output" "full_coverage_required=true"
  assert_has "$fail_closed_output" "release_version_only=false"
  assert_has "$fail_closed_output" "fail_closed=true"
  assert_has "$fail_closed_output" "ci_contracts_required=true"
  assert_has "$fail_closed_output" "coverage_lane=authoritative_full"
  assert_has "$fail_closed_output" "coverage_authority=fail_closed"
  assert_has "$fail_closed_output" "validation_profile_selected="
  assert_has "$fail_closed_output" "validation_profile_status="

  mkdir -p "$tmp_dir/fake-bin"
  cat > "$tmp_dir/fake-bin/python3" <<'EOF'
#!/usr/bin/env bash
exit 7
EOF
  chmod +x "$tmp_dir/fake-bin/python3"

  manager_failure_output="$(PATH="$tmp_dir/fake-bin:$PATH" "$POLICY" --event-name pull_request --base "$base_sha" --head "$runtime_head" --ref "refs/pull/1/merge")"
  assert_has "$manager_failure_output" "rust_required=true"
  assert_has "$manager_failure_output" "coverage_required=true"
  assert_has "$manager_failure_output" "full_coverage_required=true"
  assert_has "$manager_failure_output" "demo_smoke_required=true"
  assert_has "$manager_failure_output" "release_version_only=false"
  assert_has "$manager_failure_output" "ci_contracts_required=true"
  assert_has "$manager_failure_output" "fail_closed=true"
  assert_has "$manager_failure_output" "coverage_lane=authoritative_full"
  assert_has "$manager_failure_output" "coverage_authority=fail_closed"
  assert_has "$manager_failure_output" "reason=validation_manager_failed_closed_for_pull_request"
  assert_has "$manager_failure_output" "validation_profile_selected="
  assert_has "$manager_failure_output" "validation_profile_status="
)

echo "PASS: ci_path_policy PR-fast/full-coverage contract"
