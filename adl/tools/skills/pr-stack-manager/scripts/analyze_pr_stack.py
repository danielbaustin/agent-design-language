#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path


def node_issue(node: dict) -> int | None:
    value = node.get("issue_number")
    return int(value) if value is not None else None


def detect_cycles(nodes: list[dict]) -> bool:
    graph = {node_issue(node): list(node.get("depends_on", [])) for node in nodes if node_issue(node) is not None}
    visiting: set[int] = set()
    visited: set[int] = set()

    def visit(issue: int) -> bool:
        if issue in visiting:
            return True
        if issue in visited:
            return False
        visiting.add(issue)
        for dep in graph.get(issue, []):
            if dep in graph and visit(dep):
                return True
        visiting.remove(issue)
        visited.add(issue)
        return False

    return any(visit(issue) for issue in graph)


def main() -> int:
    parser = argparse.ArgumentParser(description="Analyze a bounded ADL PR stack packet.")
    parser.add_argument("payload_path", help="Path to a JSON stack packet.")
    parser.add_argument("--out", help="Optional JSON output path.")
    args = parser.parse_args()

    payload_path = Path(args.payload_path)
    payload = json.loads(payload_path.read_text(encoding="utf-8"))
    nodes = payload.get("nodes", [])
    known_issues = {node_issue(node) for node in nodes if node_issue(node) is not None}
    findings: list[dict] = []
    edges: list[dict] = []
    planned_actions: list[dict] = []

    for node in nodes:
        issue = node_issue(node)
        base_ref = node.get("base_ref")
        expected_base = node.get("expected_base_ref")
        depends_on = list(node.get("depends_on", []))

        for dep in depends_on:
            edges.append(
                {
                    "from_issue": issue,
                    "to_issue": dep,
                    "edge_type": "requires",
                    "confidence": "high" if dep in known_issues else "medium",
                }
            )
            if dep not in known_issues:
                findings.append(
                    {
                        "severity": "warning",
                        "area": "dependency_order",
                        "message": f"Issue {issue} depends on missing issue {dep}",
                        "evidence": {
                            "summary": "Dependency is listed but no matching node exists in the stack packet.",
                            "files": [payload_path.as_posix()],
                            "refs": [f"#{issue}", f"#{dep}"],
                        },
                        "can_auto_fix": False,
                    }
                )

        if expected_base and base_ref and expected_base != base_ref:
            findings.append(
                {
                    "severity": "blocking",
                    "area": "base_alignment",
                    "message": f"Issue {issue} base {base_ref!r} does not match expected base {expected_base!r}",
                    "evidence": {
                        "summary": "PR base drift would make merge order or review diff misleading.",
                        "files": [payload_path.as_posix()],
                        "refs": [node.get("branch"), f"PR {node.get('pr_number')}"],
                    },
                    "can_auto_fix": False,
                }
            )
            planned_actions.append(
                {
                    "type": "plan",
                    "issue_number": issue,
                    "action": "retarget_or_rebase_base",
                    "command": None,
                    "rationale": "Base alignment needs operator-confirmed stack maintenance.",
                    "safe": False,
                    "preconditions": ["operator confirms expected stack parent", "working tree is clean"],
                }
            )

    cycle_detected = detect_cycles(nodes)
    if cycle_detected:
        findings.append(
            {
                "severity": "blocking",
                "area": "stack_topology",
                "message": "Dependency cycle detected in stack packet",
                "evidence": {
                    "summary": "A stack cycle prevents a deterministic merge order.",
                    "files": [payload_path.as_posix()],
                    "refs": [],
                },
                "can_auto_fix": False,
            }
        )

    status = "blocked" if any(item["severity"] == "blocking" for item in findings) else (
        "findings" if findings else "clean"
    )
    output = {
        "schema_version": "pr_stack_manager.analysis.v1",
        "status": status,
        "target": payload.get("target", {}),
        "dependency_graph": {
            "root_issue": payload.get("root_issue"),
            "nodes": nodes,
            "edges": edges,
            "cycle_detected": cycle_detected,
        },
        "findings": findings,
        "planned_actions": planned_actions,
        "validation_performed": ["analyze_pr_stack.py"],
        "handoff_state": {
            "ready_for_editor": bool(findings),
            "ready_for_execution": status == "clean",
            "ready_for_follow_on_implementation": bool(planned_actions),
        },
    }

    rendered = json.dumps(output, indent=2, sort_keys=True) + "\n"
    if args.out:
        out = Path(args.out)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(rendered, encoding="utf-8")
    else:
        print(rendered, end="")
    return 0 if status != "blocked" else 1


if __name__ == "__main__":
    raise SystemExit(main())
