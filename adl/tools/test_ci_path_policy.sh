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
  printf '# baseline\n' > docs/readme.md
  git add .
  git commit -q -m baseline
  base_sha="$(git rev-parse HEAD)"

  printf '\nmore docs\n' >> docs/readme.md
  git add docs/readme.md
  git commit -q -m docs-change
  docs_head="$(git rev-parse HEAD)"

  docs_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$docs_head")"
  assert_has "$docs_output" "rust_required=false"
  assert_has "$docs_output" "coverage_required=false"
  assert_has "$docs_output" "full_coverage_required=false"
  assert_has "$docs_output" "demo_smoke_required=false"

  git checkout -q -b runtime-change "$base_sha"
  printf 'pub fn added_runtime() -> bool { true }\n' >> adl/src/lib.rs
  git add adl/src/lib.rs
  git commit -q -m runtime-change
  runtime_head="$(git rev-parse HEAD)"

  runtime_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$runtime_head")"
  assert_has "$runtime_output" "rust_required=true"
  assert_has "$runtime_output" "coverage_required=true"
  assert_has "$runtime_output" "full_coverage_required=false"
  assert_has "$runtime_output" "demo_smoke_required=true"
  assert_has "$runtime_output" "reason=runtime_or_rust_test_change_runs_pr_fast_validation"

  git checkout -q -b new-runtime-file "$base_sha"
  printf 'pub fn contract_schema() -> bool { true }\n' > adl/src/contract_schema.rs
  git add adl/src/contract_schema.rs
  git commit -q -m new-runtime-file
  new_runtime_file_head="$(git rev-parse HEAD)"

  new_runtime_file_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$new_runtime_file_head")"
  assert_has "$new_runtime_file_output" "rust_required=true"
  assert_has "$new_runtime_file_output" "coverage_required=true"
  assert_has "$new_runtime_file_output" "full_coverage_required=false"
  assert_has "$new_runtime_file_output" "demo_smoke_required=true"
  assert_has "$new_runtime_file_output" "reason=runtime_or_rust_test_change_runs_pr_fast_validation"

  git checkout -q -b policy-surface-change "$base_sha"
  mkdir -p adl/tools
  printf '#!/usr/bin/env bash\nprintf policy\n' > adl/tools/enforce_coverage_gates.sh
  git add adl/tools/enforce_coverage_gates.sh
  git commit -q -m policy-surface-change
  policy_surface_head="$(git rev-parse HEAD)"

  policy_surface_output="$("$POLICY" --event-name pull_request --base "$base_sha" --head "$policy_surface_head")"
  assert_has "$policy_surface_output" "rust_required=false"
  assert_has "$policy_surface_output" "coverage_required=false"
  assert_has "$policy_surface_output" "full_coverage_required=true"
  assert_has "$policy_surface_output" "demo_smoke_required=false"
  assert_has "$policy_surface_output" "reason=coverage_policy_surface_change_runs_full_coverage"

  main_output="$("$POLICY" --event-name push)"
  assert_has "$main_output" "rust_required=true"
  assert_has "$main_output" "coverage_required=true"
  assert_has "$main_output" "full_coverage_required=true"
  assert_has "$main_output" "demo_smoke_required=true"
  assert_has "$main_output" "reason=non_pull_request_event_runs_full_validation"

  fail_closed_output="$("$POLICY" --event-name pull_request --base "" --head "$runtime_head")"
  assert_has "$fail_closed_output" "rust_required=true"
  assert_has "$fail_closed_output" "coverage_required=true"
  assert_has "$fail_closed_output" "full_coverage_required=true"
  assert_has "$fail_closed_output" "fail_closed=true"
)

echo "PASS: ci_path_policy PR-fast/full-coverage contract"
