#!/usr/bin/env python3
"""Build and optionally run an ADL validation profile for a change set."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SELECTOR = ROOT / "adl/tools/select_validation_lanes.sh"


def fail(message: str) -> None:
    print(f"validation_manager: {message}", file=sys.stderr)
    raise SystemExit(2)


def selector_plan(args: argparse.Namespace) -> dict[str, Any]:
    cmd = ["bash", str(SELECTOR), "--json"]
    if args.changed_files:
        cmd.extend(["--changed-files", str(args.changed_files.resolve())])
    else:
        cmd.extend(["--base", args.base, "--head", args.head])
    if args.include_working_tree:
        cmd.append("--include-working-tree")
    result = subprocess.run(
        cmd,
        cwd=ROOT,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode != 0:
        fail(f"selector failed: {result.stderr.strip()}")
    try:
        plan = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        fail(f"selector returned invalid JSON: {exc}")
    if plan.get("schema_version") != "adl.validation_lane_plan.v1":
        fail("selector returned unsupported schema_version")
    return plan


def profile_id(plan: dict[str, Any]) -> str:
    aggregate = plan.get("aggregate_status", "unknown")
    lanes = sorted(plan.get("lanes", {}).keys())
    if not lanes:
        return "validation_none"
    if aggregate == "selected" and len(lanes) == 1:
        return f"{lanes[0]}_profile"
    return f"{aggregate}_{len(lanes)}_lane_profile"


def lane_behavior_surface(lane_id: str, lane: dict[str, Any]) -> dict[str, Any]:
    matched_paths = lane.get("matched_paths", [])
    if lane_id == "rust_pr_fast":
        mode = lane.get("mode", "unknown")
        filter_tokens = lane.get("filter_tokens", "")
        return {
            "id": f"rust_{mode}_behavior",
            "source": "selector_pr_fast_plan",
            "requirement_ids": [token for token in filter_tokens.split("|") if token],
            "matched_paths": matched_paths,
            "risk_class": "medium" if lane.get("status") == "selected" else "high",
        }
    if lane_id == "release_gate_review":
        return {
            "id": "release_or_ci_policy_boundary",
            "source": "release_gate_hints",
            "requirement_ids": ["release_gate_disposition_required"],
            "matched_paths": matched_paths,
            "risk_class": "high",
        }
    if "docs" in lane_id:
        behavior_id = "documentation_contract"
    elif "prompt" in lane_id:
        behavior_id = "prompt_template_contract"
    elif "ci" in lane_id:
        behavior_id = "ci_validation_policy"
    elif "owner" in lane_id:
        behavior_id = "owner_binary_contract"
    else:
        behavior_id = lane_id.replace("_lane", "").replace("_contracts", "_contract")
    return {
        "id": behavior_id,
        "source": "validation_lane_selector",
        "requirement_ids": [lane_id],
        "matched_paths": matched_paths,
        "risk_class": "low" if lane.get("status") == "selected" else "medium",
    }


def validation_dag_node(lane_id: str, lane: dict[str, Any], behavior_id: str) -> dict[str, Any]:
    status = lane.get("status", "unknown")
    if status == "selected":
        node_status = "runnable"
    elif status in {"escalated", "release_gate_required"}:
        node_status = "blocked_for_escalation"
    else:
        node_status = "not_selected"
    return {
        "id": f"node_{lane_id}",
        "lane_id": lane_id,
        "behavior_surface": behavior_id,
        "status": node_status,
        "proof_role": "release_gate" if status == "release_gate_required" else "regression",
        "command": lane.get("run_command") or lane.get("command", ""),
        "depends_on": [],
    }


def estimate_cost(selected: list[tuple[str, dict[str, Any]]], blocked: list[tuple[str, dict[str, Any]]]) -> dict[str, Any]:
    if blocked:
        runtime_class = "escalated"
    elif not selected:
        runtime_class = "none"
    elif len(selected) == 1:
        lane_id = selected[0][0]
        runtime_class = "tiny" if "docs" in lane_id else "normal"
    else:
        runtime_class = "normal"
    return {
        "runtime_class": runtime_class,
        "selected_lane_count": len(selected),
        "blocked_lane_count": len(blocked),
        "expected_test_scope": "focused_or_family" if not blocked else "requires_human_or_release_gate_decision",
        "token_review_cost": "low" if runtime_class in {"none", "tiny"} else "medium",
    }


def build_profile(plan: dict[str, Any], max_selected_lanes: int) -> dict[str, Any]:
    lanes = plan.get("lanes", {})
    changed_paths = plan.get("changed_paths", [])
    covered_paths = {
        path
        for lane in lanes.values()
        for path in lane.get("matched_paths", [])
    }
    uncovered_paths = [path for path in changed_paths if path not in covered_paths]
    selected = [
        (lane_id, lane)
        for lane_id, lane in lanes.items()
        if lane.get("status") == "selected"
    ]
    blocked = [
        (lane_id, lane)
        for lane_id, lane in lanes.items()
        if lane.get("status") in {"escalated", "release_gate_required"}
    ]

    run = []
    behavior_surfaces = []
    dag_nodes = []
    for lane_id, lane in selected:
        behavior = lane_behavior_surface(lane_id, lane)
        behavior_surfaces.append(behavior)
        dag_nodes.append(validation_dag_node(lane_id, lane, behavior["id"]))
        command = lane.get("run_command") or lane.get("command")
        if command:
            run.append(
                {
                    "lane_id": lane_id,
                    "command": command,
                    "reason": lane.get("reason", "selector_selected_lane"),
                    "matched_paths": lane.get("matched_paths", []),
                }
            )

    not_run = [
        {
            "surface": "full_workspace_nextest",
            "reason": "not selected by validation profile",
        },
        {
            "surface": "cargo_clippy_all_targets",
            "reason": "reserved for broad shared changes or release gates",
        },
        {
            "surface": "slow_proof",
            "reason": "reserved for explicit proof-family selection",
        },
        {
            "surface": "coverage_release_gate",
            "reason": "reserved for coverage or release policy selection",
        },
    ]

    unmapped_change_gap = bool(uncovered_paths)

    escalation_required = (
        bool(blocked)
        or len(selected) > max_selected_lanes
        or unmapped_change_gap
    )
    escalation_reasons = []
    for lane_id, lane in blocked:
        behavior = lane_behavior_surface(lane_id, lane)
        behavior_surfaces.append(behavior)
        dag_nodes.append(validation_dag_node(lane_id, lane, behavior["id"]))
        escalation_reasons.append(
            {
                "lane_id": lane_id,
                "status": lane.get("status"),
                "reason": lane.get("reason", "selector_requires_escalation"),
                "matched_paths": lane.get("matched_paths", []),
            }
        )
    if len(selected) > max_selected_lanes:
        escalation_reasons.append(
            {
                "lane_id": "selected_lane_threshold",
                "status": "escalated",
                "reason": f"selected lane count {len(selected)} exceeds limit {max_selected_lanes}",
                "matched_paths": plan.get("changed_paths", []),
            }
        )
    if unmapped_change_gap:
        escalation_reasons.append(
            {
                "lane_id": "unmapped_change_surface",
                "status": "escalated",
                "reason": "selector left changed paths without validation-lane coverage",
                "matched_paths": uncovered_paths,
            }
        )

    status = "ready_to_run"
    if not changed_paths:
        status = "no_validation_needed"
    elif escalation_required:
        status = "escalation_required"
    elif plan.get("aggregate_status") != "selected":
        status = "not_runnable"

    return {
        "schema_version": "adl.validation_profile.v1",
        "selected_profile": profile_id(plan),
        "status": status,
        "selector_aggregate_status": plan.get("aggregate_status"),
        "pr_publication_sufficient": (
            bool(plan.get("pr_publication_sufficient")) and not unmapped_change_gap
        ),
        "changed_paths": changed_paths,
        "run": run,
        "not_run": not_run,
        "deferred": [],
        "behavior_surfaces": behavior_surfaces,
        "validation_dag": {
            "nodes": dag_nodes,
            "edges": [],
            "compression_note": "profile validates behavior surfaces rather than enumerating every test-bearing module",
        },
        "estimated_cost": estimate_cost(selected, blocked),
        "escalation": {
            "required": escalation_required,
            "reasons": escalation_reasons,
        },
        "selector_plan": plan,
    }


def print_text(profile: dict[str, Any]) -> None:
    print("Validation profile")
    print(f"  selected_profile={profile['selected_profile']}")
    print(f"  status={profile['status']}")
    print(f"  selector_aggregate_status={profile['selector_aggregate_status']}")
    print(f"  pr_publication_sufficient={str(profile['pr_publication_sufficient']).lower()}")
    if profile["run"]:
        print("  run:")
        for item in profile["run"]:
            print(f"    - lane={item['lane_id']} reason={item['reason']}")
            print(f"      command={item['command']}")
    else:
        print("  run: []")
    if profile["escalation"]["required"]:
        print("  escalation:")
        for reason in profile["escalation"]["reasons"]:
            print(
                f"    - lane={reason['lane_id']} status={reason['status']} reason={reason['reason']}"
            )
    if profile["behavior_surfaces"]:
        print("  behavior_surfaces:")
        for behavior in profile["behavior_surfaces"]:
            print(
                f"    - id={behavior['id']} risk={behavior['risk_class']} source={behavior['source']}"
            )
    print(
        "  estimated_cost="
        f"{profile['estimated_cost']['runtime_class']} "
        f"lanes={profile['estimated_cost']['selected_lane_count']}"
    )


def write_report(path: Path, profile: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(profile, indent=2, sort_keys=True) + "\n")


def run_profile(profile: dict[str, Any]) -> int:
    if profile["status"] not in {"ready_to_run", "no_validation_needed"}:
        print_text(profile)
        print("validation_manager: refusing --run for non-runnable profile", file=sys.stderr)
        return 1
    failed = False
    for item in profile["run"]:
        print(f"==> {item['lane_id']}: {item['command']}", file=sys.stderr)
        result = subprocess.run(item["command"], cwd=ROOT, shell=True)
        item["run_status"] = "passed" if result.returncode == 0 else "failed"
        if result.returncode != 0:
            failed = True
    profile["run_status"] = "failed" if failed else "passed"
    return 1 if failed else 0


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group()
    source.add_argument("--changed-files", type=Path)
    source.add_argument("--include-working-tree", action="store_true")
    parser.add_argument("--base", default="origin/main")
    parser.add_argument("--head", default="HEAD")
    parser.add_argument("--max-selected-lanes", type=int, default=8)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--report-out", type=Path)
    parser.add_argument("--run", action="store_true")
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    plan = selector_plan(args)
    profile = build_profile(plan, args.max_selected_lanes)
    exit_code = 0
    if args.run:
        exit_code = run_profile(profile)
    if args.report_out:
        write_report(args.report_out, profile)
    if args.json:
        print(json.dumps(profile, indent=2, sort_keys=True))
    else:
        print_text(profile)
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
