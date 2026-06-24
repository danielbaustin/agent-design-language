#!/usr/bin/env python3
from __future__ import annotations

from copy import deepcopy
from datetime import datetime, timezone
import json
from pathlib import Path
import re
from typing import Any


AVAILABILITY_VALUES = {"known", "unknown", "not_collected", "not_applicable", "not_available"}
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
METRICS_CONFIDENCE_VALUES = {"low", "medium", "high", "unknown"}
COMPLETION_STATE_VALUES = {
    "completed",
    "completed_with_follow_on",
    "blocked",
    "failed",
    "deferred",
    "cancelled",
    "unknown",
}
STAGE_PRIORITY = {
    "issue_start": 1,
    "pr_publication": 2,
    "review_handoff": 3,
    "merge_closeout": 4,
    "sprint_closeout": 5,
}

CODEX_GOAL_STATUS_TO_COMPLETION_STATE = {
    "active": "unknown",
    "completed": "completed",
    "complete": "completed",
    "blocked": "blocked",
    "budgetlimited": "deferred",
    "budget_limited": "deferred",
}

TERMINAL_COMPLETION_STATES = {"completed", "blocked", "failed", "cancelled", "deferred"}


def iso_now_utc() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def epoch_seconds_to_iso(value: Any) -> str | None:
    if value is None:
        return None
    if not isinstance(value, int):
        return None
    return datetime.fromtimestamp(value, tz=timezone.utc).replace(microsecond=0).isoformat().replace(
        "+00:00", "Z"
    )


def parse_codex_goal_tool_snapshot(path: str) -> dict[str, Any]:
    payload = json.loads(Path(path).read_text())
    goal = payload.get("goal")
    if not isinstance(goal, dict):
        raise ValueError("goal snapshot does not contain an active goal object")

    created_at_raw = goal.get("createdAt")
    updated_at_raw = goal.get("updatedAt")
    status_raw = str(goal.get("status") or "unknown")
    normalized_status = status_raw.strip().lower()
    completion_state = CODEX_GOAL_STATUS_TO_COMPLETION_STATE.get(normalized_status, "unknown")

    started_at = epoch_seconds_to_iso(created_at_raw)
    completed_at = epoch_seconds_to_iso(updated_at_raw) if completion_state in TERMINAL_COMPLETION_STATES else None
    elapsed_seconds_raw = None
    if (
        isinstance(created_at_raw, int)
        and isinstance(updated_at_raw, int)
        and updated_at_raw >= created_at_raw
    ):
        elapsed_seconds_raw = str(updated_at_raw - created_at_raw)

    tokens_used_raw = goal.get("tokensUsed")
    time_used_raw = goal.get("timeUsedSeconds")

    return {
        "thread_id": goal.get("threadId"),
        "objective": goal.get("objective"),
        "recorded_at": epoch_seconds_to_iso(updated_at_raw) or iso_now_utc(),
        "status": normalized_status,
        "completion_state": completion_state,
        "started_at": started_at,
        "completed_at": completed_at,
        "elapsed_seconds_raw": elapsed_seconds_raw,
        "active_work_seconds_raw": str(time_used_raw) if isinstance(time_used_raw, int) else "unknown",
        "total_tokens_raw": str(tokens_used_raw) if isinstance(tokens_used_raw, int) else "unknown",
    }


def extract_first_json_object(raw: str) -> dict[str, Any] | None:
    text = raw.strip()
    if not text:
        return None
    decoder = json.JSONDecoder()
    for index, char in enumerate(text):
        if char != "{":
            continue
        try:
            value, _ = decoder.raw_decode(text[index:])
        except json.JSONDecodeError:
            continue
        if isinstance(value, dict):
            return value
    return None


def extract_issue_numbers_from_text(raw: str | None) -> set[int]:
    if not isinstance(raw, str):
        return set()
    return {
        int(match.group(1))
        for match in re.finditer(r"(?<!\w)(?:issue\s*)?#(\d+)\b", raw, flags=re.IGNORECASE)
    }


def codex_goal_payload_matches_issue_number(payload: dict[str, Any], issue_number: int | None) -> bool:
    if issue_number is None:
        return True
    goal = payload.get("goal")
    if not isinstance(goal, dict):
        return False
    objective_issue_numbers = extract_issue_numbers_from_text(goal.get("objective"))
    return issue_number in objective_issue_numbers


