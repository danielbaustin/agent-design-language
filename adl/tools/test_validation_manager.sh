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
assert profile["diagnostics"] == []
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

unity_observatory="$TMP/unity-observatory.txt"
cat >"$unity_observatory" <<'EOF'
M	demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json
M	demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs
M	adl/tools/test_v0916_unity_observatory_soak_integration.sh
M	adl/tools/test_v0916_unity_observatory_unity65_smoke.sh
EOF
bash "$SCRIPT" --changed-files "$unity_observatory" --json >"$TMP/unity-observatory.json"
python3 - <<'PY' "$TMP/unity-observatory.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["selected_profile"] == "unity_observatory_contract_surface_profile"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert [item["lane_id"] for item in profile["run"]] == ["unity_observatory_contract_surface"]
surface = profile["behavior_surfaces"][0]
assert surface["id"] == "demo_contract_unity_observatory_contract_surface"
assert surface["owner"] == "review"
assert surface["proof_role"] == "demo_contract"
assert surface["resource_class"] == "small"
assert "bash -n adl/tools/test_v0916_unity_observatory_unity65_smoke.sh" in profile["run"][0]["command"]
assert "test_v0916_unity_observatory_baseline.sh" in profile["run"][0]["command"]
assert "test_v0916_unity_observatory_contract.sh" in profile["run"][0]["command"]
assert "test_v0916_unity_observatory_soak_integration.sh" in profile["run"][0]["command"]
assert "csm_observatory_cli_writes_unity_contract_bundle" in profile["run"][0]["command"]
assert profile["diagnostics"] == []
assert profile["escalation"]["required"] is False
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
assert profile["diagnostics"] == []
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
assert profile["diagnostics"] == []
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
assert any(
    reason["triggering_surface"] == ".github/workflows/ci.yaml"
    for reason in profile["escalation"]["reasons"]
    if reason["lane_id"] == "release_gate_review"
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
assert any(
    diagnostic["code"] == "release_gate_review_requires_escalation"
    for diagnostic in profile["diagnostics"]
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
reason = profile["escalation"]["reasons"][0]
assert reason["lane_id"] == "unmapped_change_surface"
assert reason["matched_paths"] == ["totally/unmapped/path.txt"]
assert reason["reason"] == "selector left changed paths without validation-lane coverage"
assert reason["status"] == "escalated"
assert reason["manifest_rule"] == "adl/config/validation_lane_selector.v0.91.6.json"
assert "path selector" in reason["remediation_hint"]
assert profile["diagnostics"][0]["code"] == "unmapped_change_surface"
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
    reason["lane_id"] == "unmapped_change_surface"
    and reason["matched_paths"] == ["totally/unmapped/path.txt"]
    and reason["manifest_rule"] == "adl/config/validation_lane_selector.v0.91.6.json"
    for reason in profile["escalation"]["reasons"]
)
PY

if bash "$SCRIPT" --changed-files "$mixed" --run >"$TMP/mixed-run.out" 2>"$TMP/mixed-run.err"; then
  echo "expected validation manager to refuse mixed unmapped-path --run" >&2
  exit 1
fi
assert_has "$TMP/mixed-run.err" "refusing --run for non-runnable profile"

workflow_metrics_backfill="$TMP/workflow-metrics-backfill.txt"
cat >"$workflow_metrics_backfill" <<'EOF'
M	adl/config/validation_lane_selector.v0.91.6.json
M	adl/src/cli/pr_cmd.rs
M	adl/src/cli/pr_cmd/github.rs
M	adl/src/cli/pr_cmd/github/tests/transport.rs
M	adl/src/cli/pr_cmd/github/tests/watch.rs
M	adl/src/cli/tests/pr_cmd_inline/basics.rs
M	adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs
M	adl/tools/build_v0916_workflow_metric_backfill_inventory.py
M	adl/tools/test_select_validation_lanes.sh
M	adl/tools/test_validation_manager.sh
M	docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.json
EOF
bash "$SCRIPT" --changed-files "$workflow_metrics_backfill" --json >"$TMP/workflow-metrics-backfill.json"
python3 - <<'PY' "$TMP/workflow-metrics-backfill.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["selected_profile"] == "selected_4_lane_profile"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert [item["lane_id"] for item in profile["run"]] == [
    "ci_path_policy_contracts",
    "csdlc_owner_lane",
    "docs_diff_check",
    "rust_pr_fast",
]
PY

