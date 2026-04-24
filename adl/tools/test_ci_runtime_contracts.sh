#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WORKFLOW="$ROOT_DIR/.github/workflows/ci.yaml"

python3 - "$WORKFLOW" "$ROOT_DIR/adl/tools/test_run_authoritative_coverage_lane.sh" <<'PY'
import pathlib
import re
import sys

workflow = pathlib.Path(sys.argv[1]).read_text()
runner_test = pathlib.Path(sys.argv[2])

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

gate_if = step_if("Enforce coverage policy gates (workspace + per-file)")
expected_gate_fragment = "steps.path-policy.outputs.coverage_authority != 'pr_policy_surface_tooling_only'"
if expected_gate_fragment not in gate_if:
    raise SystemExit(
        "workspace coverage gate must defer for tooling-only policy authoritative PRs; "
        f"found: {gate_if}"
    )

deferred_policy_step = step_if("Full workspace coverage gate deferred for policy PR")
expected_deferred_fragment = "steps.path-policy.outputs.coverage_authority == 'pr_policy_surface_tooling_only'"
if expected_deferred_fragment not in deferred_policy_step:
    raise SystemExit(
        "tooling-only policy PR defer note must be keyed to pr_policy_surface_tooling_only; "
        f"found: {deferred_policy_step}"
    )

print("PASS test_ci_runtime_contracts")
PY
