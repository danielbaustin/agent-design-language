#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASE_SHA=""
HEAD_SHA=""
CHANGED_FILES_FILE=""
GITHUB_OUTPUT_PATH=""
PRINT_PLAN=false

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_pr_fast_test_lane.sh [options]

Options:
  --base <sha>                 Base revision for changed-file detection.
  --head <sha>                 Head revision for changed-file detection.
  --changed-files <file>       Explicit changed-file list for tests. Lines may be
                               "path" or "STATUS<TAB>path".
  --github-output <path>       Emit key=value outputs for GitHub Actions.
  --print-plan                 Print the computed plan and exit.
  -h, --help                   Show this help.

This script selects the ordinary PR-fast non-coverage Rust test lane.
It prefers:
  1. focused nextest filters for small, precisely-mapped changes
  2. bounded family filters for broader-but-still-bounded changes
  3. the full nextest sweep only for broad or ambiguous changes
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --base)
      BASE_SHA="${2:-}"
      shift 2
      ;;
    --head)
      HEAD_SHA="${2:-}"
      shift 2
      ;;
    --changed-files)
      CHANGED_FILES_FILE="${2:-}"
      shift 2
      ;;
    --github-output)
      GITHUB_OUTPUT_PATH="${2:-}"
      shift 2
      ;;
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

emit() {
  local key="$1"
  local value="$2"
  printf '%s=%s\n' "$key" "$value"
  if [ -n "$GITHUB_OUTPUT_PATH" ]; then
    printf '%s=%s\n' "$key" "$value" >> "$GITHUB_OUTPUT_PATH"
  fi
}

normalize_changed_rows() {
  awk -F '\t' '
    NF == 1 { print "M\t" $1; next }
    $1 ~ /^R/ && NF >= 3 { print $1 "\t" $3; next }
    NF >= 2 { print $1 "\t" $2; next }
  '
}

changed_rows() {
  if [ -n "$CHANGED_FILES_FILE" ]; then
    cat "$CHANGED_FILES_FILE"
    return
  fi
  if [ -z "$BASE_SHA" ] || [ -z "$HEAD_SHA" ]; then
    echo "run_pr_fast_test_lane: --base and --head are required unless --changed-files is supplied" >&2
    exit 2
  fi
  git -C "$ROOT_DIR" diff --name-status --diff-filter=ACMR "$BASE_SHA...$HEAD_SHA" 2>/dev/null \
    || git -C "$ROOT_DIR" diff --name-status --diff-filter=ACMR "$BASE_SHA" "$HEAD_SHA" 2>/dev/null \
    || true
}

is_relevant_fast_lane_surface() {
  local path="$1"
  case "$path" in
    adl/src/*.rs|adl/tests/*.rs|adl/build.rs|adl/Cargo.toml|adl/Cargo.lock|\
    docs/default_workflow.md|\
    docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md)
      return 0
      ;;
  esac
  return 1
}

file_is_structural_module_barrel() {
  local path="$1"
  [ -f "$ROOT_DIR/$path" ] || return 1

  awk '
    /^[[:space:]]*$/ { next }
    /^[[:space:]]*\/\// { next }
    /^[[:space:]]*#\[/ { next }
    /^[[:space:]]*(pub[[:space:]]+)?mod[[:space:]]+[A-Za-z_][A-Za-z0-9_]*;[[:space:]]*$/ { next }
    /^[[:space:]]*(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?use[[:space:]].*;[[:space:]]*$/ { next }
    { exit 1 }
  ' "$ROOT_DIR/$path"
}

is_structural_companion_surface() {
  local path="$1"
  case "$path" in
    adl/src/lib.rs)
      file_is_structural_module_barrel "$path"
      return $?
      ;;
  esac
  return 1
}

is_broad_rust_surface() {
  local path="$1"
  case "$path" in
    adl/build.rs|\
    adl/src/main.rs|\
    adl/src/adl.rs|\
    adl/src/schema.rs|\
    adl/src/demo.rs|\
    adl/tests/*)
      return 0
      ;;
    adl/src/runtime_v2/mod.rs|\
    adl/src/runtime_v2/tests.rs|\
    adl/src/runtime_v2/validators.rs|\
    adl/src/cli/mod.rs|\
    adl/src/cli/tests.rs)
      return 1
      ;;
    */mod.rs)
      return 0
      ;;
  esac
  return 1
}

