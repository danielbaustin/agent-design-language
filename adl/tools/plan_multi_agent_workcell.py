#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

READY_CARD_STATUSES = {"ready", "approved"}
STATUS_ORDER = [
    "ready",
    "serial_only",
    "review_ready",
    "janitor_ready",
    "closeout_ready",
    "blocked",
]


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def paths_overlap(left: str, right: str) -> bool:
    left_clean = left.rstrip("/")
    right_clean = right.rstrip("/")
    return (
        left_clean == right_clean
        or left_clean.startswith(right_clean + "/")
        or right_clean.startswith(left_clean + "/")
    )


def worker_cards_ready(cards: dict[str, str]) -> tuple[bool, list[str]]:
    reasons: list[str] = []
    for card_name in ("sip", "stp", "spp"):
        if cards.get(card_name) not in READY_CARD_STATUSES:
            reasons.append(f"{card_name.upper()} not ready")
    return (not reasons, reasons)


def dependency_blockers(shard: dict[str, Any], shards_by_id: dict[str, dict[str, Any]]) -> list[str]:
    reasons: list[str] = []
    shard_dependency_state = shard.get("dependency_state", "none")
    if shard_dependency_state not in {"none", "satisfied", "independent"}:
        reasons.append(f"dependency state is {shard_dependency_state}")
    for dep_id in shard.get("dependencies", []):
        dep = shards_by_id.get(dep_id)
        if dep is None:
            reasons.append(f"missing dependency {dep_id}")
            continue
        if dep.get("_classification") == "blocked":
            reasons.append(f"dependency {dep_id} is blocked")
            continue
        if dep.get("_base_blocked") is True:
            reasons.append(f"dependency {dep_id} is blocked")
            continue
        dep_state = dep.get("dependency_state", "unknown")
        if dep_state not in {"none", "satisfied", "independent"}:
            reasons.append(f"dependency {dep_id} is {dep_state}")
    return reasons


def worker_base_blockers(shard: dict[str, Any], conflicts: dict[str, list[str]]) -> list[str]:
    shard_id = shard["shard_id"]
    if conflicts.get(shard_id):
        return conflicts[shard_id]
    cards_ok, card_reasons = worker_cards_ready(shard.get("cards", {}))
    if not cards_ok:
        return card_reasons
    if not shard.get("write_paths"):
        return ["worker shard missing write_paths"]
    return []


def collect_conflicts(shards: list[dict[str, Any]]) -> dict[str, list[str]]:
    conflicts: dict[str, list[str]] = {shard["shard_id"]: [] for shard in shards}
    for index, left in enumerate(shards):
        for right in shards[index + 1 :]:
            for left_path in left.get("write_paths", []):
                for right_path in right.get("write_paths", []):
                    if paths_overlap(left_path, right_path):
                        conflicts[left["shard_id"]].append(
                            f"write-set conflict with {right['shard_id']} ({left_path} vs {right_path})"
                        )
                        conflicts[right["shard_id"]].append(
                            f"write-set conflict with {left['shard_id']} ({right_path} vs {left_path})"
                        )
    return conflicts


def classify_shard(
    shard: dict[str, Any],
    shards_by_id: dict[str, dict[str, Any]],
    conflicts: dict[str, list[str]],
) -> tuple[str, list[str]]:
    shard_id = shard["shard_id"]
    role = shard.get("role", "worker")
    reasons: list[str] = []

    if role == "worker":
        base_blockers = worker_base_blockers(shard, conflicts)
        if base_blockers:
            return "blocked", base_blockers
        dep_reasons = dependency_blockers(shard, shards_by_id)
        if dep_reasons:
            return "blocked", dep_reasons
        validation_lane = shard.get("validation_lane", "")
        if validation_lane == "manual" or shard.get("serialized_gate") is True:
            reasons.append("requires serialized validation/review gate")
            return "serial_only", reasons
        return "ready", ["worker shard is assignment-ready"]

    if role == "reviewer":
        if shard.get("review_input_state") == "ready":
            return "review_ready", ["review lane has published shard input"]
        return "blocked", ["review lane missing reviewable input"]

    if role == "janitor":
        if shard.get("pr_blocker_state") == "blocked_pr":
            return "janitor_ready", ["PR blocker present for janitor lane"]
        return "blocked", ["janitor lane has no explicit PR blocker"]

    if role == "closeout":
        if shard.get("closeout_state") == "ready":
            return "closeout_ready", ["closeout lane has merged/closed shard truth to finalize"]
        return "blocked", ["closeout lane not yet ready"]

    return "blocked", [f"unknown role {role}"]


