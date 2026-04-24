#!/usr/bin/env bash
set -euo pipefail

event_name="${GITHUB_EVENT_NAME:-}"
base_sha=""
head_sha=""
ref_name="${GITHUB_REF:-}"
github_output="${GITHUB_OUTPUT:-}"

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/ci_path_policy.sh [--event-name <name>] [--base <sha>] [--head <sha>] [--ref <ref>] [--github-output <path>]

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
    --ref)
      ref_name="${2:-}"
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
release_version_only="$bool_false"
fail_closed=false
coverage_lane="skip"
coverage_authority="not_required"
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

mark_authoritative_full_coverage() {
  local authority="$1"
  local reason_value="$2"
  require_full_validation
  coverage_lane="authoritative_full"
  coverage_authority="$authority"
  reason="$reason_value"
}

mark_pr_fast_coverage() {
  rust_required=true
  coverage_required=true
  demo_smoke_required=true
  coverage_lane="pr_fast"
  coverage_authority="pr_changed_surface"
  reason="runtime_or_rust_test_change_runs_pr_fast_validation"
}

mark_policy_surface_full_coverage() {
  full_coverage_required=true
  coverage_lane="authoritative_full"
  coverage_authority="pr_policy_surface"
  reason="coverage_policy_surface_change_runs_full_coverage"
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

is_release_truth_doc_surface() {
  local path="$1"
  case "$path" in
    README.md|CHANGELOG.md|REVIEW.md|docs/*)
      return 0
      ;;
  esac
  return 1
}

cargo_manifest_package_version_only_change() {
  python3 - "$base_sha" "$head_sha" <<'PY'
import io
import subprocess
import sys
import tomllib

base, head = sys.argv[1], sys.argv[2]
path = "adl/Cargo.toml"

def load(rev: str):
    try:
        text = subprocess.check_output(
            ["git", "show", f"{rev}:{path}"], text=True, stderr=subprocess.DEVNULL
        )
    except subprocess.CalledProcessError:
        return None
    return tomllib.loads(text)

before = load(base)
after = load(head)
if before is None or after is None:
    raise SystemExit(1)

before_pkg = dict(before.get("package", {}))
after_pkg = dict(after.get("package", {}))
before_version = before_pkg.pop("version", None)
after_version = after_pkg.pop("version", None)

before_norm = dict(before)
after_norm = dict(after)
before_norm["package"] = before_pkg
after_norm["package"] = after_pkg

if (
    before_version
    and after_version
    and before_version != after_version
    and before_norm == after_norm
):
    raise SystemExit(0)
raise SystemExit(1)
PY
}

cargo_lock_adl_package_version_only_change() {
  python3 - "$base_sha" "$head_sha" <<'PY'
import subprocess
import sys
import tomllib

base, head = sys.argv[1], sys.argv[2]
path = "adl/Cargo.lock"

def load(rev: str):
    try:
        text = subprocess.check_output(
            ["git", "show", f"{rev}:{path}"], text=True, stderr=subprocess.DEVNULL
        )
    except subprocess.CalledProcessError:
        return None
    return tomllib.loads(text)

def normalize(data):
    packages = []
    changed_versions = []
    for package in data.get("package", []):
        pkg = dict(package)
        if pkg.get("name") == "adl":
            changed_versions.append(pkg.get("version"))
            pkg.pop("version", None)
        packages.append(pkg)
    normalized = dict(data)
    normalized["package"] = packages
    return normalized, changed_versions

before = load(base)
after = load(head)
if before is None or after is None:
    raise SystemExit(1)

before_norm, before_versions = normalize(before)
after_norm, after_versions = normalize(after)

if (
    before_versions
    and after_versions
    and before_versions != after_versions
    and before_norm == after_norm
):
    raise SystemExit(0)
raise SystemExit(1)
PY
}

is_release_version_only_surface_change() {
  local saw_manifest=false
  local saw_lock=false
  local path=""
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    case "$path" in
      adl/Cargo.toml)
        saw_manifest=true
        ;;
      adl/Cargo.lock)
        saw_lock=true
        ;;
      *)
        if ! is_release_truth_doc_surface "$path"; then
          return 1
        fi
        ;;
    esac
  done <<EOF
$changed_files
EOF

  [ "$saw_manifest" = true ] || return 1
  [ "$saw_lock" = true ] || return 1
  cargo_manifest_package_version_only_change || return 1
  cargo_lock_adl_package_version_only_change || return 1
  return 0
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
  if [ "$event_name" = "push" ] && [ "$ref_name" = "refs/heads/main" ]; then
    mark_authoritative_full_coverage "push_main" "push_main_runs_authoritative_full_coverage"
  else
    mark_authoritative_full_coverage "non_pr_event" "non_pull_request_event_runs_full_validation"
  fi
elif [ -z "$base_sha" ] || [ -z "$head_sha" ]; then
  fail_closed=true
  mark_authoritative_full_coverage "fail_closed" "missing_pull_request_sha_runs_full_validation"
else
  changed_rows="$(git diff --name-status --diff-filter=ACMR "$base_sha" "$head_sha" 2>/dev/null | normalize_changed_rows || true)"
  changed_files="$(printf '%s\n' "$changed_rows" | awk -F '\t' 'NF >= 2 { print $2 }')"
  if [ -z "$changed_rows" ]; then
    fail_closed=true
    mark_authoritative_full_coverage "fail_closed" "empty_or_unavailable_diff_runs_full_validation"
  else
    saw_pr_finish_control_plane=false
    changed_count="$(printf '%s\n' "$changed_files" | sed '/^$/d' | wc -l | tr -d ' ')"
    if is_release_version_only_surface_change; then
      release_version_only=true
      reason="release_version_only_cargo_surface_change_runs_lightweight_validation"
    fi
    while IFS= read -r path; do
      [ -n "$path" ] || continue
      if is_pr_finish_control_plane_surface "$path"; then
        rust_required=true
        saw_pr_finish_control_plane=true
        continue
      fi
      case "$path" in
        adl/src/*|adl/tests/*|adl/Cargo.toml|adl/Cargo.lock|adl/build.rs)
          if [ "$release_version_only" = true ] && { [ "$path" = "adl/Cargo.toml" ] || [ "$path" = "adl/Cargo.lock" ]; }; then
            continue
          fi
          mark_pr_fast_coverage
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
        mark_policy_surface_full_coverage
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
emit "release_version_only" "$release_version_only"
emit "fail_closed" "$fail_closed"
emit "coverage_lane" "$coverage_lane"
emit "coverage_authority" "$coverage_authority"
emit "changed_count" "$changed_count"
emit "reason" "$reason"

printf '\nChanged path policy: %s\n' "$reason"
printf '  rust_required=%s\n' "$rust_required"
printf '  coverage_required=%s\n' "$coverage_required"
printf '  full_coverage_required=%s\n' "$full_coverage_required"
printf '  demo_smoke_required=%s\n' "$demo_smoke_required"
printf '  release_version_only=%s\n' "$release_version_only"
printf '  fail_closed=%s\n' "$fail_closed"
printf '  coverage_lane=%s\n' "$coverage_lane"
printf '  coverage_authority=%s\n' "$coverage_authority"
printf '  changed_count=%s\n' "$changed_count"
