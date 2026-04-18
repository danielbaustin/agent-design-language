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
    r"(?:OPENAI_API_KEY|ANTHROPIC_API_KEY|GOOGLE_API_KEY|Bearer\s+[A-Za-z0-9._-]+|sk-[A-Za-z0-9]{8,})"
)

REQUIRED_DOCS = [
    "docs/architecture/README.md",
    "docs/architecture/ADL_ARCHITECTURE.md",
    "docs/architecture/ARCHITECTURE_REVIEW_AUTOMATION.md",
    "docs/architecture/ARCHITECTURE_DOCUMENT_GENERATION_PLAN.md",
    "docs/architecture/adr/README.md",
    "docs/architecture/adr/CANDIDATE_ADRS.md",
    "docs/architecture/diagrams/README.md",
    "docs/architecture/diagrams/DIAGRAM_PACKET.md",
]

REQUIRED_DIAGRAMS = [
    "system_context.mmd",
    "runtime_lifecycle.mmd",
    "control_plane_lifecycle.mmd",
    "task_bundle_state.mmd",
    "skill_orchestration.mmd",
    "artifact_data_flow.mmd",
    "trust_boundaries.mmd",
]

ARCHITECTURE_SECTIONS = [
    "# ADL Architecture",
    "## Scope And Evidence",
    "## System Context",
    "## Runtime Model",
    "## Authoring And Control Plane",
    "## Task Bundle Lifecycle",
    "## Provider And Tool Boundaries",
    "## Trace And Artifact Truth",
    "## Security And Trust Boundaries",
    "## Long-Lived Agent Layer",
    "## Operational Skills",
    "## Review And Release Surfaces",
    "## Architecture Invariants",
    "## Known Gaps",
    "## Diagram Index",
    "## ADR Candidates",
]

AUTOMATION_SECTIONS = [
    "# Architecture Review Automation",
    "## Review Pipeline",
    "## Machine-Checkable Invariants",
    "## Human-Judgment Gates",
    "## Specialist Roles",
    "## Missing Or Backlog Skills",
    "## Automation Boundaries",
    "## Suggested CI Gates",
]

PLAN_SECTIONS = [
    "# Architecture Document Generation Plan",
    "## Goal",
    "## Inputs",
    "## Skill Order",
    "## Output Template",
    "## Diagram Template",
    "## Review Template",
    "## Machine Validation",
    "## Deferred Dependencies",
    "## Non-Goals",
]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


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


def check_docs(repo_root: Path) -> None:
    for rel in REQUIRED_DOCS:
        path = repo_root / rel
        require(path.is_file(), f"missing required architecture doc: {rel}")
        scan_text(path, path.read_text(encoding="utf-8"))

    architecture = (repo_root / "docs/architecture/ADL_ARCHITECTURE.md").read_text(
        encoding="utf-8"
    )
    ordered_sections(architecture, ARCHITECTURE_SECTIONS, "ADL_ARCHITECTURE.md")
    for marker in [
        "adl/src/adl/types.rs",
        "adl/src/execution_plan.rs",
        "adl/src/trace.rs",
        "adl/src/control_plane.rs",
        "adl/src/long_lived_agent.rs",
        "docs/default_workflow.md",
    ]:
        require(marker in architecture, f"ADL_ARCHITECTURE.md lacks evidence marker {marker}")

    automation = (
        repo_root / "docs/architecture/ARCHITECTURE_REVIEW_AUTOMATION.md"
    ).read_text(encoding="utf-8")
    ordered_sections(automation, AUTOMATION_SECTIONS, "ARCHITECTURE_REVIEW_AUTOMATION.md")
    require("Machine-Checkable" in automation, "automation doc lacks machine checks")
    require("Human-Judgment" in automation, "automation doc lacks human judgment gate")

    plan = (
        repo_root / "docs/architecture/ARCHITECTURE_DOCUMENT_GENERATION_PLAN.md"
    ).read_text(encoding="utf-8")
    ordered_sections(plan, PLAN_SECTIONS, "ARCHITECTURE_DOCUMENT_GENERATION_PLAN.md")
    require("#2042" in plan and "#2044" in plan, "generation plan lacks backlog dependencies")

    packet = (repo_root / "docs/architecture/diagrams/DIAGRAM_PACKET.md").read_text(
        encoding="utf-8"
    )
    for diagram in REQUIRED_DIAGRAMS:
        rel = f"docs/architecture/diagrams/{diagram}"
        path = repo_root / rel
        require(path.is_file(), f"missing required diagram source: {rel}")
        text = path.read_text(encoding="utf-8")
        scan_text(path, text)
        require(
            "%% Evidence:" in text and "%% Assumptions:" in text,
            f"{rel} lacks evidence or assumptions comments",
        )
        require(
            "flowchart" in text or "stateDiagram" in text,
            f"{rel} is not recognized as a Mermaid source",
        )
        require(diagram in packet, f"DIAGRAM_PACKET.md does not list {diagram}")


def check_artifacts(artifact_root: Path) -> None:
    require(artifact_root.is_dir(), f"artifact root not found: {artifact_root}")
    required = [
        "architecture_generation_manifest.json",
        "architecture_review_note.md",
        "diagram_review_note.md",
        "threat_boundary_note.md",
        "fitness_function_note.md",
    ]
    for rel in required:
        path = artifact_root / rel
        require(path.is_file(), f"missing required architecture artifact: {rel}")
        scan_text(path, path.read_text(encoding="utf-8"))

    manifest = json.loads(
        (artifact_root / "architecture_generation_manifest.json").read_text(
            encoding="utf-8"
        )
    )
    require(
        manifest.get("schema_version") == "adl.v090.architecture_document_generation_demo.v1",
        "architecture generation manifest schema mismatch",
    )
    require(manifest.get("classification") == "proving", "demo classification must be proving")
    require(len(manifest.get("diagram_sources", [])) >= 7, "manifest lacks diagram sources")
    require(
        "repo-architecture-review" in manifest.get("skills_represented", []),
        "manifest lacks architecture review skill representation",
    )


def main() -> int:
    repo_root = Path(sys.argv[1]).resolve() if len(sys.argv) >= 2 else Path.cwd()
    artifact_root = Path(sys.argv[2]).resolve() if len(sys.argv) >= 3 else None
    check_docs(repo_root)
    if artifact_root is not None:
        check_artifacts(artifact_root)
    print("validate_architecture_docs: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
