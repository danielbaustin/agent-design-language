#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VALIDATION_MANAGER="$ROOT_DIR/adl/tools/validation_manager.py"
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
validation_profile_selected=""
validation_profile_status=""
validation_profile_pr_publication_sufficient=""
validation_profile_escalation_required=""
validation_profile_run_lanes=""
validation_profile_primary_reason=""
validation_profile_escalation_lanes=""
validation_profile_error=""
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

mark_policy_surface_contract_validation() {
  ci_contracts_required=true
  coverage_required=false
  full_coverage_required=false
  coverage_lane="skip"
  coverage_authority="not_required"
  reason="coverage_policy_surface_tooling_change_runs_contract_validation"
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

is_ci_path_policy_contract_surface() {
  local path="$1"
  case "$path" in
    adl/config/validation_lane_selector.v0.91.6.json|\
    adl/tools/ci_path_policy.sh|\
    adl/tools/test_ci_path_policy.sh|\
    adl/tools/test_validation_manager.sh)
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
              *"Full workspace gate deferred for bounded authoritative PR"*|\
              *"Workspace coverage gate and lcov artifact generation are deferred for pull requests."*|\
              *"This PR ran the authoritative coverage pass plus changed-source coverage-impact validation."*|\
              *"Full workspace coverage gating remains required on push-to-main, nightly ratchet, and non-PR fail-closed events."*|\
              *"Policy reason: "*|\
              *"Coverage lane: "*|\
              *"Coverage authority: "*|\
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

is_validation_profile_summary_workflow_change() {
  local path="$1"
  [ "$path" = ".github/workflows/ci.yaml" ] || return 1
  local diff_text
  diff_text="$(git_pr_patch "$path")"
  [ -n "$diff_text" ] || return 1
  local saw_summary=false
  while IFS= read -r line; do
    case "$line" in
      diff\ --git*|index\ *|@@*|---*|+++*)
        continue
        ;;
      +*|-*)
        local content="${line#?}"
        case "$content" in
          "      - name: Validation profile summary (adl-ci)"|\
          "      - name: Validation profile summary (adl-coverage)"|\
          "        run: |"|\
          "          {"|\
          "            echo \"## ADL validation profile\""|\
          "            echo \"## ADL coverage validation profile\""|\
          "            echo"|\
          "            echo \"| Field | Value |\""|\
          "            echo \"|---|---|\""|\
          "            echo \"| reason | "*|\
          "            echo \"| selected profile | "*|\
          "            echo \"| profile status | "*|\
          "            echo \"| PR publication sufficient | "*|\
          "            echo \"| coverage lane | "*|\
          "            echo \"| coverage authority | "*|\
          "            echo \"| profile run lanes | "*|\
          "            echo \"| escalation required | "*|\
          "            echo \"| escalation lanes | "*|\
          "          } >> \"\$GITHUB_STEP_SUMMARY\""|\
          "        working-directory: ."|\
          "")
            saw_summary=true
            ;;
          *)
            return 1
            ;;
        esac
        ;;
    esac
  done <<<"$diff_text"
  [ "$saw_summary" = true ]
}

