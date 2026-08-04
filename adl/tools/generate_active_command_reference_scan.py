#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
from collections import Counter
from dataclasses import dataclass
from datetime import date
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
OUTPUT_PATH = (
    REPO_ROOT
    / "docs"
    / "milestones"
    / "v0.91.5"
    / "ACTIVE_COMMAND_REFERENCE_SCAN_3735.md"
)


ACTIVE_PATH_PREFIXES = (
    "AGENTS.md",
    "CONTRIBUTING.md",
    "adl/src/",
    "docs/tooling/",
    "docs/templates/",
    "adl/tools/skills/",
    "adl/tools/",
    "docs/planning/",
    ".adl/v0.91.5/tasks/",
    ".adl/v0.91.5/bodies/",
)

HISTORICAL_PATH_PREFIXES = (
    "docs/milestones/",
    "docs/planning/PR_CONTROL_PLANE_DECRUFT_COMPATIBILITY_CUT_PLAN.md",
    "docs/milestones/v0.91.5/review/",
    "docs/milestones/v0.91.5/release/",
    "docs/milestones/v0.91.4/",
    "docs/milestones/v0.91.3/",
    "docs/milestones/v0.91.2/",
    "docs/milestones/v0.91.1/",
    "docs/milestones/v0.91.0/",
    "docs/milestones/v0.90",
)

UNKNOWN_PATH_PREFIXES = (
    ".adl/cards/",
    "docs/milestones/v0.91.5/",
    "adl/tools/",
)

INCLUDE_ROOTS = (
    REPO_ROOT / "AGENTS.md",
    REPO_ROOT / "CONTRIBUTING.md",
    REPO_ROOT / "adl" / "src",
    REPO_ROOT / "docs" / "tooling",
    REPO_ROOT / "docs" / "templates",
    REPO_ROOT / "adl" / "tools" / "skills",
    REPO_ROOT / "docs" / "planning",
    REPO_ROOT / "docs" / "milestones" / "v0.91.5",
    REPO_ROOT / ".adl" / "v0.91.5" / "tasks",
    REPO_ROOT / ".adl" / "v0.91.5" / "bodies",
    REPO_ROOT / ".adl" / "cards",
    REPO_ROOT / "adl" / "tools",
)

SKIP_PATH_SUBSTRINGS = (
    "/target/",
    "/.git/",
    "/node_modules/",
)

EXCLUDED_REL_PATHS = {
    "adl/tools/generate_active_command_reference_scan.py",
    "adl/tools/test_generate_active_command_reference_scan.sh",
    # These wrappers are executable fail-closed tombstones. Their only legacy
    # strings are prohibition/migration text, not runnable workflow routes.
    "adl/tools/codex_pr.sh",
    "adl/tools/codexw.sh",
    # Negative-contract tests intentionally quote forbidden forms. Other
    # test_* files remain scanned and can still fail this gate.
    "adl/tools/test_batched_checks_no_codexpr_usage_banner.sh",
    "adl/tools/test_check_issue_metadata_parity.sh",
    "adl/tools/test_check_milestone_closed_issue_sor_truth.sh",
    "adl/tools/test_cli_owner_command_guidance.sh",
    "adl/tools/test_cli_wrapper_migration_contract.sh",
    "adl/tools/test_closeout_completed_issue_wave.sh",
    # Generated UI model; its tracked template source is validated separately.
    "docs/tooling/csdlc-prompt-editor/editor_model.js",
    "docs/milestones/v0.91.5/ACTIVE_COMMAND_REFERENCE_SCAN_3735.md",
}

HISTORICAL_EXACT_PATHS = {
    # Immutable migration/review evidence. Current contracts under
    # docs/tooling remain active even when they describe retired commands.
    "docs/tooling/ADL_OCTOCRAB_MIGRATION_REVIEW.md",
    "docs/tooling/BUILD_ACTION_LOGS.md",
    "docs/tooling/PROMPT_TEMPLATE_VALUES_RENDERER_PLAN_v0.91.5.md",
    "docs/tooling/PROMPT_CARD_VALUES_IMPORT_ROUND_TRIP_v0.91.5.md",
    "docs/tooling/active-card-lifecycle-migration-readiness-v0.91.2.md",
    "docs/tooling/examples/workflow-state/good_output_record.md",
}


