#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANIFEST="$ROOT_DIR/adl/Cargo.toml"
SURFACE=""
BUILD=0
PRINT_PLAN=0

usage() {
  cat <<'EOF' >&2
Usage:
  adl/tools/run_owner_validation_lane.sh <csdlc|runtime|review|all> [--build] [--print-plan]

Runs the focused validation lane for one CLI owner surface.

Options:
  --build       Build owner binaries once, then run compatibility scripts
                through prebuilt binary overrides instead of repeated
                `cargo run` startup.
  --print-plan  Print the commands that would run without executing them.
EOF
}

die() {
  printf 'run_owner_validation_lane: %s\n' "$*" >&2
  exit 2
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    csdlc|runtime|review|all)
      [[ -z "$SURFACE" ]] || die "surface already set to '$SURFACE'"
      SURFACE="$1"
      ;;
    --build)
      BUILD=1
      ;;
    --print-plan)
      PRINT_PLAN=1
      ;;
    -h|--help|help)
      usage
      exit 0
      ;;
    *)
      die "unsupported argument '$1'"
      ;;
  esac
  shift
done

[[ -n "$SURFACE" ]] || {
  usage
  exit 2
}

package_version() {
  cargo metadata --quiet --no-deps --format-version 1 --manifest-path "$MANIFEST" |
    python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["version"])'
}

emit_command() {
  printf '%s\n' "$*"
}

run_command() {
  local label="$1"
  shift
  emit_command "==> $label"
  if [[ "$PRINT_PLAN" == "1" ]]; then
    emit_command "$*"
    return
  fi
  (
    cd "$ROOT_DIR"
    "$@"
  )
}

build_owner_bins() {
  [[ "$BUILD" == "1" ]] || return 0
  run_command "cargo build owner binaries" \
    cargo build --quiet --manifest-path "$MANIFEST" \
      --bin adl --bin adl-csdlc --bin adl-runtime --bin adl-review \
      --bin adl-pr-create --bin adl-pr-init --bin adl-pr-repair-issue-body \
      --bin adl-pr-run --bin adl-pr-doctor --bin adl-pr-ready \
      --bin adl-pr-preflight --bin adl-pr-finish --bin adl-pr-validation \
      --bin adl-pr-inventory --bin adl-pr-closing-linkage \
      --bin adl-issue \
      --bin adl-pr-closeout \
      --bin adl-prompt-template --bin adl-validate-structured-prompt
  if [[ "$PRINT_PLAN" == "1" ]]; then
    return 0
  fi
  export ADL_BIN="$ROOT_DIR/adl/target/debug/adl"
  export ADL_CSDLC_BIN="$ROOT_DIR/adl/target/debug/adl-csdlc"
  export ADL_RUNTIME_BIN="$ROOT_DIR/adl/target/debug/adl-runtime"
  export ADL_REVIEW_BIN="$ROOT_DIR/adl/target/debug/adl-review"
  export ADL_PACKAGE_VERSION
  ADL_PACKAGE_VERSION="$(package_version)"
}

run_csdlc_lane() {
  run_command "C-SDLC owner command guidance" \
    bash adl/tools/test_cli_owner_command_guidance.sh
  run_command "C-SDLC wrapper migration contract" \
    bash adl/tools/test_cli_wrapper_migration_contract.sh
  run_command "C-SDLC run ambiguity policy" \
    bash adl/tools/test_pr_run_ambiguity_policy.sh
  run_command "C-SDLC PR small-binary delegation" \
    bash adl/tools/test_pr_small_binary_delegation.sh
  run_command "C-SDLC PR PATH-binary delegation" \
    bash adl/tools/test_pr_delegate_prefers_path_binary.sh
  run_command "C-SDLC PR delegate cargo fallback liveness" \
    bash adl/tools/test_pr_delegate_cargo_fallback_liveness.sh
  run_command "C-SDLC prompt-template wrappers avoid implicit cargo" \
    bash adl/tools/test_prompt_template_wrappers_no_implicit_cargo.sh
  run_command "C-SDLC prompt-template workflow integration" \
    bash adl/tools/test_prompt_template_workflow_integration.sh
  run_command "C-SDLC PR locked Cargo fallback" \
    bash adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh
  run_command "C-SDLC control-plane observability contract" \
    bash adl/tools/test_control_plane_observability.sh
}

run_runtime_lane() {
  run_command "runtime compatibility boundary" \
    bash adl/tools/test_adl_runtime_compatibility.sh
}

run_review_lane() {
  run_command "review compatibility boundary" \
    bash adl/tools/test_adl_review_compatibility.sh
}

build_owner_bins

case "$SURFACE" in
  csdlc)
    run_csdlc_lane
    ;;
  runtime)
    run_runtime_lane
    ;;
  review)
    run_review_lane
    ;;
  all)
    run_csdlc_lane
    run_runtime_lane
    run_review_lane
    ;;
esac

emit_command "PASS run_owner_validation_lane surface=$SURFACE"
