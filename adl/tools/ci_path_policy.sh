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
v0913_proof_required="$bool_false"
release_version_only="$bool_false"
ci_contracts_required="$bool_false"
fail_closed=false
coverage_lane="skip"
coverage_authority="not_required"
proof_validation_scope="not_required"
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
  ci_contracts_required=true
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
  ci_contracts_required=true
  coverage_lane="pr_fast"
  coverage_authority="pr_changed_surface"
  reason="runtime_or_rust_test_change_runs_pr_fast_validation"
}

mark_policy_surface_full_coverage() {
  local authority="$1"
  local reason_value="$2"
  ci_contracts_required=true
  full_coverage_required=true
  coverage_lane="authoritative_full"
  coverage_authority="$authority"
  reason="$reason_value"
}

mark_pvf_slow_proof_contract_validation() {
  rust_required=true
  ci_contracts_required=true
  coverage_required=false
  full_coverage_required=false
  demo_smoke_required=false
  coverage_lane="skip"
  coverage_authority="not_required"
  reason="pvf_slow_proof_change_runs_contract_validation"
}

mark_v0913_proof_required() {
  rust_required=true
  ci_contracts_required=true
  v0913_proof_required=true
  proof_validation_scope="v0_91_3"
  if [ "$reason" = "path_policy_docs_or_tooling_only" ]; then
    reason="v0913_proof_surface_change_runs_targeted_packet_validation"
  fi
}

normalize_changed_rows() {
  awk -F '\t' '
    NF == 1 { print "M\t" $1; next }
    $1 ~ /^R/ && NF >= 3 { print $1 "\t" $3; next }
    NF >= 2 { print $1 "\t" $2; next }
  '
}

git_pr_diff() {
  git diff "$@"
}

git_pr_name_status() {
  if [ -n "${base_sha:-}" ] && [ -n "${head_sha:-}" ]; then
    git_pr_diff --name-status --diff-filter=ACMRD "$base_sha...$head_sha" 2>/dev/null || \
      git_pr_diff --name-status --diff-filter=ACMRD "$base_sha" "$head_sha" 2>/dev/null || true
    return
  fi
  git_pr_diff --name-status --diff-filter=ACMRD "$@" 2>/dev/null || true
}

git_pr_patch() {
  local path="$1"
  if [ -n "${base_sha:-}" ] && [ -n "${head_sha:-}" ]; then
    git_pr_diff --unified=0 "$base_sha...$head_sha" -- "$path" 2>/dev/null || \
      git_pr_diff --unified=0 "$base_sha" "$head_sha" -- "$path" 2>/dev/null || true
    return
  fi
  git_pr_diff --unified=0 -- "$path" 2>/dev/null || true
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

is_reporting_only_coverage_workflow_change() {
  local path="$1"
  [ "$path" = ".github/workflows/ci.yaml" ] || return 1
  git_pr_patch "$path" \
    | while IFS= read -r line; do
        case "$line" in
          diff\ --git*|index\ *|@@*|---*|+++*)
            continue
            ;;
          +*|-*)
            local content="${line#?}"
            case "$content" in
              *"Coverage summary (text)"*|\
              *"Verify generated lcov file"*|\
              *"Verify lcov path from repository root"*|\
              *"Upload coverage artifact"*|\
              *"Upload coverage to Codecov"*|\
              *"adl/lcov.info"*|\
              *"adl/coverage-summary.json"*|\
              *"adl/coverage-summary.txt"*|\
              *"coverage-summary.txt"*|\
              *"coverage-summary.json"*|\
              *"files: adl/lcov.info"*|\
              *"flags: adl"*)
                ;;
              *)
                return 1
                ;;
            esac
            ;;
        esac
      done
}

is_pvf_slow_proof_workflow_change() {
  local path="$1"
  [ "$path" = ".github/workflows/ci.yaml" ] || return 1
  local diff_text
  diff_text="$(git_pr_patch "$path")"
  [ -n "$diff_text" ] || return 1
  grep -E 'adl-slow-proof|test_slow_proof_lane_contract|slow-proof-tests|EXCLUDE_FROM_FILE_FLOOR_REGEX' <<<"$diff_text" >/dev/null 2>&1
}

is_pvf_slow_proof_runtime_manifest_change() {
  local path="$1"
  [ "$path" = "adl/src/runtime_v2/tests.rs" ] || return 1
  local diff_text
  diff_text="$(git_pr_patch "$path")"
  [ -n "$diff_text" ] || return 1
  grep -E 'slow-proof-tests|a2a_adapter_boundary|access_control|acip_hardening|challenge|citizen_state_substrate|contract_registry_accessors|delegation_subcontract' <<<"$diff_text" >/dev/null 2>&1
}