@dataclass(frozen=True)
class CommandFamily:
    key: str
    label: str
    preferred_owner: str
    required_action: str
    pattern: str


COMMAND_FAMILIES = (
    CommandFamily(
        key="sunset_pr_sh",
        label="sunset `adl/tools/pr.sh` wrapper",
        preferred_owner="csdlc-install resolve + typed csdlc-* binaries",
        required_action="forbid if active or unknown; preserve only if historical",
        pattern=r"(?<![\w/-])(?:bash\s+)?(?:\./)?adl/tools/pr\.sh\s+(?:create|init|start|run|doctor|ready|preflight|finish|closeout|issue|shepherd|janitor|pr-inventory|project-doctor)\b|(?<![\w/-])bash\s+\"?\$ROOT_DIR/adl/tools/pr\.sh\"?\s+(?:create|init|start|run|doctor|ready|preflight|finish|closeout|issue|shepherd|janitor|pr-inventory|project-doctor)\b",
    ),
    CommandFamily(
        key="direct_adl_pr",
        label="direct `adl pr ...` issue-mode commands",
        preferred_owner="csdlc-install resolve + typed csdlc-* binaries",
        required_action="forbid if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl pr (?:create|init|doctor|ready|preflight|run|finish|closeout)\b",
    ),
    CommandFamily(
        key="runtime_through_pr",
        label="`adl pr run <adl.yaml>` runtime-through-PR",
        preferred_owner="adl-runtime run <adl.yaml> ...",
        required_action="migrate if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl pr run\s+[^`\s]+\.adl\.ya?ml\b|(?<![\w/-])adl pr run\s+[^`\s]+\.ya?ml\b",
    ),
    CommandFamily(
        key="legacy_prompt_template",
        label="sunset tooling prompt-template route",
        preferred_owner="csdlc-edit + csdlc-validate typed requests",
        required_action="forbid if active or unknown; preserve only if historical",
        pattern=r"(?<![\w/-])(?:adl|adl-csdlc) tooling prompt-template\b",
    ),
    CommandFamily(
        key="deleted_prompt_wrappers",
        label="deleted prompt/review shell wrapper",
        preferred_owner="stable direct owner binary or typed csdlc-edit/csdlc-validate",
        required_action="forbid if active or unknown; preserve only if historical",
        pattern=r"(?<![\w/-])(?:bash\s+[\"']?(?:\$ROOT_DIR/)?(?:\./)?adl/tools/(?:prompt_template|validate_structured_prompt|card_prompt|lint_prompt_spec|review_card_surface)\.sh|[\"']?\$[A-Z_]+/adl/tools/(?:prompt_template|validate_structured_prompt|card_prompt|lint_prompt_spec|review_card_surface)\.sh|(?:\./)?adl/tools/(?:prompt_template|validate_structured_prompt|card_prompt|lint_prompt_spec|review_card_surface)\.sh\s+--|[A-Z_]+=[\"'][^\n\"']*adl/tools/(?:prompt_template|validate_structured_prompt|card_prompt|lint_prompt_spec|review_card_surface)\.sh)",
    ),
    CommandFamily(
        key="sunset_workflow_conductor",
        label="sunset workflow-conductor route",
        preferred_owner="csdlc-install resolve + typed csdlc-* binary",
        required_action="forbid if active or unknown; preserve only if historical",
        pattern=r"(?:workflow-conductor|workflow_conductor|route_workflow\.py)\b",
    ),
    CommandFamily(
        key="retired_closeout_helpers",
        label="retired closeout/milestone helper",
        preferred_owner="csdlc-doctor + csdlc-finish/csdlc-clean typed requests",
        required_action="forbid if active or unknown; preserve only if historical",
        pattern=r"(?<!test_)(?:closeout_completed_issue_wave|check_milestone_closed_issue_sor_truth|check_issue_metadata_parity)\.sh\b",
    ),
    CommandFamily(
        key="legacy_codex_pr",
        label="retired `codex_pr.sh` / `codexw.sh` wrappers",
        preferred_owner="csdlc-install resolve + typed csdlc-* binary",
        required_action="migrate if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])(?:adl/tools/)?codex_pr\.sh\s+\S+|(?<![\w/-])(?:adl/tools/)?codexw\.sh\s+\S+",
    ),
    CommandFamily(
        key="csdlc_issue_run",
        label="`adl-csdlc issue run <issue>`",
        preferred_owner="csdlc-bind --root <worktree> --request <bind-request.json>",
        required_action="forbid if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl-csdlc issue run\b",
    ),
)