pr_inventory_finish="$TMP/pr-inventory-finish.txt"
cat >"$pr_inventory_finish" <<'EOF'
M	adl/Cargo.toml
M	adl/config/validation_lane_selector.v0.91.6.json
M	adl/src/bin/adl_pr_inventory.rs
M	adl/src/cli/pr_cmd.rs
M	adl/src/cli/pr_cmd/github.rs
M	adl/src/cli/pr_cmd/github/tests/validation.rs
M	adl/src/cli/pr_cmd/github/tests/watch.rs
M	adl/src/cli/pr_cmd/github/transport.rs
M	adl/src/cli/pr_cmd/lifecycle/tests.rs
M	adl/src/cli/pr_cmd_args.rs
M	adl/src/cli/tests/pr_cmd_inline/basics.rs
M	adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs
M	adl/src/cli/tests/pr_cmd_inline/repo_helpers/metadata.rs
M	adl/src/cli/tests/pr_cmd_inline/support.rs
M	adl/tools/pr.sh
M	adl/tools/pr_delegate.sh
M	adl/tools/pr_usage.sh
M	adl/tools/run_pr_fast_test_lane.sh
M	adl/tools/test_ci_path_policy.sh
M	adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh
M	adl/tools/test_validation_manager.sh
M	docs/milestones/v0.91.7/README.md
M	docs/milestones/v0.91.7/SPRINT_PLAN_v0.91.7.md
M	docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml
M	docs/tooling/PR_INVENTORY_COMMAND.md
EOF
bash "$SCRIPT" --changed-files "$pr_inventory_finish" --json >"$TMP/pr-inventory-finish.json"
python3 - <<'PY' "$TMP/pr-inventory-finish.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["selected_profile"] == "selected_4_lane_profile"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert profile["escalation"]["required"] is False
assert profile["diagnostics"] == []
assert {item["lane_id"] for item in profile["run"]} == {
    "ci_path_policy_contracts",
    "csdlc_owner_lane",
    "docs_diff_check",
    "rust_pr_fast",
}
rust_lane = next(item for item in profile["run"] if item["lane_id"] == "rust_pr_fast")
assert "adl/Cargo.toml" in rust_lane["matched_paths"]
PY

sprint_conductor="$TMP/sprint-conductor.txt"
cat >"$sprint_conductor" <<'EOF'
M	adl/tools/skills/sprint-conductor/SKILL.md
M	adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py
M	adl/tools/test_sprint_conductor_helpers.sh
M	adl/tools/test_install_adl_operational_skills.sh
EOF
bash "$SCRIPT" --changed-files "$sprint_conductor" --json >"$TMP/sprint-conductor.json"
python3 - <<'PY' "$TMP/sprint-conductor.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["selected_profile"] == "sprint_conductor_contracts_profile"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert [item["lane_id"] for item in profile["run"]] == ["sprint_conductor_contracts"]
surface_ids = {surface["id"] for surface in profile["behavior_surfaces"]}
assert "regression_sprint_conductor_contracts" in surface_ids
assert profile["diagnostics"] == []
assert profile["escalation"]["required"] is False
PY

