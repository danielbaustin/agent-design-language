#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/summarize_nextest_timings.py"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_has() {
  local haystack="$1"
  local needle="$2"
  if ! grep -Fq "$needle" <<<"$haystack"; then
    echo "expected output to contain: $needle" >&2
    echo "actual output:" >&2
    echo "$haystack" >&2
    exit 1
  fi
}

fixture="$TMP/nextest.log"
cat >"$fixture" <<'EOF'
    Starting 5 tests across 2 binaries
        PASS [ 181.281s] ( 1/5) adl runtime_v2::tests::a2a_adapter_boundary::runtime_v2_a2a_adapter_boundary_contract_registry_smoke_covers_accessors
        SLOW [> 60.000s] (─────────) adl runtime_v2::tests::theory_of_mind_foundation::runtime_v2_theory_of_mind_foundation_write_to_root_materializes_fixture
        PASS [  75.270s] ( 2/5) adl runtime_v2::tests::theory_of_mind_foundation::runtime_v2_theory_of_mind_foundation_write_to_root_materializes_fixture
        PASS [   0.011s] ( 3/5) adl acc::tests::acc_v1_rejects_hidden_delegation
        PASS [  49.000s] ( 4/5) adl runtime_v2::tests::observatory_flagship::runtime_v2_observatory_flagship_review_bundle_matches_tracked_artifacts
        PASS [   2.443s] ( 5/5) adl::bin/adl cli::tests::godel::real_godel_inspect_reads_persisted_runtime_artifacts
EOF

markdown_output="$(python3 "$SCRIPT" "$fixture" --top 5 --min-seconds 1)"
assert_has "$markdown_output" "Declared nextest run: 5 tests across 2 binaries"
assert_has "$markdown_output" "Total parsed completed runtime: 308.005s"
assert_has "$markdown_output" 'runtime-v2-contract-registry/accessors'
assert_has "$markdown_output" 'proof-materialization'
assert_has "$markdown_output" "Route to Runtime v2 registry/accessor refactor or authoritative slow-proof consolidation."
assert_has "$markdown_output" "Route to proof-materialization slow-lane review; keep one authoritative root proof where needed."

json_output="$(python3 "$SCRIPT" "$fixture" --top 2 --format json)"
assert_has "$json_output" '"completed_count": 5'
assert_has "$json_output" '"slow_marker_count": 1'
assert_has "$json_output" '"declared_tests": 5'
assert_has "$json_output" '"cluster": "runtime-v2-contract-registry/accessors"'

echo "PASS test_summarize_nextest_timings"