is_validation_summary_and_reporting_workflow_change() {
  local path="$1"
  [ "$path" = ".github/workflows/ci.yaml" ] || return 1
  local diff_text
  diff_text="$(git_pr_patch "$path")"
  [ -n "$diff_text" ] || return 1
  local saw_summary=false
  while IFS= read -r line; do
    case "$line" in
      diff\ --git*|index\ *|@@*|---*|+++*)
        continue
        ;;
      +*|-*)
        local content="${line#?}"
        case "$content" in
          "      - name: Validation profile summary (adl-ci)"|\
          "      - name: Validation profile summary (adl-coverage)"|\
          "        if: github.event_name == 'pull_request' && steps.path-policy.outputs.full_coverage_required == 'true'"|\
          "        run: |"|\
          "          {"|\
          "            echo \"## ADL validation profile\""|\
          "            echo \"## ADL coverage validation profile\""|\
          "            echo"|\
          "            echo \"| Field | Value |\""|\
          "            echo \"|---|---|\""|\
          "            echo \"| reason | "*|\
          "            echo \"| selected profile | "*|\
          "            echo \"| profile status | "*|\
          "            echo \"| PR publication sufficient | "*|\
          "            echo \"| coverage lane | "*|\
          "            echo \"| coverage authority | "*|\
          "            echo \"| profile run lanes | "*|\
          "            echo \"| escalation required | "*|\
          "            echo \"| escalation lanes | "*|\
          "          } >> \"\$GITHUB_STEP_SUMMARY\""|\
          "        working-directory: ."|\
          "      - name: Full workspace gate deferred for bounded authoritative PR"|\
          "          echo \"Workspace coverage gate and lcov artifact generation are deferred for pull requests.\""|\
          "          echo \"This PR ran the authoritative coverage pass plus changed-source coverage-impact validation.\""|\
          "          echo \"Full workspace coverage gating remains required on push-to-main, nightly ratchet, and non-PR fail-closed events.\""|\
          "          echo \"Policy reason: \${{ steps.path-policy.outputs.reason }}\""|\
          "          echo \"Coverage lane: \${{ steps.path-policy.outputs.coverage_lane }}\""|\
          "          echo \"Coverage authority: \${{ steps.path-policy.outputs.coverage_authority }}\""|\
          "")
            case "$content" in
              *"Validation profile summary"*|*"ADL validation profile"*|*"ADL coverage validation profile"*)
                saw_summary=true
                ;;
            esac
            ;;
          *)
            return 1
            ;;
        esac
        ;;
    esac
  done <<<"$diff_text"
  [ "$saw_summary" = true ]
}

is_nightly_coverage_schedule_only_change() {
  local path="$1"
  [ "$path" = ".github/workflows/nightly-coverage-ratchet.yaml" ] || return 1
  git_pr_patch "$path" \
    | while IFS= read -r line; do
        case "$line" in
          diff\ --git*|index\ *|@@*|---*|+++*)
            continue
            ;;
          +*|-*)
            local content="${line#?}"
            case "$content" in
              "  schedule:"|\
              "    - cron: "*)
                ;;
              *)
                return 1
                ;;
            esac
            ;;
        esac
      done
}

is_pr_fast_coverage_workflow_change() {
  local path="$1"
  [ "$path" = ".github/workflows/ci.yaml" ] || return 1
  local diff_text
  diff_text="$(git_pr_patch "$path")"
  [ -n "$diff_text" ] || return 1
  grep -E 'Determine PR fast coverage filters|PR fast coverage summary \(json\)|coverage-impact-filters.txt|run_cli_smoke_process_status|run_finish_bins|generic_filters|summary_files|coverage-summary-process-status.json|coverage-summary-finish.json|coverage-summary-generic.json|coverage-summary.json|process_status|adl-pr-finish|jq -s ' <<<"$diff_text" >/dev/null 2>&1 || return 1
  if grep -E 'Coverage run and summary \(json\)|Coverage-impact changed-source gate|Enforce coverage policy gates|Coverage \(ADL Rust workspace lcov\)|Upload coverage artifact|Upload coverage to Codecov|run_authoritative_coverage_lane|full_coverage_required|coverage_authority=' <<<"$diff_text" >/dev/null 2>&1; then
    return 1
  fi
  return 0
}

is_bounded_pr_fast_coverage_policy_surface() {
  local path="$1"
  case "$path" in
    .github/workflows/ci.yaml|\
    adl/tools/check_coverage_impact.sh|\
    adl/tools/ci_path_policy.sh|\
    adl/tools/test_check_coverage_impact.sh|\
    adl/tools/test_ci_path_policy.sh|\
    adl/tools/test_ci_runtime_contracts.sh)
      return 0
      ;;
  esac
  return 1
}