is_pvf_slow_proof_policy_surface() {
  local path="$1"
  case "$path" in
    .github/workflows/ci.yaml|\
    adl/src/runtime_v2/tests.rs|\
    adl/tools/ci_path_policy.sh|\
    adl/tools/run_authoritative_coverage_lane.sh|\
    adl/tools/run_pr_fast_test_lane.sh|\
    adl/tools/test_ci_path_policy.sh|\
    adl/tools/test_ci_runtime_contracts.sh|\
    adl/tools/test_run_authoritative_coverage_lane.sh|\
    adl/tools/test_run_pr_fast_test_lane.sh|\
    adl/tools/test_slow_proof_lane_contract.sh|\
    adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md|\
    docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md|\
    docs/milestones/v0.91.4/features/PARALLEL_VALIDATION_FABRIC.md|\
    docs/milestones/v0.91.4/features/PVF_CI_RELEASE_POLICY_v0.91.4.md|\
    docs/milestones/v0.91.4/features/PVF_INITIAL_LANE_INVENTORY_v0.91.4.md)
      return 0
      ;;
  esac
  return 1
}

is_pvf_slow_proof_policy_change() {
  local saw_pvf_marker=false
  local saw_other=false
  local path
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    if ! is_pvf_slow_proof_policy_surface "$path"; then
      saw_other=true
      continue
    fi
    case "$path" in
      .github/workflows/ci.yaml)
        if is_pvf_slow_proof_workflow_change "$path"; then
          saw_pvf_marker=true
        fi
        ;;
      adl/src/runtime_v2/tests.rs)
        if is_pvf_slow_proof_runtime_manifest_change "$path"; then
          saw_pvf_marker=true
        else
          saw_other=true
        fi
        ;;
      adl/tools/ci_path_policy.sh|\
      adl/tools/run_authoritative_coverage_lane.sh|\
      adl/tools/run_pr_fast_test_lane.sh|\
      adl/tools/test_ci_path_policy.sh|\
      adl/tools/test_ci_runtime_contracts.sh|\
      adl/tools/test_run_authoritative_coverage_lane.sh|\
      adl/tools/test_run_pr_fast_test_lane.sh|\
      adl/tools/test_slow_proof_lane_contract.sh|\
      adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md|\
      docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md|\
      docs/milestones/v0.91.4/features/PARALLEL_VALIDATION_FABRIC.md|\
      docs/milestones/v0.91.4/features/PVF_CI_RELEASE_POLICY_v0.91.4.md|\
      docs/milestones/v0.91.4/features/PVF_INITIAL_LANE_INVENTORY_v0.91.4.md)
        saw_pvf_marker=true
        ;;
    esac
  done <<EOF
$changed_files
EOF
  [ "$saw_pvf_marker" = true ] && [ "$saw_other" = false ]
}

is_csdlc_evidence_namespace_policy_update() {
  local path="$1"
  [ "$path" = "adl/tools/ci_path_policy.sh" ] || return 1
  git_pr_patch "$path" \
    | while IFS= read -r line; do
        case "$line" in
          diff\ --git*|index\ *|@@*|---*|+++*)
            continue
            ;;
          +*|-*)
            local content="${line#?}"
            case "$content" in
              *"workflow/c-sdlc/v0.91.3/issues/"*|\
              *"docs/milestones/v0.91.3/review/evidence/csdlc/issues/"*)
                ;;
              *)
                return 1
                ;;
            esac
            ;;
        esac
      done
}

