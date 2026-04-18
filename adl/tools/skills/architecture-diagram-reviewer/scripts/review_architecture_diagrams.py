#!/usr/bin/env python3
"""Prepare deterministic scaffolding for architecture diagram review."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from pathlib import Path

SCHEMA = "codebuddy.architecture_diagram_review.scaffold.v1"
DIAGRAM_EXTENSIONS = {".mmd", ".mermaid", ".d2", ".puml", ".plantuml", ".dsl", ".md"}
RENDERED_EXTENSIONS = {".svg", ".png", ".jpg", ".jpeg", ".pdf"}
ARCHITECTURE_TERMS = (
    "architecture",
    "component",
    "container",
    "runtime",
    "workflow",
    "state",
    "lifecycle",
    "boundary",
    "diagram",
    "skill",
    "agent",
    "dependency",
    "security",
)
UNSUPPORTED_MARKERS = (
    "unsupported",
    "todo-evidence",
    "needs evidence",
    "invented",
    "guess",
)


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def relative_to_root(root: Path, path: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.name


def evidence_entries(packet_root: Path) -> list[dict[str, object]]:
    data = load_json(packet_root / "evidence_index.json")
    if not isinstance(data, dict):
        return []
    evidence = data.get("evidence")
    if not isinstance(evidence, list):
        return []
    return [item for item in evidence if isinstance(item, dict)]


def architecture_evidence(entries: list[dict[str, object]]) -> list[dict[str, object]]:
    selected: list[dict[str, object]] = []
    for entry in entries:
        text = " ".join(
            [
                str(entry.get("path", "")),
                str(entry.get("category", "")),
                str(entry.get("reason", "")),
                " ".join(str(lane) for lane in entry.get("specialist_lanes", []) if isinstance(lane, str)),
            ]
        ).lower()
        if any(term in text for term in ARCHITECTURE_TERMS):
            selected.append(entry)
    return sorted(selected, key=lambda item: str(item.get("path", "")))[:60]


def diagram_files(diagram_root: Path) -> list[Path]:
    if diagram_root.is_file():
        return [diagram_root] if diagram_root.suffix.lower() in DIAGRAM_EXTENSIONS | RENDERED_EXTENSIONS else []
    files: list[Path] = []
    for path in diagram_root.rglob("*"):
        if path.is_file() and path.suffix.lower() in DIAGRAM_EXTENSIONS | RENDERED_EXTENSIONS:
            files.append(path)
    return sorted(files, key=lambda item: item.as_posix())


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return ""


def tokenize_evidence(entries: list[dict[str, object]]) -> set[str]:
    tokens: set[str] = set()
    for entry in entries:
        for value in (entry.get("path", ""), entry.get("reason", ""), entry.get("category", "")):
            for token in re.findall(r"[A-Za-z][A-Za-z0-9_-]{2,}", str(value).lower()):
                tokens.add(token)
    return tokens


def diagram_claim_tokens(text: str) -> set[str]:
    ignored = {
        "graph",
        "flowchart",
        "sequenceDiagram",
        "participant",
        "classDiagram",
        "stateDiagram",
        "direction",
        "subgraph",
        "style",
        "title",
        "note",
    }
    tokens = {
        token.lower()
        for token in re.findall(r"[A-Za-z][A-Za-z0-9_-]{2,}", text)
        if token not in ignored
    }
    return {token for token in tokens if not token.startswith("http")}


def unsupported_claim_checks(root: Path, diagrams: list[Path], evidence_tokens: set[str]) -> list[dict[str, str]]:
    checks: list[dict[str, str]] = []
    for diagram in diagrams:
        if diagram.suffix.lower() in RENDERED_EXTENSIONS:
            continue
        text = read_text(diagram)
        lowered = text.lower()
        if any(marker in lowered for marker in UNSUPPORTED_MARKERS):
            checks.append(
                {
                    "diagram": relative_to_root(root, diagram),
                    "claim": "source contains an explicit unsupported/evidence marker",
                    "severity": "review_required",
                    "handoff": "diagram-author",
                }
            )
        candidate_tokens = sorted(diagram_claim_tokens(text) - evidence_tokens)
        filtered = [token for token in candidate_tokens if token not in {"mermaid", "plantuml", "structurizr"}]
        if filtered:
            checks.append(
                {
                    "diagram": relative_to_root(root, diagram),
                    "claim": ", ".join(filtered[:12]),
                    "severity": "needs_evidence_check",
                    "handoff": "diagram-author",
                }
            )
    return checks[:20]


def missing_component_checks(entries: list[dict[str, object]], diagrams: list[Path]) -> list[dict[str, str]]:
    diagram_text = "\n".join(read_text(path).lower() for path in diagrams if path.suffix.lower() not in RENDERED_EXTENSIONS)
    checks: list[dict[str, str]] = []
    for entry in entries[:20]:
        path = str(entry.get("path", ""))
        if not path:
            continue
        stem = Path(path).stem.lower()
        if stem and stem not in diagram_text:
            checks.append(
                {
                    "evidence": path,
                    "check": "high-signal evidence surface may be omitted or needs explicit out-of-scope note",
                    "handoff": "repo-diagram-planner",
                }
            )
    return checks[:12]


def renderer_status(root: Path, diagrams: list[Path]) -> list[dict[str, str]]:
    statuses: list[dict[str, str]] = []
    by_stem: dict[str, set[str]] = {}
    for diagram in diagrams:
        by_stem.setdefault(diagram.stem, set()).add(diagram.suffix.lower())
    for stem, suffixes in sorted(by_stem.items()):
        source_exts = sorted(suffixes & DIAGRAM_EXTENSIONS)
        rendered_exts = sorted(suffixes & RENDERED_EXTENSIONS)
        statuses.append(
            {
                "diagram": stem,
                "source": ",".join(source_exts) if source_exts else "missing",
                "rendered": ",".join(rendered_exts) if rendered_exts else "missing",
                "status": "rendered_present" if rendered_exts else "source_only",
            }
        )
    if not statuses:
        statuses.append(
            {
                "diagram": "none",
                "source": "missing",
                "rendered": "missing",
                "status": "blocked_no_diagram_files",
            }
        )
    return statuses


def correction_handoffs(unsupported: list[dict[str, str]], missing: list[dict[str, str]]) -> list[dict[str, str]]:
    handoffs: list[dict[str, str]] = []
    if unsupported:
        handoffs.append(
            {
                "skill": "diagram-author",
                "reason": "Diagram source has unsupported or evidence-uncertain claims that need bounded source correction.",
            }
        )
    if missing:
        handoffs.append(
            {
                "skill": "repo-diagram-planner",
                "reason": "Diagram plan may need to include or explicitly exclude high-signal evidence surfaces.",
            }
        )
    if not handoffs:
        handoffs.append(
            {
                "skill": "repo-review-synthesis",
                "reason": "No scaffold-level correction handoff was generated; synthesis may consume the review artifact.",
            }
        )
    return handoffs


def lines(items: list[dict[str, str]], keys: tuple[str, ...]) -> str:
    if not items:
        return "- None."
    rendered: list[str] = []
    for item in items:
        rendered.append("- " + "; ".join(f"{key}: {item.get(key, '')}" for key in keys))
    return "\n".join(rendered)


def write_markdown(path: Path, scaffold: dict[str, object]) -> None:
    reviewed = scaffold["reviewed_diagrams"]
    reviewed_lines = "\n".join(f"- {item}" for item in reviewed) or "- No diagram files found."
    evidence_map = scaffold["evidence_coverage_map"]
    evidence_lines = "\n".join(
        f"- {diagram}: {', '.join(paths)}" for diagram, paths in sorted(evidence_map.items())
    ) or "- No evidence map generated."
    content = f"""# Architecture Diagram Review Scaffold

