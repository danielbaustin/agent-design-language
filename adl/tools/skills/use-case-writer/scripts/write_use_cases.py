#!/usr/bin/env python3
"""Create a source-grounded use-case packet from explicit brief files."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
from pathlib import Path
from typing import Any

SCHEMA = "adl.use_case_packet.v1"


def now_utc() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError:
        return ""


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def rel(root: Path, path: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.name


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def extract_bullets_after(text: str, heading: str) -> list[str]:
    pattern = re.compile(rf"^##+\s+{re.escape(heading)}\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(text)
    if not match:
        return []
    next_heading = re.search(r"^##+\s+", text[match.end() :], re.MULTILINE)
    end = match.end() + next_heading.start() if next_heading else len(text)
    section = text[match.end() : end]
    return [line.strip()[2:].strip() for line in section.splitlines() if line.strip().startswith("- ")][:80]


def bullet_file(root: Path, name: str, heading: str) -> list[str]:
    text = read_text(root / name)
    return extract_bullets_after(text, heading) or [line.strip()[2:].strip() for line in text.splitlines() if line.strip().startswith("- ")]


def line_value(text: str, key: str) -> str:
    pattern = re.compile(rf"^\s*-\s*{re.escape(key)}:\s*(.+?)\s*$", re.IGNORECASE | re.MULTILINE)
    match = pattern.search(text)
    return match.group(1).strip() if match else ""


def metadata(root: Path) -> dict[str, str]:
    manifest = load_json(root / "use_case_manifest.json")
    if not isinstance(manifest, dict):
        manifest = {}
    source_text = read_text(root / "source_brief.md")
    return {
        "run_id": str(manifest.get("run_id") or root.name),
        "mode": str(manifest.get("mode") or "write_prd_use_cases"),
        "source_ref": str(manifest.get("source_ref") or line_value(source_text, "Source") or "source_brief.md"),
        "audience": str(manifest.get("audience") or line_value(source_text, "Audience") or "reviewer"),
    }


def sentence_from(text: str, fallback: str) -> str:
    cleaned = re.sub(r"\s+", " ", text).strip()
    if not cleaned:
        return fallback
    match = re.search(r"(.+?[.!?])(?:\s|$)", cleaned)
    return match.group(1) if match else cleaned[:180]


def source_available(root: Path) -> bool:
    return bool(read_text(root / "source_brief.md").strip() or load_json(root / "use_case_manifest.json"))


def make_use_cases(
    root: Path,
    source_text: str,
    actors: list[str],
    goals: list[str],
    behaviors: list[str],
    acceptance: list[str],
    non_goals: list[str],
    unsupported: list[str],
    max_use_cases: int,
) -> list[dict[str, object]]:
    if not actors and not goals and not behaviors:
        return []
    actors = actors or ["Primary user"]
    goals = goals or [sentence_from(source_text, "Use the described capability successfully.")]
    behaviors = behaviors or ["System behavior must follow the declared source without adding unsupported scope."]
    acceptance = acceptance or ["Reviewer confirms the scenario is grounded in the declared source."]
    use_cases = []
    count = min(max_use_cases, max(len(goals), 1))
    for index in range(count):
        actor = actors[min(index, len(actors) - 1)]
        goal = goals[min(index, len(goals) - 1)]
        behavior = behaviors[min(index, len(behaviors) - 1)]
        use_cases.append(
            {
                "id": f"UC-{index + 1:03d}",
                "title": goal[:96],
                "actor": actor,
                "user_goal": goal,
                "system_behavior": behavior,
                "trigger": "Actor attempts the scenario described by the source brief.",
                "preconditions": ["Declared source has been reviewed.", "No unsupported assumptions have been promoted to requirements."],
                "success_flow": [
                    "Actor starts from the declared context.",
                    "System performs the source-supported behavior.",
                    "Reviewer checks the acceptance hooks against the source.",
                ],
                "failure_or_edge_flow": [
                    "If source evidence is missing, record an unsupported assumption instead of expanding scope.",
                    "If behavior is not implemented, do not claim implementation status.",
                ],
                "acceptance_hooks": acceptance,
                "evidence_source": "source_brief.md",
                "non_goals": non_goals or ["Implementation planning is out of scope for this packet."],
                "unsupported_assumptions": unsupported,
            }
        )
    return use_cases


def stop_boundary() -> dict[str, bool]:
    return {
        "created_issues": False,
        "created_prs": False,
        "claimed_implementation_complete": False,
        "published_externally": False,
        "mutated_repository": False,
    }


def analyze(root: Path, max_use_cases: int) -> dict[str, object]:
    meta = metadata(root)
    source_text = read_text(root / "source_brief.md")
    actors = bullet_file(root, "actors.md", "Actors")
    goals = bullet_file(root, "goals.md", "User Goals")
    behaviors = bullet_file(root, "system_behavior.md", "System Behavior")
    acceptance = bullet_file(root, "acceptance_hooks.md", "Acceptance Hooks")
    non_goals = bullet_file(root, "non_goals.md", "Non-goals")
    unsupported = bullet_file(root, "unsupported_assumptions.md", "Unsupported Assumptions")
    assumptions = bullet_file(root, "assumptions.md", "Assumptions")
    if not source_available(root):
        return {
            "schema": SCHEMA,
            "created_at": now_utc(),
            "run_id": meta["run_id"],
            "status": "not_run",
            "source": {"ref": "missing", "mode": meta["mode"]},
            "audience": meta["audience"],
            "actors": [],
            "use_cases": [],
            "acceptance_hooks": [],
            "assumptions": [],
            "unsupported_assumptions": ["Declared source brief or issue context is missing."],
            "non_goals": ["No use-case packet was written because the source is missing."],
            "stop_boundary": stop_boundary(),
        }
    use_cases = make_use_cases(root, source_text, actors, goals, behaviors, acceptance, non_goals, unsupported, max_use_cases)
    status = "ready" if actors and goals and behaviors and acceptance else "partial"
    unsupported_out = unsupported[:]
    if not actors:
        unsupported_out.append("Actors were not explicitly supplied; default actor labels require review.")
    if not acceptance:
        unsupported_out.append("Acceptance hooks were inferred as reviewer checks and require review.")
    return {
        "schema": SCHEMA,
        "created_at": now_utc(),
        "run_id": meta["run_id"],
        "status": status,
        "source": {"ref": meta["source_ref"], "mode": meta["mode"]},
        "audience": meta["audience"],
        "actors": actors or ["Primary user"],
        "use_cases": use_cases,
        "acceptance_hooks": acceptance or ["Reviewer confirms the scenario is grounded in the declared source."],
        "assumptions": assumptions or ["Only source-supported goals and behavior should be treated as requirements."],
        "unsupported_assumptions": unsupported_out,
        "non_goals": non_goals or ["Implementation planning and tracker creation are out of scope."],
        "stop_boundary": stop_boundary(),
    }


def bullet_lines(items: list[Any]) -> str:
    return "\n".join(f"- {item}" for item in items) if items else "- None."


def use_case_lines(use_cases: list[dict[str, object]]) -> str:
    if not use_cases:
        return "- No use cases written."
    parts = []
    for item in use_cases:
        parts.append(
            f"""### {item['id']}: {item['title']}

