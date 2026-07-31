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
      --bin adl --bin csdlc --bin adl-csdlc --bin adl-runtime --bin adl-review \
      --bin csm \
      --bin adl-pr-create --bin adl-pr-init --bin adl-pr-repair-issue-body \
      --bin adl-pr-run --bin adl-pr-doctor --bin adl-pr-ready \
      --bin adl-pr-preflight --bin adl-pr-finish --bin adl-pr-validation \
      --bin adl-pr-inventory --bin adl-pr-shepherd --bin adl-pr-closing-linkage \
      --bin adl-issue \
      --bin adl-pr-closeout \
      --bin adl-session --bin adl-process \
      --bin adl-prompt-template --bin adl-validate-structured-prompt \
      --bin adl-lint-prompt-spec --bin adl-remote \
      --bin adl-aws-remote-validation --bin adl-provider-adapter
  if [[ "$PRINT_PLAN" == "1" ]]; then
    return 0
  fi
  run_command "install stable owner binaries" \
    bash adl/tools/install_owner_binaries.sh --no-build
  export ADL_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/adl"
  export ADL_CSDLC_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/csdlc"
  export ADL_CSDLC_COMPAT_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/adl-csdlc"
  export ADL_RUNTIME_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/adl-runtime"
  export ADL_REVIEW_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/adl-review"
  export ADL_CSM_BIN="${ADL_OWNER_BIN_DIR:-$ROOT_DIR/.adl/bin}/csm"
  export ADL_PACKAGE_VERSION
  ADL_PACKAGE_VERSION="$(package_version)"
}

run_csdlc_lane() {
  run_command "C-SDLC owner command guidance" \
    bash adl/tools/test_cli_owner_command_guidance.sh
  run_command "C-SDLC wrapper migration contract" \
    bash adl/tools/test_cli_wrapper_migration_contract.sh
  run_command "C-SDLC editor adapter guidance" \
    bash adl/tools/test_editor_action.sh
  run_command "C-SDLC active command reference scan" \
    bash adl/tools/test_generate_active_command_reference_scan.sh
  run_command "C-SDLC prompt-template structure schemas" \
    python3 adl/tools/test_prompt_template_structure_schemas.py
  run_command "C-SDLC control-plane observability contract" \
    bash adl/tools/test_control_plane_observability.sh
}

run_runtime_lane() {
  run_command "runtime CSM binary availability contract" \
    bash adl/tools/test_ensure_csm_binary.sh
  run_command "runtime CSM binary availability guard" \
    bash adl/tools/ensure_csm_binary.sh --json
  run_command "runtime compatibility boundary" \
    bash adl/tools/test_adl_runtime_compatibility.sh
}

run_review_lane() {
  run_command "review compatibility boundary" \
    bash adl/tools/test_adl_review_compatibility.sh
}

if [[ "$PRINT_PLAN" != "1" ]]; then
  ADL_RUST_WARM_CACHE_SOURCE_TARGET="${ADL_OWNER_VALIDATION_WARM_SOURCE_TARGET:-}" \
  ADL_RUST_WARM_CACHE_DEST_TARGET="${CARGO_TARGET_DIR:-$ROOT_DIR/adl/target}" \
  ADL_RUST_WARM_CACHE_OUTPUT="${ADL_OWNER_VALIDATION_WARM_CACHE_OUTPUT:-$ROOT_DIR/adl/owner-validation-warm-cache.json}" \
    bash "$ROOT_DIR/adl/tools/rust_validation_warm_cache.sh"
fi

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
