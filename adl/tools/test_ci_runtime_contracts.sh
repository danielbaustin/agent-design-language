#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WORKFLOW="$ROOT_DIR/.github/workflows/ci.yaml"

ruby -ryaml - "$WORKFLOW" <<'RUBY'
NEXTTEST_INSTALLER = "taiki-e/install-action@50414676f9f5d50a65992c6dd2ed02641263226c"
class NextestContractError < StandardError; end

def require_nextest_contract(source)
  workflow = YAML.safe_load(
    source,
    permitted_classes: [],
    permitted_symbols: [],
    aliases: true
  )
  steps = workflow.fetch("jobs").values.flat_map { |job| job.fetch("steps", []) }
  install_steps = steps.select do |step|
    step.fetch("uses", "").start_with?("taiki-e/install-action@")
  end
  nextest_steps = install_steps.each_with_object([]) do |step, selected|
    tools = step.fetch("with", {}).fetch("tool", "").to_s.split(/[\s,]+/).reject(&:empty?)
    selected << [step, tools] if tools.any? { |tool| tool.match?(/\A(?:cargo-)?nextest(?:@.*)?\z/) }
  end
  raise NextestContractError, "CI must retain exactly seven declared nextest install steps" unless nextest_steps.length == 7

  nextest_steps.each do |step, tools|
    name = step.fetch("name", "unnamed nextest install")
    raise NextestContractError, "#{name} must use the supported immutable installer" unless step["uses"] == NEXTTEST_INSTALLER
    inputs = step.fetch("with", {})
    raise NextestContractError, "#{name} must select only nextest 0.9.140" unless tools == ["nextest@0.9.140"]
    raise NextestContractError, "#{name} must disable installer fallback" unless inputs["fallback"] == "none"
  end
end

workflow = File.read(ARGV.fetch(0))
begin
  require_nextest_contract(workflow)
rescue NextestContractError => error
  abort error.message
end

fixtures = [
  workflow + %Q(\n      - uses: taiki-e/install-action@v2\n        with:\n          tool: nextest@0.9.140\n          fallback: cargo-install\n),
  workflow.sub(
    "with:\n          tool: nextest@0.9.140\n          fallback: none",
    "with: {tool: nextest@0.9.140, fallback: cargo-install}"
  ),
  workflow.sub(NEXTTEST_INSTALLER, "taiki-e/install-action@v2"),
  workflow.sub("fallback: none", "fallback: cargo-install"),
  workflow + %Q(\n      - name: Floating nextest alias\n        uses: taiki-e/install-action@v2\n        with: {tool: nextest, fallback: cargo-install}\n),
  workflow + %Q(\n      - name: Floating cargo-nextest alias\n        uses: "taiki-e/install-action@v2"\n        with:\n          tool: cargo-nextest\n          fallback: cargo-install\n),
  workflow + %Q(\n      - {name: Inline nextest, uses: taiki-e/install-action@v2, with: {tool: nextest, fallback: cargo-install}}\n),
  workflow + %Q(\n      - name: Comma-list nextest\n        uses: taiki-e/install-action@v2\n        with: {tool: "sccache,nextest@0.9.140", fallback: cargo-install}\n),
  workflow + %Q(\n      - name: Space-list nextest\n        uses: taiki-e/install-action@v2\n        with: {tool: "sccache nextest@0.9.140", fallback: cargo-install}\n),
  workflow + %Q(\n      - name: Multi-tool cargo-nextest alias\n        uses: taiki-e/install-action@v2\n        with: {tool: "sccache,cargo-nextest@0.9.140", fallback: cargo-install}\n)
]
fixtures.each do |fixture|
  begin
    require_nextest_contract(fixture)
  rescue NextestContractError
    next
  end
  abort "invalid nextest installer fixture escaped enforcement"
end
RUBY

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

def step_optional_if(name: str) -> str:
    block = step_block(name)
    match = re.search(r"^\s+if:\s+(.+)$", block, re.MULTILINE)
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

def job_block(job_name: str) -> str:
    start = re.search(rf"^  {re.escape(job_name)}:\n", workflow, re.MULTILINE)
    if not start:
        raise SystemExit(f"missing workflow job: {job_name}")
    next_job = re.search(r"^  [A-Za-z0-9_-]+:\n", workflow[start.end() :], re.MULTILINE)
    if next_job:
        return workflow[start.start() : start.end() + next_job.start()]
    return workflow[start.start() :]

