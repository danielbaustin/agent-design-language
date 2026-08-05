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
from pathlib import Path

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["behavior_surfaces"][0]["id"] == "diff_hygiene_docs_diff_check"
assert profile["behavior_surfaces"][0]["owner"] == "docs"
assert profile["behavior_surfaces"][0]["proof_role"] == "diff_hygiene"
assert profile["behavior_surfaces"][0]["resource_class"] == "tiny"
assert profile["validation_dag"]["nodes"][0]["status"] == "runnable"
assert profile["validation_dag"]["nodes"][0]["proof_role"] == "diff_hygiene"
assert profile["estimated_cost"]["runtime_class"] == "tiny"
assert profile["validation_split"]["schema_version"] == "adl.validation_split.v1"
assert profile["validation_split"]["fast_lane"]["selected_lanes"] == ["docs_diff_check"]
assert profile["validation_split"]["fast_lane"]["runnable"] is True
assert profile["validation_split"]["fast_lane"]["pr_publication_sufficient"] is True
assert profile["validation_split"]["fanout_policy"]["missing_or_unmapped_proof"] == "fail_closed"
assert profile["validation_dag"]["compression_note"].startswith("profile validates behavior surfaces")
assert profile["diagnostics"] == []
PY

docs_report="$TMP/docs-report.json"
bash "$SCRIPT" --changed-files "$docs_only" --json --report-out "$docs_report" >"$TMP/docs-report-stdout.json"
python3 - <<'PY' "$docs_report" "$TMP/docs-report-stdout.json"
import json
import sys

recorded = json.load(open(sys.argv[1]))
stdout_profile = json.load(open(sys.argv[2]))
assert recorded["schema_version"] == "adl.validation_profile.v1"
assert recorded["selected_profile"] == "docs_diff_check_profile"
assert recorded["status"] == "ready_to_run"
assert recorded == stdout_profile
PY

podcast_static_demo="$TMP/podcast-static-demo.txt"
cat >"$podcast_static_demo" <<'EOF'
A	demos/podcast/index.html
A	demos/podcast/feed.xml
A	demos/podcast/studio/podcast-studio.html
A	demos/_preview/podcast/index.html
EOF
bash "$SCRIPT" --changed-files "$podcast_static_demo" --json >"$TMP/podcast-static-demo.json"
python3 - <<'PY' "$TMP/podcast-static-demo.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1", json.dumps(profile, indent=2, sort_keys=True)
assert profile["selected_profile"] == "selected_2_lane_profile", json.dumps(profile, indent=2, sort_keys=True)
assert profile["status"] == "ready_to_run", json.dumps(profile, indent=2, sort_keys=True)
assert profile["pr_publication_sufficient"] is True, json.dumps(profile, indent=2, sort_keys=True)
assert [item["lane_id"] for item in profile["run"]] == [
    "podcast_launch_packet",
    "podcast_static_demo_surface",
], json.dumps(profile, indent=2, sort_keys=True)
surfaces = {surface["lane_id"]: surface for surface in profile["behavior_surfaces"]}
launch_surface = surfaces["podcast_launch_packet"]
assert launch_surface["id"] == "demo_contract_podcast_launch_packet"
assert launch_surface["owner"] == "review"
assert launch_surface["proof_role"] == "demo_contract"
assert launch_surface["resource_class"] == "small"
static_surface = surfaces["podcast_static_demo_surface"]
assert static_surface["id"] == "demo_contract_podcast_static_demo_surface"
assert static_surface["owner"] == "site"
assert static_surface["proof_role"] == "demo_contract"
assert static_surface["resource_class"] == "tiny"
assert profile["escalation"]["required"] is False
assert profile["escalation"]["reasons"] == []
assert profile["diagnostics"] == []
PY

docs_run_log_dir="$TMP/build-action-logs"
bash "$SCRIPT" \
  --changed-files "$docs_only" \
  --json \
  --run \
  --build-action-log-dir "$docs_run_log_dir" \
  >"$TMP/docs-run.json"
python3 - <<'PY' "$TMP/docs-run.json" "$docs_run_log_dir"
import json
import sys
from pathlib import Path

