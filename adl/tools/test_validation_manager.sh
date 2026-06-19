#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/validation_manager.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_has() {
  local file="$1"
  local needle="$2"
  if ! grep -F -- "$needle" "$file" >/dev/null; then
    echo "expected $file to contain: $needle" >&2
    echo "actual output:" >&2
    cat "$file" >&2
    exit 1
  fi
}

docs_only="$TMP/docs-only.txt"
printf 'M\tdocs/milestones/v0.91.6/README.md\n' >"$docs_only"
bash "$SCRIPT" --changed-files "$docs_only" >"$TMP/docs.out"
assert_has "$TMP/docs.out" "selected_profile=docs_diff_check_profile"
assert_has "$TMP/docs.out" "status=ready_to_run"
assert_has "$TMP/docs.out" "lane=docs_diff_check"
assert_has "$TMP/docs.out" "behavior_surfaces:"
assert_has "$TMP/docs.out" "id=diff_hygiene_docs_diff_check"
assert_has "$TMP/docs.out" "estimated_cost=tiny"

bash "$SCRIPT" --changed-files "$docs_only" --json >"$TMP/docs.json"
python3 - <<'PY' "$TMP/docs.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["behavior_surfaces"][0]["id"] == "diff_hygiene_docs_diff_check"
assert profile["behavior_surfaces"][0]["owner"] == "docs"
assert profile["behavior_surfaces"][0]["proof_role"] == "diff_hygiene"
assert profile["behavior_surfaces"][0]["resource_class"] == "tiny"
assert profile["validation_dag"]["nodes"][0]["status"] == "runnable"
assert profile["validation_dag"]["nodes"][0]["proof_role"] == "diff_hygiene"
assert profile["estimated_cost"]["runtime_class"] == "tiny"
assert profile["validation_dag"]["compression_note"].startswith("profile validates behavior surfaces")
PY

tooling="$TMP/tooling.txt"
printf 'M\tadl/tools/ci_path_policy.sh\n' >"$tooling"
bash "$SCRIPT" --changed-files "$tooling" --json >"$TMP/tooling.json"
python3 - <<'PY' "$TMP/tooling.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert [item["lane_id"] for item in profile["run"]] == ["ci_path_policy_contracts"]
surface = profile["behavior_surfaces"][0]
assert surface["id"] == "ci_contract_ci_path_policy_contracts"
assert surface["owner"] == "tools"
assert surface["proof_role"] == "ci_contract"
assert surface["resource_class"] == "small"
assert profile["validation_dag"]["nodes"][0]["proof_role"] == "ci_contract"
PY

runtime="$TMP/runtime.txt"
printf 'M\tadl/src/runtime_v2/contract_schema.rs\n' >"$runtime"
bash "$SCRIPT" --changed-files "$runtime" --json >"$TMP/runtime.json"
python3 - <<'PY' "$TMP/runtime.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert [item["lane_id"] for item in profile["run"]] == ["rust_pr_fast"]
assert [family["id"] for family in profile["slow_proof_families"]] == [
    "runtime",
    "private_state",
    "observatory",
    "security",
]
assert profile["slow_proof_families"][0]["feature"] == "slow-proof-runtime"
surface = profile["behavior_surfaces"][0]
assert surface["id"] == "rust_focused_behavior"
assert surface["owner"] == "shared"
assert surface["default_surface"] == "shared_rust"
assert surface["proof_role"] == "regression"
assert "contract_schema" in surface["requirement_ids"]
node = profile["validation_dag"]["nodes"][0]
assert node["proof_role"] == "regression"
assert node["resource_class"] == "medium"
PY

runtime_family="$TMP/runtime-family.txt"
printf 'M\tadl/src/runtime_v2/standing/mod.rs\n' >"$runtime_family"
bash "$SCRIPT" --changed-files "$runtime_family" --json >"$TMP/runtime-family.json"
python3 - <<'PY' "$TMP/runtime-family.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert [item["lane_id"] for item in profile["run"]] == ["rust_pr_fast"]
surface = profile["behavior_surfaces"][0]
assert surface["id"] == "rust_family_behavior"
assert surface["owner"] == "shared"
assert surface["default_surface"] == "shared_rust"
assert surface["proof_role"] == "regression"
assert "runtime_v2" in surface["requirement_ids"]
node = profile["validation_dag"]["nodes"][0]
assert node["proof_role"] == "regression"
assert node["resource_class"] == "medium"
PY

