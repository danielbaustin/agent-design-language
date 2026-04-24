#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "$ROOT_DIR/.tmp-contract-market-runner.XXXXXX")"
OUT_ONE_REL="$(basename "$TMP_DIR")/run_one"
OUT_TWO_REL="$(basename "$TMP_DIR")/run_two"
OUT_ONE="$ROOT_DIR/$OUT_ONE_REL"
OUT_TWO="$ROOT_DIR/$OUT_TWO_REL"
trap 'rm -rf "$TMP_DIR"' EXIT

cd "$ROOT_DIR"

python3 adl/tools/run_v0904_contract_market_runner.py --out "$OUT_ONE_REL"
python3 adl/tools/run_v0904_contract_market_runner.py --out "$OUT_TWO_REL"

diff -ru "$OUT_ONE" "$OUT_TWO"

test -f "$OUT_ONE/runner_manifest.json"
test -f "$OUT_ONE/transition_report.json"
test -f "$OUT_ONE/negative_case_results.json"
test -f "$OUT_ONE/review_bundle.json"

python3 - "$OUT_ONE" <<'PY'
import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
manifest = json.loads((out / "runner_manifest.json").read_text())
transitions = json.loads((out / "transition_report.json").read_text())
negative = json.loads((out / "negative_case_results.json").read_text())
review = json.loads((out / "review_bundle.json").read_text())

assert manifest["schema"] == "adl.v0904.contract_market.runner_manifest.v1"
assert manifest["proof_classification"] == "contract_market_substrate_only"
assert manifest["governed_tool_proof"] is False

executed = transitions["executed_transitions"]
assert [entry["to_state"] for entry in executed] == [
    "awarded",
    "accepted",
    "executing",
    "completed",
]

results = {entry["reason_code"] for entry in negative["results"]}
assert "tool_execution_authority_forbidden" in results
assert "missing_required_artifact_refs" in results

assert review["scope"]["classification"] == "contract_market_substrate"
assert review["scope"]["governed_tool_proof"] is False
assert review["tool_boundary"]["execution_status"] == "refused_without_governed_authority"

for path in out.glob("*.json"):
    text = path.read_text()
    assert "/Users/" not in text
    assert "file://" not in text
    assert "/private/" not in text
    assert "/var/" not in text
    assert "tool_arguments" not in text
    assert "prompt text" not in text
PY

echo "v0.90.4 contract-market runner smoke: pass"
