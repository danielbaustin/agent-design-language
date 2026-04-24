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
It fails closed to the full nextest sweep unless every changed Rust surface
maps to a bounded focused filter expression.
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
    adl/src/*.rs|adl/tests/*.rs|adl/build.rs|\
    docs/default_workflow.md|\
    docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md)
      return 0
      ;;
  esac
  return 1
}

is_broad_rust_surface() {
  local path="$1"
  case "$path" in
    adl/build.rs|\
    adl/src/lib.rs|\
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
    adl/src/runtime_v2/mod.rs|adl/src/runtime_v2/tests.rs|adl/src/runtime_v2/validators.rs)
      printf 'runtime_v2'
      return 0
      ;;
    adl/src/runtime_v2/governed_episode/*.rs)
      printf 'governed_episode'
      return 0
      ;;
    adl/src/runtime_v2/*/*.rs|adl/src/runtime_v2/*/*/*.rs|adl/src/runtime_v2/*/*/*/*.rs)
      return 1
      ;;
    adl/src/runtime_v2/tests/*)
      basename "$path" .rs
      return 0
      ;;
    adl/src/runtime_v2/[^/]*.rs)
      basename "$path" .rs
      return 0
      ;;
    adl/src/cli/mod.rs|adl/src/cli/tests.rs)
      printf 'cli'
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
    adl/src/cli/tests/pr_cmd_inline/*/*|adl/src/cli/tests/pr_cmd_inline/*)
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
    adl/src/cli/[^/]*.rs|adl/src/cli/[^/]*/[^/]*.rs)
      basename "$path" .rs
      return 0
      ;;
    adl/src/demo/*/*/*/*.rs|adl/src/demo/*/*/*/*/*.rs)
      return 1
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

mode="full"
reason="ordinary_pr_fast_lane_fails_closed_to_full_nextest"
filter_tokens=""
filter_expression=""
rust_surface_count=0

declare -a tokens=()

while IFS= read -r path; do
  [ -n "$path" ] || continue
  if ! is_relevant_fast_lane_surface "$path"; then
    continue
  fi
  rust_surface_count=$((rust_surface_count + 1))
  if is_broad_rust_surface "$path"; then
    mode="full"
    reason="broad_rust_surface_requires_full_nextest"
    tokens=()
    break
  fi
  if ! token="$(filter_token_for_path "$path")"; then
    mode="full"
    reason="unmapped_rust_surface_requires_full_nextest"
    tokens=()
    break
  fi
  if ! token_already_seen "$token"; then
    tokens+=("$token")
  fi
done <<EOF
$(changed_rows \
  | normalize_changed_rows \
  | awk -F '\t' 'NF >= 2 { print $2 }')
EOF

if [ "$rust_surface_count" -eq 0 ]; then
  mode="full"
  reason="no_relevant_rust_surface_detected_for_fast_lane"
elif [ "${#tokens[@]}" -gt 0 ]; then
  if [ "${#tokens[@]}" -le 4 ]; then
    mode="focused"
    reason="bounded_rust_surface_runs_focused_nextest"
    filter_expression="$(build_filter_expression "${tokens[@]}")"
    filter_tokens="$(printf '%s\n' "${tokens[@]}" | paste -sd, -)"
  else
    mode="full"
    reason="too_many_focused_filters_require_full_nextest"
  fi
fi

emit "mode" "$mode"
emit "reason" "$reason"
emit "rust_surface_count" "$rust_surface_count"
emit "filter_tokens" "$filter_tokens"
emit "filter_expression" "$filter_expression"

if [ "$PRINT_PLAN" = true ]; then
  exit 0
fi

cd "$ROOT_DIR/adl"

if [ "$mode" = "focused" ]; then
  echo "Running focused nextest lane: $filter_expression"
  cargo nextest run --status-level all --final-status-level slow -E "$filter_expression"
else
  echo "Running full nextest lane: $reason"
  cargo nextest run --status-level all --final-status-level slow
fi