profile = json.load(open(sys.argv[1]))
log_dir = Path(sys.argv[2])
assert profile["run_status"] == "passed", json.dumps(profile, indent=2, sort_keys=True)
assert profile["build_action_logs"]["schema_version"] == "adl.build_action_log_manifest.v1", json.dumps(profile, indent=2, sort_keys=True)
assert profile["build_action_logs"]["packet_count"] >= 1, json.dumps(profile, indent=2, sort_keys=True)
docs_items = [item for item in profile["run"] if item["lane_id"] == "docs_diff_check"]
assert len(docs_items) == 1, json.dumps(profile, indent=2, sort_keys=True)
packet_ref = docs_items[0]["build_action_log"]
packet_path = Path(packet_ref)
if not packet_path.is_absolute():
    packet_path = Path.cwd() / packet_path
assert packet_path.is_file()
packet = json.load(open(packet_path))
assert packet["schema_version"] == "adl.build_action_log.v1"
assert packet["runner"] == "validation_manager"
assert packet["lane_id"] == "docs_diff_check"
assert packet["command"] == "git diff --check"
assert packet["cwd"] == "."
assert packet["binary_path"] == "shell"
assert packet["cache_posture"] == "local_target_or_repo_configured"
assert packet["exit_code"] == 0
assert packet["status"] == "passed"
for key in ("stdout_ref", "stderr_ref", "packet_ref"):
    ref = packet[key]
    assert "/Users/" not in ref and "/private/tmp" not in ref
stdout_ref = Path(packet["stdout_ref"])
stderr_ref = Path(packet["stderr_ref"])
if not stdout_ref.is_absolute():
    stdout_ref = Path.cwd() / stdout_ref
if not stderr_ref.is_absolute():
    stderr_ref = Path.cwd() / stderr_ref
assert stdout_ref.is_file()
assert stderr_ref.is_file()
manifest_path = log_dir / "manifest.json"
assert manifest_path.is_file()
manifest = json.load(open(manifest_path))
assert manifest["schema_version"] == "adl.build_action_log_manifest.v1"
assert manifest["packet_count"] == 1
assert manifest["packets"] == [packet["packet_ref"]]
PY

noisy_manifest="$TMP/noisy-manifest.json"
python3 - <<'PY' "$ROOT/adl/config/validation_lane_selector.v0.91.6.json" "$noisy_manifest"
import json
import sys

manifest = json.load(open(sys.argv[1]))
for lane in manifest["lanes"]:
    if lane["id"] == "docs_diff_check":
        lane["command"] = "printf noisy-json-safe-stdout"
        lane["run_command"] = "printf noisy-json-safe-stdout"
json.dump(manifest, open(sys.argv[2], "w"), indent=2, sort_keys=True)
PY
noisy_log_dir="$TMP/noisy-build-action-logs"
bash "$SCRIPT" \
  --manifest "$noisy_manifest" \
  --changed-files "$docs_only" \
  --json \
  --run \
  --build-action-log-dir "$noisy_log_dir" \
  >"$TMP/noisy-run.json" \
  2>"$TMP/noisy-run.stderr"
python3 - <<'PY' "$TMP/noisy-run.json"
import json
import sys
from pathlib import Path

profile = json.load(open(sys.argv[1]))
assert profile["run_status"] == "passed"
packet_ref = profile["run"][0]["build_action_log"]
packet_path = Path(packet_ref)
if not packet_path.is_absolute():
    packet_path = Path.cwd() / packet_path
packet = json.load(open(packet_path))
stdout_ref = Path(packet["stdout_ref"])
if not stdout_ref.is_absolute():
    stdout_ref = Path.cwd() / stdout_ref
stdout_text = open(stdout_ref).read()
assert stdout_text == "noisy-json-safe-stdout"
assert packet["command"] == "printf noisy-json-safe-stdout"
PY
assert_has "$TMP/noisy-run.stderr" "noisy-json-safe-stdout"

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

