#!/usr/bin/env python3
from __future__ import annotations

import re
from collections import Counter
from datetime import date
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOLS_DIR = REPO_ROOT / "adl" / "tools"
SKILLS_DIR = TOOLS_DIR / "skills"
CARGO_TOML = REPO_ROOT / "adl" / "Cargo.toml"
OUTPUT_PATH = REPO_ROOT / "docs" / "milestones" / "v0.91.5" / "TOOL_SURFACE_REGISTRY_3734.md"


PRIMARY_BINARIES = {
    "adl-csdlc": "Primary C-SDLC owner binary for workflow control-plane and prompt-card tooling.",
    "adl-runtime": "Primary runtime owner binary for execution, providers, demos, agents, and instrumentation.",
    "adl-review": "Primary review owner binary for review tooling and packet verification.",
    "adl-validate-structured-prompt": "Direct validator binary for structured prompt validation.",
    "adl-lint-prompt-spec": "Direct validator binary for Prompt Spec lint.",
    "adl-prompt-template": "Direct editor/renderer binary for prompt-template operations.",
    "adl-provider-adapter": "Primary provider adapter utility for provider invocation surfaces.",
    "adl-remote": "Primary remote workflow binary for remote execution surfaces.",
}

SUPPORTED_SHIMS = {
    "pr.sh": "Canonical agent-facing issue workflow wrapper; implementation owner may move but wrapper remains public workflow spine.",
    "codex_pr.sh": "Legacy compatibility wrapper retained as a supported shim.",
    "validate_structured_prompt.sh": "Compatibility shim over the direct structured-prompt validator binary.",
    "lint_prompt_spec.sh": "Compatibility shim over the direct prompt-spec lint binary.",
}

PRIMARY_TOOL_SCRIPTS = {
    "run_owner_validation_lane.sh": "Primary focused owner-lane runner for bounded validation.",
    "run_pr_fast_test_lane.sh": "Primary focused PR-fast local test runner.",
    "check_coverage_impact.sh": "Primary local coverage-impact gate for risky changed files.",
    "check_issue_metadata_parity.sh": "Primary metadata parity guard for issue records.",
}

HISTORICAL_EVIDENCE = [
    (
        "docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md",
        "Historical command-inventory evidence and prior ownership table.",
    ),
    (
        "docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md",
        "Historical migration contract for wrapper/public workflow spine truth.",
    ),
    (
        "docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md",
        "Historical compatibility-sunset policy and active-bundle scan gate.",
    ),
    (
        "docs/milestones/v0.91.5/review/tooling_adoption/CSDLC_SMALL_BINARIES_PROOF_3832.md",
        "Historical proof packet for the validator/editor small-binary slice.",
    ),
]

CORE_WORKFLOW_COMMANDS = [
    (
        "adl/tools/pr.sh init <issue>",
        "core workflow command",
        "primary",
        "Canonical issue bootstrap command in the current governed workflow spine.",
    ),
    (
        "adl/tools/pr.sh doctor <issue>",
        "core workflow command",
        "primary",
        "Canonical readiness/doctor command for tracked issue work.",
    ),
    (
        "adl/tools/pr.sh run <issue>",
        "core workflow command",
        "primary",
        "Canonical issue binding command for execution-time branch/worktree setup.",
    ),
    (
        "adl/tools/pr.sh ready <issue>",
        "core workflow command",
        "primary",
        "Canonical readiness classification command for tracked issue work.",
    ),
    (
        "adl/tools/pr.sh finish <issue>",
        "core workflow command",
        "primary",
        "Canonical finish/publication command for issue branches.",
    ),
    (
        "adl/tools/pr.sh closeout <issue>",
        "core workflow command",
        "primary",
        "Canonical closeout command after merge or no-PR resolution.",
    ),
    (
        "pr-janitor skill / janitor workflow",
        "core workflow command",
        "primary",
        "Canonical in-flight PR blocker routing surface once a PR has been published.",
    ),
    (
        "adl-csdlc tooling prompt-template ...",
        "core workflow command",
        "primary",
        "Primary direct prompt-template workflow surface after the small-binary split.",
    ),
    (
        "adl-validate-structured-prompt --type <kind> --phase <phase> --input <card>",
        "core workflow command",
        "primary",
        "Primary direct structured-prompt validator surface after the small-binary split.",
    ),
    (
        "adl-lint-prompt-spec --input <file>",
        "core workflow command",
        "primary",
        "Primary direct prompt-spec lint surface after the small-binary split.",
    ),
]


def repo_rel(path: Path) -> str:
    return path.relative_to(REPO_ROOT).as_posix()


def parse_bin_names() -> list[str]:
    text = CARGO_TOML.read_text()
    return re.findall(r'^name = "(adl-[^"]+)"$', text, re.MULTILINE)


def classify_tool_script(path: Path) -> tuple[str, str]:
    name = path.name
    if name in SUPPORTED_SHIMS:
        return "supported shim", SUPPORTED_SHIMS[name]
    if name in PRIMARY_TOOL_SCRIPTS:
        return "primary", PRIMARY_TOOL_SCRIPTS[name]
    if name.startswith("test_"):
        return "internal helper", "Focused regression or contract test helper."
    if name.startswith("demo_"):
        return "internal helper", "Demo/proof helper rather than a primary workflow entrypoint."
    if name.startswith("mock_"):
        return "internal helper", "Mock helper used for focused testing or proof."
    if name.startswith("check_") or name.startswith("validate_") or name.startswith("verify_") or name.startswith("run_"):
        return "internal helper", "Bounded validation/generation helper used by primary workflow entrypoints."
    if name.endswith(".py"):
        return "internal helper", "Python helper or generator used by a bounded tooling surface."
    if name.endswith(".sh"):
        return "internal helper", "Shell helper retained for bounded workflow support."
    return "internal helper", "Visible tool surface retained for bounded support work."