def find_codex_session_transcript(
    thread_id: str | None,
    session_root: str,
    *,
    issue_number: int | None = None,
) -> str | None:
    root = Path(session_root).expanduser()
    if not root.exists():
        return None
    if thread_id:
        candidates = sorted(root.rglob(f"*{thread_id}*.jsonl"), key=lambda path: path.stat().st_mtime, reverse=True)
    else:
        candidates = sorted(root.rglob("*.jsonl"), key=lambda path: path.stat().st_mtime, reverse=True)
    for candidate in candidates:
        payload = load_codex_goal_payload_from_session_transcript(
            str(candidate),
            thread_id=thread_id,
            issue_number=issue_number,
        )
        if payload is not None:
            return str(candidate)
    return None


def load_codex_goal_payload_from_session_transcript(
    transcript_path: str,
    *,
    thread_id: str | None = None,
    issue_number: int | None = None,
) -> dict[str, Any] | None:
    transcript = Path(transcript_path)
    if not transcript.exists():
        return None

    goal_call_ids: set[str] = set()
    latest_payload: dict[str, Any] | None = None
    for raw_line in transcript.read_text().splitlines():
        if not raw_line.strip():
            continue
        try:
            line = json.loads(raw_line)
        except json.JSONDecodeError:
            continue
        payload = line.get("payload")
        if not isinstance(payload, dict):
            continue
        item = payload.get("item")
        if isinstance(item, dict):
            item_type = item.get("type")
            if item_type == "function_call" and item.get("name") == "get_goal":
                call_id = item.get("call_id")
                if isinstance(call_id, str):
                    goal_call_ids.add(call_id)
                continue
            if item_type == "function_call_output":
                call_id = item.get("call_id")
                if goal_call_ids and call_id not in goal_call_ids:
                    continue
                output_text = item.get("output")
                if not isinstance(output_text, str):
                    output_text = payload.get("output")
                if not isinstance(output_text, str):
                    continue
                candidate = extract_first_json_object(output_text)
                if not isinstance(candidate, dict):
                    continue
                goal = candidate.get("goal")
                if not isinstance(goal, dict):
                    continue
                if thread_id is not None and goal.get("threadId") not in {thread_id, None}:
                    continue
                if not codex_goal_payload_matches_issue_number(candidate, issue_number):
                    continue
                latest_payload = candidate
                continue

        output_text = payload.get("output")
        if not isinstance(output_text, str):
            continue
        candidate = extract_first_json_object(output_text)
        if not isinstance(candidate, dict):
            continue
        goal = candidate.get("goal")
        if not isinstance(goal, dict):
            continue
        if thread_id is not None and goal.get("threadId") not in {thread_id, None}:
            continue
        if not codex_goal_payload_matches_issue_number(candidate, issue_number):
            continue
        latest_payload = candidate
    return latest_payload


def default_goal_metrics_summary() -> dict[str, Any]:
    return {
        "status": "not_recorded",
        "raw_log_path": None,
        "record_count": 0,
        "phases_recorded": [],
        "selected_stage": None,
        "recorded_at": None,
        "data_source": "unknown",
        "metrics_confidence": "unknown",
        "issue_goal_ref": None,
        "sprint_goal_ref": None,
        "goal_metrics_rollup_ref": None,
        "goal_id": None,
        "goal_id_availability": "unknown",
        "started_at": None,
        "completed_at": None,
        "elapsed_seconds": None,
        "elapsed_availability": "unknown",
        "active_work_seconds": None,
        "active_work_availability": "unknown",
        "validation_seconds": None,
        "validation_availability": "unknown",
        "pr_wait_seconds": None,
        "pr_wait_availability": "unknown",
        "ci_wait_seconds": None,
        "ci_wait_availability": "unknown",
        "completion_state": "unknown",
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
    if lowered in AVAILABILITY_VALUES - {"known"}:
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
    issue_goal_ref: str | None,
    sprint_goal_ref: str | None,
    goal_metrics_rollup_ref: str | None,
    goal_id: str | None,
    goal_id_state: str | None,
    started_at: str | None,
    completed_at: str | None,
    elapsed_seconds_raw: str | None,
    active_work_seconds_raw: str | None,
    validation_seconds_raw: str | None,
    pr_wait_seconds_raw: str | None,
    ci_wait_seconds_raw: str | None,
    total_tokens_raw: str | None,
    prompt_tokens_raw: str | None,
    completion_tokens_raw: str | None,
    metrics_confidence: str,
    completion_state: str,
    model_ref: str | None,
    session_ref: str | None,
    thread_id: str | None,
) -> dict[str, Any]:
    if capture_stage not in CAPTURE_STAGES:
        raise ValueError(f"invalid capture stage: {capture_stage}")
    if data_source not in DATA_SOURCE_VALUES:
        raise ValueError(f"invalid data source: {data_source}")
    if metrics_confidence not in METRICS_CONFIDENCE_VALUES:
        raise ValueError(f"invalid metrics confidence: {metrics_confidence}")
    if completion_state not in COMPLETION_STATE_VALUES:
        raise ValueError(f"invalid completion state: {completion_state}")

    goal_id_value, goal_id_availability = normalize_goal_id(goal_id, goal_id_state)
    elapsed_availability, elapsed_seconds = parse_availability_int(elapsed_seconds_raw)
    active_work_availability, active_work_seconds = parse_availability_int(active_work_seconds_raw)
    validation_availability, validation_seconds = parse_availability_int(validation_seconds_raw)
    pr_wait_availability, pr_wait_seconds = parse_availability_int(pr_wait_seconds_raw)
    ci_wait_availability, ci_wait_seconds = parse_availability_int(ci_wait_seconds_raw)
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
        "metrics_confidence": metrics_confidence,
        "raw_log_path": raw_log_path,
        "issue_goal_ref": issue_goal_ref,
        "sprint_goal_ref": sprint_goal_ref,
        "goal_metrics_rollup_ref": goal_metrics_rollup_ref,
        "goal_id": goal_id_value,
        "goal_id_availability": goal_id_availability,
        "started_at": started_at,
        "completed_at": completed_at,
        "elapsed_seconds": elapsed_seconds,
        "elapsed_availability": elapsed_availability,
        "active_work_seconds": active_work_seconds,
        "active_work_availability": active_work_availability,
        "validation_seconds": validation_seconds,
        "validation_availability": validation_availability,
        "pr_wait_seconds": pr_wait_seconds,
        "pr_wait_availability": pr_wait_availability,
        "ci_wait_seconds": ci_wait_seconds,
        "ci_wait_availability": ci_wait_availability,
        "completion_state": completion_state,
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