canonical_actions = {
    "actions/checkout": "9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0",
    "actions/download-artifact": "37930b1c2abaa49bbe596cd826c3c89aef350131",
    "actions/upload-artifact": "043fb46d1a93c77aae656e7c1c64a875d1fc6a0a",
    "Swatinem/rust-cache": "c19371144df3bb44fab255c43d04cbc2ab54d1c4",
}
deprecated_shas = {
    "34e114876b0b11c390a56381ad16ebd13914f8d5",
    "ea165f8d65b6e75b540449e92b4886f43607fa02",
    "779680da715d629ac1d338a641029a2f4372abb5",
}
seen = {action: 0 for action in canonical_actions}

def parse_uses(line: str) -> str | None:
    match = re.match(r"^(?:-\s+)?uses:\s+(.+?)\s*(?:#.*)?$", line.strip())
    if not match:
        return None
    value = match.group(1).strip()
    if value[:1] in {"'", '"'}:
        if len(value) < 2 or value[-1] != value[0]:
            raise SystemExit(f"invalid quoted workflow uses scalar: {line.strip()!r}")
        value = value[1:-1]
    return value

def require_canonical_action(uses: str, source: str) -> None:
    for action, sha in canonical_actions.items():
        if uses.startswith(f"{action}@"):
            expected = f"{action}@{sha}"
            if uses != expected:
                raise SystemExit(
                    f"workflow must pin {action} to the canonical Node 24 SHA; "
                    f"found {uses!r} in {source}"
                )
            seen[action] += 1

for candidate in sorted(workflow_root.glob("*.y*ml")):
    text = candidate.read_text()
    for deprecated_sha in deprecated_shas:
        if deprecated_sha in text:
            raise SystemExit(
                f"workflow retains deprecated Node 20 action SHA {deprecated_sha} "
                f"in {candidate.name}"
            )
    for line in text.splitlines():
        uses = parse_uses(line)
        if uses is None:
            continue
        require_canonical_action(uses, candidate.name)

for fixture in (
    'uses: "actions/checkout@v7"',
    "- uses: 'actions/upload-artifact@v7'",
):
    try:
        require_canonical_action(parse_uses(fixture), "quoted negative fixture")
    except SystemExit as exc:
        if "canonical Node 24 SHA" not in str(exc):
            raise
    else:
        raise SystemExit(f"quoted floating action pin escaped enforcement: {fixture}")

for action, count in seen.items():
    if count == 0:
        raise SystemExit(f"canonical action inventory unexpectedly contains no {action} use")

adl_profile_summary = step_block("Validation profile summary (adl-ci)")
for required_fragment in (
    "ADL validation profile",
    "steps.path-policy.outputs.validation_profile_selected",
    "steps.path-policy.outputs.validation_profile_status",
    "steps.path-policy.outputs.validation_profile_pr_publication_sufficient",
    "steps.path-policy.outputs.validation_profile_run_lanes",
    "steps.path-policy.outputs.validation_profile_escalation_required",
    "steps.path-policy.outputs.validation_profile_escalation_lanes",
    "steps.path-policy.outputs.ci_path_policy_contracts_required",
    "steps.path-policy.outputs.ci_contract_toolchain_required",
    "steps.path-policy.outputs.skill_author_contracts_required",
    "GITHUB_STEP_SUMMARY",
):
    if required_fragment not in adl_profile_summary:
        raise SystemExit(
            "adl-ci must publish validation-manager profile truth to the GitHub step summary; "
            f"missing fragment: {required_fragment}"
        )

for required_fragment in (
    "adl_path_policy:",
    "csdlc_v2_standalone:",
    "adl_v2_standalone:",
    "adl_tooling_contracts:",
    "adl_rust_fmt_clippy:",
    "adl_rust_tests:",
    "adl_demo_proof:",
    "adl-ci:",
    "needs:",
    "Aggregate split adl-ci lanes",
    r"Stable required check \`adl-ci\` is an aggregator over parallel lanes.",
):
    if required_fragment not in workflow:
        raise SystemExit(
            "adl-ci must remain split into parallel required lanes with a stable aggregator; "
            f"missing fragment: {required_fragment}"
        )

