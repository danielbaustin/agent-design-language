#!/usr/bin/env python3
from __future__ import annotations

from copy import deepcopy
from datetime import datetime, timezone
from typing import Any


AVAILABILITY_VALUES = {"known", "unknown", "not_available"}
CAPTURE_STAGES = {
    "issue_start",
    "pr_publication",
    "review_handoff",
    "merge_closeout",
    "sprint_closeout",
}
DATA_SOURCE_VALUES = {
    "codex_goal_tool",
    "manual_entry",
    "derived_sprint_state",
    "unknown",
}
STAGE_PRIORITY = {
    "issue_start": 1,
    "pr_publication": 2,
    "review_handoff": 3,
    "merge_closeout": 4,
    "sprint_closeout": 5,
}


def iso_now_utc() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def default_goal_metrics_summary() -> dict[str, Any]:
    return {
        "status": "not_recorded",
        "raw_log_path": None,
        "record_count": 0,
        "phases_recorded": [],
        "selected_stage": None,
        "recorded_at": None,
        "data_source": "unknown",
        "goal_id": None,
        "goal_id_availability": "unknown",
        "started_at": None,
        "completed_at": None,
        "elapsed_seconds": None,
        "elapsed_availability": "unknown",
        "token_usage": {
            "total_tokens": None,
            "prompt_tokens": None,
            "completion_tokens": None,
            "availability": "unknown",
            "total_availability": "unknown",
            "prompt_availability": "unknown",
            "completion_availability": "unknown",
        },
        "model_ref": None,
        "session_ref": None,
        "thread_id": None,
    }


def parse_availability_int(raw: str | None) -> tuple[str, int | None]:
    if raw is None:
        return "unknown", None
    lowered = raw.strip().lower()
    if lowered in {"unknown", "not_available"}:
        return lowered, None
    return "known", int(lowered)


def normalize_goal_id(goal_id: str | None, goal_id_state: str | None) -> tuple[str | None, str]:
    if goal_id is not None:
        return goal_id, "known"
    if goal_id_state is None:
        return None, "unknown"
    lowered = goal_id_state.strip().lower()
    if lowered not in AVAILABILITY_VALUES:
        raise ValueError(f"invalid goal_id availability: {goal_id_state}")
    return None, lowered


def build_issue_goal_metrics_record(
    *,
    sprint_issue_number: int | None,
    issue_number: int,
    capture_stage: str,
    data_source: str,
    raw_log_path: str,
    recorded_at: str | None,
    goal_id: str | None,
    goal_id_state: str | None,
    started_at: str | None,
    completed_at: str | None,
    elapsed_seconds_raw: str | None,
    total_tokens_raw: str | None,
    prompt_tokens_raw: str | None,
    completion_tokens_raw: str | None,
    model_ref: str | None,
    session_ref: str | None,
    thread_id: str | None,
) -> dict[str, Any]:
    if capture_stage not in CAPTURE_STAGES:
        raise ValueError(f"invalid capture stage: {capture_stage}")
    if data_source not in DATA_SOURCE_VALUES:
        raise ValueError(f"invalid data source: {data_source}")

    goal_id_value, goal_id_availability = normalize_goal_id(goal_id, goal_id_state)
    elapsed_availability, elapsed_seconds = parse_availability_int(elapsed_seconds_raw)
    total_availability, total_tokens = parse_availability_int(total_tokens_raw)
    prompt_availability, prompt_tokens = parse_availability_int(prompt_tokens_raw)
    completion_availability, completion_tokens = parse_availability_int(completion_tokens_raw)
    token_availability = (
        "known"
        if any(value == "known" for value in (total_availability, prompt_availability, completion_availability))
        else "not_available"
        if all(value == "not_available" for value in (total_availability, prompt_availability, completion_availability))
        else "unknown"
    )

    return {
        "schema_version": "issue_goal_metrics.v1",
        "recorded_at": recorded_at or iso_now_utc(),
        "sprint_issue_number": sprint_issue_number,
        "issue_number": issue_number,
        "capture_stage": capture_stage,
        "data_source": data_source,
        "raw_log_path": raw_log_path,
        "goal_id": goal_id_value,
        "goal_id_availability": goal_id_availability,
        "started_at": started_at,
        "completed_at": completed_at,
        "elapsed_seconds": elapsed_seconds,
        "elapsed_availability": elapsed_availability,
        "token_usage": {
            "total_tokens": total_tokens,
            "prompt_tokens": prompt_tokens,
            "completion_tokens": completion_tokens,
            "availability": token_availability,
            "total_availability": total_availability,
            "prompt_availability": prompt_availability,
            "completion_availability": completion_availability,
        },
        "model_ref": model_ref,
        "session_ref": session_ref,
        "thread_id": thread_id,
    }