def build_issue_goal_metrics_record_from_codex_goal_snapshot(
    *,
    issue_number: int,
    capture_stage: str,
    goal_state_path: str,
    raw_log_path: str,
    issue_goal_ref: str | None,
    sprint_goal_ref: str | None,
    goal_metrics_rollup_ref: str | None,
    metrics_confidence: str,
    completion_state_override: str | None,
    model_ref: str | None,
    session_ref: str | None,
) -> dict[str, Any]:
    snapshot = parse_codex_goal_tool_snapshot(goal_state_path)
    completion_state = completion_state_override or snapshot["completion_state"]
    thread_id = snapshot.get("thread_id")
    return build_issue_goal_metrics_record(
        sprint_issue_number=None,
        issue_number=issue_number,
        capture_stage=capture_stage,
        data_source="codex_goal_tool",
        raw_log_path=raw_log_path,
        recorded_at=snapshot.get("recorded_at"),
        issue_goal_ref=issue_goal_ref,
        sprint_goal_ref=sprint_goal_ref,
        goal_metrics_rollup_ref=goal_metrics_rollup_ref,
        goal_id=None,
        goal_id_state="not_available",
        started_at=snapshot.get("started_at"),
        completed_at=snapshot.get("completed_at"),
        elapsed_seconds_raw=snapshot.get("elapsed_seconds_raw"),
        active_work_seconds_raw=snapshot.get("active_work_seconds_raw"),
        validation_seconds_raw="unknown",
        pr_wait_seconds_raw="unknown",
        ci_wait_seconds_raw="unknown",
        total_tokens_raw=snapshot.get("total_tokens_raw"),
        prompt_tokens_raw="unknown",
        completion_tokens_raw="unknown",
        metrics_confidence=metrics_confidence,
        completion_state=completion_state,
        model_ref=model_ref,
        session_ref=session_ref or (f"codex-thread:{thread_id}" if isinstance(thread_id, str) else None),
        thread_id=thread_id if isinstance(thread_id, str) else None,
    )


