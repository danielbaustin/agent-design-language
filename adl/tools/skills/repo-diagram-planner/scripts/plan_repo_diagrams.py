#!/usr/bin/env python3
"""Plan source-grounded diagram tasks from a CodeBuddy review packet."""

from __future__ import annotations

import argparse
import datetime as dt
import json
from collections import defaultdict
from pathlib import Path

SCHEMA = "codebuddy.repo_diagram_plan.v1"
MAX_DEFAULT_TASKS = 8

FAMILY_KEYWORDS = {
    "system_context": ("readme", "overview", "getting-started", "architecture", "milestone"),
    "container_or_component": ("architecture", "component", "module", "package", "adapter", "runtime"),
    "workflow": ("workflow", "lifecycle", "queue", "conductor", "closeout", "planner", "run", "issue"),
    "sequence": ("api", "provider", "request", "response", "client", "server", "interaction"),
    "state": ("state", "status", "transition", "cycle", "long-lived", "resume", "pause"),
    "data_flow": ("security", "secret", "redaction", "privacy", "trust", "upload", "network", "external"),
    "dependency_graph": ("dependency", "dependencies", "manifest", "lockfile", "package", "import", "skill"),
    "responsibility_map": ("agent", "specialist", "handoff", "review", "skill", "lane", "synthesis"),
}

FAMILY_BACKENDS = {
    "system_context": "mermaid",
    "container_or_component": "structurizr",
    "workflow": "mermaid",
    "sequence": "plantuml",
    "state": "mermaid",
    "data_flow": "mermaid",
    "dependency_graph": "mermaid",
    "responsibility_map": "mermaid",
}

FAMILY_GOALS = {
    "system_context": "Orient reviewers to the repo boundary, users, adjacent systems, and primary review scope.",
    "container_or_component": "Clarify architecture boundaries and component responsibilities before diagram authoring.",
    "workflow": "Explain a lifecycle or process path with source-backed steps and handoffs.",
    "sequence": "Show time-ordered actor or service interaction only where evidence supports the messages.",
    "state": "Clarify lifecycle states, transitions, and terminal conditions.",
    "data_flow": "Map data movement and trust-boundary questions for a later source-grounded diagram.",
    "dependency_graph": "Map dependency or skill relationships without asserting unsupported direction.",
    "responsibility_map": "Clarify specialist, agent, or skill ownership and handoff boundaries.",
}

FAMILY_AUDIENCE = {
    "system_context": "reviewers and maintainers",
    "container_or_component": "architecture reviewers",
    "workflow": "operators and reviewers",
    "sequence": "implementation reviewers",
    "state": "runtime and lifecycle reviewers",
    "data_flow": "security and privacy reviewers",
    "dependency_graph": "dependency and architecture reviewers",
    "responsibility_map": "multi-agent review operators",
}


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def evidence_entries(packet_root: Path) -> list[dict[str, object]]:
    data = load_json(packet_root / "evidence_index.json")
    if not isinstance(data, dict):
        return []
    evidence = data.get("evidence")
    if not isinstance(evidence, list):
        return []
    return [item for item in evidence if isinstance(item, dict)]


def evidence_text(entry: dict[str, object]) -> str:
    parts = [
        str(entry.get("path", "")),
        str(entry.get("category", "")),
        str(entry.get("reason", "")),
        " ".join(str(lane) for lane in entry.get("specialist_lanes", []) if isinstance(lane, str)),
    ]
    return " ".join(parts).lower()


def candidate_families(entry: dict[str, object]) -> set[str]:
    text = evidence_text(entry)
    families = {
        family
        for family, keywords in FAMILY_KEYWORDS.items()
        if any(keyword in text for keyword in keywords)
    }
    lanes = entry.get("specialist_lanes")
    if isinstance(lanes, list):
        if "architecture" in lanes:
            families.add("container_or_component")
        if "security" in lanes:
            families.add("data_flow")
        if "dependency" in lanes or "dependencies" in lanes:
            families.add("dependency_graph")
        if "diagrams" in lanes:
            families.add("system_context")
    return families