is_bounded_pr_fast_coverage_policy_change() {
  local saw_bounded_marker=false
  local saw_other=false
  local path
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    if ! is_bounded_pr_fast_coverage_policy_surface "$path"; then
      if is_full_coverage_policy_surface "$path"; then
        saw_other=true
      fi
      continue
    fi
    case "$path" in
      .github/workflows/ci.yaml)
        if is_pr_fast_coverage_workflow_change "$path"; then
          saw_bounded_marker=true
        else
          saw_other=true
        fi
        ;;
      adl/tools/check_coverage_impact.sh)
        if git_pr_patch "$path" | grep -F 'adl/src/cli/process_cmd.rs' >/dev/null 2>&1; then
          saw_bounded_marker=true
        else
          saw_other=true
        fi
        ;;
      adl/tools/ci_path_policy.sh)
        if git_pr_patch "$path" | grep -E 'is_pr_fast_coverage_workflow_change|is_bounded_pr_fast_coverage_policy_surface|is_bounded_pr_fast_coverage_policy_change|bounded_pr_fast_coverage_policy_change_keeps_pr_fast_validation' >/dev/null 2>&1; then
          saw_bounded_marker=true
        else
          saw_other=true
        fi
        ;;
      adl/tools/test_check_coverage_impact.sh)
        if git_pr_patch "$path" | grep -F 'process_status' >/dev/null 2>&1; then
          saw_bounded_marker=true
        else
          saw_other=true
        fi
        ;;
      adl/tools/test_ci_path_policy.sh)
        if git_pr_patch "$path" | grep -F 'runtime-bounded-pr-fast-coverage-policy-change' >/dev/null 2>&1; then
          saw_bounded_marker=true
        else
          saw_other=true
        fi
        ;;
      adl/tools/test_ci_runtime_contracts.sh)
        if git_pr_patch "$path" | grep -F 'coverage-summary.json' >/dev/null 2>&1; then
          saw_bounded_marker=true
        else
          saw_other=true
        fi
        ;;
    esac
  done <<EOF
$changed_files
EOF
  [ "$saw_bounded_marker" = true ] && [ "$saw_other" = false ]
}

is_pvf_slow_proof_workflow_change() {
  local path="$1"
  [ "$path" = ".github/workflows/ci.yaml" ] || return 1
  local diff_text
  diff_text="$(git_pr_patch "$path")"
  [ -n "$diff_text" ] || return 1
  grep -E 'adl-slow-proof|test_slow_proof_lane_contract|run_slow_proof_family|slow-proof-tests|slow-proof-runtime|slow-proof-private-state|slow-proof-observatory|slow-proof-security|EXCLUDE_FROM_FILE_FLOOR_REGEX' <<<"$diff_text" >/dev/null 2>&1
}

is_pvf_slow_proof_runtime_manifest_change() {
  local path="$1"
  [ "$path" = "adl/src/runtime_v2/tests.rs" ] || return 1
  local diff_text
  diff_text="$(git_pr_patch "$path")"
  [ -n "$diff_text" ] || return 1
  grep -E 'slow-proof-tests|slow-proof-runtime|slow-proof-private-state|slow-proof-observatory|slow-proof-security|a2a_adapter_boundary|access_control|acip_hardening|challenge|citizen_state_substrate|contract_registry_accessors|delegation_subcontract' <<<"$diff_text" >/dev/null 2>&1
}