sprint_conductor_mixed="$TMP/sprint-conductor-mixed.txt"
cat >"$sprint_conductor_mixed" <<'EOF'
M	adl/tools/skills/sprint-conductor/SKILL.md
M	adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py
M	adl/tools/test_sprint_conductor_helpers.sh
M	adl/tools/test_install_adl_operational_skills.sh
M	docs/milestones/v0.91.6/README.md
EOF
bash "$SCRIPT" --changed-files "$sprint_conductor_mixed" --json >"$TMP/sprint-conductor-mixed.json"
python3 - <<'PY' "$TMP/sprint-conductor-mixed.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert {item["lane_id"] for item in profile["run"]} == {
    "sprint_conductor_contracts",
    "docs_diff_check",
}
surface_ids = {surface["id"] for surface in profile["behavior_surfaces"]}
assert "regression_sprint_conductor_contracts" in surface_ids
assert "diff_hygiene_docs_diff_check" in surface_ids
assert profile["diagnostics"] == []
assert profile["escalation"]["required"] is False
PY

classifier_followup="$TMP/classifier-followup.txt"
cat >"$classifier_followup" <<'EOF'
M	adl/config/validation_lane_selector.v0.91.6.json
M	adl/tools/ci_path_policy.sh
M	adl/tools/test_ci_path_policy.sh
M	adl/tools/test_validation_manager.sh
M	adl/tools/skills/sprint-conductor/SKILL.md
M	adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py
M	adl/tools/test_sprint_conductor_helpers.sh
M	adl/tools/test_install_adl_operational_skills.sh
M	docs/milestones/v0.91.6/README.md
EOF
bash "$SCRIPT" --changed-files "$classifier_followup" --json >"$TMP/classifier-followup.json"
python3 - <<'PY' "$TMP/classifier-followup.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert {item["lane_id"] for item in profile["run"]} == {
    "ci_path_policy_contracts",
    "sprint_conductor_contracts",
    "docs_diff_check",
}
assert profile["escalation"]["required"] is False
assert profile["diagnostics"] == []
PY

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

slow_proof="$TMP/slow-proof.txt"
cat >"$slow_proof" <<'EOF'
M	adl/src/runtime_v2/tests.rs
M	adl/tools/test_slow_proof_lane_contract.sh
M	docs/milestones/v0.91.4/features/PVF_INITIAL_LANE_INVENTORY_v0.91.4.md
EOF
bash "$SCRIPT" --changed-files "$slow_proof" --json >"$TMP/slow-proof.json"
python3 - <<'PY' "$TMP/slow-proof.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "escalation_required"
assert profile["escalation"]["required"] is True
assert any(
    reason["lane_id"] == "slow_proof_review"
    and reason["triggering_surface"] == "adl/tools/test_slow_proof_lane_contract.sh"
    for reason in profile["escalation"]["reasons"]
)
assert any(
    reason["lane_id"] == "rust_pr_fast"
    and reason["reason"] == "slow_proof_inventory_change_covered_by_contract_check"
    for reason in profile["escalation"]["reasons"]
)
assert any(
    diagnostic["code"] == "pr_fast_mode_contract_only"
    for diagnostic in profile["diagnostics"]
)
PY

if bash "$SCRIPT" --changed-files "$slow_proof" --run >"$TMP/slow-proof-run.out" 2>"$TMP/slow-proof-run.err"; then
  echo "expected validation manager to refuse slow-proof --run" >&2
  exit 1
fi
assert_has "$TMP/slow-proof-run.err" "refusing --run for non-runnable profile"

threshold_manifest="$TMP/threshold-manifest.json"
python3 - <<'PY' "$ROOT/adl/config/validation_lane_selector.v0.91.6.json" "$threshold_manifest"
import json
import sys

manifest = json.load(open(sys.argv[1]))
manifest["manager_guardrails"]["pr_fast"]["max_filter_token_count"] = 0
json.dump(manifest, open(sys.argv[2], "w"), indent=2, sort_keys=True)
PY
bash "$SCRIPT" --manifest "$threshold_manifest" --changed-files "$runtime" --json >"$TMP/threshold.json"
python3 - <<'PY' "$TMP/threshold.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["status"] == "escalation_required"
assert any(
    reason["manifest_rule"] == "manager_guardrails.pr_fast.max_filter_token_count"
    for reason in profile["escalation"]["reasons"]
)
assert any(
    diagnostic["code"] == "pr_fast_filter_threshold_exceeded"
    for diagnostic in profile["diagnostics"]
)
assert profile["pr_publication_sufficient"] is False
PY