def select_evidence_by_family(entries: list[dict[str, object]]) -> dict[str, list[dict[str, object]]]:
    grouped: dict[str, list[dict[str, object]]] = defaultdict(list)
    for entry in entries:
        path = str(entry.get("path", ""))
        if not path:
            continue
        for family in candidate_families(entry):
            grouped[family].append(entry)
    return {
        family: sorted(items, key=lambda item: str(item.get("path", "")))[:10]
        for family, items in sorted(grouped.items())
    }


def task_id(index: int, family: str) -> str:
    return f"diagram-{index:02d}-{family.replace('_', '-')}"


def build_tasks(grouped: dict[str, list[dict[str, object]]], max_tasks: int) -> list[dict[str, object]]:
    priority = [
        "system_context",
        "container_or_component",
        "workflow",
        "state",
        "data_flow",
        "dependency_graph",
        "responsibility_map",
        "sequence",
    ]
    tasks: list[dict[str, object]] = []
    for family in priority:
        evidence = grouped.get(family, [])
        if not evidence:
            continue
        index = len(tasks) + 1
        paths = [str(item.get("path", "")) for item in evidence if str(item.get("path", ""))]
        tasks.append(
            {
                "id": task_id(index, family),
                "diagram_family": family,
                "suggested_backend": FAMILY_BACKENDS[family],
                "audience": FAMILY_AUDIENCE[family],
                "goal": FAMILY_GOALS[family],
                "source_evidence": paths,
                "assumptions": [
                    "Diagram-author must treat this as a planning brief and verify each relationship before authoring source."
                ],
                "unknowns": [
                    "Exact diagram nodes, edges, labels, and renderer settings are not selected by this planner."
                ],
                "claims_not_allowed": [
                    "Do not claim runtime behavior, dependency direction, trust boundaries, or deployment topology unless a cited source supports it."
                ],
                "renderer_expectation": "renderer selected later by diagram-author; no rendered artifact is created by this planner",
                "handoff": {
                    "skill": "diagram-author",
                    "brief": f"Create one source-grounded {family.replace('_', ' ')} diagram from the listed evidence paths. Preserve assumptions and unknowns.",
                },
            }
        )
        if len(tasks) >= max_tasks:
            break
    return tasks


def build_evidence_map(tasks: list[dict[str, object]]) -> dict[str, list[str]]:
    return {
        str(task["id"]): list(task["source_evidence"])
        for task in tasks
    }


def build_skipped(grouped: dict[str, list[dict[str, object]]], tasks: list[dict[str, object]]) -> list[dict[str, str]]:
    selected = {str(task["diagram_family"]) for task in tasks}
    skipped: list[dict[str, str]] = []
    for family in sorted(grouped):
        if family not in selected:
            skipped.append(
                {
                    "diagram_family": family,
                    "reason": "Candidate evidence exists but max_tasks or priority ordering deferred it.",
                }
            )
    if not grouped:
        skipped.append(
            {
                "diagram_family": "all",
                "reason": "No source evidence strongly indicated a bounded diagram task.",
            }
        )
    return skipped


def task_lines(tasks: list[dict[str, object]]) -> str:
    if not tasks:
        return "- No diagram tasks selected."
    lines: list[str] = []
    for task in tasks:
        lines.extend(
            [
                f"- {task['id']}: {task['diagram_family']} via {task['suggested_backend']}",
                f"  Goal: {task['goal']}",
                f"  Audience: {task['audience']}",
                f"  Evidence: {', '.join(task['source_evidence'])}",
                f"  Handoff: {task['handoff']['skill']} - {task['handoff']['brief']}",
            ]
        )
    return "\n".join(lines)


def evidence_map_lines(evidence_map: dict[str, list[str]]) -> str:
    if not evidence_map:
        return "- No evidence map produced."
    lines: list[str] = []
    for task, paths in sorted(evidence_map.items()):
        lines.append(f"- {task}:")
        for path in paths:
            lines.append(f"  - {path}")
    return "\n".join(lines)


