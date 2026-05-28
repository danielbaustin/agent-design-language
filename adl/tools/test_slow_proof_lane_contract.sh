#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

default_list="$tmpdir/default-nextest-list.txt"
slow_list="$tmpdir/slow-proof-nextest-list.txt"

slow_tests=(
  "runtime_v2_a2a_adapter_boundary_contract_is_stable"
  "runtime_v2_acip_hardening_contract_is_stable"
  "runtime_v2_citizen_state_substrate_contract_is_stable"
  "runtime_v2_continuity_challenge_contract_is_stable"
  "runtime_v2_delegation_subcontract_contract_and_authority_matrix_is_stable"
)

(
  cd "$ADL_DIR"
  cargo nextest list runtime_v2_ > "$default_list"
  cargo nextest list --features slow-proof-tests runtime_v2_ > "$slow_list"
)

for test_name in "${slow_tests[@]}"; do
  if grep -Fq "$test_name" "$default_list"; then
    echo "slow proof test leaked into default PR lane: $test_name" >&2
    exit 1
  fi
  if ! grep -Fq "$test_name" "$slow_list"; then
    echo "slow proof test missing from explicit slow-proof lane: $test_name" >&2
    exit 1
  fi
done

echo "PASS test_slow_proof_lane_contract"