def summarize_issue_goal_metrics(records: list[dict[str, Any]], raw_log_path: str | None) -> dict[str, Any]:
    if not records:
        summary = default_goal_metrics_summary()
        summary["raw_log_path"] = raw_log_path
        return summary

    def sort_key(record: dict[str, Any]) -> tuple[int, str]:
        return (
            STAGE_PRIORITY.get(record.get("capture_stage"), 0),
            record.get("recorded_at") or "",
        )

    selected = sorted(records, key=sort_key)[-1]
    summary = default_goal_metrics_summary()
    summary["status"] = "recorded"
    summary["raw_log_path"] = raw_log_path
    summary["record_count"] = len(records)
    summary["phases_recorded"] = sorted(
        {record.get("capture_stage") for record in records if record.get("capture_stage")}
    )
    summary["selected_stage"] = selected.get("capture_stage")
    summary["recorded_at"] = selected.get("recorded_at")
    summary["data_source"] = selected.get("data_source") or "unknown"
    summary["goal_id"] = selected.get("goal_id")
    summary["goal_id_availability"] = selected.get("goal_id_availability") or "unknown"
    summary["started_at"] = selected.get("started_at")
    summary["completed_at"] = selected.get("completed_at")
    summary["elapsed_seconds"] = selected.get("elapsed_seconds")
    summary["elapsed_availability"] = selected.get("elapsed_availability") or "unknown"
    summary["token_usage"] = deepcopy(selected.get("token_usage") or summary["token_usage"])
    summary["model_ref"] = selected.get("model_ref")
    summary["session_ref"] = selected.get("session_ref")
    summary["thread_id"] = selected.get("thread_id")
    return summary


def compute_goal_metrics_rollup(issue_records: list[dict[str, Any]]) -> dict[str, Any]:
    rollup = {
        "issue_count": 0,
        "issues_with_recorded_metrics": 0,
        "issues_without_recorded_metrics": 0,
        "issues_with_known_elapsed": 0,
        "issues_with_unknown_elapsed": 0,
        "issues_with_known_total_tokens": 0,
        "issues_with_unknown_total_tokens": 0,
        "total_elapsed_seconds_known_sum": 0,
        "total_tokens_known_sum": 0,
    }
    for record in issue_records:
        rollup["issue_count"] += 1
        summary = record.get("goal_metrics") or default_goal_metrics_summary()
        if summary.get("status") == "recorded":
            rollup["issues_with_recorded_metrics"] += 1
        else:
            rollup["issues_without_recorded_metrics"] += 1

        if summary.get("elapsed_availability") == "known" and isinstance(summary.get("elapsed_seconds"), int):
            rollup["issues_with_known_elapsed"] += 1
            rollup["total_elapsed_seconds_known_sum"] += int(summary["elapsed_seconds"])
        else:
            rollup["issues_with_unknown_elapsed"] += 1

        token_usage = summary.get("token_usage") or {}
        if token_usage.get("total_availability") == "known" and isinstance(token_usage.get("total_tokens"), int):
            rollup["issues_with_known_total_tokens"] += 1
            rollup["total_tokens_known_sum"] += int(token_usage["total_tokens"])
        else:
            rollup["issues_with_unknown_total_tokens"] += 1
    return rollup