def classify_skill_dir(path: Path) -> tuple[str, str]:
    name = path.name
    if name in {
        "workflow-conductor",
        "pr-init",
        "pr-ready",
        "pr-run",
        "pr-finish",
        "pr-closeout",
        "pr-janitor",
        "sprint-conductor",
    }:
        return "primary", "Primary skill in the governed issue/sprint lifecycle."
    if name.endswith("-editor") or name in {"sip-editor", "stp-editor", "spp-editor", "srp-editor", "sor-editor"}:
        return "supported shim", "Lifecycle editor skill used to normalize durable card truth."
    return "internal helper", "Specialized skill available for bounded tasks but not part of the primary issue workflow spine."


def render_table(rows: list[tuple[str, str, str, str]]) -> list[str]:
    lines = ["| Surface | Kind | Classification | Notes |", "| --- | --- | --- | --- |"]
    for surface, kind, classification, notes in rows:
        lines.append(f"| `{surface}` | {kind} | {classification} | {notes} |")
    return lines


def build_document() -> str:
    bin_rows: list[tuple[str, str, str, str]] = []
    counts = Counter()

    for name in parse_bin_names():
        notes = PRIMARY_BINARIES.get(name, "Owner binary present in Cargo manifest.")
        classification = "primary"
        counts[classification] += 1
        bin_rows.append((name, "owner binary", classification, notes))

    tool_rows: list[tuple[str, str, str, str]] = []
    for path in sorted(TOOLS_DIR.iterdir()):
        if path.name == "skills":
            continue
        if path.is_dir():
            classification = "internal helper"
            notes = "Directory containing bounded helper assets, scripts, or proof inputs."
        else:
            classification, notes = classify_tool_script(path)
        counts[classification] += 1
        tool_rows.append((repo_rel(path), "tool surface", classification, notes))

    skill_rows: list[tuple[str, str, str, str]] = []
    for path in sorted(
        p for p in SKILLS_DIR.iterdir() if p.is_dir() and (p / "SKILL.md").exists()
    ):
        classification, notes = classify_skill_dir(path)
        counts[classification] += 1
        skill_rows.append((repo_rel(path), "skill", classification, notes))

    historical_rows: list[tuple[str, str, str, str]] = []
    for rel, notes in HISTORICAL_EVIDENCE:
        counts["historical evidence"] += 1
        historical_rows.append((rel, "historical evidence", "historical evidence", notes))

    command_rows: list[tuple[str, str, str, str]] = []
    for row in CORE_WORKFLOW_COMMANDS:
        counts[row[2]] += 1
        command_rows.append(row)

    lines = [
        "# v0.91.5 Tool Surface Registry",
        "",
        "Issue: #3734",
        "Parent sprint: #3732",
        f"Captured: {date.today().isoformat()}",
        "Status: generated_live_registry",
        "",
        "## Summary",
        "",
        "This registry records the current visible ADL toolkit surface for the",
        "toolkit-simplification mini-sprint. It is a live inventory intended to",
        "drive later simplification cuts without confusing primary workflow",
        "entrypoints, supported compatibility shims, internal helpers, and",
        "historical evidence artifacts.",
        "",
        "No surface is marked `remove candidate` in this registry. Removal or",
        "fail-closed cuts still require the later active-reference scan gate.",
        "",
        "## Generation Command",
        "",
        "```bash",
        "python3 adl/tools/generate_tool_surface_registry.py",
        "```",
        "",
        "## Freshness Check",
        "",
        "```bash",
        "python3 adl/tools/generate_tool_surface_registry.py --check",
        "```",
        "",
        "## Classification Counts",
        "",
        f"- `primary`: {counts['primary']}",
        f"- `supported shim`: {counts['supported shim']}",
        f"- `internal helper`: {counts['internal helper']}",
        f"- `historical evidence`: {counts['historical evidence']}",
        f"- `archive candidate`: {counts['archive candidate']}",
        f"- `remove candidate`: {counts['remove candidate']}",
        "",
        "## Owner Binaries",
        "",
        *render_table(bin_rows),
        "",
        "## Core Workflow Commands",
        "",
        *render_table(command_rows),
        "",
        "## Tool Surfaces Under `adl/tools`",
        "",
        *render_table(tool_rows),
        "",
        "## ADL Skills",
        "",
        *render_table(skill_rows),
        "",
        "## Historical Evidence Artifacts",
        "",
        *render_table(historical_rows),
        "",
        "## Non-Claims",
        "",
        "- This registry does not approve any deletion.",
        "- This registry does not prove active-reference absence for any shim.",
        "- This registry does not replace the later simplification review and scan issues.",
    ]
    return "\n".join(lines) + "\n"


def main() -> int:
    import sys

    check_mode = len(sys.argv) > 1 and sys.argv[1] == "--check"
    rendered = build_document()

    if check_mode:
        current = OUTPUT_PATH.read_text()
        if current != rendered:
            print(f"FAIL: {repo_rel(OUTPUT_PATH)} is stale; rerun generator")
            return 1
        print(f"PASS: {repo_rel(OUTPUT_PATH)} is current")
        return 0

    OUTPUT_PATH.write_text(rendered)
    print(f"WROTE {repo_rel(OUTPUT_PATH)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
