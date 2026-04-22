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
demo_smoke_required="$bool_false"
fail_closed=false
reason="path_policy_docs_or_tooling_only"
changed_count=0
changed_files=""

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
  demo_smoke_required=true
}

if [ "$event_name" != "pull_request" ]; then
  require_full_validation
  reason="non_pull_request_event_runs_full_validation"
elif [ -z "$base_sha" ] || [ -z "$head_sha" ]; then
  require_full_validation
  fail_closed=true
  reason="missing_pull_request_sha_runs_full_validation"
else
  changed_files="$(git diff --name-only "$base_sha" "$head_sha" 2>/dev/null || true)"
  if [ -z "$changed_files" ]; then
    require_full_validation
    fail_closed=true
    reason="empty_or_unavailable_diff_runs_full_validation"
  else
    changed_count="$(printf '%s\n' "$changed_files" | sed '/^$/d' | wc -l | tr -d ' ')"
    while IFS= read -r path; do
      [ -n "$path" ] || continue
      case "$path" in
        adl/src/*|adl/tests/*|adl/Cargo.toml|adl/Cargo.lock|adl/build.rs)
          rust_required=true
          coverage_required=true
          demo_smoke_required=true
          reason="runtime_or_rust_test_change_runs_full_validation"
          ;;
        demos/*|adl/tools/demo_*|adl/tools/test_demo_*)
          demo_smoke_required=true
          ;;
      esac
    done <<EOF
$changed_files
EOF
  fi
fi

emit "rust_required" "$rust_required"
emit "coverage_required" "$coverage_required"
emit "demo_smoke_required" "$demo_smoke_required"
emit "fail_closed" "$fail_closed"
emit "changed_count" "$changed_count"
emit "reason" "$reason"

printf '\nChanged path policy: %s\n' "$reason"
printf '  rust_required=%s\n' "$rust_required"
printf '  coverage_required=%s\n' "$coverage_required"
printf '  demo_smoke_required=%s\n' "$demo_smoke_required"
printf '  fail_closed=%s\n' "$fail_closed"
printf '  changed_count=%s\n' "$changed_count"