rust_cache_tooling="$TMP/rust-cache-tooling.txt"
cat >"$rust_cache_tooling" <<'EOF'
A	adl/tools/rust_cache_env.sh
A	adl/tools/test_rust_cache_env.sh
EOF
bash "$SCRIPT" --changed-files "$rust_cache_tooling" --json >"$TMP/rust-cache-tooling.json"
python3 - <<'PY' "$TMP/rust-cache-tooling.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert [item["lane_id"] for item in profile["run"]] == ["ci_path_policy_contracts"]
surface = profile["behavior_surfaces"][0]
assert surface["id"] == "ci_contract_ci_path_policy_contracts"
assert "adl/tools/rust_cache_env.sh" in surface["matched_paths"]
assert "adl/tools/test_rust_cache_env.sh" in surface["matched_paths"]
PY

scheduler_fixture_repair="$TMP/scheduler-fixture-repair.txt"
cat >"$scheduler_fixture_repair" <<'EOF'
M	adl/tests/fixtures/scheduler/local_agent_delegation_readiness_inputs_v1.json
M	docs/milestones/v0.91.7/review/provider/artifacts/local_agent_delegation_readiness_plan_4675.json
EOF
bash "$SCRIPT" --changed-files "$scheduler_fixture_repair" --json >"$TMP/scheduler-fixture-repair.json"
python3 - <<'PY' "$TMP/scheduler-fixture-repair.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["selected_profile"] == "scheduler_fixture_validation_profile"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert [item["lane_id"] for item in profile["run"]] == ["scheduler_fixture_validation"]
assert profile["run"][0]["command"] == "cargo test --manifest-path adl/Cargo.toml --lib scheduler_economics"
assert profile["run"][0]["matched_paths"] == [
    "adl/tests/fixtures/scheduler/local_agent_delegation_readiness_inputs_v1.json",
    "docs/milestones/v0.91.7/review/provider/artifacts/local_agent_delegation_readiness_plan_4675.json",
]
surface = profile["behavior_surfaces"][0]
assert surface["id"] == "scheduler_fixture_contract_scheduler_fixture_validation"
assert surface["owner"] == "tools"
assert surface["proof_role"] == "scheduler_fixture_contract"
assert profile["validation_dag"]["nodes"][0]["status"] == "runnable"
assert profile["validation_dag"]["nodes"][0]["proof_role"] == "scheduler_fixture_contract"
assert profile["escalation"]["required"] is False
assert profile["escalation"]["reasons"] == []
assert profile["diagnostics"] == []
assert not any(
    reason.get("reason") == "no_rust_surface_detected_for_fast_lane"
    for reason in profile["escalation"]["reasons"]
)
assert "rust_pr_fast" not in profile["selector_plan"]["lanes"]
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
split = profile["validation_split"]
assert split["fast_lane"]["selected_lanes"] == ["rust_pr_fast"]
assert split["fast_lane"]["execution_model"] == "local_fast_lane"
assert [family["id"] for family in split["slow_families"]] == [
    "runtime",
    "private_state",
    "observatory",
    "security",
]
assert split["slow_families"][0]["disposition"] == "reserved_for_explicit_family_selection"
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

daemon_wave="$TMP/daemon-wave.txt"
cat >"$daemon_wave" <<'EOF'
M	adl/src/cli/agent_cmd.rs
M	adl/src/cli/usage.rs
M	adl/src/long_lived_agent.rs
M	adl/src/long_lived_agent/schema.rs
M	adl/src/long_lived_agent/storage.rs
M	adl/src/long_lived_agent/tests.rs
M	adl/src/long_lived_agent/types.rs
M	adl/tests/cli_smoke/agent.rs
M	docs/milestones/v0.91.7/review/runtime/RUNTIME_DAEMON_SUPERVISION_4885.md
EOF
bash "$SCRIPT" --changed-files "$daemon_wave" --json >"$TMP/daemon-wave.json"
python3 - <<'PY' "$TMP/daemon-wave.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert profile["escalation"]["required"] is False
assert [item["lane_id"] for item in profile["run"]] == [
    "docs_diff_check",
    "rust_pr_fast",
]
rust = next(item for item in profile["run"] if item["lane_id"] == "rust_pr_fast")
assert rust["reason"] == "bounded_rust_surface_runs_focused_nextest"
assert rust["matched_paths"] == [
    "adl/src/cli/agent_cmd.rs",
    "adl/src/cli/usage.rs",
    "adl/src/long_lived_agent.rs",
    "adl/src/long_lived_agent/schema.rs",
    "adl/src/long_lived_agent/storage.rs",
    "adl/src/long_lived_agent/tests.rs",
    "adl/src/long_lived_agent/types.rs",
    "adl/tests/cli_smoke/agent.rs",
]
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

