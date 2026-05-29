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
SECRET_MARKER_RE = re.compile(r"(?:OPENAI_API_KEY|ANTHROPIC_API_KEY|sk-[A-Za-z0-9]{8,}|Bearer\s+[A-Za-z0-9._-]+)")

REQUIRED_FILES = [
    "run_manifest.json",
    "review_packet/repo_scope.md",
    "review_packet/heuristic_contract.json",
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

SPECIALIST_SECTIONS = [
    "## Metadata",
    "## Scope",
    "## Findings",
    "## Explicit Non-Findings",
    "## Role-Specific Caveats",
    "## Residual Risk",
    "## Recommended Action",
    "## Final Assessment",
]

SYNTHESIS_SECTIONS = [
    "## Findings",
    "## Explicit Non-Findings",
    "## Specialist Coverage Matrix",
    "## Dedupe And Disagreement Notes",
    "## Residual Risk",
    "## Review Boundary",
]

FINDING_SEVERITY_RE = re.compile(r"(?m)^\d+\.\s+\[(P[1-4])\]\s+")
EVIDENCE_MARKER_RE = re.compile(r"(?m)^\s*-\s+Evidence:")
RECOMMENDED_ACTION_MARKER_RE = re.compile(r"(?m)^\s*-\s+Recommended Action:")
RESIDUAL_RISK_MARKER_RE = re.compile(r"(?m)^\s*-\s+Residual Risk:")


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
        require(idx > last, f"{label}: section out of order for {section}")
        last = idx


def scan_text(rel: str, text: str) -> None:
    m = ABSOLUTE_HOST_PATH_RE.search(text)
    require(m is None, f"{rel}: absolute host path leaked: {m.group(0) if m else ''}")
    s = SECRET_MARKER_RE.search(text)
    require(s is None, f"{rel}: secret-like marker leaked: {s.group(0) if s else ''}")


def require_text(path: Path, needle: str) -> None:
    require(needle in path.read_text(encoding="utf-8"), f"{path}: missing text {needle!r}")


def count_specialist_findings(text: str) -> int:
    findings_section = text.split("## Findings", 1)[1].split("## Explicit Non-Findings", 1)[0]
    if "No material findings." in findings_section:
        return 0
    count = len(FINDING_SEVERITY_RE.findall(findings_section))
    require(count > 0, "specialist findings section must contain severity-tagged findings or explicit no-finding text")
    require(EVIDENCE_MARKER_RE.search(findings_section) is not None, "specialist findings section missing Evidence marker")
    require(RECOMMENDED_ACTION_MARKER_RE.search(findings_section) is not None, "specialist findings section missing Recommended Action marker")
    return count


def count_synthesis_findings(text: str) -> int:
    findings_section = text.split("## Findings", 1)[1].split("## Explicit Non-Findings", 1)[0]
    count = len(FINDING_SEVERITY_RE.findall(findings_section))
    require(count > 0, "synthesis findings section must contain severity-tagged findings")
    require(EVIDENCE_MARKER_RE.search(findings_section) is not None, "synthesis findings section missing Evidence marker")
    require(RECOMMENDED_ACTION_MARKER_RE.search(findings_section) is not None, "synthesis findings section missing Recommended Action marker")
    require(RESIDUAL_RISK_MARKER_RE.search(findings_section) is not None, "synthesis findings section missing Residual Risk marker")
    return count


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_v0914_multi_agent_repo_review_serious_proof.py <artifact-root>", file=sys.stderr)
        return 2

    root = Path(sys.argv[1])
    require(root.is_dir(), f"artifact root not found: {root}")

    for rel in REQUIRED_FILES:
        require((root / rel).is_file(), f"missing required artifact: {rel}")

    manifest = read_json(root / "run_manifest.json")
    require(manifest.get("schema_version") == "adl.v0914.multi_agent_repo_review_serious_proof.v1", "run manifest schema mismatch")
    require(manifest.get("classification") == "proving_fixture", "classification mismatch")
    require(manifest.get("publication_allowed") is False, "publication must be blocked")
    require(manifest.get("merge_approval_claimed") is False, "merge approval must be false")
    require(manifest.get("live_provider_execution") is False, "live provider execution must be false")
    require(manifest.get("heuristics_visible") is True, "heuristics visibility must be true")
    require(manifest.get("role_caveats_required") is True, "role caveats requirement must be true")

    heuristics = read_json(root / "review_packet/heuristic_contract.json")
    roles = {entry["role"] for entry in heuristics.get("domains", [])}
    require({"code", "security", "tests", "docs", "synthesis"}.issubset(roles), "heuristic contract missing roles")

    assignments = read_json(root / "review_packet/specialist_assignments.json")
    assigned_roles = {entry["role"] for entry in assignments.get("assignments", [])}
    require({"code", "security", "tests", "docs", "synthesis"}.issubset(assigned_roles), "missing specialist assignment")

    coverage = read_json(root / "synthesis/coverage_matrix.json")
    require(coverage.get("required_roles_present") is True, "coverage matrix missing required roles")
    require(coverage["roles"]["security"]["findings"] == 0, "security should be explicit no-finding lane")
    require(coverage["roles"]["security"]["explicit_non_findings"] is True, "security explicit non-findings missing")

    specialist_counts: dict[str, int] = {}
    for rel in [
        "specialist_reviews/code.md",
        "specialist_reviews/security.md",
        "specialist_reviews/tests.md",
        "specialist_reviews/docs.md",
    ]:
        text = (root / rel).read_text(encoding="utf-8")
        ordered_sections(text, SPECIALIST_SECTIONS, rel)
        specialist_counts[Path(rel).stem] = count_specialist_findings(text)

    synthesis = (root / "synthesis/final_findings_first_review.md").read_text(encoding="utf-8")
    ordered_sections(synthesis, SYNTHESIS_SECTIONS, "synthesis/final_findings_first_review.md")
    require("Security found no material findings" in synthesis, "synthesis must preserve explicit security non-finding")
    require("not merge approval" in synthesis, "synthesis must preserve review boundary")
    synthesis_count = count_synthesis_findings(synthesis)

    require(coverage["roles"]["code"]["findings"] == specialist_counts["code"], "coverage matrix code finding count mismatch")
    require(coverage["roles"]["security"]["findings"] == specialist_counts["security"], "coverage matrix security finding count mismatch")
    require(coverage["roles"]["tests"]["findings"] == specialist_counts["tests"], "coverage matrix tests finding count mismatch")
    require(coverage["roles"]["docs"]["findings"] == specialist_counts["docs"], "coverage matrix docs finding count mismatch")
    require(coverage["roles"]["synthesis"]["findings"] == synthesis_count, "coverage matrix synthesis finding count mismatch")

    require_text(root / "quality_gate/review_quality_evaluation.md", "Heuristic domains visible: PASS")
    require_text(root / "quality_gate/redaction_and_publication_gate.md", "Publication allowed: false")
    require_text(root / "README.md", "heuristic_contract.json")

    for rel in REQUIRED_FILES:
        scan_text(rel, (root / rel).read_text(encoding="utf-8"))

    print("validate_v0914_multi_agent_repo_review_serious_proof: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