def repo_rel(path: Path, root: Path = REPO_ROOT) -> str:
    return path.relative_to(root).as_posix()


def should_skip(path: Path) -> bool:
    rel = repo_rel(path)
    if rel in EXCLUDED_REL_PATHS:
        return True
    return any(token in f"/{rel}" for token in SKIP_PATH_SUBSTRINGS)


def iter_paths() -> list[Path]:
    paths: list[Path] = []
    for root in INCLUDE_ROOTS:
        if not root.exists():
            continue
        if root.is_file():
            paths.append(root)
            continue
        for path in sorted(root.rglob("*")):
            if not path.is_file():
                continue
            if should_skip(path):
                continue
            paths.append(path)
    return paths


def classify_path(rel: str) -> str:
    if rel in HISTORICAL_EXACT_PATHS:
        return "historical"
    # These are legacy-full rendered template directories. The independently
    # compiled compact-native v2 identity is governed by native-card-shape.json,
    # not by these filesystem template paths.
    if rel.startswith(
        (
            "docs/templates/prompts/1.0.0/",
            "docs/templates/prompts/1.0.1/",
            "docs/templates/prompts/1.0.2/",
        )
    ):
        return "historical"
    if rel.startswith(HISTORICAL_PATH_PREFIXES):
        return "historical"
    if rel.startswith(ACTIVE_PATH_PREFIXES):
        return "active"
    if rel.startswith(UNKNOWN_PATH_PREFIXES):
        return "unknown"
    return "unknown"


def load_text(path: Path) -> str | None:
    try:
        return path.read_text()
    except UnicodeDecodeError:
        return None


def evidence_pointer(text: str, index: int) -> tuple[int, str, int]:
    line = text.count("\n", 0, index) + 1
    line_start = text.rfind("\n", 0, index) + 1
    line_end = text.find("\n", index)
    if line_end == -1:
        line_end = len(text)
    raw_excerpt = text[line_start:line_end]
    leading_space = len(raw_excerpt) - len(raw_excerpt.lstrip())
    excerpt = raw_excerpt.strip()
    match_offset = index - line_start - leading_space
    return line, excerpt, match_offset


def is_prohibition_reference(excerpt: str, match_offset: int = 0) -> bool:
    clause_breaks = [
        match.start()
        for match in re.finditer(
            r"[;—]|,(?=\s*(?:invoke|run|route|execute|call|use)\b)|[.!?](?=\s|$)",
            excerpt,
            re.IGNORECASE,
        )
    ]
    clause_start = max((position for position in clause_breaks if position < match_offset), default=-1) + 1
    clause_ends = [position for position in clause_breaks if position >= match_offset]
    clause_end = min(clause_ends, default=len(excerpt))
    lowered = excerpt[clause_start:clause_end].lower()
    return any(
        marker in lowered
        for marker in (
            "historical v1",
            "retired",
            "sunset",
            "removed",
            "must not use",
            "do not use",
            "do not invoke",
            "forbidden",
        )
    )


def is_absence_assertion(excerpt: str) -> bool:
    return bool(
        re.fullmatch(
            r"\[\[\s+!\s+-[ef]\s+[^;|&]+\s+\]\](?:\s+\|\|\s+\{)?",
            excerpt,
        )
    )


