#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
FAMILY_CONFIG="$ADL_DIR/config/slow_proof_families.v0.91.6.json"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

default_list="$tmpdir/default-nextest-list.txt"
full_list="$tmpdir/full-nextest-list.txt"
family_matrix="$tmpdir/families.tsv"

python3 - "$FAMILY_CONFIG" >"$family_matrix" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text())
for family in payload["families"]:
    for sample in family.get("sample_tests", []):
        print(f"{family['id']}\t{family['feature']}\t{sample}")
PY

runtime_plan="$tmpdir/runtime-plan.json"
bash "$ROOT_DIR/adl/tools/run_slow_proof_family.sh" --family runtime --json >"$runtime_plan"
python3 - "$runtime_plan" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text())
assert payload["run_command"][:6] == [
    "cargo",
    "nextest",
    "run",
    "--lib",
    "--features",
    "slow-proof-runtime",
]
assert "runtime_v2_" in payload["run_command"]
PY

(
  cd "$ADL_DIR"
  cargo nextest list --lib runtime_v2_ > "$default_list"
  cargo nextest list --lib --features slow-proof-tests runtime_v2_ > "$full_list"
)

while IFS=$'\t' read -r family feature sample_test; do
  [ -n "$family" ] || continue
  family_list="$tmpdir/${family}-nextest-list.txt"
  bash "$ROOT_DIR/adl/tools/run_slow_proof_family.sh" --family "$family" --list >"$family_list"

  if grep -Fq "$sample_test" "$default_list"; then
    echo "slow proof test leaked into default PR lane: $sample_test" >&2
    exit 1
  fi
  if ! grep -Fq "$sample_test" "$family_list"; then
    echo "family '$family' missing expected slow-proof test: $sample_test" >&2
    exit 1
  fi
  if ! grep -Fq "$sample_test" "$full_list"; then
    echo "umbrella slow-proof lane missing family sample test: $sample_test" >&2
    exit 1
  fi

  while IFS=$'\t' read -r other_family _other_feature other_sample; do
    [ -n "$other_family" ] || continue
    if [ "$other_family" = "$family" ]; then
      continue
    fi
    if grep -Fq "$other_sample" "$family_list"; then
      echo "family '$family' leaked unrelated slow-proof test from '$other_family': $other_sample" >&2
      exit 1
    fi
  done <"$family_matrix"
done <"$family_matrix"

echo "PASS test_slow_proof_lane_contract"