release_gate="$TMP/release-gate.txt"
printf 'M\t.github/workflows/ci.yaml\n' >"$release_gate"
bash "$SCRIPT" --changed-files "$release_gate" --json >"$TMP/release.json"
python3 - <<'PY' "$TMP/release.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "escalation_required"
assert profile["escalation"]["required"] is True
assert any(item["surface"] == "slow_proof/runtime" for item in profile["not_run"])
assert any(
    reason["lane_id"] == "release_gate_review"
    for reason in profile["escalation"]["reasons"]
)
assert any(item["lane_id"] == "ci_path_policy_contracts" for item in profile["run"])
assert any(
    behavior["id"] == "release_gate_release_gate_review"
    for behavior in profile["behavior_surfaces"]
)
assert any(
    behavior["proof_role"] == "release_gate"
    and behavior["owner"] == "tools"
    for behavior in profile["behavior_surfaces"]
)
assert profile["estimated_cost"]["runtime_class"] == "escalated"
PY

if bash "$SCRIPT" --changed-files "$release_gate" --run >"$TMP/refuse.out" 2>"$TMP/refuse.err"; then
  echo "expected validation manager to refuse escalated --run" >&2
  exit 1
fi
assert_has "$TMP/refuse.err" "refusing --run for non-runnable profile"

unmapped="$TMP/unmapped.txt"
printf 'M\ttotally/unmapped/path.txt\n' >"$unmapped"
bash "$SCRIPT" --changed-files "$unmapped" --json >"$TMP/unmapped.json"
python3 - <<'PY' "$TMP/unmapped.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["selected_profile"] == "validation_none"
assert profile["status"] == "escalation_required"
assert profile["pr_publication_sufficient"] is False
assert profile["run"] == []
assert profile["escalation"]["required"] is True
assert profile["escalation"]["reasons"] == [
    {
        "lane_id": "unmapped_change_surface",
        "matched_paths": ["totally/unmapped/path.txt"],
        "reason": "selector left changed paths without validation-lane coverage",
        "status": "escalated",
    }
]
PY

if bash "$SCRIPT" --changed-files "$unmapped" --run >"$TMP/unmapped-run.out" 2>"$TMP/unmapped-run.err"; then
  echo "expected validation manager to refuse unmapped-path --run" >&2
  exit 1
fi
assert_has "$TMP/unmapped-run.err" "refusing --run for non-runnable profile"

mixed="$TMP/mixed.txt"
printf 'M\tdocs/milestones/v0.91.6/README.md\nM\ttotally/unmapped/path.txt\n' >"$mixed"
bash "$SCRIPT" --changed-files "$mixed" --json >"$TMP/mixed.json"
python3 - <<'PY' "$TMP/mixed.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "escalation_required"
assert profile["pr_publication_sufficient"] is False
assert [item["lane_id"] for item in profile["run"]] == ["docs_diff_check"]
assert profile["escalation"]["required"] is True
assert any(
    reason == {
        "lane_id": "unmapped_change_surface",
        "matched_paths": ["totally/unmapped/path.txt"],
        "reason": "selector left changed paths without validation-lane coverage",
        "status": "escalated",
    }
    for reason in profile["escalation"]["reasons"]
)
PY

if bash "$SCRIPT" --changed-files "$mixed" --run >"$TMP/mixed-run.out" 2>"$TMP/mixed-run.err"; then
  echo "expected validation manager to refuse mixed unmapped-path --run" >&2
  exit 1
fi
assert_has "$TMP/mixed-run.err" "refusing --run for non-runnable profile"

owner_mix="$TMP/owner-mix.txt"
printf 'M\tadl/tools/pr.sh\nM\tadl/src/bin/adl_runtime.rs\n' >"$owner_mix"
bash "$SCRIPT" --changed-files "$owner_mix" --json >"$TMP/owner-mix.json"
python3 - <<'PY' "$TMP/owner-mix.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
surface_ids = [surface["id"] for surface in profile["behavior_surfaces"]]
assert "owner_lane_csdlc_owner_lane" in surface_ids
assert "owner_lane_runtime_owner_lane" in surface_ids
assert len(surface_ids) == len(set(surface_ids))
node_ids = [node["behavior_surface"] for node in profile["validation_dag"]["nodes"]]
assert len(node_ids) == len(set(node_ids))
PY

portable_dir="$TMP/portable"
mkdir -p "$portable_dir"
portable_changed="$portable_dir/changed.txt"
printf 'M\tdocs/milestones/v0.91.6/README.md\n' >"$portable_changed"
(
  cd "$portable_dir"
  bash "$SCRIPT" --changed-files "changed.txt" --json >"$TMP/portable.json"
)
python3 - <<'PY' "$TMP/portable.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["selected_profile"] == "docs_diff_check_profile"
assert profile["status"] == "ready_to_run"
assert [item["lane_id"] for item in profile["run"]] == ["docs_diff_check"]
PY

echo "PASS test_validation_manager"