- Actor: {item['actor']}
- User goal: {item['user_goal']}
- System behavior: {item['system_behavior']}
- Trigger: {item['trigger']}
- Preconditions: {', '.join(str(value) for value in item['preconditions'])}
- Success flow: {'; '.join(str(value) for value in item['success_flow'])}
- Failure or edge flow: {'; '.join(str(value) for value in item['failure_or_edge_flow'])}
- Acceptance hooks: {', '.join(str(value) for value in item['acceptance_hooks'])}
- Evidence source: {item['evidence_source']}
- Non-goals: {', '.join(str(value) for value in item['non_goals'])}
- Unsupported assumptions: {', '.join(str(value) for value in item['unsupported_assumptions']) if item['unsupported_assumptions'] else 'None.'}
"""
        )
    return "\n".join(parts)


def write_markdown(path: Path, report: dict[str, object]) -> None:
    boundary = report["stop_boundary"]
    content = f"""# Use Case Packet: {report['source']['ref']}

## Use Case Packet Summary

- Status: {report['status']}
- Run id: {report['run_id']}
- Use cases: {len(report['use_cases'])}

## Source

- Source ref: {report['source']['ref']}
- Mode: {report['source']['mode']}

## Audience

- {report['audience']}

## Actors

{bullet_lines(report['actors'])}

## Use Cases

{use_case_lines(report['use_cases'])}

## Acceptance Hooks

{bullet_lines(report['acceptance_hooks'])}

## Assumptions

{bullet_lines(report['assumptions'])}

## Unsupported Assumptions

{bullet_lines(report['unsupported_assumptions'])}

## Non-goals

{bullet_lines(report['non_goals'])}

## Stop Boundary

- Created issues: {str(boundary['created_issues']).lower()}.
- Created PRs: {str(boundary['created_prs']).lower()}.
- Claimed implementation complete: {str(boundary['claimed_implementation_complete']).lower()}.
- Published externally: {str(boundary['published_externally']).lower()}.
- Mutated repository: {str(boundary['mutated_repository']).lower()}.
"""
    path.write_text(content, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("use_case_root", help="Directory containing use-case input evidence")
    parser.add_argument("--out", default=None, help="Use-case packet output root")
    parser.add_argument("--run-id", default=None, help="Run id override")
    parser.add_argument("--max-use-cases", type=int, default=5, help="Maximum use cases to emit")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = Path(args.use_case_root)
    if not root.is_dir():
        raise SystemExit(f"use-case root does not exist: {root}")
    out_root = Path(args.out) if args.out else root / "use-case-writer"
    if not out_root.is_absolute():
        out_root = Path.cwd() / out_root
    out_root.mkdir(parents=True, exist_ok=True)
    report = analyze(root, args.max_use_cases)
    if args.run_id:
        report["run_id"] = args.run_id
    write_json(out_root / "use_case_packet.json", report)
    write_markdown(out_root / "use_case_packet.md", report)
    print(out_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
