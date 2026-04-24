#!/usr/bin/env bash
set -euo pipefail

event_name="${GITHUB_EVENT_NAME:-}"
base_sha=""
head_sha=""
github_output="${GITHUB_OUTPUT:-}"

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/ci_path_policy.sh [--event-name <name>] [--base <sha>] [--head <sha>] [--github-output <path>]

Classifies changed paths for CI so docs/planning/tools-only PRs can skip
expensive Rust test and coverage phases while runtime/source changes still run
the full gates.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --event-name)
      event_name="${2:-}"
      shift 2
      ;;
    --base)
      base_sha="${2:-}"
      shift 2
      ;;
    --head)
      head_sha="${2:-}"
      shift 2
      ;;
    --github-output)
      github_output="${2:-}"
      shift 2
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

bool_false=false
rust_required="$bool_false"
coverage_required="$bool_false"
full_coverage_required="$bool_false"
demo_smoke_required="$bool_false"
fail_closed=false
reason="path_policy_docs_or_tooling_only"
changed_count=0
changed_files=""
changed_rows=""
large_file_lines="${COVERAGE_IMPACT_LARGE_FILE_LINES:-200}"
large_file_delta="${COVERAGE_IMPACT_LARGE_FILE_DELTA:-80}"

emit() {
  local key="$1"
  local value="$2"
  printf '%s=%s\n' "$key" "$value"
  if [ -n "$github_output" ]; then
    printf '%s=%s\n' "$key" "$value" >> "$github_output"
  fi
}

require_full_validation() {
  rust_required=true
  coverage_required=true
  full_coverage_required=true
  demo_smoke_required=true
}

normalize_changed_rows() {
  awk -F '\t' '
    NF == 1 { print "M\t" $1; next }
    $1 ~ /^R/ && NF >= 3 { print $1 "\t" $3; next }
    NF >= 2 { print $1 "\t" $2; next }
  '
}

is_production_rust_source() {
  local path="$1"
  case "$path" in
    adl/src/*.rs)
      case "$path" in
        adl/src/tests.rs|adl/src/*/tests.rs|adl/src/*/tests/*)
          return 1
          ;;
      esac
      return 0
      ;;
  esac
  return 1
}

is_pr_finish_control_plane_surface() {
  local path="$1"
  case "$path" in
    adl/src/cli/pr_cmd.rs|\
    adl/src/cli/pr_cmd/finish_support.rs|\
    adl/src/cli/tests/pr_cmd_inline/finish/*|\
    docs/default_workflow.md|\
    docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md)
      return 0
      ;;
  esac
  return 1
}

is_full_coverage_policy_surface() {
  local path="$1"
  case "$path" in
    .github/workflows/ci.yaml|\
    .github/workflows/nightly-coverage-ratchet.yaml|\
    adl/tools/ci_path_policy.sh|\
    adl/tools/check_coverage_impact.sh|\
    adl/tools/enforce_coverage_gates.sh)
      return 0
      ;;
  esac
  return 1
}

line_count_for_path() {
  local path="$1"
  if [ -f "$path" ]; then
    wc -l <"$path" | tr -d ' '
  else
    echo 0
  fi
}

changed_line_delta_for_path() {
  local path="$1"
  git diff --numstat "$base_sha" "$head_sha" -- "$path" 2>/dev/null \
    | awk '($1 ~ /^[0-9]+$/ && $2 ~ /^[0-9]+$/) { total += $1 + $2 } END { print total + 0 }'
}

if [ "$event_name" != "pull_request" ]; then
  require_full_validation
  reason="non_pull_request_event_runs_full_validation"
elif [ -z "$base_sha" ] || [ -z "$head_sha" ]; then
  require_full_validation
  fail_closed=true
  reason="missing_pull_request_sha_runs_full_validation"
else
  changed_rows="$(git diff --name-status --diff-filter=ACMR "$base_sha" "$head_sha" 2>/dev/null | normalize_changed_rows || true)"
  changed_files="$(printf '%s\n' "$changed_rows" | awk -F '\t' 'NF >= 2 { print $2 }')"
  if [ -z "$changed_rows" ]; then
    require_full_validation
    fail_closed=true
    reason="empty_or_unavailable_diff_runs_full_validation"
  else
    saw_pr_finish_control_plane=false
    changed_count="$(printf '%s\n' "$changed_files" | sed '/^$/d' | wc -l | tr -d ' ')"
    while IFS= read -r path; do
      [ -n "$path" ] || continue
      if is_pr_finish_control_plane_surface "$path"; then
        rust_required=true
        saw_pr_finish_control_plane=true
        continue
      fi
      case "$path" in
        adl/src/*|adl/tests/*|adl/Cargo.toml|adl/Cargo.lock|adl/build.rs)
          rust_required=true
          coverage_required=true
          demo_smoke_required=true
          reason="runtime_or_rust_test_change_runs_pr_fast_validation"
          ;;
        demos/*|adl/tools/demo_*|adl/tools/test_demo_*)
          demo_smoke_required=true
          ;;
      esac
    done <<EOF
$changed_files
EOF
    while IFS=$'\t' read -r _status path; do
      [ -n "$path" ] || continue
      if is_full_coverage_policy_surface "$path"; then
        full_coverage_required=true
        reason="coverage_policy_surface_change_runs_full_coverage"
        continue
      fi
    done <<EOF
$changed_rows
EOF
    if [ "$saw_pr_finish_control_plane" = true ] \
      && [ "$coverage_required" != true ] \
      && [ "$full_coverage_required" != true ] \
      && [ "$demo_smoke_required" != true ]; then
      reason="publication_control_plane_change_runs_focused_rust_validation"
    fi
  fi
fi

emit "rust_required" "$rust_required"
emit "coverage_required" "$coverage_required"
emit "full_coverage_required" "$full_coverage_required"
emit "demo_smoke_required" "$demo_smoke_required"
emit "fail_closed" "$fail_closed"
emit "changed_count" "$changed_count"
emit "reason" "$reason"

printf '\nChanged path policy: %s\n' "$reason"
printf '  rust_required=%s\n' "$rust_required"
printf '  coverage_required=%s\n' "$coverage_required"
printf '  full_coverage_required=%s\n' "$full_coverage_required"
printf '  demo_smoke_required=%s\n' "$demo_smoke_required"
printf '  fail_closed=%s\n' "$fail_closed"
printf '  changed_count=%s\n' "$changed_count"