csdlc_v2_job = job_block("csdlc_v2_standalone")
for required_fragment in (
    'build_root="$RUNNER_TEMP/adl-csdlc-v2"',
    'echo "ADL_CARGO_BUILD_ROOT=$build_root" >> "$GITHUB_ENV"',
):
    if required_fragment not in csdlc_v2_job:
        raise SystemExit(
            "C-SDLC v2 standalone job must derive its external Cargo root from RUNNER_TEMP; "
            f"missing fragment: {required_fragment}"
        )
if "runner.temp" in csdlc_v2_job:
    raise SystemExit(
        "C-SDLC v2 standalone job must not use unavailable runner context in job-level env"
    )

aggregator_block = step_block("Aggregate split adl-ci lanes")
for required_fragment in (
    "needs.adl_path_policy.result",
    "needs.csdlc_v2_standalone.result",
    "needs.adl_tooling_contracts.result",
    "needs.adl_rust_fmt_clippy.result",
    "needs.adl_rust_tests.result",
    "needs.adl_demo_proof.result",
    "success|skipped",
    "selected C-SDLC v2 standalone lane",
    "unselected C-SDLC v2 standalone lane",
    "csdlc_v2_standalone_required must be exactly true or false",
    "::error::split adl-ci lane failure(s):",
):
    if required_fragment not in aggregator_block:
        raise SystemExit(
            "adl-ci aggregator must fail closed on split lane failures while accepting skipped lanes; "
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
ordinary_test_if = step_optional_if("test")
expected_ordinary_test_if = (
    "needs.adl_path_policy.outputs.full_coverage_required != 'true' && "
    "needs.adl_path_policy.outputs.validation_profile_escalation_required != 'true'"
)
if ordinary_test_if != expected_ordinary_test_if:
    raise SystemExit(
        "ordinary adl-ci test lane must not run the fail-closed PR-fast runner after validation-manager escalation; "
        f"found: {ordinary_test_if}"
    )

escalated_test_if = step_optional_if("test deferred to validation-manager escalation")
expected_escalated_test_if = (
    "needs.adl_path_policy.outputs.full_coverage_required != 'true' && "
    "needs.adl_path_policy.outputs.validation_profile_escalation_required == 'true'"
)
if escalated_test_if != expected_escalated_test_if:
    raise SystemExit(
        "adl-ci must publish a truthful deferred-test step when validation-manager escalation owns the Rust proof; "
        f"found: {escalated_test_if}"
    )
escalated_test_block = step_block("test deferred to validation-manager escalation")
for required_fragment in (
    "Ordinary PR-fast Rust test lane deferred",
    "needs.adl_path_policy.outputs.validation_profile_selected",
    "needs.adl_path_policy.outputs.validation_profile_status",
    "needs.adl_path_policy.outputs.validation_profile_pr_publication_sufficient",
    "needs.adl_path_policy.outputs.validation_profile_run_lanes",
    "needs.adl_path_policy.outputs.validation_profile_escalation_lanes",
    "needs.adl_path_policy.outputs.validation_profile_primary_reason",
):
    if required_fragment not in escalated_test_block:
        raise SystemExit(
            "deferred PR-fast test step must report validation-manager escalation truth; "
            f"missing fragment: {required_fragment}"
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
expected_split_conditions = {
    "PVF CI release policy contract": "needs.adl_path_policy.outputs.pvf_ci_release_contract_required == 'true'",
    "tracked proof-validation lane contract": "needs.adl_path_policy.outputs.v0913_proof_contract_required == 'true'",
    "PR-fast test lane contract": "needs.adl_path_policy.outputs.ci_path_policy_contracts_required == 'true' || needs.adl_path_policy.outputs.rust_required == 'true'",
    "slow-proof lane contract": "needs.adl_path_policy.outputs.slow_proof_contract_required == 'true'",
    "authoritative coverage lane contract": "needs.adl_path_policy.outputs.ci_path_policy_contracts_required == 'true' || needs.adl_path_policy.outputs.full_coverage_required == 'true'",
    "repo-code-review contract check": "needs.adl_path_policy.outputs.skill_author_contracts_required == 'true'",
    "test-generator contract check": "needs.adl_path_policy.outputs.skill_author_contracts_required == 'true'",
    "demo-operator contract check": "needs.adl_path_policy.outputs.skill_author_contracts_required == 'true'",
    "arxiv-paper-writer contract check": "needs.adl_path_policy.outputs.skill_author_contracts_required == 'true'",
    "ANRM/Gemma trace dataset tooling check": "needs.adl_path_policy.outputs.skill_author_contracts_required == 'true'",
    "ci runtime contract check": "needs.adl_path_policy.outputs.ci_path_policy_contracts_required == 'true'",
    "ci runtime budget report contract check": "needs.adl_path_policy.outputs.ci_path_policy_contracts_required == 'true'",
    "ci cache/linker contract check": "needs.adl_path_policy.outputs.ci_path_policy_contracts_required == 'true'",
}
for step_name, expected_if in expected_split_conditions.items():
    observed_if = step_if(step_name)
    if observed_if != expected_if:
        raise SystemExit(
            "adl-ci contract checks must use granular path-policy outputs so narrow policy PRs do not run unrelated contracts; "
            f"{step_name!r} has if: {observed_if!r}"
        )

for step_name in (
    "Install cargo-llvm-cov for CI contract checks",
    "Install cargo-nextest for CI contract checks",
):
    install_block = step_block(step_name)
    conditional = re.search(r"^\s+if:\s+(.+)$", install_block, re.MULTILINE)
    if conditional:
        raise SystemExit(
            "always-run tooling contracts require their llvm-cov and nextest prerequisites to be installed unconditionally; "
            f"{step_name!r} has if: {conditional.group(1).strip()!r}"
        )

slow_proof_job = job_block("adl-slow-proof")
if "needs: adl_path_policy" not in slow_proof_job:
    raise SystemExit("adl-slow-proof must depend on path policy so PR slow-proof requests can trigger the slow lane")
expected_slow_proof_if = (
    "github.event_name == 'push' || github.event_name == 'schedule' || "
    "github.event_name == 'workflow_dispatch' || "
    "needs.adl_path_policy.outputs.slow_proof_contract_required == 'true'"
)
slow_proof_if_match = re.search(r"^\s+if:\s+(.+)$", slow_proof_job, re.MULTILINE)
if not slow_proof_if_match or slow_proof_if_match.group(1).strip() != expected_slow_proof_if:
    raise SystemExit("adl-slow-proof must run on PRs when slow_proof_contract_required is true")
if "shard: [1, 2, 3, 4]" not in slow_proof_job:
    raise SystemExit("adl-slow-proof must keep the long slow-proof lane fanned out across four shards")
if 'bash tools/run_slow_proof_family.sh --family all --run --partition "count:${{ matrix.shard }}/4"' not in slow_proof_job:
    raise SystemExit("adl-slow-proof must use the configured slow-proof family filter with nextest partition fanout")
if "cargo nextest run --features slow-proof-tests --partition" in slow_proof_job:
    raise SystemExit("adl-slow-proof must not use a broad slow-proof-tests run that mixes fast and slow runtime_v2 tests")

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

workspace_coverage_step = step_run("Workspace coverage run and summary (json)")
for required_fragment in (
    '--name "coverage-workspace-profraw-shard-${{ matrix.shard }}" --log-root ci-step-logs',
    'run_authoritative_coverage_lane.sh --profile workspace',
    '--authority "${{ steps.path-policy.outputs.coverage_authority }}"',
    '--event-name "${{ github.event_name }}"',
):
    if required_fragment not in workspace_coverage_step:
        raise SystemExit(
            "workspace producer must run only the workspace authoritative coverage profile; "
            f"missing fragment: {required_fragment}"
        )
if step_if("Workspace coverage run and summary (json)") != "steps.path-policy.outputs.full_coverage_required == 'true'":
    raise SystemExit("workspace authoritative coverage must remain limited to full_coverage_required surfaces")
workspace_coverage_block = step_block("Workspace coverage run and summary (json)")
for required_fragment in (
    "ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE: collect",
    "ADL_AUTHORITATIVE_COVERAGE_SHARD_COUNT: 2",
    "ADL_AUTHORITATIVE_COVERAGE_SHARD_INDEX: ${{ matrix.shard }}",
    "ADL_COVERAGE_RUN_ID: ${{ github.run_id }}-${{ github.run_attempt }}-workspace-shard-${{ matrix.shard }}",
):
    if required_fragment not in workspace_coverage_block:
        raise SystemExit(
            "workspace producer must collect run-scoped profraw profiles for its assigned shard; "
            f"missing fragment: {required_fragment}"
        )

runtime_coverage_step = step_run("Runtime coverage run and summary (json)")
for required_fragment in (
    '--name "coverage-runtime-summary-json" --log-root ci-step-logs',
    'run_authoritative_coverage_lane.sh --profile adl-runtime',
    '--authority "${{ steps.path-policy.outputs.coverage_authority }}"',
    '--event-name "${{ github.event_name }}"',
):
    if required_fragment not in runtime_coverage_step:
        raise SystemExit(
            "runtime producer must run only the adl-runtime authoritative coverage profile; "
            f"missing fragment: {required_fragment}"
        )
for root_script_step in (
    "Install lld for coverage",
    "Configure Rust acceleration for coverage",
    "Verify required coverage toolchain",
    "Workspace coverage run and summary (json)",
    "Install lld for runtime coverage",
    "Configure Rust acceleration for runtime coverage",
    "Verify required runtime coverage toolchain",
    "Runtime coverage run and summary (json)",
    "PR fast coverage summary (json)",
    "Rust acceleration stats for coverage",
    "Rust acceleration stats for runtime coverage",
):
    if step_working_directory(root_script_step) != ".":
        raise SystemExit(
            "coverage workflow steps that call repo-root adl/tools scripts must run from the repository root; "
            f"{root_script_step!r} has working-directory: {step_working_directory(root_script_step)!r}"
        )

runtime_job = job_block("adl_coverage_runtime_hosted")
workspace_fast_job = job_block("adl_coverage_workspace_fast_hosted")
workspace_job = job_block("adl_coverage_workspace_hosted")
hosted_aggregator = job_block("adl_coverage_hosted")
required_status_job = job_block("adl-coverage")
if "cargo llvm-cov report --lcov" in workspace_job + workspace_fast_job:
    raise SystemExit("workspace workflow must not run detached post-profile lcov commands")

if "runs-on: ubuntu-latest" not in runtime_job or "runs-on: ubuntu-latest" not in workspace_job or "runs-on: ubuntu-latest" not in workspace_fast_job:
    raise SystemExit("both isolated coverage producers must use fresh GitHub-hosted runners")
if "needs.adl_path_policy.outputs.full_coverage_required == 'true'" not in runtime_job.split("runs-on:", 1)[0]:
    raise SystemExit("runtime coverage producer must run only for full authoritative coverage")
if "needs.adl_path_policy.outputs.full_coverage_required == 'true'" not in workspace_job.split("runs-on:", 1)[0]:
    raise SystemExit("workspace shard producer must run only for full authoritative coverage")
if "needs.adl_path_policy.outputs.coverage_required == 'true'" not in workspace_fast_job.split("runs-on:", 1)[0] or "needs.adl_path_policy.outputs.full_coverage_required != 'true'" not in workspace_fast_job.split("runs-on:", 1)[0]:
    raise SystemExit("workspace fast producer must retain non-full coverage-required PR-fast routing")
if "PR fast coverage summary (json)" not in workspace_fast_job or "PR fast coverage summary (json)" in workspace_job or "PR fast coverage summary (json)" in runtime_job:
    raise SystemExit("only the workspace fast producer may own PR-fast coverage")

for required_fragment in (
    "adl-coverage-runtime-${{ github.run_id }}-${{ github.run_attempt }}",
    "coverage-summary.adl-runtime.json",
    "coverage-provenance.adl-runtime.json",
    "ci-step-logs/",
):
    if required_fragment not in runtime_job:
        raise SystemExit(f"runtime producer artifact is missing {required_fragment}")
for required_fragment in (
    "adl-coverage-workspace-${{ github.run_id }}-${{ github.run_attempt }}",
    "coverage-summary.adl.json",
    "coverage-provenance.workspace.json",
    "ci-step-logs/",
):
    if required_fragment not in workspace_fast_job:
        raise SystemExit(f"workspace fast producer artifact is missing {required_fragment}")
for required_fragment in (
    "matrix:\n        shard: [1, 2]",
    "adl-coverage-workspace-profraw-${{ github.run_id }}-${{ github.run_attempt }}-${{ matrix.shard }}",
    "adl/target/llvm-cov-target/${{ github.run_id }}-${{ github.run_attempt }}-workspace-shard-${{ matrix.shard }}/workspace/*.profraw",
    "coverage-provenance.workspace.json",
    "COVERAGE_SHARD_COUNT: 2",
    "ci-step-logs/",
):
    if required_fragment not in workspace_job:
        raise SystemExit(f"workspace shard producer artifact is missing {required_fragment}")
if '"shard_count": os.environ["COVERAGE_SHARD_COUNT"]' not in workspace_job or '"shard_count": os.environ["COVERAGE_SHARD_COUNT"]' in workspace_fast_job:
    raise SystemExit("only full workspace shard provenance may include shard_count")
if "adl/ci-step-logs/" in runtime_job or "adl/ci-step-logs/" in workspace_job or "adl/ci-step-logs/" in workspace_fast_job:
    raise SystemExit("coverage log artifacts must upload from repo-root ci-step-logs/")

for job, profile in ((runtime_job, "adl-runtime"), (workspace_job, "workspace"), (workspace_fast_job, "workspace")):
    for required_fragment in (
        'COVERAGE_HEAD_SHA: ${{ github.event.pull_request.head.sha || github.sha }}',
        'COVERAGE_RUN_ID: ${{ github.run_id }}',
        '"head_sha"',
        f'"profile": "{profile}"',
        '"run_id"',
    ):
        if required_fragment not in job:
            raise SystemExit(f"{profile} producer provenance is missing exact run/head/profile evidence: {required_fragment}")

aggregator_header = hosted_aggregator.split("steps:", 1)[0]
for required_fragment in (
    "if: always()",
    "adl_coverage_runtime_hosted",
    "adl_coverage_workspace_fast_hosted",
    "adl_coverage_workspace_hosted",
    "route_result: ${{ steps.producer-results.outputs.route_result }}",
):
    if required_fragment not in aggregator_header:
        raise SystemExit(f"hosted coverage aggregator topology is missing {required_fragment}")
for required_fragment in (
    "actions/download-artifact@37930b1c2abaa49bbe596cd826c3c89aef350131",
    "coverage-artifacts/runtime",
    "coverage-artifacts/workspace",
    "coverage-artifacts/workspace-profraw",
    "pattern: adl-coverage-workspace-profraw-${{ github.run_id }}-${{ github.run_attempt }}-*",
    'document != expected',
    'coverage provenance mismatch',
    "expected 2 workspace shard provenance files",
    '"shard_count": "2"',
    'seen_shards != {"1", "2"}',
    "workspace shard profraw profiles missing",
    'expected_workspace=skipped',
    'expected_workspace_fast=skipped',
    'expected_runtime=skipped',
    'expected_workspace=success',
    'expected_workspace_fast=success',
    'expected_runtime=success',
    'PATH_POLICY_RESULT" != success',
    'WORKSPACE_RESULT" != "$expected_workspace',
    'WORKSPACE_FAST_RESULT" != "$expected_workspace_fast',
    'RUNTIME_RESULT" != "$expected_runtime',
    'echo "route_result=success" >> "$GITHUB_OUTPUT"',
    'echo "route_result=skipped" >> "$GITHUB_OUTPUT"',
    "ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE: report",
    "ADL_AUTHORITATIVE_COVERAGE_IMPORT_PROFRAW_DIR: ${{ github.workspace }}/coverage-artifacts/workspace-profraw",
    '--name "coverage-workspace-aggregate-summary-json" --log-root ci-step-logs',
    "python3 adl/tools/merge_coverage_summaries.py",
    "--workspace adl/coverage-summary.adl.json",
    "--adl-runtime coverage-artifacts/runtime/adl/coverage-summary.adl-runtime.json",
    "--output adl/coverage-summary.json",
):
    if required_fragment not in hosted_aggregator:
        raise SystemExit(f"hosted coverage aggregator is missing fail-closed evidence handling: {required_fragment}")

if "Enforce coverage policy gates (workspace + per-file)" not in hosted_aggregator:
    raise SystemExit("existing coverage gates must run in the hosted aggregator after summary merge")
if "Enforce coverage policy gates (workspace + per-file)" in workspace_job:
    raise SystemExit("workspace producer must not gate an unmerged profile summary")
if re.search(r"(?:^|\s)aws(?:\s|$)", runtime_job + workspace_job + hosted_aggregator, re.IGNORECASE):
    raise SystemExit("hosted coverage isolation must not invoke AWS execution")

required_status_names = re.findall(r"^    name:\s+adl-coverage\s*$", workflow, re.MULTILINE)
if len(required_status_names) != 1 or "name: adl-coverage" not in required_status_job:
    raise SystemExit("CI must expose exactly one stable required adl-coverage status")
if '--hosted-result "coverage=${{ needs.adl_coverage_hosted.outputs.route_result }}"' not in required_status_job:
    raise SystemExit("stable adl-coverage status must verify the aggregator's semantic hosted/Spot route result")
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
    'COVERAGE_CACHE_TARGET_DIR="$COVERAGE_BUILD_ROOT/target"',
    'COVERAGE_RUN_TARGET_ROOT="$COVERAGE_CACHE_TARGET_DIR/llvm-cov-target/$COVERAGE_RUN_ID"',
    'mkdir -p "$COVERAGE_CACHE_TARGET_DIR" "$COVERAGE_RUN_TARGET_ROOT" "$COVERAGE_OUTPUT_ROOT"',
    'local profile_target="$COVERAGE_RUN_TARGET_ROOT/$coverage_profile_namespace"',
    'export CARGO_TARGET_DIR="$profile_target"',
    'export CARGO_LLVM_COV_TARGET_DIR="$profile_target"',
    'COVERAGE_REPORT_MODE="${ADL_AUTHORITATIVE_COVERAGE_REPORT_MODE:-run-and-report}"',
    'COVERAGE_SHARD_COUNT="${ADL_AUTHORITATIVE_COVERAGE_SHARD_COUNT:-1}"',
    'COVERAGE_SHARD_INDEX="${ADL_AUTHORITATIVE_COVERAGE_SHARD_INDEX:-1}"',
    'IMPORT_PROFRAW_DIR="${ADL_AUTHORITATIVE_COVERAGE_IMPORT_PROFRAW_DIR:-}"',
):
    if required_fragment not in runner_script_text:
        raise SystemExit(
            "authoritative coverage runner must relocate llvm-cov build outputs onto the runner scratch mount; "
            f"missing fragment: {required_fragment}"
        )

pr_fast_step = step_block("PR fast coverage summary (json)")
expected_pr_fast_step = 'bash adl/tools/run_pr_fast_coverage_lane.sh --filter-expression "${{ steps.coverage-impact.outputs.filter_expression }}"'
if expected_pr_fast_step not in pr_fast_step:
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
if "github.event_name == 'pull_request'" not in filter_if:
    raise SystemExit(
        "PR-fast filter determination must be limited to coverage-requiring pull requests; "
        f"found: {filter_if}"
    )
if "needs.adl_path_policy.outputs.coverage_required == 'true'" not in workspace_fast_job.split("runs-on:", 1)[0]:
    raise SystemExit(
        "PR-fast filter determination must be inside the coverage-required workspace fast producer job"
    )
if "steps.path-policy.outputs.full_coverage_required != 'true'" in filter_if:
    raise SystemExit(
        "PR-fast filter determination must also run for full-coverage PRs so the changed-source gate can use focused evidence; "
        f"found: {filter_if}"
    )
pr_fast_runner_text = pr_fast_runner.read_text()
for required_fragment in (
    'FILTER_EXPRESSION=""',
    'TEST_THREADS="${ADL_PR_FAST_COVERAGE_TEST_THREADS:-}"',
    '--filter-expression',
    'coverage_args=(',
    'llvm-cov nextest',
    '--workspace',
    '--status-level all',
    '--final-status-level slow',
    '--no-report',
    '-E "$adl_filter_expression"',
    'coverage_args+=(--test-threads "$TEST_THREADS")',
    'PR-fast coverage test threads: nextest-default',
    'CARGO_INCREMENTAL=0 cargo "${coverage_args[@]}"',
    'cargo llvm-cov report',
    '--json',
    '--summary-only',
    'COMBINED_SUMMARY_PATH="$ADL_DIR/target/coverage-impact-summary.json"',
    '--output-path "$COMBINED_SUMMARY_PATH"',
):
    if required_fragment not in pr_fast_runner_text:
        raise SystemExit(
            "PR-fast coverage runner must execute targeted nextest coverage and produce focused impact summary JSON; "
            f"missing fragment: {required_fragment}"
        )
if "--test-threads 1" in pr_fast_runner_text:
    raise SystemExit("PR-fast coverage runner must not force single-threaded nextest execution by default")
for required_fragment in (
    "cargo nextest run \\",
    "    --workspace \\",
    "prepare_coverage_environment",
    "prepare_coverage_report_environment",
    "cargo llvm-cov clean --workspace",
    "cargo llvm-cov show-env --sh",
    "run_workspace_coverage_partitions",
    "import_profraw_profiles",
    "cargo llvm-cov report \\",
    "--json \\",
    "--summary-only \\",
    '--output-path "$summary_path"',
    'coverage-summary.adl-runtime.json',
    'COVERAGE_OUTPUT_ROOT="$COVERAGE_BUILD_ROOT/coverage-output/$COVERAGE_RUN_ID"',
    'FINAL_SUMMARY_PATH="$COVERAGE_OUTPUT_ROOT/coverage-summary.json"',
    'if [ "$PROFILE" = "adl-runtime" ] || [ "$PROFILE" = "all" ]; then',
    'if [ "$PROFILE" = "workspace" ] || [ "$PROFILE" = "all" ]; then',
    'python3 "$MERGE_HELPER"',
    'cp "$FINAL_SUMMARY_PATH" "$LEGACY_FINAL_SUMMARY_PATH"',
):
    if required_fragment not in runner_script_text:
        raise SystemExit(
            "authoritative coverage runner must execute direct workspace coverage without narrowing source targets; "
            f"missing fragment: {required_fragment}"
        )
if "    --lib \\" in runner_script_text or "    --tests \\" in runner_script_text or "    --bins \\" in runner_script_text or "    --all-targets \\" in runner_script_text:
    raise SystemExit("authoritative coverage runner must not narrow workspace coverage targets")

authoritative_gate_step = step_block("Coverage-impact changed-source gate")
if '--summary adl/coverage-summary.json' not in authoritative_gate_step:
    raise SystemExit(
        "authoritative changed-source coverage gate must consume the merged isolated summary"
    )
if "coverage-artifacts/workspace/adl/target/coverage-impact-summary.json" in authoritative_gate_step:
    raise SystemExit(
        "full authoritative changed-source gate must not replace merged coverage with a producer-local focused summary"
    )

pr_preflight_if = step_if("PR coverage-impact preflight")
if "needs.adl_path_policy.outputs.full_coverage_required != 'true'" not in pr_preflight_if:
    raise SystemExit(
        "PR coverage-impact preflight must be limited to non-full PR coverage; "
        f"found: {pr_preflight_if}"
    )
pr_preflight_step = step_block("PR coverage-impact preflight")
if "args+=(--summary coverage-artifacts/workspace/adl/target/coverage-impact-summary.json)" not in pr_preflight_step:
    raise SystemExit(
        "PR coverage-impact preflight must validate the focused summary emitted by the PR-fast coverage runner; "
        "workflow is still reading a stale default coverage-summary.json path"
    )

gate_if = step_if("Enforce coverage policy gates (workspace + per-file)")
if "github.event_name != 'pull_request'" not in gate_if:
    raise SystemExit(
        "workspace coverage gate must be skipped for pull_request coverage runs; "
        f"found: {gate_if}"
    )
expected_gate_fragment = "needs.adl_path_policy.outputs.coverage_authority != 'pr_policy_surface_tooling_only'"
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
    "needs.adl_path_policy.outputs.coverage_lane",
    "needs.adl_path_policy.outputs.coverage_authority",
    "needs.adl_path_policy.outputs.validation_profile_status",
    "needs.adl_path_policy.outputs.validation_profile_run_lanes",
    "needs.adl_path_policy.outputs.validation_profile_escalation_required",
    "needs.adl_coverage_workspace_hosted.result",
    "needs.adl_coverage_runtime_hosted.result",
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
    "Verify generated lcov file",
    "Verify lcov path from repository root",
):
    step_condition = step_if(step_name)
    if "github.event_name != 'pull_request'" not in step_condition:
        raise SystemExit(
            f"{step_name} must be skipped for pull_request authoritative coverage runs so PRs avoid nonessential reporting tail; "
            f"found: {step_condition}"
        )

workspace_artifact_if = step_if("Upload workspace coverage evidence")
expected_workspace_artifact_if = "always()"
if workspace_artifact_if != expected_workspace_artifact_if:
    raise SystemExit(
        "non-full workspace summary/log evidence must upload from the dedicated PR-fast producer; "
        f"found: {workspace_artifact_if}"
    )
workspace_profile_artifact_if = step_if("Upload workspace coverage shard profiles")
expected_workspace_profile_artifact_if = "always() && steps.path-policy.outputs.full_coverage_required == 'true'"
if workspace_profile_artifact_if != expected_workspace_profile_artifact_if:
    raise SystemExit(
        "full workspace profraw shard evidence must upload even when a shard producer fails; "
        f"found: {workspace_profile_artifact_if}"
    )

print("PASS test_ci_runtime_contracts")
PY

bash "$ROOT_DIR/adl/tools/test_run_pr_fast_coverage_lane.sh"
