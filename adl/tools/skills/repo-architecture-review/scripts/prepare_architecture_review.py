#!/usr/bin/env python3
"""Prepare deterministic scaffolding for a CodeBuddy architecture review."""

from __future__ import annotations

import argparse
import datetime as dt
import json
from pathlib import Path

SCHEMA = "codebuddy.repo_architecture_review.scaffold.v1"
ARCHITECTURE_CATEGORIES = {"architecture_docs", "docs", "manifest", "ci", "code", "test"}
ARCHITECTURE_TERMS = (
    "architecture",
    "runtime",
    "state",
    "lifecycle",
    "workflow",
    "orchestration",
    "boundary",
    "diagram",
    "adr",
    "review",
    "skill",
    "agent",
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


def packet_relative(packet_root: Path, path: Path) -> str:
    try:
        return path.relative_to(packet_root).as_posix()
    except ValueError:
        return path.name


def evidence_entries(packet_root: Path) -> list[dict[str, object]]:
    data = load_json(packet_root / "evidence_index.json")
    if not isinstance(data, dict):
        return []
    evidence = data.get("evidence")
    if not isinstance(evidence, list):
        return []
    entries: list[dict[str, object]] = []
    for item in evidence:
        if isinstance(item, dict):
            entries.append(item)
    return entries


def is_architecture_evidence(entry: dict[str, object]) -> bool:
    category = str(entry.get("category", ""))
    path = str(entry.get("path", "")).lower()
    reason = str(entry.get("reason", "")).lower()
    lanes = entry.get("specialist_lanes")
    lane_match = isinstance(lanes, list) and "architecture" in lanes
    term_match = any(term in path or term in reason for term in ARCHITECTURE_TERMS)
    return lane_match or category in ARCHITECTURE_CATEGORIES and term_match


def build_candidate_diagram_tasks(entries: list[dict[str, object]]) -> list[dict[str, str]]:
    tasks: list[dict[str, str]] = []
    seen: set[str] = set()
    for entry in entries:
        path = str(entry.get("path", ""))
        lowered = path.lower()
        if not path or path in seen:
            continue
        if any(term in lowered for term in ("workflow", "runtime", "state", "lifecycle", "architecture")):
            tasks.append(
                {
                    "type": "architecture_diagram",
                    "source": path,
                    "handoff": "diagram-author",
                    "reason": "High-signal architecture surface likely benefits from a visual boundary or lifecycle view.",
                }
            )
            seen.add(path)
    return tasks[:12]


def build_candidate_adr_topics(entries: list[dict[str, object]]) -> list[dict[str, str]]:
    topics: list[dict[str, str]] = []
    seen: set[str] = set()
    for entry in entries:
        path = str(entry.get("path", ""))
        lowered = path.lower()
        if not path or path in seen:
            continue
        if any(term in lowered for term in ("policy", "runtime", "workflow", "provider", "skill", "agent")):
            topics.append(
                {
                    "topic": f"Architecture decision around {path}",
                    "source": path,
                    "handoff": "adr-curator",
                    "reason": "Surface appears to encode durable architecture policy or runtime structure.",
                }
            )
            seen.add(path)
    return topics[:12]


def build_candidate_fitness_functions(entries: list[dict[str, object]]) -> list[dict[str, str]]:
    candidates: list[dict[str, str]] = []
    seen: set[str] = set()
    for entry in entries:
        path = str(entry.get("path", ""))
        lowered = path.lower()
        if not path or path in seen:
            continue
        if any(term in lowered for term in ("test", "check", "lint", "validate", "policy", "contract")):
            candidates.append(
                {
                    "candidate": f"Guard architecture contract represented by {path}",
                    "source": path,
                    "handoff": "architecture-fitness-function-author",
                    "reason": "Surface suggests an executable architecture rule or contract boundary.",
                }
            )
            seen.add(path)
    return candidates[:12]


def write_markdown(path: Path, scaffold: dict[str, object]) -> None:
    evidence = scaffold["architecture_evidence"]
    evidence_lines = "\n".join(
        f"- {item['path']} ({item.get('category', 'unknown')}): {item.get('reason', 'architecture evidence')}"
        for item in evidence
    ) or "- No architecture evidence selected from packet."
    diagram_lines = "\n".join(
        f"- {item['source']}: {item['reason']} Handoff: {item['handoff']}"
        for item in scaffold["candidate_diagram_tasks"]
    ) or "- None identified by scaffold."
    adr_lines = "\n".join(
        f"- {item['topic']} Source: {item['source']}. Handoff: {item['handoff']}"
        for item in scaffold["candidate_adr_topics"]
    ) or "- None identified by scaffold."
    fitness_lines = "\n".join(
        f"- {item['candidate']} Source: {item['source']}. Handoff: {item['handoff']}"
        for item in scaffold["candidate_fitness_functions"]
    ) or "- None identified by scaffold."

    content = f"""# Repo Architecture Review Scaffold

## Metadata

- Skill: repo-architecture-review
- Repo: {scaffold["repo_name"]}
- Packet: {scaffold["packet_root"]}
- Date: {scaffold["created_at"]}

## Findings

- No findings have been written yet. Replace this section with findings-first architecture review output after inspection.

## Architecture Map

- Use the evidence below to map modules, layers, runtime boundaries, state ownership, integration points, and drift surfaces.

## Reviewed Surfaces

{evidence_lines}

## Candidate Diagram Tasks

{diagram_lines}

## Candidate ADRs

{adr_lines}

## Candidate Fitness Functions

{fitness_lines}

## Validation Performed

- Scaffold generation only; no repository validation commands were run by this helper.

## Residual Risk

- This scaffold is not a review finding artifact. A reviewer must inspect the selected surfaces and record findings or an explicit no-material-findings result.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet_root", help="CodeBuddy review packet root")
    parser.add_argument("--out", default=None, help="Architecture review scaffold output root")
    parser.add_argument("--repo-name", default=None, help="Repo name override")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    packet_root = Path(args.packet_root).resolve()
    if not packet_root.is_dir():
        raise SystemExit(f"packet root does not exist: {packet_root}")

    out_root = Path(args.out) if args.out else packet_root / "architecture-review"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    manifest = load_json(packet_root / "run_manifest.json")
    repo_name = args.repo_name
    if repo_name is None and isinstance(manifest, dict):
        repo_name = str(manifest.get("repo_name", "") or "")
    repo_name = repo_name or packet_root.name

    entries = [entry for entry in evidence_entries(packet_root) if is_architecture_evidence(entry)]
    entries = sorted(entries, key=lambda item: str(item.get("path", "")))[:60]
    scaffold = {
        "schema": SCHEMA,
        "repo_name": repo_name,
        "packet_root": packet_root.name,
        "created_at": now_utc(),
        "architecture_evidence": entries,
        "candidate_diagram_tasks": build_candidate_diagram_tasks(entries),
        "candidate_adr_topics": build_candidate_adr_topics(entries),
        "candidate_fitness_functions": build_candidate_fitness_functions(entries),
        "notes": [
            "Scaffold is deterministic except for created_at.",
            "Paths are packet evidence paths, not absolute host paths.",
            "Reviewer must replace scaffold findings with source-grounded architecture review findings.",
        ],
    }
    write_json(out_root / "architecture_review_scaffold.json", scaffold)
    write_markdown(out_root / "architecture_review_scaffold.md", scaffold)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