def run_regression_fixtures() -> None:
    fixtures = {
        "variable-bound wrapper": 'VALIDATOR="$ROOT/adl/tools/validate_structured_prompt.sh"',
        "bare owner value": "owner_skill: workflow-conductor | pr-ready | none",
        "nonadjacent routing": "Route the selected issue after readiness through `workflow-conductor`.",
        "sunset prompt route": "adl-csdlc tooling prompt-template render --kind sip",
        "quoted root execution": '"$ROOT/adl/tools/review_card_surface.sh" --input card.md',
        "retired closeout helper": "bash adl/tools/closeout_completed_issue_wave.sh --version v0.91.7",
    }
    for name, fixture in fixtures.items():
        assert any(re.search(family.pattern, fixture) for family in COMMAND_FAMILIES), name
    prohibited = "workflow-conductor and pr.sh are historical v1 routes and must not be used"
    assert is_prohibition_reference(prohibited, prohibited.index("workflow-conductor"))
    for mixed in (
        "Do not use pr.sh; invoke workflow-conductor for the next issue.",
        "Do not use pr.sh. Invoke workflow-conductor for the next issue.",
        "Do not use pr.sh, invoke workflow-conductor for the next issue.",
        "Do not use pr.sh — invoke workflow-conductor for the next issue.",
    ):
        assert not is_prohibition_reference(mixed, mixed.index("workflow-conductor"))
    assert is_absence_assertion(
        '[[ ! -e "$ROOT/adl/tools/review_card_surface.sh" ]]'
    )
    assert not is_absence_assertion(
        '[[ ! -e "$ROOT/adl/tools/review_card_surface.sh" ]] || workflow-conductor run'
    )


def build_rows(
    paths: list[Path] | None = None, root: Path = REPO_ROOT
) -> tuple[list[tuple[str, str, int, str, str, str, str]], Counter]:
    rows: list[tuple[str, str, int, str, str, str, str]] = []
    counts: Counter = Counter()
    seen: set[tuple[str, str, int]] = set()
    for path in paths if paths is not None else iter_paths():
        text = load_text(path)
        if text is None:
            continue
        rel = repo_rel(path, root)
        path_class = classify_path(rel)
        for family in COMMAND_FAMILIES:
            for match in re.finditer(family.pattern, text):
                line, excerpt, match_offset = evidence_pointer(text, match.start())
                if path_class == "active" and (
                    is_prohibition_reference(excerpt, match_offset)
                    or is_absence_assertion(excerpt)
                ):
                    continue
                dedupe_key = (family.key, rel, line)
                if dedupe_key in seen:
                    continue
                seen.add(dedupe_key)
                counts[(family.key, path_class)] += 1
                rows.append(
                    (
                        family.label,
                        rel,
                        line,
                        path_class,
                        family.required_action,
                        family.preferred_owner,
                        excerpt,
                    )
                )
    rows.sort(key=lambda row: (row[3], row[0], row[1], row[2]))
    return rows, counts


def summarize_family_counts(counts: Counter) -> list[tuple[str, int, int, int]]:
    summary: list[tuple[str, int, int, int]] = []
    for family in COMMAND_FAMILIES:
        summary.append(
            (
                family.label,
                counts[(family.key, "active")],
                counts[(family.key, "historical")],
                counts[(family.key, "unknown")],
            )
        )
    return summary


def summarize_unique_class_counts(
    rows: list[tuple[str, str, int, str, str, str, str]]
) -> Counter:
    counts: Counter = Counter()
    seen: set[tuple[str, int, str]] = set()
    for _, rel, line, class_name, _, _, _ in rows:
        key = (rel, line, class_name)
        if key in seen:
            continue
        seen.add(key)
        counts[class_name] += 1
    return counts


