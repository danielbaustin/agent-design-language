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
    "README.md",
    "source_packet/source_manifest.json",
    "source_packet/idea_summary.md",
    "source_packet/lab_notes.md",
    "source_packet/experiment_results.json",
    "source_packet/target_venue.md",
    "source_packet/citations_seed.json",
    "source_packet/paper_constraints.md",
    "role_outputs/conductor_plan.json",
    "role_outputs/scholar_literature_review.md",
    "role_outputs/analyst_results_summary.md",
    "manuscript/draft.md",
    "review/editor_review_notes.md",
    "review/revision_requests.json",
    "review/reviewer_brief.md",
    "revision/revised_manuscript.md",
    "publication_gate/no_submission.md",
]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def require_text(path: Path, needle: str) -> None:
    require(needle in path.read_text(encoding="utf-8"), f"{path}: missing text {needle!r}")


def scan_text(rel: str, text: str) -> None:
    path_match = ABSOLUTE_HOST_PATH_RE.search(text)
    if path_match:
        raise SystemExit(f"{rel}: absolute host path leaked: {path_match.group(0)}")
    secret_match = SECRET_MARKER_RE.search(text)
    if secret_match:
        raise SystemExit(f"{rel}: secret-like marker leaked: {secret_match.group(0)}")


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_v0902_paper_sonata_expansion.py <artifact-root>", file=sys.stderr)
        return 2

    root = Path(sys.argv[1])
    require(root.is_dir(), f"artifact root not found: {root}")

    for rel in REQUIRED_FILES:
        require((root / rel).is_file(), f"missing required artifact: {rel}")

    manifest = read_json(root / "run_manifest.json")
    require(
        manifest.get("schema_version") == "adl.v0902.paper_sonata_expansion.v1",
        "run manifest schema mismatch",
    )
    require(manifest.get("classification") == "proving_fixture", "classification mismatch")
    require(manifest.get("baseline_preserved") is True, "baseline must be preserved")
    require(manifest.get("publication_allowed") is False, "publication must be blocked")
    require(manifest.get("submission_attempted") is False, "submission must not be attempted")
    require(manifest.get("live_web_citations") is False, "live-web citation claim must be false")
    require(manifest.get("publication_ready_claimed") is False, "publication-ready claim must be false")
    require(
        manifest.get("autonomous_scientific_discovery_claimed") is False,
        "autonomous discovery claim must be false",
    )

    expected_roles = {"conductor", "scholar", "analyst", "composer", "editor"}
    require(expected_roles.issubset(set(manifest.get("roles_represented", []))), "missing role representation")

    artifact_type_map = manifest.get("artifact_type_map", {})
    for key in [
        "source_material",
        "generated_role_outputs",
        "generated_draft_text",
        "review_feedback",
        "revision_requests",
        "revision_output",
        "publication_boundary",
    ]:
        require(key in artifact_type_map, f"artifact type map missing {key}")

    source_manifest = read_json(root / "source_packet/source_manifest.json")
    require(
        source_manifest.get("schema_version") == "adl.v0902.paper_sonata.source_packet.v1",
        "source manifest schema mismatch",
    )
    require("no publication-ready manuscript claim" in source_manifest.get("non_goals", []), "missing non-goal")

    revision_requests = read_json(root / "review/revision_requests.json")
    require(
        revision_requests.get("schema_version") == "adl.v0902.paper_sonata.revision_requests.v1",
        "revision request schema mismatch",
    )
    requests = revision_requests.get("requests", [])
    require(len(requests) == 3, "expected exactly three revision requests")
    require(all(item.get("status") == "addressed" for item in requests), "all revision requests must be addressed")

    require_text(root / "role_outputs/scholar_literature_review.md", "not a live-web literature review")
    require_text(root / "role_outputs/analyst_results_summary.md", "Unsupported Claim")
    require_text(root / "manuscript/draft.md", "Generated draft text")
    require_text(root / "review/editor_review_notes.md", "Review feedback")
    require_text(root / "revision/revised_manuscript.md", "Revision output")
    require_text(root / "revision/revised_manuscript.md", "Source material: `source_packet/`")
    require_text(root / "revision/revised_manuscript.md", "does not claim autonomous")
    require_text(root / "publication_gate/no_submission.md", "Publication allowed: false")
    require_text(root / "publication_gate/no_submission.md", "Submission attempted: false")
    require_text(root / "README.md", "publication readiness")

    for rel in REQUIRED_FILES:
        scan_text(rel, (root / rel).read_text(encoding="utf-8"))

    print("validate_v0902_paper_sonata_expansion: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