slow_proof_workflow="$TMP/slow-proof-workflow.txt"
cat >"$slow_proof_workflow" <<'EOF'
M	.github/workflows/ci.yaml
M	adl/tools/test_ci_runtime_contracts.sh
EOF
bash "$SCRIPT" --changed-files "$slow_proof_workflow" --json >"$TMP/slow-proof-workflow.json"
python3 - <<'PY' "$TMP/slow-proof-workflow.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert profile["escalation"]["required"] is False
assert profile["escalation"]["reasons"] == []
assert profile["diagnostics"] == []
assert any(item["lane_id"] == "ci_path_policy_contracts" for item in profile["run"])
assert any(
    behavior["id"] == "release_gate_release_gate_review"
    for behavior in profile["behavior_surfaces"]
)
assert any(
    node["lane_id"] == "release_gate_review"
    and node["status"] == "disposition_recorded"
    for node in profile["validation_dag"]["nodes"]
)
assert profile["validation_split"]["fast_lane"]["pr_publication_sufficient"] is True
assert profile["validation_split"]["fail_closed"]["required"] is False
PY

slow_proof_filtered_fanout="$TMP/slow-proof-filtered-fanout.txt"
cat >"$slow_proof_filtered_fanout" <<'EOF'
M	.github/workflows/ci.yaml
M	adl/config/slow_proof_families.v0.91.6.json
M	adl/config/validation_lane_selector.v0.91.6.json
M	adl/src/runtime_v2/private_state_observatory.rs
M	adl/src/runtime_v2/tests.rs
M	adl/tools/ci_path_policy.sh
M	adl/tools/run_pr_fast_test_lane.sh
M	adl/tools/run_slow_proof_family.sh
M	adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md
M	adl/tools/test_ci_runtime_contracts.sh
M	adl/tools/test_run_pr_fast_test_lane.sh
M	adl/tools/test_slow_proof_lane_contract.sh
M	adl/tools/test_validation_manager.sh
M	adl/tools/validation_manager.py
EOF
bash "$SCRIPT" --changed-files "$slow_proof_filtered_fanout" --json >"$TMP/slow-proof-filtered-fanout.json"
python3 - <<'PY' "$TMP/slow-proof-filtered-fanout.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert profile["escalation"]["required"] is False
assert profile["escalation"]["reasons"] == []
assert profile["diagnostics"] == []
assert any(
    node["lane_id"] == "release_gate_review"
    and node["status"] == "disposition_recorded"
    for node in profile["validation_dag"]["nodes"]
)
assert any(
    node["lane_id"] == "slow_proof_review"
    and node["status"] == "disposition_recorded"
    for node in profile["validation_dag"]["nodes"]
)
assert profile["validation_split"]["fast_lane"]["pr_publication_sufficient"] is True
assert profile["validation_split"]["fail_closed"]["required"] is False
PY

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
assert profile["validation_split"]["fast_lane"]["runnable"] is False
assert profile["validation_split"]["fail_closed"]["required"] is True
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
M	adl/tools/build_v0916_workflow_metric_backfill_inventory.py
M	adl/tools/test_build_v0916_workflow_metric_backfill_inventory.py
M	adl/tools/test_select_validation_lanes.sh
M	adl/tools/test_validation_manager.sh
M	csdlc-v2/src/github.rs
M	csdlc-v2/tests/gate_github_actions.rs
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
    "csdlc_v2_standalone",
    "docs_diff_check",
]
assert profile["escalation"]["required"] is False
assert profile["escalation"]["reasons"] == []
assert profile["diagnostics"] == []
lanes = {item["lane_id"]: item for item in profile["run"]}
assert lanes["csdlc_owner_lane"]["matched_paths"] == [
    "adl/tools/build_v0916_workflow_metric_backfill_inventory.py",
    "adl/tools/test_build_v0916_workflow_metric_backfill_inventory.py",
]
assert lanes["csdlc_v2_standalone"]["matched_paths"] == [
    "csdlc-v2/src/github.rs",
    "csdlc-v2/tests/gate_github_actions.rs",
]
PY