is_release_truth_doc_surface() {
  local path="$1"
  case "$path" in
    README.md|CHANGELOG.md|REVIEW.md|adl/README.md|docs/*)
      return 0
      ;;
  esac
  return 1
}

is_v0913_proof_surface() {
  local path="$1"
  case "$path" in
    docs/milestones/v0.91.3/review/*|\
    docs/milestones/v0.91.3/features/*|\
    docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md|\
    docs/milestones/v0.91.3/FEATURE_PROOF_COVERAGE_v0.91.3.md|\
    docs/milestones/v0.91.3/QUALITY_GATE_v0.91.3.md|\
    demos/v0.91.3/*|\
    adl/tools/validate_transition_dag_packet.py|\
    adl/tools/test_transition_dag_packet.sh|\
    adl/tools/validate_evidence_bundle_packet.py|\
    adl/tools/test_evidence_bundle_packet.sh|\
    adl/tools/validate_merge_readiness_packet.py|\
    adl/tools/test_merge_readiness_packet.sh|\
    adl/tools/validate_obsmem_handoff_packet.py|\
    adl/tools/test_obsmem_handoff_packet.sh|\
    adl/tools/validate_first_proof_readiness_packet.py|\
    adl/tools/test_first_proof_readiness_packet.sh|\
    adl/tools/validate_first_proof_demo_packet.py|\
    adl/tools/test_first_proof_demo_packet.sh|\
    adl/tools/validate_five_minute_html_game_packet.py|\
    adl/tools/test_five_minute_html_game_packet.sh|\
    adl/tools/validate_five_minute_sprint_console_packet.py|\
    adl/tools/test_five_minute_sprint_console_packet.sh|\
    adl/tools/validate_podcast_studio_v2_packet.py|\
    adl/tools/test_podcast_studio_v2_packet.sh|\
    adl/tools/validate_csdlc_demo_proof_contract_packet.py|\
    adl/tools/test_csdlc_demo_proof_contract_packet.sh|\
    adl/tools/validate_v0913_quality_gate_review_surfaces.py|\
    adl/tools/demo_v0913_quality_gate.sh|\
    adl/tools/run_v0913_proof_validation_lane.sh|\
    adl/tools/test_run_v0913_proof_validation_lane.sh|\
    docs/milestones/v0.91.3/review/evidence/csdlc/issues/*)
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
  git_pr_diff --numstat "$base_sha...$head_sha" -- "$path" 2>/dev/null \
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
  changed_rows="$(git_pr_name_status | normalize_changed_rows || true)"
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
	    pvf_slow_proof_policy_change=false
	    if is_pvf_slow_proof_policy_change; then
	      pvf_slow_proof_policy_change=true
	      mark_pvf_slow_proof_contract_validation
	    fi
	    while IFS= read -r path; do
      [ -n "$path" ] || continue
      if is_pr_finish_control_plane_surface "$path"; then
        rust_required=true
        ci_contracts_required=true
        saw_pr_finish_control_plane=true
        continue
      fi
      if is_v0913_proof_surface "$path"; then
        mark_v0913_proof_required
      fi
      case "$path" in
        adl/src/tests.rs|adl/src/*/tests.rs)
          rust_required=true
          ci_contracts_required=true
          if [ "$reason" = "path_policy_docs_or_tooling_only" ]; then
            reason="rust_test_manifest_change_runs_focused_validation"
          fi
          ;;
        adl/src/*|adl/tests/*|adl/Cargo.toml|adl/Cargo.lock|adl/build.rs)
          if [ "$release_version_only" = true ] && { [ "$path" = "adl/Cargo.toml" ] || [ "$path" = "adl/Cargo.lock" ]; }; then
            continue
          fi
          mark_pr_fast_coverage
          ;;
        demos/*|adl/tools/demo_*|adl/tools/test_demo_*)
          ci_contracts_required=true
          demo_smoke_required=true
          ;;
        .github/workflows/*|adl/tools/*)
          ci_contracts_required=true
          ;;
      esac
    done <<EOF
$changed_files
EOF
	    while IFS=$'\t' read -r _status path; do
	      [ -n "$path" ] || continue
	      if [ "$pvf_slow_proof_policy_change" = true ] && is_pvf_slow_proof_policy_surface "$path"; then
	        continue
	      fi
	      if is_full_coverage_policy_surface "$path"; then
        if is_reporting_only_coverage_workflow_change "$path"; then
          reason="coverage_reporting_workflow_change_skips_authoritative_coverage"
          continue
        fi
        if is_pvf_slow_proof_workflow_change "$path"; then
          ci_contracts_required=true
          if [ "$reason" = "path_policy_docs_or_tooling_only" ]; then
            reason="pvf_slow_proof_workflow_change_runs_contract_validation"
          fi
          continue
        fi
        if is_csdlc_evidence_namespace_policy_update "$path"; then
          if [ "$reason" = "path_policy_docs_or_tooling_only" ]; then
            reason="csdlc_evidence_namespace_policy_update_skips_authoritative_coverage"
          fi
          continue
        fi
        if [ "$coverage_required" = true ]; then
          mark_policy_surface_full_coverage \
            "pr_policy_surface_runtime_mixed" \
            "coverage_policy_surface_change_with_runtime_surface_runs_full_coverage"
        else
          mark_policy_surface_full_coverage \
            "pr_policy_surface_tooling_only" \
            "coverage_policy_surface_change_runs_bounded_authoritative_coverage"
        fi
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
emit "v0913_proof_required" "$v0913_proof_required"
emit "release_version_only" "$release_version_only"
emit "ci_contracts_required" "$ci_contracts_required"
emit "fail_closed" "$fail_closed"
emit "coverage_lane" "$coverage_lane"
emit "coverage_authority" "$coverage_authority"
emit "proof_validation_scope" "$proof_validation_scope"
emit "changed_count" "$changed_count"
emit "reason" "$reason"

printf '\nChanged path policy: %s\n' "$reason"
printf '  rust_required=%s\n' "$rust_required"
printf '  coverage_required=%s\n' "$coverage_required"
printf '  full_coverage_required=%s\n' "$full_coverage_required"
printf '  demo_smoke_required=%s\n' "$demo_smoke_required"
printf '  v0913_proof_required=%s\n' "$v0913_proof_required"
printf '  release_version_only=%s\n' "$release_version_only"
printf '  ci_contracts_required=%s\n' "$ci_contracts_required"
printf '  fail_closed=%s\n' "$fail_closed"
printf '  coverage_lane=%s\n' "$coverage_lane"
printf '  coverage_authority=%s\n' "$coverage_authority"
printf '  proof_validation_scope=%s\n' "$proof_validation_scope"
printf '  changed_count=%s\n' "$changed_count"