is_pvf_slow_proof_policy_surface() {
  local path="$1"
  case "$path" in
    .github/workflows/ci.yaml|\
    adl/src/runtime_v2/tests.rs|\
    adl/tools/ci_path_policy.sh|\
    adl/tools/run_authoritative_coverage_lane.sh|\
    adl/tools/run_slow_proof_family.sh|\
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
      adl/tools/run_slow_proof_family.sh|\
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

load_validation_manager_profile() {
  local tmp_dir changed_file_list
  tmp_dir="$(mktemp -d)"
  changed_file_list="$tmp_dir/changed-files.txt"
  printf '%s\n' "$changed_rows" >"$changed_file_list"
  if ! python3 - "$VALIDATION_MANAGER" "$changed_file_list" <<'PY' >"$tmp_dir/meta.txt" 2>"$tmp_dir/error.txt"
import json
import subprocess
import sys

manager, changed_files = sys.argv[1], sys.argv[2]
result = subprocess.run(
    ["python3", manager, "--changed-files", changed_files, "--json"],
    text=True,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    check=False,
)
if result.returncode != 0:
    sys.stderr.write(result.stderr)
    raise SystemExit(result.returncode)
profile = json.loads(result.stdout)
run_lanes = ",".join(item.get("lane_id", "") for item in profile.get("run", []))
primary_reason = ""
if profile.get("run"):
    primary_reason = profile["run"][0].get("reason", "")
elif profile.get("escalation", {}).get("reasons"):
    primary_reason = profile["escalation"]["reasons"][0].get("reason", "")
escalation_lanes = ",".join(
    item.get("lane_id", "") for item in profile.get("escalation", {}).get("reasons", [])
)
print(f"selected={profile.get('selected_profile', '')}")
print(f"status={profile.get('status', '')}")
print(
    f"pr_publication_sufficient={str(profile.get('pr_publication_sufficient', False)).lower()}"
)
print(
    f"escalation_required={str(profile.get('escalation', {}).get('required', False)).lower()}"
)
print(f"run_lanes={run_lanes}")
print(f"primary_reason={primary_reason}")
print(f"escalation_lanes={escalation_lanes}")
PY
  then
    validation_profile_error="$(tr '\n' ' ' <"$tmp_dir/error.txt" | sed 's/[[:space:]]\+/ /g; s/^ //; s/ $//')"
    if [ -z "$validation_profile_error" ]; then
      validation_profile_error="validation_manager_invocation_failed_without_error_output"
    fi
    rm -rf "$tmp_dir"
    return 1
  fi
  validation_profile_selected="$(awk -F= '$1=="selected"{print substr($0, index($0, "=")+1)}' "$tmp_dir/meta.txt")"
  validation_profile_status="$(awk -F= '$1=="status"{print substr($0, index($0, "=")+1)}' "$tmp_dir/meta.txt")"
  validation_profile_pr_publication_sufficient="$(awk -F= '$1=="pr_publication_sufficient"{print substr($0, index($0, "=")+1)}' "$tmp_dir/meta.txt")"
  validation_profile_escalation_required="$(awk -F= '$1=="escalation_required"{print substr($0, index($0, "=")+1)}' "$tmp_dir/meta.txt")"
  validation_profile_run_lanes="$(awk -F= '$1=="run_lanes"{print substr($0, index($0, "=")+1)}' "$tmp_dir/meta.txt")"
  validation_profile_primary_reason="$(awk -F= '$1=="primary_reason"{print substr($0, index($0, "=")+1)}' "$tmp_dir/meta.txt")"
  validation_profile_escalation_lanes="$(awk -F= '$1=="escalation_lanes"{print substr($0, index($0, "=")+1)}' "$tmp_dir/meta.txt")"
  rm -rf "$tmp_dir"
}

manager_profile_is_release_gate_only_escalation() {
  [ "$validation_profile_escalation_required" = true ] || return 1
  [ "$validation_profile_escalation_lanes" = "release_gate_review" ]
}

apply_validation_manager_routing() {
  case "$validation_profile_status:$validation_profile_run_lanes:$validation_profile_escalation_required" in
    ready_to_run:*ci_path_policy_contracts*:false)
      ci_contracts_required=true
      reason="${validation_profile_primary_reason:-ci_policy_surface_requires_path_policy_contract_checks}"
      return 0
      ;;
    ready_to_run:docs_diff_check:false)
      reason="${validation_profile_primary_reason:-docs_only_surface_requires_diff_hygiene}"
      return 0
      ;;
    ready_to_run:sprint_conductor_contracts:false|\
    ready_to_run:docs_diff_check,sprint_conductor_contracts:false)
      reason="sprint_conductor_surface_requires_helper_contract_checks"
      return 0
      ;;
    ready_to_run:rust_pr_fast:false)
      mark_pr_fast_coverage
      reason="${validation_profile_primary_reason:-bounded_rust_surface_runs_focused_nextest}"
      return 0
      ;;
  esac
  if [ "$validation_profile_status" = "escalation_required" ] && ! manager_profile_is_release_gate_only_escalation; then
    fail_closed=true
    mark_authoritative_full_coverage "fail_closed" "validation_manager_escalation_requires_authoritative_full_coverage"
    return 0
  fi
  return 1
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
    saw_full_coverage_policy_surface=false
    saw_v0913_proof_surface=false
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
        saw_pr_finish_control_plane=true
      fi
      if is_full_coverage_policy_surface "$path"; then
        if ! is_ci_path_policy_contract_surface "$path"; then
          saw_full_coverage_policy_surface=true
        fi
      fi
      if is_v0913_proof_surface "$path"; then
        saw_v0913_proof_surface=true
      fi
    done <<EOF