def render(rows: list[tuple[str, str, int, str, str, str, str]], counts: Counter) -> str:
    by_class = summarize_unique_class_counts(rows)
    summary_rows = summarize_family_counts(counts)
    lines = [
        "# Active Command Reference Scan 3735",
        "",
        "Issue: #3735",
        "Parent sprint: #3732",
        f"Captured: {date.today().isoformat()}",
        "Status: generated_scan_report",
        "",
        "## Summary",
        "",
        "This report implements the `#3628` active-bundle scan gate for the",
        "toolkit-simplification sprint. It scans the current repo surfaces for",
        "legacy command-family references and classifies matching command-family",
        "hits as `active`, `historical`, or `unknown` using the path classes",
        "defined for shim-cut review.",
        "",
        "Deletion or fail-closed shim cuts remain blocked while any relevant",
        "`active` or `unknown` references remain unrouted.",
        "",
        "## Generation Command",
        "",
        "```bash",
        "python3 adl/tools/generate_active_command_reference_scan.py",
        "python3 adl/tools/generate_active_command_reference_scan.py --check",
        "```",
        "",
        "## Scan Inputs",
        "",
        "- `AGENTS.md`",
        "- `docs/templates/`",
        "- `adl/tools/skills/`",
        "- `docs/planning/`",
        "- `docs/milestones/v0.91.5/`",
        "- `.adl/v0.91.5/tasks/`",
        "- `.adl/v0.91.5/bodies/`",
        "- `.adl/cards/`",
        "- `adl/tools/`",
        "",
        "## Unique Evidence Totals",
        "",
        f"- `active`: {by_class['active']}",
        f"- `historical`: {by_class['historical']}",
        f"- `unknown`: {by_class['unknown']}",
        "",
        "## Command Family Hit Totals",
        "",
        "| Command family | Active | Historical | Unknown |",
        "| --- | --- | --- | --- |",
    ]
    for label, active, historical, unknown in summary_rows:
        lines.append(f"| {label} | {active} | {historical} | {unknown} |")

    lines.extend(
        [
            "",
            "## Findings",
            "",
            "| Command reference | Path | Line | Class | Required action before deletion | Preferred owner | Evidence excerpt |",
            "| --- | --- | --- | --- | --- | --- | --- |",
        ]
    )
    for label, rel, line, class_name, action, owner, excerpt in rows:
        safe_excerpt = excerpt.replace("|", "\\|").replace("`", "\\`")
        lines.append(
            f"| {label} | `{rel}` | {line} | `{class_name}` | {action} | `{owner}` | `{safe_excerpt}` |"
        )

    lines.extend(
        [
            "",
            "## Deletion Recommendation",
            "",
            "- Do not delete or fail-close any scanned command family while `active` findings remain.",
            "- Route every `unknown` finding through a bounded follow-on before compatibility removal.",
            "- Treat `historical` findings as readability evidence, not as executable dependencies.",
            "",
            "## Known Classification Rules",
            "",
            "- `AGENTS.md`, prompt templates, skills, planning docs, open issue bodies, and task bundles classify as `active`.",
            "- `adl/tools/` scripts and active validation/demo helpers classify as `active` unless a later issue carves out an explicit historical-fixture rule.",
            "- `docs/milestones/v0.91.5/review/` and older closed-milestone evidence classify as `historical`.",
            "- `docs/milestones/v0.91.5/` outside review packets and `.adl/cards/` classify as `unknown` until a later issue narrows them further.",
            "- Unique evidence totals count one row per path/line/class; command-family hit totals may overlap when one source line names multiple legacy families.",
            "",
            "## Non-Claims",
            "",
            "- This issue does not delete any compatibility shim.",
            "- This issue does not rewrite historical records solely to remove old command strings.",
            "- This issue does not claim every `unknown` reference is unsafe; it only routes them for future review.",
        ]
    )
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--fixture-root", type=Path)
    args = parser.parse_args()

    if args.self_test:
        run_regression_fixtures()
        print("active command reference scan regression fixtures: ok")
        return 0

    if args.fixture_root:
        fixture_root = args.fixture_root.resolve()
        fixture_paths = sorted(path for path in fixture_root.rglob("*") if path.is_file())
        rows, counts = build_rows(fixture_paths, fixture_root)
    else:
        rows, counts = build_rows()
    rendered = render(rows, counts)

    if args.check:
        blocking = [
            row
            for row in rows
            if row[3] in {"active", "unknown"}
        ]
        if blocking:
            details = "\n".join(
                f"{row[1]}:{row[2]}: {row[0]}" for row in blocking[:200]
            )
            raise SystemExit(
                f"active command reference scan found {len(blocking)} blocking legacy references:\n{details}"
            )
        return 0

    OUTPUT_PATH.write_text(rendered)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
