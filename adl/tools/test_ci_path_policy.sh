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
  assert_has "$docs_output" "coverage_lane=skip"
  assert_has "$docs_output" "coverage_authority=not_required"
  assert_has "$docs_output" "proof_validation_scope=not_required"

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
  assert_has "$cargo_structural_output" "coverage_lane=pr_fast"
  assert_has "$cargo_structural_output" "coverage_authority=pr_changed_surface"
  assert_has "$cargo_structural_output" "proof_validation_scope=not_required"

  git checkout -q -b runtime-change "$base_sha"
  printf 'pub fn added_runtime() -> bool { true }\n' >> adl/src/lib.rs
  git add adl/src/lib.rs
  git commit -q -m runtime-change
  runtime_head="$(git rev-parse HEAD)"

  runtime_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$runtime_head" --ref "refs/pull/1/merge")"
  assert_has "$runtime_output" "rust_required=true"
  assert_has "$runtime_output" "coverage_required=true"
  assert_has "$runtime_output" "full_coverage_required=false"
  assert_has "$runtime_output" "demo_smoke_required=true"
  assert_has "$runtime_output" "v0913_proof_required=false"
  assert_has "$runtime_output" "release_version_only=false"
  assert_has "$runtime_output" "coverage_lane=pr_fast"
  assert_has "$runtime_output" "coverage_authority=pr_changed_surface"
  assert_has "$runtime_output" "proof_validation_scope=not_required"
  assert_has "$runtime_output" "reason=runtime_or_rust_test_change_runs_pr_fast_validation"

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
  assert_has "$new_runtime_file_output" "coverage_lane=pr_fast"
  assert_has "$new_runtime_file_output" "coverage_authority=pr_changed_surface"
  assert_has "$new_runtime_file_output" "proof_validation_scope=not_required"
  assert_has "$new_runtime_file_output" "reason=runtime_or_rust_test_change_runs_pr_fast_validation"

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
  assert_has "$policy_surface_output" "full_coverage_required=true"
  assert_has "$policy_surface_output" "demo_smoke_required=false"
  assert_has "$policy_surface_output" "v0913_proof_required=false"
  assert_has "$policy_surface_output" "release_version_only=false"
  assert_has "$policy_surface_output" "coverage_lane=authoritative_full"
  assert_has "$policy_surface_output" "coverage_authority=pr_policy_surface_tooling_only"
  assert_has "$policy_surface_output" "proof_validation_scope=not_required"
  assert_has "$policy_surface_output" "reason=coverage_policy_surface_change_runs_bounded_authoritative_coverage"

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
  assert_has "$workflow_reporting_output" "coverage_lane=skip"
  assert_has "$workflow_reporting_output" "coverage_authority=not_required"
  assert_has "$workflow_reporting_output" "proof_validation_scope=not_required"
  assert_has "$workflow_reporting_output" "reason=coverage_reporting_workflow_change_skips_authoritative_coverage"

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
  assert_has "$workflow_policy_output" "full_coverage_required=true"
  assert_has "$workflow_policy_output" "demo_smoke_required=false"
  assert_has "$workflow_policy_output" "v0913_proof_required=false"
  assert_has "$workflow_policy_output" "coverage_lane=authoritative_full"
  assert_has "$workflow_policy_output" "coverage_authority=pr_policy_surface_tooling_only"
  assert_has "$workflow_policy_output" "proof_validation_scope=not_required"

  git checkout -q -b v0913-proof-surface "$base_sha"
  mkdir -p docs/milestones/v0.91.3/review/merge_readiness
  printf '\nproof note\n' >> docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md
  git add docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md
  git commit -q -m v0913-proof-surface
  v0913_proof_head="$(git rev-parse HEAD)"

  v0913_proof_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$v0913_proof_head" --ref "refs/pull/1/merge")"
  assert_has "$v0913_proof_output" "rust_required=false"
  assert_has "$v0913_proof_output" "coverage_required=false"
  assert_has "$v0913_proof_output" "full_coverage_required=false"
  assert_has "$v0913_proof_output" "demo_smoke_required=false"
  assert_has "$v0913_proof_output" "v0913_proof_required=true"
  assert_has "$v0913_proof_output" "proof_validation_scope=v0_91_3"
  assert_has "$v0913_proof_output" "reason=v0913_proof_surface_change_runs_targeted_packet_validation"
  assert_has "$workflow_policy_output" "reason=coverage_policy_surface_change_runs_bounded_authoritative_coverage"

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
  assert_has "$runtime_policy_surface_output" "coverage_lane=authoritative_full"
  assert_has "$runtime_policy_surface_output" "coverage_authority=pr_policy_surface_runtime_mixed"
  assert_has "$runtime_policy_surface_output" "reason=coverage_policy_surface_change_with_runtime_surface_runs_full_coverage"

  main_output="$("$POLICY" --event-name push --ref "refs/heads/main")"
  assert_has "$main_output" "rust_required=true"
  assert_has "$main_output" "coverage_required=true"
  assert_has "$main_output" "full_coverage_required=true"
  assert_has "$main_output" "demo_smoke_required=true"
  assert_has "$main_output" "release_version_only=false"
  assert_has "$main_output" "coverage_lane=authoritative_full"
  assert_has "$main_output" "coverage_authority=push_main"
  assert_has "$main_output" "reason=push_main_runs_authoritative_full_coverage"

  fail_closed_output="$("$POLICY" --event-name pull_request --base "" --head "$runtime_head" --ref "refs/pull/1/merge")"
  assert_has "$fail_closed_output" "rust_required=true"
  assert_has "$fail_closed_output" "coverage_required=true"
  assert_has "$fail_closed_output" "full_coverage_required=true"
  assert_has "$fail_closed_output" "release_version_only=false"
  assert_has "$fail_closed_output" "fail_closed=true"
  assert_has "$fail_closed_output" "coverage_lane=authoritative_full"
  assert_has "$fail_closed_output" "coverage_authority=fail_closed"
)

echo "PASS: ci_path_policy PR-fast/full-coverage contract"