$changed_files
EOF
    used_validation_manager=false
    if [ "$release_version_only" != true ] && [ "$pvf_slow_proof_policy_change" != true ]; then
      if load_validation_manager_profile; then
        if [ "$saw_pr_finish_control_plane" != true ] && [ "$saw_full_coverage_policy_surface" != true ] && [ "$saw_v0913_proof_surface" != true ] && apply_validation_manager_routing; then
          used_validation_manager=true
        fi
      elif [ -n "$validation_profile_error" ]; then
        fail_closed=true
        mark_authoritative_full_coverage "fail_closed" "validation_manager_failed_closed_for_pull_request"
        used_validation_manager=true
      fi
    fi
    if [ "$used_validation_manager" != true ]; then
      while IFS= read -r path; do
        [ -n "$path" ] || continue
        if is_pr_finish_control_plane_surface "$path"; then
          rust_required=true
          ci_contracts_required=true
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
      bounded_pr_fast_coverage_policy_change=false
      if [ "$coverage_required" = true ] && is_bounded_pr_fast_coverage_policy_change; then
        bounded_pr_fast_coverage_policy_change=true
      fi
      while IFS=$'\t' read -r _status path; do
        [ -n "$path" ] || continue
        if [ "$pvf_slow_proof_policy_change" = true ] && is_pvf_slow_proof_policy_surface "$path"; then
          continue
        fi
        if [ "$bounded_pr_fast_coverage_policy_change" = true ] && is_bounded_pr_fast_coverage_policy_surface "$path"; then
          if [ "$reason" = "runtime_or_rust_test_change_runs_pr_fast_validation" ]; then
            reason="bounded_pr_fast_coverage_policy_change_keeps_pr_fast_validation"
          fi
          continue
        fi
        if is_full_coverage_policy_surface "$path"; then
          if is_validation_profile_summary_workflow_change "$path"; then
            reason="validation_profile_summary_workflow_change_skips_authoritative_coverage"
            continue
          fi
          if is_reporting_only_coverage_workflow_change "$path"; then
            reason="coverage_reporting_workflow_change_skips_authoritative_coverage"
            continue
          fi
          if is_nightly_coverage_schedule_only_change "$path"; then
            reason="nightly_coverage_schedule_only_change_skips_authoritative_coverage"
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
            mark_policy_surface_contract_validation
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
emit "validation_profile_selected" "$validation_profile_selected"
emit "validation_profile_status" "$validation_profile_status"
emit "validation_profile_pr_publication_sufficient" "$validation_profile_pr_publication_sufficient"
emit "validation_profile_escalation_required" "$validation_profile_escalation_required"
emit "validation_profile_run_lanes" "$validation_profile_run_lanes"
emit "validation_profile_primary_reason" "$validation_profile_primary_reason"
emit "validation_profile_escalation_lanes" "$validation_profile_escalation_lanes"
emit "validation_profile_error" "$validation_profile_error"

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
