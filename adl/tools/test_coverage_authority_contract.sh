#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
NIGHTLY="$ROOT_DIR/.github/workflows/nightly-coverage-ratchet.yaml"
AUTHORITATIVE="$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh"
FAST="$ROOT_DIR/adl/tools/run_pr_fast_coverage_lane.sh"
DOC="$ROOT_DIR/docs/tooling/COVERAGE_AUTHORITY_AND_RELEASE_PROOF.md"

grep -Fq 'EXCLUDE_FROM_FILE_FLOOR_REGEX: "^$"' "$NIGHTLY"
grep -Fq 'full_authoritative_default_features' "$AUTHORITATIVE"
grep -Fq 'bounded_policy_surface_pr' "$AUTHORITATIVE"
grep -Fq 'PR-fast coverage is non-authoritative' "$DOC"
grep -Fq 'nightly coverage is release-authoritative' "$DOC"
grep -Fq 'run_pr_fast_coverage_lane.sh' "$FAST"
grep -Fq 'bash tools/run_authoritative_coverage_lane.sh --authority nightly_main --event-name push' "$NIGHTLY"
if grep -Fq 'continue-on-error: true' "$NIGHTLY"; then
  echo "nightly coverage must fail when the authoritative lane fails" >&2
  exit 1
fi
grep -Fq 'adl-runtime/src/' "$NIGHTLY"

if grep -Fq 'EXCLUDE_FROM_FILE_FLOOR_REGEX: "^adl/' "$NIGHTLY"; then
  echo "nightly coverage must not exclude active per-file paths" >&2
  exit 1
fi

echo "PASS test_coverage_authority_contract"