def plan_parallel_workers(results: list[dict[str, Any]]) -> tuple[list[str], list[str]]:
    ready_workers = [item["shard_id"] for item in results if item["classification"] == "ready"]
    serial_workers = [item["shard_id"] for item in results if item["classification"] == "serial_only"]
    return ready_workers, serial_workers


def build_report(data: dict[str, Any]) -> dict[str, Any]:
    shards = data.get("shards", [])
    if not isinstance(shards, list) or not shards:
        raise SystemExit("manifest must contain a non-empty shards list")

    shards_by_id = {shard["shard_id"]: shard for shard in shards}
    conflicts = collect_conflicts(shards)
    for shard in shards:
        if shard.get("role", "worker") == "worker":
            shard["_base_blocked"] = bool(worker_base_blockers(shard, conflicts))
            shard["_classification"] = "blocked" if shard["_base_blocked"] else "unknown"
        else:
            shard["_base_blocked"] = False
            shard["_classification"] = "unknown"

    for _ in range(len(shards) + 1):
        changed = False
        for shard in shards:
            classification, reasons = classify_shard(shard, shards_by_id, conflicts)
            if (
                shard.get("_classification") != classification
                or shard.get("_reasons") != reasons
            ):
                shard["_classification"] = classification
                shard["_reasons"] = reasons
                changed = True
        if not changed:
            break

    results = []
    for shard in shards:
        results.append(
            {
                "shard_id": shard["shard_id"],
                "issue_number": shard.get("issue_number"),
                "role": shard.get("role", "worker"),
                "execution_backend": shard.get("execution_backend"),
                "model_hint": shard.get("model_hint"),
                "classification": shard.get("_classification"),
                "reasons": shard.get("_reasons", []),
                "write_paths": shard.get("write_paths", []),
            }
        )

    ordered_results = sorted(results, key=lambda item: (STATUS_ORDER.index(item["classification"]), item["shard_id"]))
    ready_workers, serial_workers = plan_parallel_workers(ordered_results)
    return {
        "manifest_id": data.get("manifest_id", "unknown-manifest"),
        "safe_parallel_workers": ready_workers,
        "serial_only_workers": serial_workers,
        "results": ordered_results,
        "limitations": [
            "planner does not assign or launch agents automatically",
            "write-set safety depends on declared write_paths, not inferred edits",
            "review/janitor/closeout lanes remain bounded lifecycle states rather than autonomous execution",
        ],
    }


def render_text(report: dict[str, Any]) -> str:
    lines = []
    lines.append(f"Multi-Agent Workcell Assignment Plan: {report['manifest_id']}")
    lines.append("")
    if report["safe_parallel_workers"]:
        lines.append("Safe parallel worker set:")
        for shard_id in report["safe_parallel_workers"]:
            lines.append(f"- {shard_id}")
    else:
        lines.append("Safe parallel worker set: none")
    lines.append("")
    if report["serial_only_workers"]:
        lines.append("Serial-only worker set:")
        for shard_id in report["serial_only_workers"]:
            lines.append(f"- {shard_id}")
        lines.append("")
    lines.append("Shard classifications:")
    for item in report["results"]:
        reason_text = "; ".join(item["reasons"]) if item["reasons"] else "no notes"
        backend_suffix = ""
        if item.get("execution_backend") or item.get("model_hint"):
            backend = item.get("execution_backend") or "unspecified-backend"
            model = item.get("model_hint") or "unspecified-model"
            backend_suffix = f", backend={backend}, model={model}"
        lines.append(
            f"- {item['shard_id']} (issue #{item.get('issue_number')}, role={item['role']}{backend_suffix}): "
            f"{item['classification']} — {reason_text}"
        )
    lines.append("")
    lines.append("Planner limitations:")
    for limitation in report["limitations"]:
        lines.append(f"- {limitation}")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("manifest")
    parser.add_argument("--json-out")
    args = parser.parse_args()

    manifest_path = Path(args.manifest)
    report = build_report(load_json(manifest_path))
    print(render_text(report))
    if args.json_out:
        Path(args.json_out).write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