def build_unknown_issue_goal_metrics_record(
    *,
    issue_number: int,
    capture_stage: str,
    raw_log_path: str,
    issue_goal_ref: str | None,
    sprint_goal_ref: str | None,
    goal_metrics_rollup_ref: str | None,
    metrics_confidence: str,
    completion_state: str,
    model_ref: str | None,
    session_ref: str | None,
    thread_id: str | None,
) -> dict[str, Any]:
    return build_issue_goal_metrics_record(
        sprint_issue_number=None,
        issue_number=issue_number,
        capture_stage=capture_stage,
        data_source="unknown",
        raw_log_path=raw_log_path,
        recorded_at=iso_now_utc(),
        issue_goal_ref=issue_goal_ref,
        sprint_goal_ref=sprint_goal_ref,
        goal_metrics_rollup_ref=goal_metrics_rollup_ref,
        goal_id=None,
        goal_id_state="not_available",
        started_at=None,
        completed_at=None,
        elapsed_seconds_raw="unknown",
        active_work_seconds_raw="unknown",
        validation_seconds_raw="unknown",
        pr_wait_seconds_raw="unknown",
        ci_wait_seconds_raw="unknown",
        total_tokens_raw="unknown",
        prompt_tokens_raw="unknown",
        completion_tokens_raw="unknown",
        metrics_confidence=metrics_confidence,
        completion_state=completion_state,
        model_ref=model_ref,
        session_ref=session_ref,
        thread_id=thread_id,
    )


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
    summary["metrics_confidence"] = selected.get("metrics_confidence") or "unknown"
    summary["issue_goal_ref"] = selected.get("issue_goal_ref")
    summary["sprint_goal_ref"] = selected.get("sprint_goal_ref")
    summary["goal_metrics_rollup_ref"] = selected.get("goal_metrics_rollup_ref")
    summary["goal_id"] = selected.get("goal_id")
    summary["goal_id_availability"] = selected.get("goal_id_availability") or "unknown"
    summary["started_at"] = selected.get("started_at")
    summary["completed_at"] = selected.get("completed_at")
    summary["elapsed_seconds"] = selected.get("elapsed_seconds")
    summary["elapsed_availability"] = selected.get("elapsed_availability") or "unknown"
    summary["active_work_seconds"] = selected.get("active_work_seconds")
    summary["active_work_availability"] = selected.get("active_work_availability") or "unknown"
    summary["validation_seconds"] = selected.get("validation_seconds")
    summary["validation_availability"] = selected.get("validation_availability") or "unknown"
    summary["pr_wait_seconds"] = selected.get("pr_wait_seconds")
    summary["pr_wait_availability"] = selected.get("pr_wait_availability") or "unknown"
    summary["ci_wait_seconds"] = selected.get("ci_wait_seconds")
    summary["ci_wait_availability"] = selected.get("ci_wait_availability") or "unknown"
    summary["completion_state"] = selected.get("completion_state") or "unknown"
    summary["token_usage"] = deepcopy(selected.get("token_usage") or summary["token_usage"])
    summary["model_ref"] = selected.get("model_ref")
    summary["session_ref"] = selected.get("session_ref")
    summary["thread_id"] = selected.get("thread_id")
    return summary