typed_csdlc_finish="$TMP/typed-csdlc-finish.txt"
cat >"$typed_csdlc_finish" <<'EOF'
M	adl/config/validation_lane_selector.v0.91.6.json
M	adl/tools/test_validation_manager.sh
M	csdlc-v2/src/github.rs
M	csdlc-v2/tests/gate_github_actions.rs
EOF
bash "$SCRIPT" --changed-files "$typed_csdlc_finish" --json >"$TMP/typed-csdlc-finish.json"
python3 - <<'PY' "$TMP/typed-csdlc-finish.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["selected_profile"] == "selected_2_lane_profile"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert profile["escalation"]["required"] is False
assert {item["lane_id"] for item in profile["run"]} == {
    "ci_path_policy_contracts",
    "csdlc_v2_standalone",
}
assert profile["escalation"]["reasons"] == []
assert profile["diagnostics"] == []
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

csm_binary_availability="$TMP/csm-binary-availability.txt"
cat >"$csm_binary_availability" <<'EOF'
M	adl/src/cli/csm_service_cmd.rs
M	adl/src/csm_runtime_api.rs
M	adl/tools/csm_binary_availability.sh
M	adl/tools/demo_d11_signed_remote.sh
M	adl/tools/demo_smoke_v07_story.sh
M	adl/tools/ensure_csm_binary.sh
M	adl/tools/install_owner_binaries.sh
M	adl/tools/owner_binary_resolution.sh
M	adl/tools/run_owner_validation_lane.sh
M	adl/tools/run_v0917_csm_continuity_capsule_4910_proof.sh
M	adl/tools/run_v0917_csm_otlp_4904_proof.sh
M	adl/tools/run_v0917_no_sparrow_4909_proof.sh
M	adl/tools/run_wp08_acip_sns_live_proof.sh
M	adl/tools/run_wp08_aws_signal_integration_live_proof.sh
M	adl/tools/run_wp08_cloudfront_control_proof.sh
M	adl/tools/run_wp08_heartbeat_live_proof.sh
M	adl/tools/run_wp08_polis_storage_live_proof.sh
M	adl/tools/test_ensure_csm_binary.sh
M	adl/tools/test_owner_binary_install.sh
M	adl/tools/test_owner_validation_lane.sh
M	adl/tools/test_run_wp08_aws_signal_integration_live_proof.sh
M	docs/milestones/v0.91.7/review/runtime/csm_binary_availability_4977/README.md
M	docs/milestones/v0.91.7/review/runtime/csm_binary_availability_4977/restoration.json
M	docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md
M	docs/tooling/RUNTIME_AWS_HEARTBEAT.md
EOF
bash "$SCRIPT" --changed-files "$csm_binary_availability" --json >"$TMP/csm-binary-availability.json"
python3 - <<'PY' "$TMP/csm-binary-availability.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert profile["escalation"]["required"] is False
assert len(profile["run"]) == 12
assert not any(
    diagnostic["code"] in {"unmapped_change_surface", "selected_lane_threshold_exceeded"}
    for diagnostic in profile["diagnostics"]
)
lanes = {item["lane_id"]: item for item in profile["run"]}
assert lanes["csdlc_owner_lane"]["matched_paths"] == [
    "adl/tools/install_owner_binaries.sh",
    "adl/tools/run_owner_validation_lane.sh",
    "adl/tools/test_owner_validation_lane.sh",
]
runtime_owner_paths = lanes["runtime_owner_lane"]["matched_paths"]
for path in [
    "adl/tools/csm_binary_availability.sh",
    "adl/tools/demo_d11_signed_remote.sh",
    "adl/tools/demo_smoke_v07_story.sh",
    "adl/tools/ensure_csm_binary.sh",
    "adl/tools/owner_binary_resolution.sh",
    "adl/tools/test_ensure_csm_binary.sh",
    "adl/tools/test_owner_binary_install.sh",
]:
    assert path in runtime_owner_paths
