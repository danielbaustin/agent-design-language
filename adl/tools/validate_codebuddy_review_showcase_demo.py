#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ABSOLUTE_HOST_PATH_RE = re.compile(
    r"(?:(?<![A-Za-z0-9._-])/(?:Users|home)/[^/\s`]+(?:/[^\s`]*)*)|"
    r"(?:(?<![A-Za-z0-9._-])/(?:private|var|tmp|etc|opt|srv|root)/[^\s`]+)|"
    r"(?:[A-Za-z]:\\\\[^\s`]+)"
)

SECRET_MARKER_RE = re.compile(
    r"(?:OPENAI_API_KEY|ANTHROPIC_API_KEY|Bearer\s+[A-Za-z0-9._-]+|sk-[A-Za-z0-9]{8,})"
)

REQUIRED_FILES = [
    "run_manifest.json",
    "repo_scope.md",
    "repo_inventory.json",
    "specialist_reviews/code.md",
    "specialist_reviews/security.md",
    "specialist_reviews/tests.md",
    "specialist_reviews/docs.md",
    "specialist_reviews/architecture.md",
    "specialist_reviews/dependencies.md",
    "diagrams/system_map.mmd",
    "diagrams/diagram_manifest.md",
    "diagrams/diagram_review.md",
    "redaction_report.md",
    "test_recommendations/test_gap_report.md",
    "issue_planning/issue_candidates.md",
    "adr_candidates/adr_candidates.md",
    "fitness_functions/fitness_function_plan.md",
    "final_report.md",
    "quality_evaluation.md",
    "demo_operator_result.json",
    "README.md",
]

FINAL_REPORT_SECTIONS = [
    "# CodeBuddy Review Report",
    "## Executive Summary",
    "## Review Scope",
    "## Top Findings",
    "## Architecture Summary",
    "## Security And Privacy Notes",
    "## Test Recommendations",
    "## Documentation And Onboarding Notes",
    "## Remediation Sequence",
    "## Residual Risks",
    "## Caveats",
]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def ordered_sections(text: str, sections: list[str], label: str) -> None:
    last = -1
    for section in sections:
        idx = text.find(section)
        require(idx >= 0, f"{label}: missing section {section}")
        require(idx > last, f"{label}: section out of order: {section}")
        last = idx


def scan_text(path: Path, text: str) -> None:
    path_match = ABSOLUTE_HOST_PATH_RE.search(text)
    if path_match is not None:
        raise SystemExit(f"{path}: absolute host path leaked: {path_match.group(0)}")
    secret_match = SECRET_MARKER_RE.search(text)
    if secret_match is not None:
        raise SystemExit(f"{path}: secret-like marker leaked: {secret_match.group(0)}")


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_codebuddy_review_showcase_demo.py <artifact-root>", file=sys.stderr)
        return 2

    root = Path(sys.argv[1])
    require(root.is_dir(), f"artifact root not found: {root}")

    for rel in REQUIRED_FILES:
        require((root / rel).is_file(), f"missing required artifact: {rel}")

    manifest = read_json(root / "run_manifest.json")
    require(
        manifest.get("schema_version") == "codebuddy.review_showcase.v1",
        "run manifest schema mismatch",
    )
    require(manifest.get("classification") == "non_proving", "demo must remain non_proving")
    require(manifest.get("publication_allowed") is False, "publication_allowed must be false")
    lanes = manifest.get("skill_lanes", [])
    require(len(lanes) >= 17, "skill lane count too small")
    by_skill = {lane.get("skill"): lane for lane in lanes}
    require(
        by_skill.get("review-quality-evaluator", {}).get("status") == "staged_pending_2070",
        "review-quality-evaluator must be staged pending #2070",
    )
    require(
        by_skill.get("product-report-writer", {}).get("status") == "represented",
        "product-report-writer lane missing",
    )
    require(
        by_skill.get("redaction-and-evidence-auditor", {}).get("status") == "represented",
        "redaction lane missing",
    )

    inventory = read_json(root / "repo_inventory.json")
    require(
        inventory.get("review_packet_policy", {}).get("redaction_required_before_report") is True,
        "inventory must require redaction before report",
    )

    demo_result = read_json(root / "demo_operator_result.json")
    require(
        demo_result.get("classification") == "non_proving",
        "demo operator classification mismatch",
    )
    require(
        demo_result.get("prerequisite_state", {}).get("review_quality_evaluator")
        == "staged_pending_2070",
        "demo operator staged-lane truth missing",
    )

    final_report = (root / "final_report.md").read_text(encoding="utf-8")
    ordered_sections(final_report, FINAL_REPORT_SECTIONS, "final_report.md")
    require("[P1]" in final_report and "[P2]" in final_report, "final report lacks severity markers")
    require("publication_allowed=false" in final_report, "final report lacks publication block")
    require("#2070" in final_report, "final report lacks staged issue reference")

    redaction = (root / "redaction_report.md").read_text(encoding="utf-8")
    require("Publication allowed: false" in redaction, "redaction report must block publication")

    quality = (root / "quality_evaluation.md").read_text(encoding="utf-8")
    require("Staged pending #2070" in quality, "quality evaluation staged truth missing")

    for rel in REQUIRED_FILES:
        path = root / rel
        if path.suffix.lower() in {".md", ".json", ".mmd", ".txt"}:
            scan_text(Path(rel), path.read_text(encoding="utf-8"))

    print("validate_codebuddy_review_showcase_demo: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
