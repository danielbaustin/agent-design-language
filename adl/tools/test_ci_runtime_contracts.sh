#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WORKFLOW="$ROOT_DIR/.github/workflows/ci.yaml"

python3 - "$WORKFLOW" "$ROOT_DIR/adl/tools/test_run_authoritative_coverage_lane.sh" "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" "$ROOT_DIR/adl/tools/run_pr_fast_coverage_lane.sh" <<'PY'
import pathlib
import re
import sys

workflow_path = pathlib.Path(sys.argv[1])
workflow = workflow_path.read_text()
runner_test = pathlib.Path(sys.argv[2])
runner_script = pathlib.Path(sys.argv[3])
pr_fast_runner = pathlib.Path(sys.argv[4])
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
    start = re.search(
        rf"^\s*-\s+name:\s+{re.escape(name)}\s*$",
        workflow,
        re.MULTILINE,
    )
    if not start:
        raise SystemExit(f"missing workflow step block: {name}")
    next_step = re.search(r"^\s*-\s+name:\s+", workflow[start.end() :], re.MULTILINE)
    if next_step:
        return workflow[start.end() : start.end() + next_step.start()]
    return workflow[start.end() :]

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

def step_working_directory(name: str) -> str:
    block = step_block(name)
    match = re.search(r"^\s+working-directory:\s+(.+)$", block, re.MULTILINE)
    if not match:
        raise SystemExit(f"missing workflow working-directory for step: {name}")
    return match.group(1).strip()

def step_count(name: str) -> int:
    return len(
        re.findall(
            rf"^\s*-\s+name:\s+{re.escape(name)}\s*$",
            workflow,
            re.MULTILINE,
        )
    )

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

