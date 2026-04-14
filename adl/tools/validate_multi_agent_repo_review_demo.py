#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path

REQUIRED_REVIEW_SECTIONS = [
    "## Metadata",
    "## Scope",
    "## Findings",
    "## System-Level Assessment",
    "## Recommended Action Plan",
    "## Follow-ups / Deferred Work",
    "## Final Assessment",
]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def ordered_sections(text: str, sections: list[str], label: str) -> None:
    last = -1
    for section in sections:
        idx = text.find(section)
        require(idx >= 0, f"{label}: missing section {section}")
        require(idx > last, f"{label}: section out of order for {section}")
        last = idx


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_multi_agent_repo_review_demo.py <artifact-root>", file=sys.stderr)
        return 2

    root = Path(sys.argv[1]).resolve()
    require(root.exists(), f"artifact root not found: {root}")

    manifest_path = root / "demo_manifest.json"
    packet_manifest_path = root / "review_packet" / "review_packet_manifest.json"
    selected_paths_path = root / "review_packet" / "selected_paths.txt"
    synthesis_path = root / "synthesis" / "final_synthesis_review.md"
    reviewer_paths = [
        root / "reviewers" / "code_review.md",
        root / "reviewers" / "security_review.md",
        root / "reviewers" / "test_review.md",
        root / "reviewers" / "docs_review.md",
    ]

    for path in [manifest_path, packet_manifest_path, selected_paths_path, synthesis_path, *reviewer_paths]:
        require(path.exists(), f"missing artifact: {path}")

    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    require(
        manifest.get("schema_version") == "adl.v089.multi_agent_repo_review_demo.v1",
        "demo manifest schema mismatch",
    )
    require(
        manifest.get("execution_shape", {}).get("specialist_reviewers") == "parallel",
        "demo manifest missing parallel specialist reviewers",
    )

    packet_manifest = json.loads(packet_manifest_path.read_text(encoding="utf-8"))
    require(
        packet_manifest.get("schema") == "adl.v089.multi_agent_repo_review.packet.v1",
        "packet manifest schema mismatch",
    )
    require(len(packet_manifest.get("selected_paths", [])) >= 6, "packet manifest selected_paths too small")
    require(len(packet_manifest.get("reviewers", [])) == 5, "packet manifest reviewer count mismatch")

    selected_paths = [line.strip() for line in selected_paths_path.read_text(encoding="utf-8").splitlines() if line.strip()]
    require(len(selected_paths) >= 6, "selected_paths.txt too small")

    for reviewer_path in reviewer_paths:
        ordered_sections(reviewer_path.read_text(encoding="utf-8"), REQUIRED_REVIEW_SECTIONS, reviewer_path.name)

    synthesis_text = synthesis_path.read_text(encoding="utf-8")
    ordered_sections(synthesis_text, REQUIRED_REVIEW_SECTIONS, synthesis_path.name)
    require("Blocking Findings:" in synthesis_text, "synthesis missing blocking findings classification")
    require("Lower-Priority Observations:" in synthesis_text, "synthesis missing lower-priority observations classification")

    root_text = "\n".join(path.read_text(encoding="utf-8") for path in [selected_paths_path, synthesis_path, *reviewer_paths])
    require("/Users/daniel/" not in root_text, "artifact leakage: absolute host path found")

    print("validate_multi_agent_repo_review_demo: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
