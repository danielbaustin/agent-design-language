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
  assert_has "$docs_output" "release_version_only=false"
  assert_has "$docs_output" "coverage_lane=skip"
  assert_has "$docs_output" "coverage_authority=not_required"

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
  assert_has "$release_version_output" "release_version_only=true"
  assert_has "$release_version_output" "coverage_lane=skip"
  assert_has "$release_version_output" "coverage_authority=not_required"
  assert_has "$release_version_output" "reason=release_version_only_cargo_surface_change_runs_lightweight_validation"

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
  assert_has "$cargo_structural_output" "release_version_only=false"
  assert_has "$cargo_structural_output" "coverage_lane=pr_fast"
  assert_has "$cargo_structural_output" "coverage_authority=pr_changed_surface"

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
  assert_has "$runtime_output" "release_version_only=false"
  assert_has "$runtime_output" "coverage_lane=pr_fast"
  assert_has "$runtime_output" "coverage_authority=pr_changed_surface"
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
  assert_has "$new_runtime_file_output" "release_version_only=false"
  assert_has "$new_runtime_file_output" "coverage_lane=pr_fast"
  assert_has "$new_runtime_file_output" "coverage_authority=pr_changed_surface"
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
  assert_has "$policy_surface_output" "release_version_only=false"
  assert_has "$policy_surface_output" "coverage_lane=authoritative_full"
  assert_has "$policy_surface_output" "coverage_authority=pr_policy_surface_tooling_only"
  assert_has "$policy_surface_output" "reason=coverage_policy_surface_change_runs_bounded_authoritative_coverage"

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
