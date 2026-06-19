#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/validation_inventory.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

json_one="$TMP/inventory-1.json"
json_two="$TMP/inventory-2.json"
md_one="$TMP/inventory-1.md"
md_two="$TMP/inventory-2.md"

bash "$SCRIPT" --format json >"$json_one"
bash "$SCRIPT" --format json >"$json_two"
diff -u "$json_one" "$json_two" >/dev/null

bash "$SCRIPT" --format markdown >"$md_one"
bash "$SCRIPT" --format markdown >"$md_two"
diff -u "$md_one" "$md_two" >/dev/null

portable_dir="$TMP/portable"
mkdir -p "$portable_dir"
(
  cd "$portable_dir"
  bash "$SCRIPT" --format json --json-out "$TMP/portable.json" --markdown-out "$TMP/portable.md" >/dev/null
)
[[ -s "$TMP/portable.json" ]]
[[ -s "$TMP/portable.md" ]]

python3 - <<'PY' "$json_one" "$md_one"
import json
import sys
from pathlib import Path

inventory = json.loads(Path(sys.argv[1]).read_text())
markdown = Path(sys.argv[2]).read_text()

assert inventory["schema_version"] == "adl.validation_inventory.v1"
assert inventory["manifest_path"] == "adl/config/validation_lane_selector.v0.91.6.json"
assert inventory["repo_root"] == "."
assert inventory["tracked_file_count"] > 0

rust = inventory["rust"]
assert rust["rust_lib_unit_tests"]["file_count"] > 0
assert rust["rust_integration_tests"]["file_count"] > 0
assert "slow-proof-tests" in inventory["feature_inventory"]
assert len(inventory["release_gate_surfaces"]) > 0
assert len(inventory["coverage_only_surfaces"]) > 0
assert len(inventory["manifest_backed_surfaces"]) > 0
assert isinstance(inventory["unknown_or_unclassified_surfaces"], list)
assert len(inventory["shell_validators"]) > 0
assert len(inventory["python_validators"]) > 0
assert len(inventory["demo_proof_surfaces"]) > 0
assert "## Shell validators" in markdown
assert "## Python validators" in markdown
assert "## Demo proof surfaces" in markdown

pr_finish_records = [
    record for record in inventory["all_surface_records"]
    if record["path"] == "adl/src/bin/adl_pr_finish.rs"
]
assert any(record["owner"] == "csdlc" for record in pr_finish_records)
csdlc_records = [
    record for record in inventory["all_surface_records"]
    if record["path"] == "adl/src/bin/adl_csdlc.rs"
]
assert any(record["owner"] == "csdlc" for record in csdlc_records)

demo_paths = {record["path"] for record in inventory["demo_proof_surfaces"]}
assert "adl/tools/demo_smoke_v07_story.sh" in demo_paths
assert all(
    record["path"] != "adl/tools/demo_smoke_v07_story.sh"
    for record in inventory["unknown_or_unclassified_surfaces"]
)
python_validator_paths = {record["path"] for record in inventory["python_validators"]}
assert "adl/tools/test_prompt_template_structure_schemas.py" in python_validator_paths
assert any(
    record["path"] == "adl/tools/test_prompt_template_structure_schemas.py"
    and record["classification_status"] == "partial"
    for record in inventory["unknown_or_unclassified_surfaces"]
)
assert any(
    record["path"] == "adl/Cargo.toml" and record["owner"] == "runtime"
    for record in inventory["slow_proof_surfaces"]
)

assert "## Rust test-bearing surfaces" in markdown
assert "## Unknown or unclassified surfaces" in markdown
assert "status=partial" in markdown
PY

echo "PASS test_validation_inventory"