assert "adl/src/cli/csm_service_cmd.rs" in lanes["rust_pr_fast"]["matched_paths"]
assert "adl/src/csm_runtime_api.rs" in lanes["rust_pr_fast"]["matched_paths"]
PY

owner_mix="$TMP/owner-mix.txt"
printf 'M\tadl/tools/build_v0916_workflow_metric_backfill_inventory.py\nM\tadl/src/bin/adl_runtime.rs\n' >"$owner_mix"
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
assert profile["status"] == "ready_to_run"
assert profile["pr_publication_sufficient"] is True
assert profile["escalation"]["required"] is False
assert profile["escalation"]["reasons"] == []
assert profile["diagnostics"] == []
surface_ids = [surface["id"] for surface in profile["behavior_surfaces"]]
assert "rust_contract_only_behavior" in surface_ids
assert "slow_proof_slow_proof_review" in surface_ids
assert profile["validation_split"]["fast_lane"]["pr_publication_sufficient"] is True
assert profile["validation_split"]["fail_closed"]["required"] is False
PY

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

bash "$SCRIPT" \
  --changed-files "$docs_only" \
  --platform-routing \
  --json >"$TMP/platform-docs.json"
python3 - <<'PY' "$TMP/platform-docs.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
routing = profile["platform_routing"]
assert routing["schema_version"] == "adl.validation_platform_routing.v1"
assert routing["requested_platform"] == "auto"
assert routing["decision"] == "selected"
assert routing["selected_platform"] == "local"
assert routing["no_launch"] is True
candidates = {item["platform"]: item for item in routing["candidates"]}
assert candidates["local"]["decision"] == "eligible"
assert candidates["local"]["cache_posture"] == "local_target_or_repo_configured"
assert candidates["aws_spot"]["decision"] == "rejected"
assert "not cost-appropriate" in candidates["aws_spot"]["reason"]
assert candidates["codebuild"]["decision"] == "eligible"
assert "run_aws_codefriend_build_lane.sh" in candidates["codebuild"]["command"]
assert "--project-name adl-codefriend-build" in candidates["codebuild"]["command"]
assert "--print-command" in candidates["codebuild"]["command"]
assert "<branch-or-ref>" not in candidates["codebuild"]["command"]
PY

bash "$SCRIPT" \
  --changed-files "$remote_profile_changed" \
  --validation-platform nessus \
  --json >"$TMP/platform-nessus.json"
python3 - <<'PY' "$TMP/platform-nessus.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
routing = profile["platform_routing"]
assert routing["requested_platform"] == "nessus"
assert routing["decision"] == "selected"
assert routing["selected_platform"] == "nessus"
candidates = {item["platform"]: item for item in routing["candidates"]}
assert candidates["nessus"]["decision"] == "eligible"
assert "run_nessus_remote_validation.sh" in candidates["nessus"]["command"]
assert candidates["nessus"]["cache_posture"] == "remote_target_sccache_warm"
PY

bash "$SCRIPT" \
  --changed-files "$remote_profile_changed" \
  --validation-platform aws_spot \
  --json >"$TMP/platform-spot.json"
python3 - <<'PY' "$TMP/platform-spot.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
routing = profile["platform_routing"]
assert routing["requested_platform"] == "aws_spot"
assert routing["decision"] == "selected"
assert routing["selected_platform"] == "aws_spot"
candidates = {item["platform"]: item for item in routing["candidates"]}
spot = candidates["aws_spot"]
assert spot["decision"] == "eligible"
assert "run_aws_spot_remote_validation_lane.sh" in spot["command"]
assert "--print-command" in spot["command"]
assert "<branch-or-ref>" not in spot["command"]
assert spot["cache_posture"] == "warm_ebs_cache:/mnt/adl-cache"
assert any("retained EBS cache" in caveat for caveat in spot["caveats"])
PY

bash "$SCRIPT" \
  --changed-files "$daemon_wave" \
  --validation-platform codebuild \
  --json >"$TMP/platform-codebuild.json"
