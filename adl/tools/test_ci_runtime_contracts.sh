#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WORKFLOW="$ROOT_DIR/.github/workflows/ci.yaml"

python3 - "$WORKFLOW" "$ROOT_DIR/adl/tools/test_run_authoritative_coverage_lane.sh" <<'PY'
import pathlib
import re
import sys

workflow_path = pathlib.Path(sys.argv[1])
workflow = workflow_path.read_text()
runner_test = pathlib.Path(sys.argv[2])
workflow_root = workflow_path.parent

def step_run(name: str) -> str:
    pattern = re.compile(
        rf"^\s*-\s+name:\s+{re.escape(name)}\s*$"
        rf"(?:\n^\s+.*$)*?"
        rf"\n^\s+run:\s+(.+)$",
        re.MULTILINE,
    )
    match = pattern.search(workflow)
    if not match:
        raise SystemExit(f"missing workflow step: {name}")
    return match.group(1).strip()

def step_block(name: str) -> str:
    pattern = re.compile(
        rf"^\s*-\s+name:\s+{re.escape(name)}\s*$"
        rf"((?:\n^\s+.*$)*?)(?=\n^\s*-\s+name:|\Z)",
        re.MULTILINE,
    )
    match = pattern.search(workflow)
    if not match:
        raise SystemExit(f"missing workflow step block: {name}")
    return match.group(1)

def step_if(name: str) -> str:
    pattern = re.compile(
        rf"^\s*-\s+name:\s+{re.escape(name)}\s*$"
        rf"(?:\n^\s+.*$)*?"
        rf"\n^\s+if:\s+(.+)$",
        re.MULTILINE,
    )
    match = pattern.search(workflow)
    if not match:
        raise SystemExit(f"missing workflow if condition for step: {name}")
    return match.group(1).strip()

checkout_sha = "actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5"
for candidate in sorted(workflow_root.glob("*.y*ml")):
    text = candidate.read_text()
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("uses: actions/checkout@") and checkout_sha not in stripped:
            raise SystemExit(
                f"workflow must pin actions/checkout to the canonical SHA; "
                f"found {stripped!r} in {candidate.name}"
            )

ordinary_test = step_run("test")
expected_ordinary_test = (
    'bash adl/tools/run_pr_fast_test_lane.sh --base "${{ github.event.pull_request.base.sha }}" '
    '--head "${{ github.event.pull_request.head.sha }}"'
)
if ordinary_test != expected_ordinary_test:
    raise SystemExit(
        "ordinary adl-ci test lane must run through the fail-closed PR-fast runner; "
        f"found: {ordinary_test}"
    )

ordinary_doc_test = step_run("doc test")
if ordinary_doc_test != "cargo test --doc":
    raise SystemExit(
        "ordinary adl-ci doc-test lane must be 'cargo test --doc' without --all-features; "
        f"found: {ordinary_doc_test}"
    )

authoritative_contract = step_run("authoritative coverage lane contract")
if authoritative_contract != "bash adl/tools/test_run_authoritative_coverage_lane.sh":
    raise SystemExit(
        "adl-ci must validate the authoritative coverage split contract explicitly; "
        f"found: {authoritative_contract}"
    )

release_version_truth = step_run("release version truth check")
if release_version_truth != "bash adl/tools/check_release_version_surfaces.sh":
    raise SystemExit(
        "release-version-only PRs must run the bounded release version truth check; "
        f"found: {release_version_truth}"
    )

if "tool: nextest" not in workflow:
    raise SystemExit(
        "coverage lanes must install cargo-nextest before running cargo llvm-cov nextest"
    )

expected_coverage = (
    'bash tools/run_authoritative_coverage_lane.sh --authority "${{ steps.path-policy.outputs.coverage_authority }}" '
    '--event-name "${{ github.event_name }}"'
)
coverage_step = step_run("Coverage run and summary (json)")
if coverage_step != expected_coverage:
    raise SystemExit(
        "authoritative coverage lane must route through the bounded runner; "
        f"found: {coverage_step}"
    )

if not runner_test.exists():
    raise SystemExit(
        "authoritative coverage runner contract test must exist"
    )

fast_summary_step = step_block("PR fast coverage summary (json)")
if 'cargo llvm-cov report --json --summary-only --output-path coverage-summary.json' not in fast_summary_step:
    raise SystemExit(
        "PR fast coverage summary must emit coverage-summary.json inside the adl working directory; "
        "workflow is missing that output path"
    )

authoritative_gate_step = step_block("Coverage-impact changed-source gate")
if '--summary adl/coverage-summary.json \\' not in authoritative_gate_step:
    raise SystemExit(
        "authoritative changed-source coverage gate must read adl/coverage-summary.json from the runner output; "
        "workflow is missing that summary reference"
    )

pr_preflight_step = step_block("PR coverage-impact preflight")
if '--summary adl/coverage-summary.json \\' not in pr_preflight_step:
    raise SystemExit(
        "PR coverage-impact preflight must read adl/coverage-summary.json emitted by the fast lane working directory; "
        "workflow is missing that summary reference"
    )

gate_if = step_if("Enforce coverage policy gates (workspace + per-file)")
expected_gate_fragment = "steps.path-policy.outputs.coverage_authority != 'pr_policy_surface_tooling_only'"
if expected_gate_fragment not in gate_if:
    raise SystemExit(
        "workspace coverage gate must defer for tooling-only policy authoritative PRs; "
        f"found: {gate_if}"
    )

gate_block = step_block("Enforce coverage policy gates (workspace + per-file)")
slow_proof_exclusion = (
    "adl/src/runtime_v2/(a2a_adapter_boundary|access_control|acip_hardening|challenge|contract_registry_accessors)"
)
if slow_proof_exclusion not in gate_block:
    raise SystemExit(
        "default-feature coverage gate must exclude source files whose tests are explicitly owned by slow-proof-tests; "
        "workflow is missing the slow-proof per-file exclusion"
    )

deferred_policy_step = step_if("Full workspace coverage gate deferred for policy PR")
expected_deferred_fragment = "steps.path-policy.outputs.coverage_authority == 'pr_policy_surface_tooling_only'"
if expected_deferred_fragment not in deferred_policy_step:
    raise SystemExit(
        "tooling-only policy PR defer note must be keyed to pr_policy_surface_tooling_only; "
        f"found: {deferred_policy_step}"
    )

for step_name in (
    "Coverage (ADL Rust workspace lcov)",
    "Coverage summary (text)",
    "Verify generated lcov file",
    "Verify lcov path from repository root",
    "Upload coverage artifact",
):
    step_condition = step_if(step_name)
    if "github.event_name != 'pull_request'" not in step_condition:
        raise SystemExit(
            f"{step_name} must be skipped for pull_request authoritative coverage runs so PRs avoid nonessential reporting tail; "
            f"found: {step_condition}"
        )

print("PASS test_ci_runtime_contracts")
PY
