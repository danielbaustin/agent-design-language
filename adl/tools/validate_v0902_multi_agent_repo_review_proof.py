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
    "review_packet/repo_scope.md",
    "review_packet/evidence_index.json",
    "review_packet/specialist_assignments.json",
    "specialist_reviews/code.md",
    "specialist_reviews/security.md",
    "specialist_reviews/tests.md",
    "specialist_reviews/docs.md",
    "synthesis/final_findings_first_review.md",
    "synthesis/coverage_matrix.json",
    "quality_gate/review_quality_evaluation.md",
    "quality_gate/redaction_and_publication_gate.md",
    "README.md",
]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def scan_text(rel: str, text: str) -> None:
    path_match = ABSOLUTE_HOST_PATH_RE.search(text)
    if path_match:
        raise SystemExit(f"{rel}: absolute host path leaked: {path_match.group(0)}")
    secret_match = SECRET_MARKER_RE.search(text)
    if secret_match:
        raise SystemExit(f"{rel}: secret-like marker leaked: {secret_match.group(0)}")


def require_text(path: Path, needle: str) -> None:
    require(needle in path.read_text(encoding="utf-8"), f"{path}: missing text {needle!r}")


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_v0902_multi_agent_repo_review_proof.py <artifact-root>", file=sys.stderr)
        return 2

    root = Path(sys.argv[1])
    require(root.is_dir(), f"artifact root not found: {root}")

    for rel in REQUIRED_FILES:
        require((root / rel).is_file(), f"missing required artifact: {rel}")

    manifest = read_json(root / "run_manifest.json")
    require(
        manifest.get("schema_version") == "adl.v0902.multi_agent_repo_review_proof.v1",
        "run manifest schema mismatch",
    )
    require(manifest.get("classification") == "proving_fixture", "classification mismatch")
    require(manifest.get("publication_allowed") is False, "publication must be blocked")
    require(manifest.get("merge_approval_claimed") is False, "merge approval must be false")
    require(manifest.get("live_provider_execution") is False, "live provider execution must be false")

    expected_skills = {
        "repo-packet-builder",
        "repo-review-code",
        "repo-review-security",
        "repo-review-tests",
        "repo-review-docs",
        "repo-review-synthesis",
        "review-quality-evaluator",
        "redaction-and-evidence-auditor",
    }
    require(expected_skills.issubset(set(manifest.get("skills_represented", []))), "missing represented skill")

    assignments = read_json(root / "review_packet/specialist_assignments.json")
    roles = {entry["role"] for entry in assignments.get("assignments", [])}
    require({"code", "security", "tests", "docs", "synthesis"}.issubset(roles), "missing specialist assignment")

    coverage = read_json(root / "synthesis/coverage_matrix.json")
    require(coverage.get("required_roles_present") is True, "coverage matrix missing required roles")
    require(coverage["roles"]["security"]["findings"] == 0, "security should be explicit no-finding lane")
    require(coverage["roles"]["security"]["non_findings"] is True, "security non-finding not recorded")

    final_review = root / "synthesis/final_findings_first_review.md"
    for required in [
        "## Findings",
        "Finding MR-CODE-001: [P2]",
        "Finding MR-TEST-001: [P2]",
        "Finding MR-DOCS-001: [P3]",
        "## Explicit Non-Findings",
        "Security found no material issue",
        "## Specialist Coverage Matrix",
        "## Dedupe And Disagreement Notes",
        "## Residual Risk",
        "This review is not merge approval",
    ]:
        require_text(final_review, required)

    require_text(root / "quality_gate/review_quality_evaluation.md", "PASS for internal demo proof")
    require_text(root / "quality_gate/redaction_and_publication_gate.md", "Publication allowed: false")
    require_text(root / "README.md", "proving_fixture")

    for rel in REQUIRED_FILES:
        path = root / rel
        scan_text(rel, path.read_text(encoding="utf-8"))

    print("validate_v0902_multi_agent_repo_review_proof: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