filter_token_for_path() {
  local path="$1"
  case "$path" in
    adl/src/lib.rs)
      return 1
      ;;
    adl/src/acc.rs|adl/src/acc/*.rs)
      printf 'acc'
      return 0
      ;;
    adl/src/runtime_v2/mod.rs|adl/src/runtime_v2/tests.rs|adl/src/runtime_v2/validators.rs)
      printf 'runtime_v2'
      return 0
      ;;
    adl/src/runtime_v2/cultivating_intelligence.rs|adl/src/runtime_v2/cultivating_intelligence_parts/*.rs)
      printf 'cultivating_intelligence'
      return 0
      ;;
    adl/src/runtime_v2/wellbeing_metrics.rs|adl/src/runtime_v2/wellbeing_metrics_parts/*.rs)
      printf 'wellbeing_metrics'
      return 0
      ;;
    adl/src/runtime_v2/governed_episode/*.rs)
      printf 'governed_episode'
      return 0
      ;;
    adl/src/runtime_v2/private_state_sanctuary/*.rs)
      printf 'private_state_sanctuary'
      return 0
      ;;
    adl/src/runtime_v2/tests/common.rs)
      return 1
      ;;
    adl/src/runtime_v2/tests/*)
      basename "$path" .rs
      return 0
      ;;
    adl/src/runtime_v2/*/*.rs|adl/src/runtime_v2/*/*/*.rs|adl/src/runtime_v2/*/*/*/*.rs)
      return 1
      ;;
    adl/src/runtime_v2/[^/]*.rs)
      basename "$path" .rs
      return 0
      ;;
    adl/src/cli/mod.rs|adl/src/cli/tests.rs)
      printf 'cli'
      return 0
      ;;
    adl/src/csdlc_prompt_editor.rs|adl/src/csdlc_prompt_editor/*.rs)
      printf 'csdlc_prompt_editor'
      return 0
      ;;
    adl/src/cli/identity_cmd/*.rs|adl/src/cli/identity_cmd/tests.rs|adl/src/cli/identity_cmd/tests/*.rs)
      printf 'identity_cmd'
      return 0
      ;;
    adl/src/cli/tests/internal_commands/*.rs)
      printf 'internal_commands'
      return 0
      ;;
    adl/src/cli/tests/artifact_builders/*.rs|adl/src/cli/tests/artifact_builders/*/*.rs)
      printf 'artifact_builders'
      return 0
      ;;
    adl/src/cli/tests/run_state/*.rs)
      printf 'run_state'
      return 0
      ;;
    adl/src/cli/run_artifacts/runtime/*.rs)
      printf 'run_state'
      return 0
      ;;
    adl/src/cli/tests/pr_cmd_inline/*/*|adl/src/cli/tests/pr_cmd_inline/*)
      printf 'pr_cmd'
      return 0
      ;;
    adl/src/cli/pr_cmd_cards.rs|adl/src/cli/pr_cmd_cards/*.rs)
      printf 'pr_cmd'
      return 0
      ;;
    adl/src/cli/tests/tooling_cmd*|adl/src/cli/tooling_cmd*|adl/src/cli/tests/tooling_cmd/*)
      printf 'tooling_cmd'
      return 0
      ;;
    adl/src/cli/pr_cmd*|adl/src/cli/tests/pr_cmd*|adl/src/cli/pr_cmd/*)
      printf 'pr_cmd'
      return 0
      ;;
    docs/default_workflow.md|docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md)
      printf 'pr_cmd'
      return 0
      ;;
    adl/src/cli/*/*/*.rs|adl/src/cli/*/*/*/*.rs)
      return 1
      ;;
    adl/src/uts_acc_compiler.rs|adl/src/uts_acc_compiler/*.rs)
      printf 'uts_acc_compiler'
      return 0
      ;;
    adl/src/cli/[^/]*.rs|adl/src/cli/[^/]*/[^/]*.rs)
      basename "$path" .rs
      return 0
      ;;
    adl/src/demo/*/*/*/*.rs|adl/src/demo/*/*/*/*/*.rs)
      return 1
      ;;
    adl/src/bin/adl_lint_prompt_spec.rs|\
    adl/src/bin/adl_prompt_template.rs|\
    adl/src/bin/adl_validate_structured_prompt.rs)
      printf 'tooling_cmd'
      return 0
      ;;
    adl/src/bin/[^/]*.rs)
      basename "$path" .rs
      return 0
      ;;
    adl/src/demo/[^/]*.rs|adl/src/demo/[^/]*/[^/]*.rs|adl/src/demo/[^/]*/[^/]*/[^/]*.rs)
      basename "$path" .rs
      return 0
      ;;
    adl/src/*/*)
      return 1
      ;;
    adl/src/[^/]*.rs)
      basename "$path" .rs
      return 0
      ;;
  esac
  return 1
}

family_token_for_path() {
  local path="$1"
  case "$path" in
    adl/src/lib.rs)
      return 1
      ;;
    adl/src/runtime_v2/*)
      printf 'runtime_v2'
      return 0
      ;;
    adl/src/cli/*)
      printf 'cli'
      return 0
      ;;
    adl/src/demo/*)
      printf 'demo'
      return 0
      ;;
  esac
  return 1
}

is_manifest_only_rust_wave() {
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
        return 1
        ;;
    esac
  done <<EOF
$(changed_rows \
  | normalize_changed_rows \
  | awk -F '\t' 'NF >= 2 { print $2 }')
EOF

  [ "$saw_manifest" = true ] || [ "$saw_lock" = true ]
}

build_filter_expression() {
  python3 - "$@" <<'PY'
import sys

tokens = [token for token in sys.argv[1:] if token]
if not tokens:
    raise SystemExit(1)
print(" or ".join(f"test({token})" for token in tokens))
PY
}

token_already_seen() {
  local needle="$1"
  local token
  for token in "${tokens[@]:-}"; do
    if [ "$token" = "$needle" ]; then
      return 0
    fi
  done
  return 1
}

family_token_already_seen() {
  local needle="$1"
  local token
  for token in "${family_tokens[@]:-}"; do
    if [ "$token" = "$needle" ]; then
      return 0
    fi
  done
  return 1
}

mode="full"
reason="ordinary_pr_fast_lane_fails_closed_to_full_nextest"
filter_tokens=""
filter_expression=""
rust_surface_count=0
structural_surface_count=0
slow_proof_inventory_surface_count=0
all_paths_have_precise_token=true
all_paths_have_family_token=true
classification_locked=false
saw_slow_proof_contract_surface=false

declare -a tokens=()
declare -a family_tokens=()

while IFS= read -r path; do
  [ -n "$path" ] || continue
  case "$path" in
    .github/workflows/ci.yaml|\
    adl/tools/test_slow_proof_lane_contract.sh|\
    adl/tools/ci_path_policy.sh|\
    docs/milestones/v0.91.4/features/PVF_INITIAL_LANE_INVENTORY_v0.91.4.md|\
    docs/milestones/v0.91.4/features/PVF_CI_RELEASE_POLICY_v0.91.4.md)
      saw_slow_proof_contract_surface=true
      ;;
  esac
done <<EOF
$(changed_rows \
  | normalize_changed_rows \
  | awk -F '\t' 'NF >= 2 { print $2 }')
EOF

while IFS= read -r path; do
  [ -n "$path" ] || continue
  if ! is_relevant_fast_lane_surface "$path"; then
    continue
  fi
  if is_structural_companion_surface "$path"; then
    structural_surface_count=$((structural_surface_count + 1))
    continue
  fi
  rust_surface_count=$((rust_surface_count + 1))
  if [ "$path" = "adl/src/runtime_v2/tests.rs" ] && [ "$saw_slow_proof_contract_surface" = true ]; then
    slow_proof_inventory_surface_count=$((slow_proof_inventory_surface_count + 1))
    continue
  fi
  if is_broad_rust_surface "$path"; then
    mode="full"
    reason="broad_rust_surface_requires_full_nextest"
    classification_locked=true
    tokens=()
    family_tokens=()
    break
  fi
  if token="$(filter_token_for_path "$path")"; then
    if ! token_already_seen "$token"; then
      tokens+=("$token")
    fi
  else
    all_paths_have_precise_token=false
  fi
  if family_token="$(family_token_for_path "$path")"; then
    if ! family_token_already_seen "$family_token"; then
      family_tokens+=("$family_token")
    fi
  else
    all_paths_have_family_token=false
  fi
done <<EOF
$(changed_rows \
  | normalize_changed_rows \
  | awk -F '\t' 'NF >= 2 { print $2 }')
EOF

if [ "$classification_locked" = true ]; then
  :
elif is_manifest_only_rust_wave; then
  mode="focused"
  reason="manifest_only_rust_wave_runs_focused_nextest"
  filter_expression="$(build_filter_expression "pr_cmd::github" "github_release_" "long_lived_agent")"
  filter_tokens="pr_cmd::github,github_release_,long_lived_agent"
elif [ "$slow_proof_inventory_surface_count" -gt 0 ] && [ "$rust_surface_count" -eq "$slow_proof_inventory_surface_count" ]; then
  mode="contract_only"
  reason="slow_proof_inventory_change_covered_by_contract_check"
elif [ "$rust_surface_count" -eq 0 ]; then
  mode="full"
  reason="no_relevant_rust_surface_detected_for_fast_lane"
elif [ "$all_paths_have_precise_token" = true ] && [ "${#tokens[@]}" -gt 0 ]; then
  if [ "${#tokens[@]}" -le 4 ]; then
    mode="focused"
    reason="bounded_rust_surface_runs_focused_nextest"
    filter_expression="$(build_filter_expression "${tokens[@]}")"
    filter_tokens="$(printf '%s\n' "${tokens[@]}" | paste -sd, -)"
  elif [ "$all_paths_have_family_token" = true ] && [ "${#family_tokens[@]}" -gt 0 ] && [ "${#family_tokens[@]}" -le 3 ]; then
    mode="family"
    reason="bounded_rust_surface_runs_family_nextest"
    filter_expression="$(build_filter_expression "${family_tokens[@]}")"
    filter_tokens="$(printf '%s\n' "${family_tokens[@]}" | paste -sd, -)"
  else
    mode="full"
    reason="too_many_focused_filters_require_full_nextest"
  fi
elif [ "$all_paths_have_family_token" = true ] && [ "${#family_tokens[@]}" -gt 0 ]; then
  if [ "${#family_tokens[@]}" -le 3 ]; then
    mode="family"
    reason="bounded_family_surface_runs_family_nextest"
    filter_expression="$(build_filter_expression "${family_tokens[@]}")"
    filter_tokens="$(printf '%s\n' "${family_tokens[@]}" | paste -sd, -)"
  else
    mode="full"
    reason="too_many_family_filters_require_full_nextest"
  fi
else
  mode="full"
  reason="unmapped_rust_surface_requires_full_nextest"
fi

emit "mode" "$mode"
emit "reason" "$reason"
emit "rust_surface_count" "$rust_surface_count"
emit "structural_surface_count" "$structural_surface_count"
emit "slow_proof_inventory_surface_count" "$slow_proof_inventory_surface_count"
emit "filter_tokens" "$filter_tokens"
emit "filter_expression" "$filter_expression"

if [ "$PRINT_PLAN" = true ]; then
  exit 0
fi

cd "$ROOT_DIR/adl"

if [ "$mode" = "focused" ] || [ "$mode" = "family" ]; then
  echo "Running $mode nextest lane: $filter_expression"
  cargo nextest run --status-level all --final-status-level slow -E "$filter_expression"
elif [ "$mode" = "contract_only" ]; then
  echo "Skipping ordinary nextest lane: slow-proof inventory change is covered by the slow-proof lane contract."
else
  echo "Running full nextest lane: $reason"
  cargo nextest run --status-level all --final-status-level slow
fi