adl_profile_summary = step_block("Validation profile summary (adl-ci)")
for required_fragment in (
    "ADL validation profile",
    "steps.path-policy.outputs.validation_profile_selected",
    "steps.path-policy.outputs.validation_profile_status",
    "steps.path-policy.outputs.validation_profile_pr_publication_sufficient",
    "steps.path-policy.outputs.validation_profile_run_lanes",
    "steps.path-policy.outputs.validation_profile_escalation_required",
    "steps.path-policy.outputs.validation_profile_escalation_lanes",
    "GITHUB_STEP_SUMMARY",
):
    if required_fragment not in adl_profile_summary:
        raise SystemExit(
            "adl-ci must publish validation-manager profile truth to the GitHub step summary; "
            f"missing fragment: {required_fragment}"
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

for root_script_step in (
    "docs command check",
    "ci runtime contract check",
    "ci runtime budget report contract check",
    "ci cache/linker contract check",
    "release version truth check",
):
    if step_working_directory(root_script_step) != ".":
        raise SystemExit(
            "adl-ci workflow steps that call repo-root adl/tools scripts must run from the repository root; "
            f"{root_script_step!r} has working-directory: {step_working_directory(root_script_step)!r}"
        )

if "tool: nextest" not in workflow:
    raise SystemExit(
        "coverage lanes must install cargo-nextest as a required coverage toolchain dependency"
    )
if "cargo llvm-cov nextest" in workflow:
    raise SystemExit("adl-coverage workflow must delegate coverage execution to runner scripts, not inline nextest")

expected_coverage = (
    'bash adl/tools/run_authoritative_coverage_lane.sh --authority "${{ steps.path-policy.outputs.coverage_authority }}" '
    '--event-name "${{ github.event_name }}"'
)
expected_wrapped_coverage = (
    'bash adl/tools/run_ci_step_with_log.sh --name "coverage-run-summary-json" --log-root ci-step-logs -- '
    + expected_coverage
)
coverage_step = step_run("Coverage run and summary (json)")
if coverage_step not in {expected_coverage, expected_wrapped_coverage}:
    raise SystemExit(
        "authoritative coverage lane must route through the bounded runner, optionally via the ADL-owned step-log wrapper; "
        f"found: {coverage_step}"
    )
coverage_step_if = step_if("Coverage run and summary (json)")
if coverage_step_if != "steps.path-policy.outputs.full_coverage_required == 'true'":
    raise SystemExit(
        "authoritative coverage execution must be limited to full_coverage_required surfaces; "
        f"found: {coverage_step_if}"
    )
for root_script_step in (
    "Install lld for coverage",
    "Configure Rust acceleration for coverage",
    "Verify required coverage toolchain",
    "Coverage run and summary (json)",
    "PR fast coverage summary (json)",
    "Enforce coverage policy gates (workspace + per-file)",
    "Rust acceleration stats for coverage",
):
    if step_working_directory(root_script_step) != ".":
        raise SystemExit(
            "coverage workflow steps that call repo-root adl/tools scripts must run from the repository root; "
            f"{root_script_step!r} has working-directory: {step_working_directory(root_script_step)!r}"
        )
coverage_not_required_step = step_if("Coverage not required by path policy")
if coverage_not_required_step != "steps.path-policy.outputs.coverage_required != 'true'":
    raise SystemExit(
        "adl-coverage must report a truthful non-rust/no-coverage-required state instead of compiling unrelated Rust; "
        f"found: {coverage_not_required_step}"
    )

if not runner_test.exists():
    raise SystemExit(
        "authoritative coverage runner contract test must exist"
    )
if not runner_script.exists():
    raise SystemExit(
        "authoritative coverage runner script must exist"
    )
if not pr_fast_runner.exists():
    raise SystemExit(
        "PR-fast coverage runner script must exist"
    )

runner_script_text = runner_script.read_text()
for required_fragment in (
    'default_coverage_build_root()',
    'if [ -d /mnt ] && [ -w /mnt ]; then',
    'printf \'/mnt/adl-authoritative-coverage\\n\'',
    'printf \'%s\\n\' "$ADL_DIR"',
    'COVERAGE_BUILD_ROOT="${ADL_COVERAGE_BUILD_ROOT:-$(default_coverage_build_root)}"',
    'mkdir -p "$COVERAGE_BUILD_ROOT/target" "$COVERAGE_BUILD_ROOT/target/llvm-cov-target"',
    'export CARGO_TARGET_DIR="$COVERAGE_BUILD_ROOT/target"',
    'export CARGO_LLVM_COV_TARGET_DIR="$COVERAGE_BUILD_ROOT/target/llvm-cov-target"',
):
    if required_fragment not in runner_script_text:
        raise SystemExit(
            "authoritative coverage runner must relocate llvm-cov build outputs onto the runner scratch mount; "
            f"missing fragment: {required_fragment}"
        )

pr_fast_step = step_run("PR fast coverage summary (json)")
expected_pr_fast_step = 'bash adl/tools/run_pr_fast_coverage_lane.sh --filter-expression "${{ steps.coverage-impact.outputs.filter_expression }}"'
if pr_fast_step != expected_pr_fast_step:
    raise SystemExit(
        "PR-fast coverage must delegate to the bounded runner script; "
        f"found: {pr_fast_step}"
    )
pr_fast_if = step_if("PR fast coverage summary (json)")
if "steps.coverage-impact.outputs.needs_fast_summary == 'true'" not in pr_fast_if:
    raise SystemExit(
        "PR-fast coverage must run only when coverage-impact requires a focused summary; "
        f"found: {pr_fast_if}"
    )
pr_fast_runner_text = pr_fast_runner.read_text()
for required_fragment in (
    'COVERAGE_BUILD_ROOT="${ADL_PR_FAST_COVERAGE_BUILD_ROOT:-$ADL_DIR/target/pr-fast-coverage}"',
    'export CARGO_TARGET_DIR="$COVERAGE_BUILD_ROOT"',
    'export CARGO_LLVM_COV_TARGET_DIR="$COVERAGE_BUILD_ROOT/llvm-cov-target"',
    "PR-fast coverage target:",
):
    if required_fragment not in pr_fast_runner_text:
        raise SystemExit(
            "PR-fast coverage must use cacheable repo target subdirs; "
            f"missing fragment: {required_fragment}"
        )
for forbidden_fragment in (
    "RUNNER_TEMP",
    "rm -rf target/debug",
    "rm -rf target/llvm-cov-target",
):
    if forbidden_fragment in pr_fast_runner_text:
        raise SystemExit(
            "PR-fast coverage must not destroy or bypass the Rust cache; "
            f"forbidden fragment: {forbidden_fragment}"
        )
filter_if = step_if("Determine PR fast coverage filters")
if "steps.path-policy.outputs.full_coverage_required != 'true'" not in filter_if:
    raise SystemExit(
        "PR-fast filter determination must be limited to non-full PR coverage; "
        f"found: {filter_if}"
    )
pr_fast_runner_text = pr_fast_runner.read_text()
for required_fragment in (
    'FILTER_EXPRESSION=""',
    '--filter-expression',
    'CARGO_INCREMENTAL=0 cargo llvm-cov nextest',
    '--workspace',
    '--status-level all',
    '--final-status-level slow',
    '--no-report',
    '-E "$FILTER_EXPRESSION"',
    'cargo llvm-cov report',
    '--json',
    '--summary-only',
    '--output-path coverage-summary.json',
):
    if required_fragment not in pr_fast_runner_text:
        raise SystemExit(
            "PR-fast coverage runner must execute targeted nextest coverage and produce summary JSON; "
            f"missing fragment: {required_fragment}"
        )
for required_fragment in (
    "cargo llvm-cov nextest \\",
    "    --workspace \\",
    "    --lib \\",
    "    --no-report",
    "cargo llvm-cov report \\",
    "--json \\",
    "--summary-only \\",
    "--output-path coverage-summary.json",
):
    if required_fragment not in runner_script_text:
        raise SystemExit(
            "authoritative coverage runner must execute direct library-only coverage without linking ADL binaries; "
            f"missing fragment: {required_fragment}"
        )
if "    --tests \\" in runner_script_text or "    --bins \\" in runner_script_text or "    --all-targets \\" in runner_script_text:
    raise SystemExit("authoritative coverage runner must not link test/bin/all-target surfaces")

authoritative_gate_step = step_block("Coverage-impact changed-source gate")
if '--summary adl/coverage-summary.json \\' not in authoritative_gate_step:
    raise SystemExit(
        "authoritative changed-source coverage gate must read adl/coverage-summary.json from the runner output; "
        "workflow is missing that summary reference"
    )

pr_preflight_if = step_if("PR coverage-impact preflight")
if "steps.path-policy.outputs.full_coverage_required != 'true'" not in pr_preflight_if:
    raise SystemExit(
        "PR coverage-impact preflight must be limited to non-full PR coverage; "
        f"found: {pr_preflight_if}"
    )

gate_if = step_if("Enforce coverage policy gates (workspace + per-file)")
if "github.event_name != 'pull_request'" not in gate_if:
    raise SystemExit(
        "workspace coverage gate must be skipped for pull_request coverage runs; "
        f"found: {gate_if}"
    )
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

if step_count("Full workspace gate deferred for bounded authoritative PR") != 0:
    raise SystemExit(
        "coverage workflow must not carry the duplicate bounded-authoritative PR defer note"
    )

deferred_policy_step = step_if("Full workspace coverage gate deferred for PR")
expected_deferred_fragment = "github.event_name == 'pull_request'"
if expected_deferred_fragment not in deferred_policy_step:
    raise SystemExit(
        "PR defer note must be keyed to pull_request coverage runs; "
        f"found: {deferred_policy_step}"
    )

coverage_profile_summary = step_block("Validation profile summary (adl-coverage)")
for required_fragment in (
    "ADL coverage validation profile",
    "steps.path-policy.outputs.coverage_lane",
    "steps.path-policy.outputs.coverage_authority",
    "steps.path-policy.outputs.validation_profile_status",
    "steps.path-policy.outputs.validation_profile_run_lanes",
    "steps.path-policy.outputs.validation_profile_escalation_required",
    "GITHUB_STEP_SUMMARY",
):
    if required_fragment not in coverage_profile_summary:
        raise SystemExit(
            "adl-coverage must publish validation profile and coverage authority truth to the GitHub step summary; "
            f"missing fragment: {required_fragment}"
        )

nightly = (workflow_root / "nightly-coverage-ratchet.yaml").read_text()
if "schedule:" not in nightly or 'cron: "43 11 * * *"' not in nightly:
    raise SystemExit(
        "nightly-coverage-ratchet must have an actual scheduled trigger or stop calling itself nightly"
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