custom_unmapped_manifest="$TMP/custom-unmapped-manifest.json"
python3 - <<'PY' "$ROOT/adl/config/validation_lane_selector.v0.91.6.json" "$custom_unmapped_manifest"
import json
import sys

manifest = json.load(open(sys.argv[1]))
json.dump(manifest, open(sys.argv[2], "w"), indent=2, sort_keys=True)
PY
bash "$SCRIPT" --manifest "$custom_unmapped_manifest" --changed-files "$unmapped" --json >"$TMP/custom-unmapped.json"
python3 - <<'PY' "$TMP/custom-unmapped.json" "$custom_unmapped_manifest"
import json
import sys
from pathlib import Path

profile = json.load(open(sys.argv[1]))
expected_manifest = str(Path(sys.argv[2]).resolve())
assert profile["status"] == "escalation_required"
assert profile["escalation"]["reasons"][0]["manifest_rule"] == expected_manifest
assert profile["diagnostics"][0]["manifest_rule"] == expected_manifest
PY

bad_guardrail_manifest="$TMP/bad-guardrail-manifest.json"
python3 - <<'PY' "$ROOT/adl/config/validation_lane_selector.v0.91.6.json" "$bad_guardrail_manifest"
import json
import sys

manifest = json.load(open(sys.argv[1]))
manifest["manager_guardrails"]["pr_fast"]["max_filter_token_count"] = "oops"
json.dump(manifest, open(sys.argv[2], "w"), indent=2, sort_keys=True)
PY
if bash "$SCRIPT" --manifest "$bad_guardrail_manifest" --changed-files "$runtime" >"$TMP/bad-guardrail.out" 2>"$TMP/bad-guardrail.err"; then
  echo "expected validation manager to fail closed on malformed guardrail config" >&2
  exit 1
fi
assert_has "$TMP/bad-guardrail.err" "validation_manager: manager guardrail pr_fast.max_filter_token_count must be an integer"

remote_origin_src="$TMP/remote-origin-src"
remote_origin_bare="$TMP/remote-origin.git"
mkdir -p "$remote_origin_src"
git -C "$remote_origin_src" init -q
git -C "$remote_origin_src" branch -M main
cat >"$remote_origin_src/README.md" <<'EOF'
# validation manager remote fixture
EOF
git -C "$remote_origin_src" add README.md
git -C "$remote_origin_src" -c user.name=Codex -c user.email=codex@example.com commit -q -m "fixture"
git clone -q --bare "$remote_origin_src" "$remote_origin_bare"

remote_fake_bin="$TMP/remote-fake-bin"
mkdir -p "$remote_fake_bin"
cat >"$remote_fake_bin/rustc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "rustc 1.96.0 (fixture)"
  exit 0
fi
echo "unexpected rustc invocation: $*" >&2
exit 1
EOF
cat >"$remote_fake_bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "cargo 1.96.0 (fixture)"
  exit 0
fi
echo "unexpected cargo invocation: $*" >&2
exit 1
EOF
cat >"$remote_fake_bin/sccache" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  --version)
    echo "sccache 0.16.0"
    ;;
  --zero-stats)
    exit 0
    ;;
  --show-stats)
    cat <<'STATS'
Compile requests                      5
Compile requests executed             2
Cache hits                            3
Cache misses                          2
STATS
    ;;
  *)
    echo "unexpected sccache invocation: $*" >&2
    exit 1
    ;;
esac
EOF
cat >"$remote_fake_bin/apt-get" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
echo "apt-get update fixture ok"
EOF
chmod +x "$remote_fake_bin/"*