python3 - <<'PY' "$TMP/platform-codebuild.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
routing = profile["platform_routing"]
assert routing["requested_platform"] == "codebuild"
assert routing["decision"] == "selected"
assert routing["selected_platform"] == "codebuild"
candidates = {item["platform"]: item for item in routing["candidates"]}
codebuild = candidates["codebuild"]
assert codebuild["decision"] == "eligible"
assert "run_aws_codefriend_build_lane.sh" in codebuild["command"]
assert "--project-name adl-codefriend-build" in codebuild["command"]
assert "--print-command" in codebuild["command"]
assert "<branch-or-ref>" not in codebuild["command"]
assert "--git-ref" not in codebuild["command"]
assert "--compute-type" not in codebuild["command"]
assert codebuild["cache_posture"] == "stable_local_target_cache_plus_s3_sccache"
assert "requires builder image and S3 sccache to be configured" in codebuild["caveats"]
PY

if bash "$SCRIPT" \
  --changed-files "$daemon_wave" \
  --validation-platform codebuild \
  --run >"$TMP/platform-codebuild-run.out" 2>"$TMP/platform-codebuild-run.err"; then
  echo "expected selected CodeBuild platform request to remain dry-run only under --run" >&2
  exit 1
fi
assert_has "$TMP/platform-codebuild-run.err" "platform routing is dry-run only for non-local platforms"

if bash "$SCRIPT" \
  --changed-files "$remote_profile_changed" \
  --validation-platform aws_spot \
  --run >"$TMP/platform-spot-run.out" 2>"$TMP/platform-spot-run.err"; then
  echo "expected selected non-local platform request to remain dry-run only under --run" >&2
  exit 1
fi
assert_has "$TMP/platform-spot-run.err" "platform routing is dry-run only for non-local platforms"

bash "$SCRIPT" \
  --changed-files "$runtime" \
  --validation-platform wuji \
  --json >"$TMP/platform-wuji.json"
python3 - <<'PY' "$TMP/platform-wuji.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
routing = profile["platform_routing"]
assert routing["requested_platform"] == "wuji"
assert routing["decision"] == "rejected"
assert routing["selected_platform"] is None
candidates = {item["platform"]: item for item in routing["candidates"]}
wuji = candidates["wuji"]
assert "ARM" in wuji["reason"]
assert wuji["cache_posture"] == "linked_target_cache_warm_arm64"
assert "arm64_builder_image_gap" in wuji["caveats"]
PY

scheduler_provider_policy="$TMP/scheduler-provider-policy.txt"
cat >"$scheduler_provider_policy" <<'EOF'
M	adl/src/provider/profiles.rs
M	adl/src/scheduler.rs
M	adl/tests/fixtures/scheduler/cheapest_validated_outcome_inputs_v1.json
M	docs/milestones/v0.91.7/review/provider/CHEAPEST_VALIDATED_OUTCOME_POLICY_4674.md
M	docs/milestones/v0.91.7/review/provider/artifacts/cheapest_validated_outcome_plan_4674.json
EOF
bash "$SCRIPT" --changed-files "$scheduler_provider_policy" --json >"$TMP/scheduler-provider-policy.json"
python3 - <<'PY' "$TMP/scheduler-provider-policy.json"
import json
import sys

profile = json.load(open(sys.argv[1]))
assert profile["schema_version"] == "adl.validation_profile.v1"
assert profile["status"] == "ready_to_run"
assert profile["selected_profile"] == "selected_2_lane_profile"
assert profile["pr_publication_sufficient"] is True
assert profile["escalation"]["required"] is False
assert profile["validation_split"]["fast_lane"]["selected_lanes"] == ["docs_diff_check", "rust_pr_fast"]
assert profile["validation_split"]["fast_lane"]["runnable"] is True
assert profile["validation_split"]["fail_closed"]["required"] is False
assert [item["lane_id"] for item in profile["run"]] == ["docs_diff_check", "rust_pr_fast"]
assert "run_pr_fast_test_lane.sh" in profile["run"][1]["command"]
surfaces = {surface["id"]: surface for surface in profile["behavior_surfaces"]}
surface = surfaces["rust_focused_behavior"]
assert surface["id"] == "rust_focused_behavior"
assert "scheduler_economics" in surface["requirement_ids"]
assert "adl/src/provider/profiles.rs" in surface["matched_paths"]
assert "adl/src/scheduler.rs" in surface["matched_paths"]
assert profile["diagnostics"] == []
PY

echo "PASS test_validation_manager"