def skipped_lines(skipped: list[dict[str, str]]) -> str:
    return "\n".join(f"- {item['diagram_family']}: {item['reason']}" for item in skipped) or "- None."


def rationale_lines(tasks: list[dict[str, object]]) -> str:
    return "\n".join(
        f"- {task['id']}: {task['diagram_family']} fits the selected evidence; {task['suggested_backend']} is the suggested backend."
        for task in tasks
    ) or "- No backend rationale because no tasks were selected."


def handoff_lines(tasks: list[dict[str, object]]) -> str:
    return "\n".join(
        f"- {task['id']}: Send to `diagram-author` with evidence {', '.join(task['source_evidence'])}."
        for task in tasks
    ) or "- No handoff because no tasks were selected."


def write_markdown(path: Path, plan: dict[str, object]) -> None:
    content = f"""# Repo Diagram Plan

## Metadata

- Skill: repo-diagram-planner
- Repo: {plan["repo_name"]}
- Packet: {plan["packet_root"]}
- Date: {plan["created_at"]}

## Diagram Tasks

{task_lines(plan["diagram_tasks"])}

## Source Evidence Map

{evidence_map_lines(plan["source_evidence_map"])}

## Family / Backend Rationale

{rationale_lines(plan["diagram_tasks"])}

## Assumptions And Unknowns

- Planner output is a handoff brief, not diagram source.
- Diagram-author must verify every node, edge, label, and claim before creating diagram source.
- Renderer availability is not checked by this planner.

## Blocked Or Skipped Candidates

{skipped_lines(plan["blocked_or_skipped_candidates"])}

## Validation Performed

- Scaffold generation only; no rendering, publication, or repository mutation was performed.

## Diagram Author Handoff

{handoff_lines(plan["diagram_tasks"])}

## Residual Risk

- This plan is only as complete as the packet evidence. Missing specialist artifacts may hide better diagram candidates.
- No diagram source, SVG, PNG, or other rendered artifact was created by this planner.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet_root", help="CodeBuddy review packet root")
    parser.add_argument("--out", default=None, help="Diagram plan output root")
    parser.add_argument("--repo-name", default=None, help="Repo name override")
    parser.add_argument("--max-tasks", type=int, default=MAX_DEFAULT_TASKS, help="Maximum diagram tasks to emit")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    packet_root = Path(args.packet_root).resolve()
    if not packet_root.is_dir():
        raise SystemExit(f"packet root does not exist: {packet_root}")
    if args.max_tasks < 1:
        raise SystemExit("--max-tasks must be greater than zero")

    out_root = Path(args.out) if args.out else packet_root / "diagram-plan"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)

    manifest = load_json(packet_root / "run_manifest.json")
    repo_name = args.repo_name
    if repo_name is None and isinstance(manifest, dict):
        repo_name = str(manifest.get("repo_name", "") or "")
    repo_name = repo_name or packet_root.name

    entries = sorted(evidence_entries(packet_root), key=lambda item: str(item.get("path", "")))
    grouped = select_evidence_by_family(entries)
    tasks = build_tasks(grouped, args.max_tasks)
    plan = {
        "schema": SCHEMA,
        "repo_name": repo_name,
        "packet_root": packet_root.name,
        "created_at": now_utc(),
        "diagram_tasks": tasks,
        "source_evidence_map": build_evidence_map(tasks),
        "blocked_or_skipped_candidates": build_skipped(grouped, tasks),
        "notes": [
            "Planner output is deterministic except for created_at.",
            "Paths are packet evidence paths, not absolute host paths.",
            "This planner does not author diagram source, render diagrams, publish diagrams, or mutate repositories.",
            "Each selected task must be handed to diagram-author as a separate bounded task.",
        ],
    }
    write_json(out_root / "repo_diagram_plan.json", plan)
    write_markdown(out_root / "repo_diagram_plan.md", plan)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

