#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANIFEST="$ROOT_DIR/adl/Cargo.toml"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
ADL_BIN="${ADL_BIN:-}"
ADL_REVIEW_BIN="${ADL_REVIEW_BIN:-}"
ADL_PACKAGE_VERSION="${ADL_PACKAGE_VERSION:-}"

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
  if [[ -n "$ADL_BIN" ]]; then
    "$ADL_BIN" "$@"
    return
  fi
  cargo run --quiet --manifest-path "$MANIFEST" --bin adl -- "$@"
}

run_review() {
  if [[ -n "$ADL_REVIEW_BIN" ]]; then
    "$ADL_REVIEW_BIN" "$@"
    return
  fi
  cargo run --quiet --manifest-path "$MANIFEST" --bin adl-review -- "$@"
}

package_version() {
  if [[ -n "$ADL_PACKAGE_VERSION" ]]; then
    printf '%s\n' "$ADL_PACKAGE_VERSION"
    return
  fi
  cargo metadata --quiet --no-deps --format-version 1 --manifest-path "$MANIFEST" | python3 -c 'import json,sys; print(json.load(sys.stdin)["packages"][0]["version"])'
}

help_output="$(run_review --help)"
assert_contains "adl-review - ADL review tooling compatibility binary" "$help_output" "review help title"
assert_contains "adl-review code-review --out <dir>" "$help_output" "review code-review help"
assert_contains "C-SDLC issue work belongs to adl/tools/pr.sh run <issue>" "$help_output" "csdlc handoff help"

version_output="$(run_review --version)"
expected_version="$(package_version)"
assert_contains "$expected_version" "$version_output" "review version"

review_fixture="$TMP_DIR/review.md"
cat >"$review_fixture" <<'EOF'
# Repository Review

## Metadata

- Review Type: fixture
- Subject: adl-review compatibility
- Reviewer: fixture

## Scope

- Reviewed: review compatibility surface
- Not Reviewed: runtime behavior
- Review Mode: fixture
- Gate: non-blocking

## Findings

No material findings.

## System-Level Assessment

The review packet is structurally valid for compatibility smoke coverage.

## Recommended Action Plan

- Fix now: none
- Fix before milestone closeout: none
- Defer: none

## Follow-ups / Deferred Work

None.

## Final Assessment

Pass.
EOF

legacy_out="$TMP_DIR/legacy-review-contract.txt"
review_out="$TMP_DIR/review-contract.txt"
run_adl tooling verify-repo-review-contract --review "$review_fixture" >"$legacy_out"
run_review verify-repo-contract --review "$review_fixture" >"$review_out"
cmp "$legacy_out" "$review_out" || {
  echo "assertion failed: adl-review verify-repo-contract output drifted from legacy tooling command" >&2
  exit 1
}

set +e
issue_output="$(run_review pr run 3599 2>&1)"
issue_status=$?
set -e
assert_status_nonzero "$issue_status" "review pr ownership"
assert_contains "review tooling only" "$issue_output" "review issue handoff"

set +e
runtime_output="$(run_review run workflow.adl.yaml 2>&1)"
runtime_status=$?
set -e
assert_status_nonzero "$runtime_status" "review runtime ownership"
assert_contains "does not run ADL runtime commands" "$runtime_output" "review runtime rejection"

echo "PASS test_adl_review_compatibility"
