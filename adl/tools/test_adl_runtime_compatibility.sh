#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANIFEST="$ROOT_DIR/adl/Cargo.toml"
FIXTURE="$ROOT_DIR/adl/examples/v0-3-concurrency-fork-join.adl.yaml"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

assert_status_nonzero() {
  local status="$1" label="$2"
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed ($label): expected nonzero exit" >&2
    exit 1
  }
}

run_adl() {
  cargo run --quiet --manifest-path "$MANIFEST" --bin adl -- "$@"
}

run_runtime() {
  cargo run --quiet --manifest-path "$MANIFEST" --bin adl-runtime -- "$@"
}

help_output="$(run_runtime --help)"
assert_contains "adl-runtime - ADL runtime compatibility binary" "$help_output" "runtime help title"
assert_contains "adl-runtime run <adl.yaml>" "$help_output" "runtime run help"
assert_contains "C-SDLC issue work belongs to adl/tools/pr.sh run <issue>" "$help_output" "csdlc handoff help"

run_help_output="$(run_runtime run --help)"
assert_contains "adl-runtime run <adl.yaml>" "$run_help_output" "runtime run subcommand help"

version_output="$(run_runtime --version)"
assert_contains "$(cargo metadata --quiet --no-deps --format-version 1 --manifest-path "$MANIFEST" | python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["version"])')" "$version_output" "runtime version"

legacy_plan="$TMP_DIR/legacy-plan.txt"
runtime_plan="$TMP_DIR/runtime-plan.txt"
run_adl "$FIXTURE" --print-plan >"$legacy_plan"
run_runtime run "$FIXTURE" --print-plan >"$runtime_plan"
cmp "$legacy_plan" "$runtime_plan" || {
  echo "assertion failed: adl-runtime run output drifted from adl <yaml> compatibility shortcut" >&2
  exit 1
}

set +e
issue_output="$(run_runtime run 3598 2>&1)"
issue_status=$?
set -e
assert_status_nonzero "$issue_status" "runtime run issue id"
assert_contains "C-SDLC issue work belongs to adl/tools/pr.sh run <issue>" "$issue_output" "runtime issue handoff"

set +e
hash_issue_output="$(run_runtime run '#3598' 2>&1)"
hash_issue_status=$?
set -e
assert_status_nonzero "$hash_issue_status" "runtime run hash-prefixed issue id"
assert_contains "C-SDLC issue work belongs to adl/tools/pr.sh run <issue>" "$hash_issue_output" "runtime hash issue handoff"

set +e
pr_output="$(run_runtime pr run 3598 2>&1)"
pr_status=$?
set -e
assert_status_nonzero "$pr_status" "runtime pr ownership"
assert_contains "adl-runtime does not own C-SDLC workflow commands" "$pr_output" "runtime pr rejection"

set +e
tooling_output="$(run_runtime tooling prompt-template 2>&1)"
tooling_status=$?
set -e
assert_status_nonzero "$tooling_status" "runtime tooling ownership"
assert_contains "adl-runtime does not own C-SDLC workflow commands" "$tooling_output" "runtime tooling rejection"

echo "PASS test_adl_runtime_compatibility"