remote_sources="$TMP/remote-sources.list"
remote_kubernetes="$TMP/remote-kubernetes.list"
cat >"$remote_sources" <<'EOF'
deb https://apt.releases.hashicorp.com focal main
EOF
cat >"$remote_kubernetes" <<'EOF'
deb https://apt.kubernetes.io/ kubernetes-xenial main
EOF

remote_profile_changed="$TMP/remote-profile.txt"
printf 'M\tadl/src/provider_communication.rs\n' >"$remote_profile_changed"
ADL_NESSUS_REMOTE_EXECUTOR=local \
ADL_NESSUS_REMOTE_ROOT="$TMP/validation-manager-remote-root" \
ADL_NESSUS_REMOTE_REPO_URL="$remote_origin_bare" \
ADL_NESSUS_REMOTE_GIT_REF=origin/main \
ADL_NESSUS_APT_SOURCES_LIST="$remote_sources" \
ADL_NESSUS_APT_KUBERNETES_LIST="$remote_kubernetes" \
PATH="$remote_fake_bin:$PATH" \
bash "$SCRIPT" \
  --changed-files "$remote_profile_changed" \
  --remote-runner nessus \
  --remote-command "printf remote-manager-ok" \
  --remote-artifact-dir "$TMP/remote-manager-artifacts" \
  --json >"$TMP/remote-manager.json"
python3 - <<'PY' "$TMP/remote-manager.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["remote_runner"]["requested"] == "nessus"
assert profile["remote_runner"]["decision"] == "selected"
assert "run_nessus_remote_validation.sh" in profile["remote_runner"]["command"]
assert profile["run"][0]["lane_id"] == "nessus_remote_validation"
assert profile["status"] == "ready_to_run"
PY

ADL_NESSUS_REMOTE_EXECUTOR=local \
ADL_NESSUS_REMOTE_ROOT="$TMP/validation-manager-remote-root-run" \
ADL_NESSUS_REMOTE_REPO_URL="$remote_origin_bare" \
ADL_NESSUS_REMOTE_GIT_REF=origin/main \
ADL_NESSUS_APT_SOURCES_LIST="$remote_sources" \
ADL_NESSUS_APT_KUBERNETES_LIST="$remote_kubernetes" \
PATH="$remote_fake_bin:$PATH" \
bash "$SCRIPT" \
  --changed-files "$remote_profile_changed" \
  --remote-runner nessus \
  --remote-command "printf remote-manager-ok" \
  --remote-artifact-dir "$TMP/remote-manager-artifacts-run" \
  --run \
  --report-out "$TMP/remote-manager-run-report.json" >/dev/null
python3 - <<'PY' "$TMP/remote-manager-run-report.json" "$TMP/remote-manager-artifacts-run/summary.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
summary = json.load(open(sys.argv[2]))
assert profile["run_status"] == "passed"
assert profile["remote_runner"]["decision"] == "selected"
assert summary["status"] == "passed"
assert summary["command"] == "printf remote-manager-ok"
PY

bash "$SCRIPT" \
  --changed-files "$docs_only" \
  --remote-runner nessus \
  --remote-command "printf no-remote-docs" \
  --json >"$TMP/remote-docs.json"
python3 - <<'PY' "$TMP/remote-docs.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["remote_runner"]["decision"] == "rejected"
assert "runtime_class tiny" in profile["remote_runner"]["reason"]
assert profile["status"] == "ready_to_run"
PY

if bash "$SCRIPT" \
  --changed-files "$docs_only" \
  --remote-runner nessus \
  --remote-command "printf no-remote-docs" \
  --run >"$TMP/remote-docs-run.out" 2>"$TMP/remote-docs-run.err"; then
  echo "expected docs-only remote-runner request to be rejected" >&2
  exit 1
fi
assert_has "$TMP/remote-docs-run.err" "requested remote runner is not eligible"

echo "PASS test_validation_manager"