## Metadata

- Skill: architecture-diagram-reviewer
- Repo: {scaffold["repo_name"]}
- Packet: {scaffold["packet_root"]}
- Diagram Root: {scaffold["diagram_root"]}
- Date: {scaffold["created_at"]}

## Findings

- No findings have been written yet. Replace this section with findings-first diagram review output after inspection.

## Reviewed Diagrams

{reviewed_lines}

## Evidence Coverage Map

{evidence_lines}

## Unsupported Claim Checks

{lines(scaffold["unsupported_claim_checks"], ("diagram", "claim", "severity", "handoff"))}

## Missing Component Checks

{lines(scaffold["missing_component_checks"], ("evidence", "check", "handoff"))}

## Renderer Status Checks

{lines(scaffold["renderer_status_checks"], ("diagram", "source", "rendered", "status"))}

## Accessibility And Readability Notes

- Scaffold does not perform visual inspection. Reviewer should check title, caption, legend, label length, contrast, and audience fit.

## Correction Handoffs

{lines(scaffold["correction_handoffs"], ("skill", "reason"))}

## Validation Performed

- Scaffold generation only; no rendering, publication, or repository mutation was performed.

## Residual Risk

- Source parsing is heuristic and does not prove diagram correctness.
- Rendered visual quality was not inspected by this helper.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet_root", help="CodeBuddy review packet root")
    parser.add_argument("diagram_root", help="Diagram packet, diagram source file, or rendered artifact root")
    parser.add_argument("--out", default=None, help="Architecture diagram review scaffold output root")
    parser.add_argument("--repo-name", default=None, help="Repo name override")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    packet_root = Path(args.packet_root).resolve()
    diagram_root = Path(args.diagram_root).resolve()
    if not packet_root.is_dir():
        raise SystemExit(f"packet root does not exist: {packet_root}")
    if not diagram_root.exists():
        raise SystemExit(f"diagram root does not exist: {diagram_root}")

    out_root = Path(args.out) if args.out else packet_root / "architecture-diagram-review"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    manifest = load_json(packet_root / "run_manifest.json")
    repo_name = args.repo_name
    if repo_name is None and isinstance(manifest, dict):
        repo_name = str(manifest.get("repo_name", "") or "")
    repo_name = repo_name or packet_root.name

    evidence = architecture_evidence(evidence_entries(packet_root))
    diagrams = diagram_files(diagram_root)
    evidence_tokens = tokenize_evidence(evidence)
    unsupported = unsupported_claim_checks(diagram_root, diagrams, evidence_tokens)
    missing = missing_component_checks(evidence, diagrams)
    reviewed = [relative_to_root(diagram_root, path) for path in diagrams]
    evidence_paths = [str(entry.get("path", "")) for entry in evidence if str(entry.get("path", ""))]
    scaffold = {
        "schema": SCHEMA,
        "repo_name": repo_name,
        "packet_root": packet_root.name,
        "diagram_root": diagram_root.name,
        "created_at": now_utc(),
        "reviewed_diagrams": reviewed,
        "evidence_coverage_map": {diagram: evidence_paths[:12] for diagram in reviewed},
        "unsupported_claim_checks": unsupported,
        "missing_component_checks": missing,
        "renderer_status_checks": renderer_status(diagram_root, diagrams),
        "correction_handoffs": correction_handoffs(unsupported, missing),
        "notes": [
            "Scaffold is deterministic except for created_at.",
            "Paths are packet-relative or diagram-root-relative, not absolute host paths.",
            "Helper does not author, edit, render, publish, or mutate diagrams.",
        ],
    }
    write_json(out_root / "architecture_diagram_review_scaffold.json", scaffold)
    write_markdown(out_root / "architecture_diagram_review_scaffold.md", scaffold)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