def compute_goal_metrics_rollup(issue_records: list[dict[str, Any]]) -> dict[str, Any]:
    availability_counts = lambda: {availability: 0 for availability in sorted(AVAILABILITY_VALUES)}
    rollup = {
        "issue_count": 0,
        "issues_with_recorded_metrics": 0,
        "issues_without_recorded_metrics": 0,
        "issues_with_known_elapsed": 0,
        "issues_with_unknown_elapsed": 0,
        "issues_with_known_active_work": 0,
        "issues_with_unknown_active_work": 0,
        "issues_with_known_validation_seconds": 0,
        "issues_with_unknown_validation_seconds": 0,
        "issues_with_known_pr_wait": 0,
        "issues_with_unknown_pr_wait": 0,
        "issues_with_known_ci_wait": 0,
        "issues_with_unknown_ci_wait": 0,
        "issues_with_known_total_tokens": 0,
        "issues_with_unknown_total_tokens": 0,
        "total_elapsed_seconds_known_sum": 0,
        "total_active_work_seconds_known_sum": 0,
        "total_validation_seconds_known_sum": 0,
        "total_pr_wait_seconds_known_sum": 0,
        "total_ci_wait_seconds_known_sum": 0,
        "total_tokens_known_sum": 0,
        "data_source_counts": {source: 0 for source in sorted(DATA_SOURCE_VALUES)},
        "goal_id_availability_counts": {availability: 0 for availability in sorted(AVAILABILITY_VALUES)},
        "completion_state_counts": {state: 0 for state in sorted(COMPLETION_STATE_VALUES)},
        "elapsed_availability_counts": availability_counts(),
        "active_work_availability_counts": availability_counts(),
        "validation_availability_counts": availability_counts(),
        "pr_wait_availability_counts": availability_counts(),
        "ci_wait_availability_counts": availability_counts(),
        "total_token_availability_counts": availability_counts(),
    }
    for record in issue_records:
        rollup["issue_count"] += 1
        summary = record.get("goal_metrics") or default_goal_metrics_summary()
        if summary.get("status") == "recorded":
            rollup["issues_with_recorded_metrics"] += 1
        else:
            rollup["issues_without_recorded_metrics"] += 1

        data_source = summary.get("data_source") or "unknown"
        if data_source not in rollup["data_source_counts"]:
            rollup["data_source_counts"][data_source] = 0
        rollup["data_source_counts"][data_source] += 1

        goal_id_availability = summary.get("goal_id_availability") or "unknown"
        if goal_id_availability not in rollup["goal_id_availability_counts"]:
            rollup["goal_id_availability_counts"][goal_id_availability] = 0
        rollup["goal_id_availability_counts"][goal_id_availability] += 1

        completion_state = summary.get("completion_state") or "unknown"
        if completion_state not in rollup["completion_state_counts"]:
            rollup["completion_state_counts"][completion_state] = 0
        rollup["completion_state_counts"][completion_state] += 1

        elapsed_availability = summary.get("elapsed_availability") or "unknown"
        if elapsed_availability not in rollup["elapsed_availability_counts"]:
            rollup["elapsed_availability_counts"][elapsed_availability] = 0
        rollup["elapsed_availability_counts"][elapsed_availability] += 1
        if elapsed_availability == "known" and isinstance(summary.get("elapsed_seconds"), int):
            rollup["issues_with_known_elapsed"] += 1
            rollup["total_elapsed_seconds_known_sum"] += int(summary["elapsed_seconds"])
        elif elapsed_availability == "unknown":
            rollup["issues_with_unknown_elapsed"] += 1

        active_work_availability = summary.get("active_work_availability") or "unknown"
        if active_work_availability not in rollup["active_work_availability_counts"]:
            rollup["active_work_availability_counts"][active_work_availability] = 0
        rollup["active_work_availability_counts"][active_work_availability] += 1
        if active_work_availability == "known" and isinstance(summary.get("active_work_seconds"), int):
            rollup["issues_with_known_active_work"] += 1
            rollup["total_active_work_seconds_known_sum"] += int(summary["active_work_seconds"])
        elif active_work_availability == "unknown":
            rollup["issues_with_unknown_active_work"] += 1

        validation_availability = summary.get("validation_availability") or "unknown"
        if validation_availability not in rollup["validation_availability_counts"]:
            rollup["validation_availability_counts"][validation_availability] = 0
        rollup["validation_availability_counts"][validation_availability] += 1
        if validation_availability == "known" and isinstance(summary.get("validation_seconds"), int):
            rollup["issues_with_known_validation_seconds"] += 1
            rollup["total_validation_seconds_known_sum"] += int(summary["validation_seconds"])
        elif validation_availability == "unknown":
            rollup["issues_with_unknown_validation_seconds"] += 1

        pr_wait_availability = summary.get("pr_wait_availability") or "unknown"
        if pr_wait_availability not in rollup["pr_wait_availability_counts"]:
            rollup["pr_wait_availability_counts"][pr_wait_availability] = 0
        rollup["pr_wait_availability_counts"][pr_wait_availability] += 1
        if pr_wait_availability == "known" and isinstance(summary.get("pr_wait_seconds"), int):
            rollup["issues_with_known_pr_wait"] += 1
            rollup["total_pr_wait_seconds_known_sum"] += int(summary["pr_wait_seconds"])
        elif pr_wait_availability == "unknown":
            rollup["issues_with_unknown_pr_wait"] += 1

        ci_wait_availability = summary.get("ci_wait_availability") or "unknown"
        if ci_wait_availability not in rollup["ci_wait_availability_counts"]:
            rollup["ci_wait_availability_counts"][ci_wait_availability] = 0
        rollup["ci_wait_availability_counts"][ci_wait_availability] += 1
        if ci_wait_availability == "known" and isinstance(summary.get("ci_wait_seconds"), int):
            rollup["issues_with_known_ci_wait"] += 1
            rollup["total_ci_wait_seconds_known_sum"] += int(summary["ci_wait_seconds"])
        elif ci_wait_availability == "unknown":
            rollup["issues_with_unknown_ci_wait"] += 1

        token_usage = summary.get("token_usage") or {}
        total_token_availability = token_usage.get("total_availability") or "unknown"
        if total_token_availability not in rollup["total_token_availability_counts"]:
            rollup["total_token_availability_counts"][total_token_availability] = 0
        rollup["total_token_availability_counts"][total_token_availability] += 1
        if total_token_availability == "known" and isinstance(token_usage.get("total_tokens"), int):
            rollup["issues_with_known_total_tokens"] += 1
            rollup["total_tokens_known_sum"] += int(token_usage["total_tokens"])
        elif total_token_availability == "unknown":
            rollup["issues_with_unknown_total_tokens"] += 1
    return rollup
